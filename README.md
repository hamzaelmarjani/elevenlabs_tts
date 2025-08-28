# elevenlabs_tts

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_tts.svg)](https://crates.io/crates/elevenlabs_tts)
[![Docs.rs](https://docs.rs/elevenlabs_tts/badge.svg)](https://docs.rs/elevenlabs_tts)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Text-to-Speech API](https://elevenlabs.io). Generate high-quality speech from text with a simple, ergonomic API.

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
elevenlabs_tts = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_tts::ElevenLabsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsClient::new("your-api-key");

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
use elevenlabs_tts::{ElevenLabsClient, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")
        .expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsClient::new(api_key);

    let audio = client
        .text_to_speech("Happiness often hides in ordinary moments...")
        .voice(&voices::all_voices::ARNOLD)
        .model(models::elevenlabs_models::ELEVEN_TURBO_V2_5)
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
use elevenlabs_tts::{ElevenLabsClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")?;
    let client = ElevenLabsClient::new(api_key);

    // Customize voice settings for more control
    let voice_settings = VoiceSettings::new(1.0, 0.9); // stability, similarity_boost

    let audio = client
        .text_to_speech("Life feels lighter when you slow down...")
        .voice_settings(voice_settings)
        .voice(&voices::all_voices::IVANA)
        .model(models::elevenlabs_models::ELEVEN_V3)
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

| Method                                                       | Description                               |
| ------------------------------------------------------------ | ----------------------------------------- |
| `ElevenLabsClient::new(api_key)`                             | Create a new client instance              |
| `.text_to_speech(text)`                                      | Start building a TTS request              |
| `.voice(&voices::all_voices::RACHEL)`                        | Use a predefined static voice             |
| `.voice_id("custom-id")`                                     | Use a custom voice ID                     |
| `.model(models::elevenlabs_models::ELEVEN_MULTILINGUAL_V2)`  | Select an ElevenLabs model                |
| `.voice_settings(VoiceSettings::new(stability, similarity))` | Fine-tune voice parameters                |
| `.execute()`                                                 | Execute the request and return audio data |

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

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs) for the most up-to-date API information.
