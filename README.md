# Bevy 2D Shooter
This is a 2d top-down shooter written in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine. It's capable of handling 100K enemies and uses a kd-tree to efficiently handle the collisions.

Link to the tutorial & timelapse video below.

![screenshot](/screenshot.png)

# Tutorial
Here's the entire timelapse of the AI learning to drive

[![youtube](https://img.youtube.com/vi/p8d8TKo59LU/0.jpg)](https://youtu.be/p8d8TKo59LU)

# Showcase Video
[![youtube](https://img.youtube.com/vi/RiKPrOx2jmE/0.jpg)](https://youtu.be/RiKPrOx2jmE)

## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/bevy-2d-shooter.git
cd bevy-2d-shooter
```
- Run
```bash
cargo run
```

## Configurations
- The project config file is located at `src/configs.rs`

## Credits
- Game assets - [https://0x72.itch.io/dungeontileset-ii](https://0x72.itch.io/dungeontileset-ii)
- Monogram Font - [https://datagoblin.itch.io/monogram](https://datagoblin.itch.io/monogram)

## Controls
- `WASD` for movement
- Mouse wheel to change camera zoom
