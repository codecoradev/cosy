//! Template schema definitions.
//! A template defines: dimensions, brand fields (global), slide fields (per-slide).

use serde::{Deserialize, Serialize};

/// Complete template definition loaded from `schema.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDef {
    pub id: String,
    pub name: String,
    pub dimensions: Dimensions,
    #[serde(default)]
    pub fonts: Vec<String>,
    #[serde(default)]
    pub brand_fields: Fields,
    #[serde(default)]
    pub slide_fields: Fields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// Field definitions — a map of field name to field spec.
pub type Fields = std::collections::BTreeMap<String, FieldSpec>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub required: bool,
    /// Maximum character count (used for validation).
    #[serde(default)]
    pub max: Option<usize>,
    /// Character width for visual text wrapping (used for line breaking in SVG).
    /// If not set, defaults to `max`.
    #[serde(default)]
    pub wrap_width: Option<usize>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Image,
    /// Background: solid, gradient, or image
    Bg,
}

/// Input data — what the user fills in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputData {
    /// Global brand fields (same across all slides).
    pub brand: serde_json::Value,
    /// Per-slide content. For single-slide templates, use one element.
    pub slides: Vec<serde_json::Value>,
}

impl InputData {
    /// Load from a JSON file.
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content)
    }

    /// Parse from JSON string.
    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    /// Check if this is a single-slide input.
    pub fn is_single_slide(&self) -> bool {
        self.slides.len() == 1
    }
}
