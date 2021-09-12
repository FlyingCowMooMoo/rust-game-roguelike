## Roguelike Tutorial - In Rust
[![Release](https://github.com/FlyingCowMooMoo/rust-game-roguelike/actions/workflows/release.yml/badge.svg)](https://github.com/FlyingCowMooMoo/rust-game-roguelike/actions/workflows/release.yml)

* Working repo on my progress on the tutorial: https://bfnightly.bracketproductions.com/rustbook/

* Implementations:
  * Chapter 2: [chapter2.rs](src/chapter2.rs)
  * Chapter 3: [chapter3.rs](src/chapter3.rs)
  * Chapter 4: [/chapter4](src/chapter4)
    * [chapter4.rs](src/chapter4/chapter4.rs) - Main for running the application
    * [components.rs](src/chapter4/components.rs) - The components for the ECS
    * [map.rs](src/chapter4/map.rs) - Types and methods for the map/world
    * [player.rs](src/chapter4/player.rs) - Methods for player input
    * [rect.rs](src/chapter4/rect.rs) - Rectangle type and methods

You must run each chapter independently of the [main.rs](src/main.rs) main method, i.e. add the chapter you wish to run
```rust
fn main() -> rltk::BError {
    chapter3::run()
}
```
