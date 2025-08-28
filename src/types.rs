use serde::{Deserialize, Serialize};

/// Request body for text-to-speech API calls
#[derive(Debug, Clone, Serialize)]
pub struct TtsRequest {
    pub text: String,
    #[serde(skip_serializing)]
    pub voice_id: String, // This goes in URL, not body
    pub model_id: String,
    pub voice_settings: VoiceSettings,
}

/// Voice settings for fine-tuning speech output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Stability of the voice, Must be one of: 0.0, 0.5 and 1.0
    /// 0.0 : Creative, 0.5 : Natural, 1.0 : Robust
    /// Higher values make the voice more stable but less expressive
    pub stability: f32,

    /// Similarity boost (0.0 - 1.0)
    /// Higher values make the voice more similar to the original
    pub similarity_boost: f32,

    /// Style exaggeration (0.0 - 1.0)
    /// Higher values exaggerate the style more
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f32>,

    /// Speaker boost (true/false)
    /// Boost the similarity to the original speaker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: 0.5,
            similarity_boost: 0.8,
            style: None,
            use_speaker_boost: None,
        }
    }
}

impl VoiceSettings {
    /// Create new voice settings with custom stability and similarity
    pub fn new(stability: f32, similarity_boost: f32) -> Self {
        Self {
            stability: stability.clamp(0.0, 1.0),
            similarity_boost: similarity_boost.clamp(0.0, 1.0),
            style: None,
            use_speaker_boost: None,
        }
    }

    /// Set style exaggeration
    pub fn with_style(mut self, style: f32) -> Self {
        self.style = Some(style.clamp(0.0, 1.0));
        self
    }

    /// Enable speaker boost
    pub fn with_speaker_boost(mut self, enabled: bool) -> Self {
        self.use_speaker_boost = Some(enabled);
        self
    }
}

/// Represents a static voice
#[derive(Debug, Clone, Deserialize)]
pub struct StaticVoice {
    pub voice_id: &'static str,
    pub name: &'static str,
    pub gender: &'static str,
}

impl StaticVoice {
    pub const fn new(voice_id: &'static str, name: &'static str, gender: &'static str) -> Self {
        Self {
            voice_id: voice_id,
            name: name,
            gender: gender,
        }
    }

    /// Get the voice ID for API calls
    pub fn id(&self) -> &str {
        &self.voice_id
    }
}
