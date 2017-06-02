use cube::*;

#[test]
fn it_can_rotate_yellow_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_yellow_cw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_yellow_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_yellow_ccw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_white_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_white_cw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Blue, Color::Blue, Color::Blue],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Red, Color::Red, Color::Red],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Green, Color::Green, Color::Green],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Orange, Color::Orange, Color::Orange],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_white_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_white_ccw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Green, Color::Green, Color::Green],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Orange, Color::Orange, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Blue, Color::Blue, Color::Blue],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Red, Color::Red, Color::Red],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_red_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_red_cw();
    assert_eq!(
        vec![Color::Blue, Color::Yellow, Color::Yellow, Color::Blue, Color::Yellow, Color::Yellow, Color::Blue, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Yellow, Color::Green, Color::Green, Color::Yellow, Color::Green, Color::Green, Color::Yellow, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::White, Color::Blue, Color::Blue, Color::White, Color::Blue, Color::Blue, Color::White],
        cube.blue
    );
    assert_eq!(
        vec![Color::Green, Color::White, Color::White, Color::Green, Color::White, Color::White, Color::Green, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_red_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_red_ccw();
    assert_eq!(
        vec![Color::Green, Color::Yellow, Color::Yellow, Color::Green, Color::Yellow, Color::Yellow, Color::Green, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::White, Color::Green, Color::Green, Color::White, Color::Green, Color::Green, Color::White, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Yellow, Color::Blue, Color::Blue, Color::Yellow, Color::Blue, Color::Blue, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Blue, Color::White, Color::White, Color::Blue, Color::White, Color::White, Color::Blue, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_green_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_green_cw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Red, Color::Red, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::White, Color::Red, Color::Red, Color::White, Color::Red, Color::Red, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Orange, Color::Orange, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_green_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_green_ccw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Orange, Color::Orange, Color::Orange],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Yellow, Color::Red, Color::Red, Color::Yellow, Color::Red, Color::Red, Color::Yellow],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::White, Color::Orange, Color::Orange, Color::White, Color::Orange, Color::Orange, Color::White, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White],
        cube.white
    );
}

#[test]
fn it_can_rotate_orange_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_orange_cw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Green, Color::Yellow, Color::Yellow, Color::Green, Color::Yellow, Color::Yellow, Color::Green],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::White, Color::Green, Color::Green, Color::White, Color::Green, Color::Green, Color::White],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::Blue, Color::Blue, Color::Yellow, Color::Blue, Color::Blue, Color::Yellow, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::Blue, Color::White, Color::White, Color::Blue, Color::White, Color::White, Color::Blue],
        cube.white
    );
}

#[test]
fn it_can_rotate_orange_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_orange_ccw();
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Yellow, Color::Yellow, Color::Blue, Color::Yellow, Color::Yellow, Color::Blue],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Yellow, Color::Green, Color::Green, Color::Yellow, Color::Green, Color::Green, Color::Yellow],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::White, Color::Blue, Color::Blue, Color::White, Color::Blue, Color::Blue, Color::White, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::Green, Color::White, Color::White, Color::Green, Color::White, Color::White, Color::Green],
        cube.white
    );
}

#[test]
fn it_can_rotate_blue_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_blue_cw();
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Orange, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Yellow, Color::Red, Color::Red, Color::Yellow, Color::Red, Color::Red, Color::Yellow, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::White, Color::Orange, Color::Orange, Color::White, Color::Orange, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::Red, Color::Red, Color::Red],
        cube.white
    );
}

#[test]
fn it_can_rotate_blue_counter_clockwise() {
    let mut cube = Cube::new();
    cube.rotate_blue_ccw();
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Red, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Red, Color::Red, Color::White, Color::Red, Color::Red, Color::White, Color::Red, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow],
        cube.orange
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue],
        cube.blue
    );
    assert_eq!(
        vec![Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::Orange, Color::Orange, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_scramble() {
    let mut cube1 = Cube::new();
    let mut cube2 = Cube::new();
    cube2.scramble();
    let mut cube3 = cube2.clone();
    cube3.scramble();
    assert_ne!(cube1, cube2);
    assert_ne!(cube2, cube3);
}
