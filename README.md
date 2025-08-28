# elevenlabs_tts

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_tts.svg)](https://crates.io/crates/elevenlabs_tts)
[![Docs.rs](https://docs.rs/elevenlabs_tts/badge.svg)](https://docs.rs/elevenlabs_tts)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Text-to-Speech API](https://elevenlabs.io/app/speech-synthesis/text-to-speech). Generate high-quality speech from text with a simple, ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring TTS requests
- **Predefined Voices**: Access to static voice definitions (`voices::all_voices::*`)
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Voice settings, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_tts = "0.0.2"
```

## Quick Start

```rust
use elevenlabs_tts::ElevenLabsTTSClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsTTSClient::new("your-api-key");

    let audio = client
        .text_to_speech("Hello, world!")
        .voice(&voices::all_voices::ARNOLD) // Arnold Voice
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_V2) // Eleven Multilingual v2
        .execute()
        .await?;

    std::fs::write("output.mp3", audio)?;
    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_tts::{ElevenLabsTTSClient, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")
        .expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTSClient::new(api_key);

    let prompt = "Happiness often hides in ordinary moments, waiting for you to pause, smile, and simply enjoy being present.";

    let audio = client
        .text_to_speech(prompt)
        .voice(&voices::all_voices::ARNOLD) // Arnold Voice
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_V2) // Eleven Multilingual v2
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    std::fs::write("outputs/output.mp3", audio)?;
    println!("Audio saved to outputs/output.mp3");
    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_tts::{ElevenLabsTTSClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")?;
    let client = ElevenLabsTTSClient::new(api_key);

    // Customize voice settings for more control
    // Stability: 0.5 (50%)
    // Similarity: 0.85 (85%)
    // Style Exaggeration: 0.5 (50%)
    // Speaker boost: true
    let voice_settings = VoiceSettings::new(0.5, 0.85)
        .with_style(0.5)
        .with_speaker_boost(true);

    let prompt = "Life feels lighter when you slow down, take a deep breath, and notice the small details around you.";

    let audio = client
        .text_to_speech(prompt)
        .voice_settings(voice_settings)
        .voice(&voices::all_voices::ARNOLD) // Arnold Voice
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_V2) // Eleven Multilingual v2
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    std::fs::write("outputs/advanced_output.mp3", audio)?;
    println!("Advanced audio saved to outputs/advanced_output.mp3");
    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_tts

# Run the advanced example
cargo run --example advanced_tts
```

## API Overview

| Method                                                                                                | Description                               |
| ----------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| `ElevenLabsTTSClient::new(api_key)`                                                                   | Create a new client instance              |
| `.text_to_speech(text)`                                                                               | Start building a TTS request              |
| `.voice(&voices::all_voices::RACHEL)`                                                                 | Use a predefined static voice             |
| `.voice_id("custom-id")`                                                                              | Use a custom voice ID                     |
| `.model(models::elevenlabs_models::ELEVEN_MULTILINGUAL_V2)`                                           | Select an ElevenLabs model                |
| `.voice_settings(VoiceSettings::new(stability, similarity).with_style(0.5).with_speaker_boost(true))` | Fine-tune voice parameters                |
| `.execute()`                                                                                          | Execute the request and return audio data |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.text_to_speech("Hello").execute().await {
    Ok(audio) => println!("Generated {} bytes of audio", audio.len()),
    Err(e) => eprintln!("TTS generation failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/text-to-speech/convert) for the most up-to-date API information.
