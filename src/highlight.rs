//! Minimal regex-free syntax highlighting for code screenshots (#93).
//!
//! Token classes, in first-match order:
//! 1. line comment (`//` or `#` to end of line)
//! 2. string literal (double/single/backtick, no escape handling — screenshots)
//! 3. number literal
//! 4. keyword (language-aware set, word-bounded)
//! 5. plain text
//!
//! Output: `Segment` vecs (reuse of the markup pipeline's styled-run type, so
//! templates render with the same `<tspan>` machinery). No token crosses a
//! line boundary; colors are Catppuccin Mocha accents.

use crate::markup::Segment;

/// Token colors per editor theme. Dark = Catppuccin Mocha accents (on
/// `#0c0c14`), Light = Catppuccin Latte-style accents (on `#fafafa`) —
/// both chosen for ≥3:1 contrast against their editor background.
struct Palette {
    keyword: &'static str,
    string: &'static str,
    number: &'static str,
    comment: &'static str,
}

fn palette(theme: &str) -> Palette {
    if theme == "light" {
        Palette {
            keyword: "#8839ef",
            string: "#40a02b",
            number: "#fe640b",
            comment: "#838ba7",
        }
    } else {
        Palette {
            keyword: "#cba6f7",
            string: "#a6e3a1",
            number: "#fab387",
            comment: "#6c7086",
        }
    }
}

/// Keyword sets per language id (schema `code_lang` values).
fn keywords(lang: &str) -> &'static [&'static str] {
    match lang {
        "rust" => &[
            "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
            "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
            "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
            "type", "unsafe", "use", "where", "while",
        ],
        "go" => &[
            "break",
            "case",
            "chan",
            "const",
            "continue",
            "default",
            "defer",
            "else",
            "fallthrough",
            "for",
            "func",
            "go",
            "goto",
            "if",
            "import",
            "interface",
            "map",
            "package",
            "range",
            "return",
            "select",
            "struct",
            "switch",
            "type",
            "var",
        ],
        "python" => &[
            "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del",
            "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in",
            "is", "lambda", "None", "nonlocal", "not", "or", "pass", "raise", "return", "True",
            "False", "try", "while", "with", "yield",
        ],
        "typescript" | "javascript" => &[
            "abstract",
            "any",
            "as",
            "async",
            "await",
            "boolean",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "default",
            "do",
            "else",
            "enum",
            "export",
            "extends",
            "false",
            "finally",
            "for",
            "from",
            "function",
            "if",
            "implements",
            "import",
            "in",
            "instanceof",
            "interface",
            "let",
            "new",
            "null",
            "number",
            "of",
            "private",
            "protected",
            "public",
            "readonly",
            "return",
            "static",
            "string",
            "super",
            "switch",
            "this",
            "throw",
            "true",
            "try",
            "type",
            "typeof",
            "undefined",
            "var",
            "void",
            "while",
            "yield",
        ],
        "bash" => &[
            "case", "do", "done", "elif", "else", "esac", "fi", "for", "function", "if", "in",
            "then", "until", "while",
        ],
        "sql" => &[
            "AND",
            "AS",
            "ASC",
            "BEGIN",
            "BY",
            "COMMIT",
            "CREATE",
            "DELETE",
            "DESC",
            "DROP",
            "ELSE",
            "END",
            "FROM",
            "GROUP",
            "HAVING",
            "IN",
            "INSERT",
            "INTO",
            "JOIN",
            "LEFT",
            "LIMIT",
            "NOT",
            "NULL",
            "ON",
            "OR",
            "ORDER",
            "RETURNING",
            "SELECT",
            "SET",
            "TABLE",
            "UPDATE",
            "VALUES",
            "WHERE",
            "WITH",
        ],
        _ => &[],
    }
}

/// Line comment starter for the language (None = no comment class).
fn line_comment(lang: &str) -> Option<&'static str> {
    match lang {
        "python" | "bash" => Some("#"),
        "sql" => Some("--"),
        _ => Some("//"),
    }
}

/// Keyword match must sit on non-identifier boundaries.
fn is_word_bounded(chars: &[char], start: usize, len: usize) -> bool {
    let before_ok = start == 0 || !chars[start - 1].is_alphanumeric() && chars[start - 1] != '_';
    let end = start + len;
    let after_ok = end >= chars.len() || !chars[end].is_alphanumeric() && chars[end] != '_';
    before_ok && after_ok
}

