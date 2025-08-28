//! ElevenLabs Text-to-Speech API client
//!
//! A type-safe, async Rust client for the ElevenLabs TTS API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_tts::ElevenLabsTTSClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsTTSClient::new("your-api-key");
//!     
//!     let audio = client
//!         .text_to_speech("Hello, world!")
//!         .voice_id("21m00Tcm4TlvDq8ikWAM") // Rachel voice
//!         .model("eleven_monolingual_v1")
//!         .execute()
//!         .await?;
//!     
//!     // audio is Vec<u8> - raw audio data
//!     std::fs::write("output.mp3", audio)?;
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;
pub mod voices;

pub use error::ElevenLabsTTSError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsTTSClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsTTSClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a text-to-speech request
    pub fn text_to_speech<S: Into<String>>(&self, text: S) -> TextToSpeechBuilder {
        TextToSpeechBuilder::new(self.clone(), text.into())
    }

    /// Internal method to execute TTS request
    pub(crate) async fn execute_tts(
        &self,
        request: TtsRequest,
    ) -> Result<Vec<u8>, ElevenLabsTTSError> {
        let url = format!("{}/text-to-speech/{}", self.base_url, request.voice_id);

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsTTSError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

/// Builder for text-to-speech requests
pub struct TextToSpeechBuilder {
    client: ElevenLabsTTSClient,
    text: String,
    voice_id: Option<String>,
    model_id: Option<String>,
    output_format: Option<String>,
    language_code: Option<String>,
    seed: Option<u32>,
    previous_text: Option<String>,
    next_text: Option<String>,
    previous_request_ids: Option<Vec<String>>,
    next_request_ids: Option<Vec<String>>,
    apply_text_normalization: Option<String>,
    apply_language_text_normalization: Option<bool>,
    voice_settings: Option<VoiceSettings>,
}

impl TextToSpeechBuilder {
    fn new(client: ElevenLabsTTSClient, text: String) -> Self {
        Self {
            client,
            text,
            voice_id: None,
            model_id: None,
            output_format: None,
            language_code: None,
            seed: None,
            previous_text: None,
            next_text: None,
            previous_request_ids: None,
            next_request_ids: None,
            apply_text_normalization: None,
            apply_language_text_normalization: None,
            voice_settings: None,
        }
    }

    /// Set the voice to use (accepts StaticVoice reference)
    pub fn voice(mut self, voice: &StaticVoice) -> Self {
        self.voice_id = Some(voice.voice_id.to_string());
        self
    }

    /// Set the voice ID to use directly (for custom voices)
    pub fn voice_id<S: Into<String>>(mut self, voice_id: S) -> Self {
        self.voice_id = Some(voice_id.into());
        self
    }

    /// Set the output format to use
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Set the model to use
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Set the language code to use
    pub fn language_code<S: Into<String>>(mut self, language_code: S) -> Self {
        self.language_code = Some(language_code.into());
        self
    }

    /// Set voice settings (stability, similarity_boost, style, user_speaker_boost and speed).
    pub fn voice_settings(mut self, settings: VoiceSettings) -> Self {
        self.voice_settings = Some(settings);
        self
    }

    /// Set seeds to use
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the previous text
    pub fn previous_text<S: Into<String>>(mut self, previous_text: S) -> Self {
        self.previous_text = Some(previous_text.into());
        self
    }

    /// Set the next text
    pub fn next_text<S: Into<String>>(mut self, next_text: S) -> Self {
        self.next_text = Some(next_text.into());
        self
    }

    /// Set the prebious requests ids
    pub fn previous_request_ids<S: Into<Vec<String>>>(mut self, previous_request_ids: S) -> Self {
        self.previous_request_ids = Some(previous_request_ids.into());
        self
    }

    /// Set the next text to use
    pub fn next_request_ids<S: Into<Vec<String>>>(mut self, next_request_ids: S) -> Self {
        self.next_request_ids = Some(next_request_ids.into());
        self
    }

    /// Set the apply text normalization
    pub fn apply_text_normalization<S: Into<String>>(
        mut self,
        apply_text_normalization: S,
    ) -> Self {
        self.apply_text_normalization = Some(apply_text_normalization.into());
        self
    }

    /// Set the apply language text normalization
    pub fn apply_language_text_normalization<B: Into<bool>>(
        mut self,
        apply_language_text_normalization: B,
    ) -> Self {
        self.apply_language_text_normalization = Some(apply_language_text_normalization.into());
        self
    }

    /// Execute the text-to-speech request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsTTSError> {
        let voice_id = self
            .voice_id
            .unwrap_or_else(|| voices::all_voices::RACHEL.voice_id.to_string()); // Default to: Rachel

        let output_format = self
            .output_format
            .unwrap_or_else(|| "mp3_44100_128".to_string()); // Default to: mp3_44100_128

        let request = TtsRequest {
            text: self.text,
            voice_id: voice_id.clone(),
            output_format: Some(output_format.clone()),
            model_id: self
                .model_id
                .unwrap_or_else(|| models::elevanlabs_models::ELEVEN_MULTILINGUAL_V2.to_string()), // Default to: eleven_multilingual_v2
            language_code: self.language_code.or(None), // Default to null
            voice_settings: self.voice_settings.unwrap_or_default(), // Default voice settings
            seed: self.seed.or(None),                   // Default to null
            previous_text: self.previous_text.or(None), // Default to null
            next_text: self.next_text.or(None),         // Default to null
            previous_request_ids: self.previous_request_ids.or(None), // Default to null
            next_request_ids: self.next_request_ids.or(None), // Default to null
            apply_text_normalization: Some(
                self.apply_text_normalization
                    .unwrap_or_else(|| "auto".to_string()),
            ), // Default to: auto
            apply_language_text_normalization: Some(
                self.apply_language_text_normalization
                    .unwrap_or_else(|| false),
            ), // Default to: false
        };

        self.client.execute_tts(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsTTSClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern() {
        let client = ElevenLabsTTSClient::new("test-key");
        let builder = client
            .text_to_speech("Hello")
            .voice_id("voice-123")
            .model("model-456");

        // Builder pattern works
        assert_eq!(builder.text, "Hello");
        assert_eq!(builder.voice_id, Some("voice-123".to_string()));
    }
}
