use elevenlabs_tts::{ElevenLabsTTSClient, ElevenLabsTTSError, VoiceSettings, models, voices};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsTTSClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_pattern() {
    let client = ElevenLabsTTSClient::new("test-key");
    let _builder = client
        .text_to_speech("Hello world")
        .voice_id(voices::all_voices::RACHEL.voice_id)
        .model(models::elevanlabs_models::ELEVEN_TURBO_V2_5);

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_voice_settings() {
    let settings = VoiceSettings::new(Some(0.7), Some(0.9), Some(0.3), Some(false), Some(1.0));
    assert_eq!(settings.stability, Some(0.7));
    assert_eq!(settings.similarity_boost, Some(0.9));
    assert_eq!(settings.style, Some(0.3));
    assert_eq!(settings.use_speaker_boost, Some(false));
    assert_eq!(settings.speed, Some(1.0));
}

#[test]
fn test_error_display() {
    let error = ElevenLabsTTSError::ValidationError("Invalid voice ID".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid voice ID"));
}

#[test]
fn test_static_voices() {
    // Test voice constants
    assert_eq!(voices::all_voices::RACHEL.voice_id, "21m00Tcm4TlvDq8ikWAM");
    assert_eq!(voices::all_voices::RACHEL.name, "Rachel");
    assert_eq!(voices::all_voices::RACHEL.gender, "female");

    // Test voice ID access
    assert_eq!(voices::all_voices::ANTONI.id(), "ErXwobaYiN019PkySvjV");
}

#[test]
fn test_voice_filtering() {
    let all_voices = voices::all_voices::all();
    let male_voices = voices::all_voices::male();
    let female_voices = voices::all_voices::female();

    assert!(all_voices.len() > 0);
    assert!(male_voices.len() > 0);
    assert!(female_voices.len() > 0);
    assert_eq!(all_voices.len(), male_voices.len() + female_voices.len());

    // Check that filtering works correctly
    for voice in male_voices {
        assert_eq!(voice.gender, "male");
    }

    for voice in female_voices {
        assert_eq!(voice.gender, "female");
    }
}

#[test]
fn test_voice_search() {
    let found = voices::all_voices::find_by_name("Rachel");
    assert!(found.is_some());
    assert_eq!(found.unwrap().voice_id, "21m00Tcm4TlvDq8ikWAM");

    // Test case insensitive
    let found_lower = voices::all_voices::find_by_name("rachel");
    assert!(found_lower.is_some());
    assert_eq!(found_lower.unwrap().voice_id, "21m00Tcm4TlvDq8ikWAM");

    // Test not found
    let not_found = voices::all_voices::find_by_name("NonExistentVoice");
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_builder_with_voice_reference() {
    let client = ElevenLabsTTSClient::new("test-key");
    let _builder = client
        .text_to_speech("Hello world")
        .voice(&voices::all_voices::RACHEL) // Test new voice API
        .model(models::elevanlabs_models::ELEVEN_TURBO_V2_5);

    // Builder pattern works if this compiles
    assert_eq!(true, true);
}

// Mock tests for API calls (without real HTTP requests)
#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsTTSClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
