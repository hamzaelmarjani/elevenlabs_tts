use elevenlabs_tts::{ElevenLabsClient, models, voices};
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
        voices::all_voices::ARNOLD.name
    );

    // Example prompt text
    let prompt = "Happiness often hides in ordinary moments, waiting for you to pause, smile, and simply enjoy being present.";
    let audio = client
        .text_to_speech(prompt)
        .voice(&voices::all_voices::ARNOLD) // Use StaticVoice reference
        .model(models::elevanlabs_models::ELEVEN_TURBO_V2_5)
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
