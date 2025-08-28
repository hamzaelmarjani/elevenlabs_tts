use elevenlabs_tts::{ElevenLabsClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    println!("Creating ElevenLabs client...");
    let client = ElevenLabsClient::new(api_key);

    // Test basic TTS with new voice API
    println!(
        "Converting text to speech with {}...",
        voices::all_voices::IVANA.name
    );

    // Example prompt text
    let prompt = "Life feels lighter when you slow down, take a deep breath, and notice the small details around you.";
    let audio = client
        .text_to_speech(prompt)
        // Set voice settings
        .voice_settings(VoiceSettings::new(1.0, 0.9)) // Use default voice settings
        .voice(&voices::all_voices::IVANA) // Use StaticVoice reference
        .model(models::elevanlabs_models::ELEVEN_V3)
        .execute()
        .await?;

    println!("Generated {} bytes of audio", audio.len());

    // Save to file to outputs directory
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
