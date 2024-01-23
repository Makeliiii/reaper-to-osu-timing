# Reaper to osu! timing
Quick and dirty thing to turn REAPER timing points to an osu! compatible form.

## Why?
Timing variable BPM songs on osu! is a pain. Timing variable BPM songs on REAPER is not a pain. osu! editor does not have a spectrogram to help with timing. REAPER has a spectrogram to help with timing.

## How to run
1. Install [rust](https://www.rust-lang.org/tools/install)
2. Clone this thing
3. Navigate to the root of this thing
4. Run `cargo build` to build this thing
    - Alternatively run `cargo build --release` to build an optimized version
5. ???
6. Now you have this thing working

## Usage
Just run `reaper-to-osu-timing /path/to/reaper_file.RPP` and this thing will print the timing to your console.

## Features
* Turn REAPER timing points to osu! timing points!!!
* x/4 meter support
* Maybe something in the future
