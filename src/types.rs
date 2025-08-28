use serde::{Deserialize, Serialize};

/// Request body for text-to-speech API calls
#[derive(Debug, Clone, Serialize)]
pub struct TtsRequest {
    pub text: String,
    #[serde(skip_serializing)]
    // ID of the voice to be used. Use the Get voices: https://elevenlabs.io/docs/api-reference/voices/search endpoint list all the available voices.
    // This goes in the URL path, not in the body.
    pub voice_id: String,

    // Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    // MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    // Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    // Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    // Default to: mp3_44100_128
    // This goes in the URL path, not in the body.
    pub output_format: Option<String>,

    // Identifier of the model that will be used, you can query them using GET https://api.elevenlabs.io/v1/models.
    // The model needs to have support for text to speech, you can check this using the can_do_text_to_speech property.
    pub model_id: String,

    // Language code (ISO 639-1) used to enforce a language for the model. Currently only Turbo v2.5 and Flash v2.5 support language enforcement.
    // For other models, an error will be returned if language code is provided.
    // You can see all supported languages for each model: https://help.elevenlabs.io/hc/en-us/articles/13313366263441-What-languages-do-you-support
    // Note: this parameter in ElevenLabs API doesn't translate text - it only controls the pronunciation/accent when speaking the text.
    // The text itself remains in the original language. i.e: If you want French audio, you need to provide French text.
    pub language_code: Option<String>,

    // If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result.
    // Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    pub seed: Option<u32>,

    //The text that came before the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations
    // or to influence the speech's continuity in the current generation.
    pub previous_text: Option<String>,

    // The text that comes after the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations
    // or to influence the speech's continuity in the current generation.
    pub next_text: Option<String>,

    // A list of request_id of the samples that were generated before this generation. Can be used to improve the speech’s continuity when splitting up a large task into multiple requests.
    // The results will be best when the same model is used across the generations. In case both previous_text and previous_request_ids is send, previous_text will be ignored. A maximum of 3 request_ids can be send.
    pub previous_request_ids: Option<Vec<String>>,

    // A list of request_id of the samples that come after this generation. next_request_ids is especially useful for maintaining the speech’s continuity when regenerating a sample that has had some audio quality issues.
    // For example, if you have generated 3 speech clips, and you want to improve clip 2, passing the request id of clip 3 as a next_request_id (and that of clip 1 as a previous_request_id) will help maintain natural flow in the combined speech.
    // The results will be best when the same model is used across the generations. In case both next_text and next_request_ids is send, next_text will be ignored. A maximum of 3 request_ids can be send.
    pub next_request_ids: Option<Vec<String>>,

    // This parameter controls text normalization with three modes: ‘auto’, ‘on’, and ‘off’. When set to ‘auto’, the system will automatically decide whether to apply text normalization (e.g., spelling out numbers). With ‘on’,
    // text normalization will always be applied, while with ‘off’, it will be skipped. For ‘eleven_turbo_v2_5’ and ‘eleven_flash_v2_5’ models, text normalization can only be enabled with Enterprise plans.
    // Defaults to: auto
    pub apply_text_normalization: Option<String>,

    // This parameter controls language text normalization. This helps with proper pronunciation of text in some supported languages.
    // WARNING: This parameter can heavily increase the latency of the request. Currently only supported for Japanese.
    // Defaults to: false
    pub apply_language_text_normalization: Option<bool>,

    // Voice settings overriding stored settings for the given voice. They are applied only on the given request.
    pub voice_settings: VoiceSettings,
}

/// Voice settings for fine-tuning speech output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Stability of the voice, Must be one of: 0.0, 0.5 and 1.0
    /// 0.0 : Creative, 0.5 : Natural, 1.0 : Robust
    /// Higher values make the voice more stable but less expressive
    pub stability: Option<f32>,

    /// Similarity boost (0.0 - 1.0)
    /// Higher values make the voice more similar to the original
    pub similarity_boost: Option<f32>,

    /// Style exaggeration (0.0 - 1.0)
    /// Higher values exaggerate the style more
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f32>,

    /// Speaker boost (true/false)
    /// Boost the similarity to the original speaker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,

    /// Adjusts the speed of the voice.
    /// A value of 1.0 is the default speed, while values less than 1.0 slow down the speech,
    /// and values greater than 1.0 speed it up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: Some(0.5),
            similarity_boost: Some(0.8),
            style: Some(0.0),
            use_speaker_boost: Some(true),
            speed: Some(1.0),
        }
    }
}

impl VoiceSettings {
    /// Create new voice settings with custom stability and similarity
    pub fn new(
        stability: Option<f32>,
        similarity_boost: Option<f32>,
        style: Option<f32>,
        use_speaker_boost: Option<bool>,
        speed: Option<f32>,
    ) -> Self {
        Self {
            // Default stability is 0.5 (natural)
            stability: Some((stability.unwrap_or(0.5)).clamp(0.0, 1.0)),
            // Default similarity boost is 0.75
            similarity_boost: Some((similarity_boost.unwrap_or(0.75)).clamp(0.0, 1.0)),
            // Default style is 0
            style: Some((style.unwrap_or(0.0)).clamp(0.0, 1.0)),
            // Default to true
            use_speaker_boost: Some(use_speaker_boost.unwrap_or(true)),
            // Default speed is 1.0 (normal speed)
            speed: Some((speed.unwrap_or(1.0)).clamp(0.70, 1.20)),
        }
    }

    /// Set stability
    pub fn stability(mut self, stability: f32) -> Self {
        self.stability = Some(stability.clamp(0.0, 1.0));
        self
    }

    /// Set similarity boost
    pub fn similarity_boost(mut self, similarity_boost: f32) -> Self {
        self.similarity_boost = Some(similarity_boost.clamp(0.0, 1.0));
        self
    }

    /// Set style exaggeration
    pub fn style(mut self, style: f32) -> Self {
        self.style = Some(style.clamp(0.0, 1.0));
        self
    }

    /// Enable speaker boost
    pub fn speaker_boost(mut self, enabled: bool) -> Self {
        self.use_speaker_boost = Some(enabled);
        self
    }

    /// Set speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
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
