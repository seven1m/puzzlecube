extern crate rand;

mod cube;
mod tests;

use cube::*;

fn main() {
    let mut cube = Cube::new();
    cube.scramble_with_debug(true);
    println!("{}", cube);
}
