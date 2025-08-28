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
    voice_settings: Option<VoiceSettings>,
}

impl TextToSpeechBuilder {
    fn new(client: ElevenLabsTTSClient, text: String) -> Self {
        Self {
            client,
            text,
            voice_id: None,
            model_id: None,
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

    /// Set the model to use
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Set voice settings (stability, similarity_boost, etc.)
    pub fn voice_settings(mut self, settings: VoiceSettings) -> Self {
        if settings.stability != 0.0 && settings.stability != 0.5 && settings.stability != 1.0 {
            panic!("Invalid stability value. Must be one of: 0.0, 0.5, 1.0");
        }

        self.voice_settings = Some(settings);
        self
    }

    /// Execute the text-to-speech request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsTTSError> {
        let voice_id = self
            .voice_id
            .unwrap_or_else(|| voices::all_voices::RACHEL.voice_id.to_string()); // Default to Rachel

        let request = TtsRequest {
            text: self.text,
            voice_id: voice_id.clone(),
            model_id: self
                .model_id
                .unwrap_or_else(|| models::elevanlabs_models::ELEVEN_MULTILINGUAL_V2.to_string()), // Default to eleven_monolingual_v2
            voice_settings: self.voice_settings.unwrap_or_default(),
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
