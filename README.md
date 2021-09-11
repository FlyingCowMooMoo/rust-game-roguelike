## Roguelike Tutorial - In Rust
* Working repo on my progress on the book: [Hands-on Rust
  Effective Learning through 2D Game Development and Play](https://pragprog.com/titles/hwrust/hands-on-rust/)

* Implementations:
  * Chapter 2: [chapter2.rs](src/chapter2.rs)
  * Chapter 2: [chapter3.rs](src/chapter3.rs)

You must run each chapter independently of the [main.rs](src/main.rs) main method, i.e. add the chapter you wish to run
```rust
fn main() -> rltk::BError {
    chapter3::run()
}
```