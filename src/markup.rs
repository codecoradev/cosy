//! Inline text markup: `*bold*`, `_italic_`, `*color:#hex*` ... `*color*`.
//!
//! Markup is opt-in per field: a schema field with `"options": ["markup"]` gets a
//! `<field>_segments` context variable (array of lines, each an array of styled
//! segments) whenever the value contains marker characters. Text without markers
//! keeps the legacy `wrap_text` path byte-for-byte, so existing renders are
//! unchanged.
//!
//! Grammar (toggle-style, like chat apps):
//! - `*text*`   → bold while the toggle is active
//! - `_text_`   → italic while the toggle is active
//! - `*color:#rrggbb*` → color following text until `*color*` resets it
//! - `\*`, `\_`, `\\` → literal escapes
//!
//! Toggle rules (keeps stray characters and identifiers intact):
//! - A marker toggles when it closes an active toggle, OR a same-kind marker
//!   appears later AND the marker sits on a word boundary (not surrounded by
//!   alphanumerics on both sides). So `snake_case_name` and `5 * 3 = 15` render
//!   literally; `ini _miring_ ya` renders the italic run.
//! - Invalid color tokens (not `#rgb`/`#rrggbb`) render literally.

/// One styled run of text. `color` is `None` (inherit) or a `#rgb`/`#rrggbb` hex.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub color: Option<String>,
}

impl Segment {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            italic: false,
            color: None,
        }
    }
}

/// Quick gate: does the text contain any marker character at all?
pub fn has_markup(text: &str) -> bool {
    text.contains('*') || text.contains('_')
}

fn valid_color(s: &str) -> bool {
    let body = s.strip_prefix('#').unwrap_or("");
    (body.len() == 3 || body.len() == 6) && body.chars().all(|c| c.is_ascii_hexdigit())
}

/// Set-token prefix: `*color:` (the leading `*` is at the scan index).
const COLOR_SET: &str = "color:";
/// Reset-token body: `color` — valid only when followed by the closing `*`.
const COLOR_RESET: &str = "color";

/// Does `chars[pos..]` start with the characters of `prefix`?
fn matches_at(chars: &[char], pos: usize, prefix: &str) -> bool {
    let n = prefix.chars().count();
    pos + n <= chars.len() && chars[pos..pos + n].iter().copied().eq(prefix.chars())
}

/// A marker toggles only on a word boundary: not sandwiched between two
/// alphanumerics (start/end of string count as boundaries).
fn on_word_boundary(chars: &[char], i: usize) -> bool {
    let prev_alnum = i > 0 && chars[i - 1].is_alphanumeric();
    let next_alnum = i + 1 < chars.len() && chars[i + 1].is_alphanumeric();
    !(prev_alnum && next_alnum)
}

/// Parse markup into styled segments. Text without active markup comes back as a
/// single unstyled segment.
pub fn parse(text: &str) -> Vec<Segment> {
    if !has_markup(text) {
        return vec![Segment::plain(text)];
    }

    let chars: Vec<char> = text.chars().collect();
    let closes_later = |from: usize, marker: char| chars[from..].contains(&marker);

    let mut segs: Vec<Segment> = Vec::new();
    let mut cur = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut color: Option<String> = None;

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '\\' if i + 1 < chars.len() && matches!(chars[i + 1], '*' | '_' | '\\') => {
                cur.push(chars[i + 1]);
                i += 2;
            }
            '*' => {
                // color token: *color:#hex* (set) or *color* (reset).
                // Scanned in char space — byte offsets would break on multibyte input.
                if matches_at(&chars, i + 1, COLOR_SET) {
                    let value_start = i + 1 + COLOR_SET.chars().count();
                    if let Some(rel) = chars[value_start..].iter().position(|&c| c == '*') {
                        let val: String = chars[value_start..value_start + rel].iter().collect();
                        if valid_color(&val) {
                            flush(&mut segs, &mut cur, bold, italic, &color);
                            color = Some(val);
                            i = value_start + rel + 1; // consume the whole token incl. closing '*'
                            continue;
                        }
                        // Invalid color value: render the whole token literally.
                        cur.extend(chars[i..value_start + rel + 1].iter());
                        i = value_start + rel + 1;
                        continue;
                    }
                    // No closing '*': fall through, the '*' renders literally.
                } else if color.is_some()
                    && matches_at(&chars, i + 1, COLOR_RESET)
                    && chars.get(i + 6) == Some(&'*')
                {
                    flush(&mut segs, &mut cur, bold, italic, &color);
                    color = None;
                    i += 7; // consume "*color*"
                    continue;
                }
                if bold || (closes_later(i + 1, '*') && on_word_boundary(&chars, i)) {
                    flush(&mut segs, &mut cur, bold, italic, &color);
                    bold = !bold;
                } else {
                    cur.push('*');
                }
                i += 1;
            }
            '_' => {
                if italic || (closes_later(i + 1, '_') && on_word_boundary(&chars, i)) {
                    flush(&mut segs, &mut cur, bold, italic, &color);
                    italic = !italic;
                } else {
                    cur.push('_');
                }
                i += 1;
            }
            _ => {
                cur.push(c);
                i += 1;
            }
        }
    }
    flush(&mut segs, &mut cur, bold, italic, &color);
    segs
}

