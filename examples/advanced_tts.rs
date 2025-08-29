use elevenlabs_tts::{ElevenLabsTTSClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    println!("Creating ElevenLabs client...");
    let client = ElevenLabsTTSClient::new(api_key);

    // Test basic TTS with new voice API
    println!(
        "Converting text to speech with {}...",
        voices::all_voices::IVANA.name
    );

    // Example prompt text

    let voice_settings = VoiceSettings::default()
        .style(0.3)
        .speaker_boost(false)
        .speed(1.05);

    let prompt = "Life feels lighter when you slow down, take a deep breath, and notice the small details around you.";

    let audio = client
        .text_to_speech(prompt)
        .voice_settings(voice_settings.clone())
        .voice(&voices::all_voices::IVANA)
        // Only Turbo v2.5 & Flash v2.5 support language_code for pronunciation/accent
        .model(models::elevanlabs_models::ELEVEN_FLASH_V2_5)
        .language_code("fr")
        .output_format("mp3_44100_192")
        .seed(4000)
        .execute()
        .await?;

    println!("Generated {} bytes of french audio", audio.len());

    // Save to file to outputs directory:
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
