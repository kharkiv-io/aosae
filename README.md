<h1 align="center">Advanced open-source audio engine</h1>
<p align="center"><a href="#project-description">Project Description</a> - <a href="features">Features</a></p>

## Project Description

Audio engine written for the project ( Advanced open-source digital audio workstation ). At the moment is under development.

## Features

At the moment, the engine supports the following functions and features :

*   Play audio from samples.
*   Encoding and finalization of .wav files from samples
*   Creating a pool of sounds from samples and then manipulating the sounds in the pool.
*   Record microphone sound into samples.
*   Decoder that supports almost any sample format and audio file formats. Thanks to Symphonia 
https://github.com/pdeljanov/Symphonia
*   Reading "metadata" from audio files.
*   Create multitrack and manipulate ( tracks ) in 

## Examples 

Create pool and play audio from samples : 
```rust 
fn main() {
  let samples: Vec<f32> = Vec::new(); // Your samples here
  let pool_s = Arc::new(sounds_pool::SoundThreadPool::new(1)); // Create a pool 
  if let Ok(sound_thread_pool) = Arc::clone(&pool_s).as_ref() {
        sound_thread_pool.execute( // Process samples 
            0, 
            samples, 
            Duration::from_secs_f32(10.0), 
            Duration::from_secs_f32(1.0),
        );
    } else {
        eprintln!("Failed to initialize pool!");
    }
} 
```
