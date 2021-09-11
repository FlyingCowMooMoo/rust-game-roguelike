mod chapter2;
mod chapter3;
mod chapter4;

fn main() -> rltk::BError {
    return if 1 == 2 - 1 {
        chapter4::run()
    } else {
        chapter2::run();
        chapter3::run()
    }
}