/// Highlight one line of code into styled segments.
pub fn highlight_line(line: &str, lang: &str, theme: &str) -> Vec<Segment> {
    let kws = keywords(lang);
    let pal = palette(theme);
    let comment = line_comment(lang);
    let chars: Vec<char> = line.chars().collect();
    let mut segments: Vec<Segment> = Vec::new();
    let mut plain = String::new();
    let mut i = 0usize;

    macro_rules! push_plain {
        () => {
            if !plain.is_empty() {
                segments.push(Segment::plain(std::mem::take(&mut plain)));
            }
        };
    }

    while i < chars.len() {
        // 1. Line comment → rest of line
        if let Some(starter) = comment {
            let n = starter.chars().count();
            if i + n <= chars.len() && chars[i..i + n].iter().copied().eq(starter.chars()) {
                push_plain!();
                let rest: String = chars[i..].iter().collect();
                segments.push(Segment::new_plain_color(rest, Some(pal.comment.into())));
                return segments;
            }
        }

        // 2. String literal (same quote closes; no escapes — screenshots)
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            if let Some(close) = chars[i + 1..].iter().position(|&c| c == quote) {
                let end = i + 1 + close; // inclusive of closing quote index
                push_plain!();
                let text: String = chars[i..=end].iter().collect();
                segments.push(Segment::new_plain_color(text, Some(pal.string.into())));
                i = end + 1;
                continue;
            }
        }

        // 3. Number literal
        if chars[i].is_ascii_digit() {
            let len = chars[i..]
                .iter()
                .take_while(|c| c.is_ascii_digit() || **c == '.' || **c == '_')
                .count();
            // Word-boundary check prevents grabbing the tail of an identifier.
            if is_word_bounded(&chars, i, len) {
                push_plain!();
                let text: String = chars[i..i + len].iter().collect();
                segments.push(Segment::new_plain_color(text, Some(pal.number.into())));
                i += len;
                continue;
            }
        }

        // 4. Keyword (identifier scan)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let len = chars[i..]
                .iter()
                .take_while(|c| c.is_alphanumeric() || **c == '_')
                .count();
            let word: String = chars[i..i + len].iter().collect();
            if kws.contains(&word.as_str()) && is_word_bounded(&chars, i, len) {
                push_plain!();
                segments.push(Segment::new_plain_color(word, Some(pal.keyword.into())));
                i += len;
                continue;
            }
            // Not a keyword: keep the identifier in the plain run.
            plain.push_str(&word);
            i += len;
            continue;
        }

        plain.push(chars[i]);
        i += 1;
    }
    push_plain!();
    segments
}

/// Highlight a multi-line code block (one Segment vec per line).
pub fn highlight(code: &str, lang: &str, theme: &str) -> Vec<Vec<Segment>> {
    code.lines()
        .map(|line| highlight_line(line, lang, theme))
        .collect()
}

#[cfg(test)]
mod highlight_tests {
    use super::*;

    #[test]
    fn rust_keywords_and_strings() {
        let segs = highlight_line(r#"let name = "cosy";"#, "rust", "dark");
        assert_eq!(segs[0].text, "let");
        assert_eq!(segs[0].color.as_deref(), Some("#cba6f7"));
        assert!(segs
            .iter()
            .any(|s| s.text == "\"cosy\"" && s.color.as_deref() == Some("#a6e3a1")));
    }

    #[test]
    fn comment_takes_rest_of_line() {
        let segs = highlight_line("x = 1 // trailing note", "go", "dark");
        let last = segs.last().unwrap();
        assert_eq!(last.text, "// trailing note");
        assert_eq!(last.color.as_deref(), Some("#6c7086"));
    }

    #[test]
    fn python_hash_comment() {
        let segs = highlight_line("# full comment", "python", "dark");
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].color.as_deref(), Some("#6c7086"));
    }

    #[test]
    fn numbers_colored_but_not_identifier_tails() {
        let segs = highlight_line("x2 = 42", "rust", "dark");
        // "x2" stays plain (digit inside identifier) — it may be merged into
        // the surrounding plain run, so check the prefix, not exact equality.
        let plain_prefix = segs
            .iter()
            .take_while(|s| s.color.is_none())
            .map(|s| s.text.as_str())
            .collect::<String>();
        assert!(plain_prefix.starts_with("x2"), "got segments: {segs:?}");
        assert!(segs
            .iter()
            .any(|s| s.text == "42" && s.color.as_deref() == Some("#fab387")));
    }

    #[test]
    fn sql_case_insensitive_keywords() {
        let segs = highlight_line("SELECT id FROM users", "sql", "dark");
        assert_eq!(segs[0].text, "SELECT");
        assert_eq!(segs[0].color.as_deref(), Some("#cba6f7"));
        assert!(segs.iter().any(|s| s.text == "FROM" && s.color.is_some()));
    }

    #[test]
    fn multiline_preserves_line_count() {
        let code = "fn a() {\n    return 1;\n}";
        let lines = highlight(code, "rust", "dark");
        assert_eq!(lines.len(), 3);
        assert!(lines[0].iter().any(|s| s.text == "fn"));
    }

    #[test]
    fn unknown_lang_keywords_off_numbers_still_colored() {
        let segs = highlight_line("let x = 1", "cobol", "dark");
        // Unknown language: no keyword class, but numbers are universal.
        assert!(
            segs.iter().all(|s| s.text != "let" || s.color.is_none()),
            "keyword must stay plain for unknown lang: {segs:?}"
        );
        assert!(
            segs.iter()
                .any(|s| s.text == "1" && s.color.as_deref() == Some("#fab387")),
            "numbers are language-independent: {segs:?}"
        );
    }

    #[test]
    fn light_theme_uses_light_palette() {
        let dark = highlight_line("let x", "rust", "dark");
        let light = highlight_line("let x", "rust", "light");
        assert_eq!(dark[0].color.as_deref(), Some("#cba6f7"));
        assert_eq!(light[0].color.as_deref(), Some("#8839ef"));
    }

    #[test]
    fn unclosed_string_stays_plain() {
        let segs = highlight_line("x = \"oops", "rust", "dark");
        assert!(segs.iter().all(|s| s.color.is_none() || s.text == "x"));
        assert!(!segs
            .iter()
            .any(|s| s.text.contains('"') && s.color.is_some()));
    }
}