fn flush(
    segs: &mut Vec<Segment>,
    cur: &mut String,
    bold: bool,
    italic: bool,
    color: &Option<String>,
) {
    if !cur.is_empty() {
        segs.push(Segment {
            text: std::mem::take(cur),
            bold,
            italic,
            color: color.clone(),
        });
    }
}

/// Plain text with markers stripped — for width counting.
pub fn strip_markup(text: &str) -> String {
    parse(text).into_iter().map(|s| s.text).collect()
}

/// A styled chunk smaller than a word: word-internal styling splits a word into
/// consecutive chunks.
#[derive(Debug, Clone)]
struct Chunk {
    text: String,
    bold: bool,
    italic: bool,
    color: Option<String>,
}

/// Wrap marked text into lines of styled segments. Line-break decisions use the
/// plain (marker-free) text; greedy char-count wrap with hard breaks for
/// overlong words. Marker chars never count toward line width. Spaces between
/// words are emitted as unstyled segments when lines are assembled, so they never
/// land at a line start or end.
pub fn wrap_segments(text: &str, max_chars: usize) -> Vec<Vec<Segment>> {
    // A zero/negative width would never advance the scan (empty lines forever).
    let max_chars = max_chars.max(1);
    let segments = parse(text);

    // 1. Tokenize into words (Vec of styled chunks, no spaces inside).
    //    A word closes at a space, even across segment boundaries (e.g. the
    //    space after a bold run ends the word but a later segment continues
    //    the same word when there is no space between them).
    let mut words: Vec<Vec<Chunk>> = Vec::new();
    let mut cur: Vec<Chunk> = Vec::new();
    for seg in &segments {
        let mut buf = String::new();
        for ch in seg.text.chars() {
            if ch == ' ' || ch == '\n' {
                if !buf.is_empty() {
                    cur.push(Chunk {
                        text: std::mem::take(&mut buf),
                        bold: seg.bold,
                        italic: seg.italic,
                        color: seg.color.clone(),
                    });
                }
                if !cur.is_empty() {
                    words.push(std::mem::take(&mut cur));
                }
            } else {
                buf.push(ch);
            }
        }
        if !buf.is_empty() {
            cur.push(Chunk {
                text: buf,
                bold: seg.bold,
                italic: seg.italic,
                color: seg.color.clone(),
            });
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }

    // 2. Greedy wrap by plain char count (chunk text is marker-free).
    let word_len = |w: &[Chunk]| -> usize { w.iter().map(|c| c.text.chars().count()).sum() };
    let mut lines: Vec<Vec<Vec<Chunk>>> = Vec::new();
    let mut cur_line: Vec<Vec<Chunk>> = Vec::new();
    let mut cur_len = 0usize;
    for w in words {
        let wl = word_len(&w);
        let sep = usize::from(!cur_line.is_empty());
        if sep == 1 && cur_len + sep + wl > max_chars {
            lines.push(std::mem::take(&mut cur_line));
            cur_len = 0;
        }
        if cur_line.is_empty() && wl > max_chars {
            // Hard-break an overlong word across lines.
            let mut take: Vec<Chunk> = Vec::new();
            let mut taken = 0usize;
            for chunk in w {
                let chars: Vec<char> = chunk.text.chars().collect();
                let mut pos = 0usize;
                while pos < chars.len() {
                    if taken == max_chars {
                        let done = std::mem::take(&mut take);
                        lines.push(vec![done]);
                        taken = 0;
                    }
                    let end = (pos + (max_chars - taken)).min(chars.len());
                    take.push(Chunk {
                        text: chars[pos..end].iter().collect(),
                        bold: chunk.bold,
                        italic: chunk.italic,
                        color: chunk.color.clone(),
                    });
                    taken += end - pos;
                    pos = end;
                }
            }
            if !take.is_empty() {
                cur_len = word_len(&take);
                cur_line.push(take);
            }
            continue;
        }
        cur_len += sep + wl;
        cur_line.push(w);
    }
    if !cur_line.is_empty() {
        lines.push(cur_line);
    }

    // 3. Merge chunks per line; inter-word spaces become unstyled segments.
    lines
        .into_iter()
        .map(|line_words| {
            let mut segs: Vec<Segment> = Vec::new();
            for (wi, word) in line_words.into_iter().enumerate() {
                if wi > 0 {
                    segs.push(Segment::plain(" "));
                }
                for chunk in word {
                    match segs.last_mut() {
                        Some(last)
                            if last.bold == chunk.bold
                                && last.italic == chunk.italic
                                && last.color == chunk.color =>
                        {
                            last.text.push_str(&chunk.text)
                        }
                        _ => segs.push(Segment {
                            text: chunk.text,
                            bold: chunk.bold,
                            italic: chunk.italic,
                            color: chunk.color,
                        }),
                    }
                }
            }
            segs
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(segs: &[Segment]) -> Vec<&str> {
        segs.iter().map(|s| s.text.as_str()).collect()
    }

    #[test]
    fn plain_text_single_segment() {
        let s = parse("no markers here");
        assert_eq!(s.len(), 1);
        assert!(!s[0].bold && !s[0].italic && s[0].color.is_none());
    }

    #[test]
    fn bold_toggle() {
        let s = parse("paling *total* sama");
        assert_eq!(texts(&s), vec!["paling ", "total", " sama"]);
        assert!(!s[0].bold);
        assert!(s[1].bold);
        assert!(!s[2].bold);
    }

    #[test]
    fn italic_toggle() {
        let s = parse("ini _miring_ ya");
        assert_eq!(texts(&s), vec!["ini ", "miring", " ya"]);
        assert!(s[1].italic && !s[0].italic);
    }

    #[test]
    fn color_token_set_and_reset() {
        let s = parse("a *color:#b45309*merah*color* b");
        assert_eq!(texts(&s), vec!["a ", "merah", " b"]);
        assert_eq!(s[1].color.as_deref(), Some("#b45309"));
        assert!(s[0].color.is_none() && s[2].color.is_none());
    }

    #[test]
    fn invalid_color_stays_literal() {
        let s = parse("*color:notahex* x");
        assert_eq!(texts(&s), vec!["*color:notahex* x"]);
    }

    #[test]
    fn lone_marker_is_literal() {
        let s = parse("5 * 3 = 15");
        assert_eq!(texts(&s), vec!["5 * 3 = 15"]);
    }

    #[test]
    fn snake_case_stays_literal() {
        let s = parse("snake_case_name");
        assert_eq!(texts(&s), vec!["snake_case_name"]);
        assert!(!s[0].italic);
    }

    #[test]
    fn escapes_render_literal() {
        let s = parse("literal \\*star\\* dan \\_under\\_");
        assert_eq!(texts(&s), vec!["literal *star* dan _under_"]);
    }

    #[test]
    fn strip_matches_plain_concat() {
        let t = "a *b* _c_ *color:#112233*d*color*";
        assert_eq!(strip_markup(t), "a b c d");
    }

    #[test]
    fn wrap_splits_by_plain_width() {
        let lines = wrap_segments("aaa *bbb* ccc", 7);
        assert_eq!(lines.len(), 2);
        let l0: String = lines[0].iter().map(|s| s.text.as_str()).collect();
        assert_eq!(l0, "aaa bbb");
        assert_eq!(lines[1][0].text, "ccc");
        assert!(lines[0].iter().any(|s| s.bold));
        assert!(lines[0][0].text.starts_with("aaa"));
    }

    #[test]
    fn wrap_hard_breaks_overlong_word() {
        let lines = wrap_segments("*bold*abcdefghij", 7);
        let l0: String = lines[0].iter().map(|s| s.text.as_str()).collect();
        assert_eq!(l0, "boldabc");
        assert!(lines[0][0].bold);
        let l1: String = lines[1].iter().map(|s| s.text.as_str()).collect();
        assert_eq!(l1, "defghij");
    }

    #[test]
    fn wrap_preserves_styles_across_lines() {
        let lines = wrap_segments("*color:#ff0000*merah merah merah*color*", 6);
        assert_eq!(lines.len(), 3);
        for line in &lines {
            assert_eq!(line[0].color.as_deref(), Some("#ff0000"));
        }
    }

    #[test]
    fn wrap_mixed_bold_italic_and_color() {
        let lines = wrap_segments("*bold* dan _miring_ dan *color:#123456*biru*color*", 9);
        assert!(!lines.is_empty());
        for line in &lines {
            let joined: usize = line.iter().map(|s| s.text.chars().count()).sum();
            assert!(joined <= 9);
        }
        assert!(lines.iter().any(|l| l.iter().any(|s| s.bold)));
        assert!(lines.iter().any(|l| l.iter().any(|s| s.italic)));
        assert!(lines
            .iter()
            .any(|l| l.iter().any(|s| s.color.as_deref() == Some("#123456"))));
    }

    #[test]
    fn invalid_color_with_multibyte_is_safe_and_literal() {
        // Regression: byte-offset vs char-offset mismatch used to panic here.
        // The literal token carries no style, so the following plain text
        // merges into the same segment.
        let s = parse("*color:éééééééé* x");
        assert_eq!(texts(&s), vec!["*color:éééééééé* x"]);
        assert!(!s[0].bold && !s[0].italic && s[0].color.is_none());
    }

    #[test]
    fn wrap_zero_width_does_not_hang() {
        // Regression: max_chars == 0 used to push empty lines forever.
        let lines = wrap_segments("a b c", 0);
        assert!(!lines.is_empty());
        let joined: String = lines
            .iter()
            .flat_map(|l| l.iter().map(|s| s.text.as_str()))
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(joined.replace(' ', ""), "abc");
    }

    #[test]
    fn color_reset_without_tint_is_bold_toggle() {
        // `*color*` with no active tint is not a reset — it degrades to the
        // regular bold-toggle rule. With no closing marker on a boundary
        // later... there IS one ('*color*' itself), so: '*' toggles bold,
        // 'color' is content, final '*' closes. Chat-app semantics.
        let s = parse("*color* x");
        assert_eq!(texts(&s), vec!["color", " x"]);
        assert!(s[0].color.is_none() && s[0].bold);
    }

    #[test]
    fn reset_requires_closing_star() {
        // Regression: `*colorful` with an active tint used to be consumed as
        // a reset token, swallowing the 'f' and losing user content. Now the
        // reset only fires on a real closing '*' and the stray '*' renders
        // literally — no character loss.
        let s = parse("*color:#f00*hi *colorful world");
        let joined: String = s.iter().map(|x| x.text.as_str()).collect();
        assert_eq!(joined, "hi *colorful world");
        // The tint set at the start stays active through the literal '*'.
        assert!(s.iter().take(2).any(|x| x.color.as_deref() == Some("#f00")));
        let after = s.last().unwrap();
        assert_eq!(after.color.as_deref(), Some("#f00"));
        assert!(after.text.ends_with("world"));
    }

    #[test]
    fn wrap_no_leading_trailing_spaces() {
        let lines = wrap_segments("aa bb cc", 5);
        for line in &lines {
            assert!(!line[0].text.starts_with(' '));
            assert!(!line.last().unwrap().text.ends_with(' '));
        }
    }
}
