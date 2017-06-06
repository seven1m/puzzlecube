use cube::*;

#[test]
fn it_can_orient() {
    let mut cube = Cube::new();
    cube.orient(Color::White, Color::Blue);
    assert_eq!(Color::White, cube.up);
    assert_eq!(Color::Blue, cube.front);
    assert_eq!(Color::White, cube.up_color());
    assert_eq!(Color::Yellow, cube.down_color());
    assert_eq!(Color::Blue, cube.front_color());
    assert_eq!(Color::Green, cube.back_color());
    assert_eq!(Color::Red, cube.left_color());
    assert_eq!(Color::Orange, cube.right_color());
}

#[test]
fn it_can_rotate_yellow_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_yellow_cw();
    assert_eq!(
        vec![Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Red, Color::Red, Color::Orange, Color::Orange],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Red, Color::Orange, Color::Red, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Blue, Color::Green, Color::White, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Green, Color::Blue, Color::Yellow, Color::Green, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Yellow, Color::White, Color::White, Color::Green, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_yellow_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_yellow_ccw();
    assert_eq!(
        vec![Color::Orange, Color::Orange, Color::Red, Color::Red, Color::Yellow, Color::Blue, Color::Green, Color::Red, Color::White],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Blue, Color::Red, Color::Orange, Color::Red, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Green, Color::White, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Green, Color::Blue, Color::Yellow, Color::Green, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Yellow, Color::White, Color::White, Color::Green, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_white_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_white_cw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Orange, Color::Red, Color::Yellow, Color::Orange, Color::White, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Red, Color::Orange, Color::Green, Color::Orange, Color::Yellow],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Green, Color::White, Color::Red, Color::Green, Color::White],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Blue, Color::Orange, Color::Red, Color::Blue, Color::Yellow, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Green, Color::Orange, Color::White],
        cube.blue
    );
    assert_eq!(
        vec![Color::Blue, Color::White, Color::Red, Color::White, Color::White, Color::Green, Color::Orange, Color::Green, Color::Yellow],
        cube.white
    );
}

#[test]
fn it_can_rotate_white_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_white_ccw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Orange, Color::Red, Color::Yellow, Color::Orange, Color::White, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Red, Color::Orange, Color::Blue, Color::Yellow, Color::Orange],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Green, Color::White, Color::Green, Color::Orange, Color::White],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::Yellow],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Red, Color::Green, Color::White],
        cube.blue
    );
    assert_eq!(
        vec![Color::Yellow, Color::Green, Color::Orange, Color::Green, Color::White, Color::White, Color::Red, Color::White, Color::Blue],
        cube.white
    );
}

#[test]
fn it_can_rotate_red_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_red_cw();
    assert_eq!(
        vec![Color::Yellow, Color::Red, Color::Orange, Color::Yellow, Color::Yellow, Color::Orange, Color::Red, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Red, Color::Blue, Color::White, Color::Green, Color::Red, Color::Yellow, Color::White, Color::Orange, Color::Blue],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Green, Color::Red, Color::Green, Color::White, Color::White, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Blue, Color::Green, Color::Blue, Color::White, Color::Green, Color::Orange, Color::Red],
        cube.blue
    );
    assert_eq!(
        vec![Color::Orange, Color::Green, Color::Yellow, Color::Blue, Color::White, Color::Green, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_red_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_red_ccw();
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Orange, Color::Blue, Color::Yellow, Color::Orange, Color::Blue, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Blue, Color::Orange, Color::White, Color::Yellow, Color::Red, Color::Green, Color::White, Color::Blue, Color::Red],
        cube.red
    );
    assert_eq!(
        vec![Color::Red, Color::Red, Color::Green, Color::White, Color::Green, Color::White, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::White, Color::Green, Color::Blue, Color::Red, Color::Green, Color::Orange, Color::Green],
        cube.blue
    );
    assert_eq!(
        vec![Color::Yellow, Color::Green, Color::Yellow, Color::Yellow, Color::White, Color::Green, Color::Red, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_green_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_green_cw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Orange, Color::Red, Color::Yellow, Color::Orange, Color::White, Color::Orange, Color::Blue],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Red, Color::Blue, Color::Red, Color::Green, Color::Red, Color::Green, Color::Yellow],
        cube.red
    );
    assert_eq!(
        vec![Color::Blue, Color::Blue, Color::Orange, Color::Yellow, Color::Green, Color::Red, Color::Orange, Color::White, Color::Green],
        cube.green
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Orange, Color::Red, Color::Red, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Green, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Green, Color::Blue, Color::Yellow, Color::White, Color::White, Color::Green, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_green_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_green_ccw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Orange, Color::Red, Color::Yellow, Color::Orange, Color::Yellow, Color::Blue, Color::Green],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Red, Color::Blue, Color::Red, Color::Blue, Color::Red, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Green, Color::White, Color::Orange, Color::Red, Color::Green, Color::Yellow, Color::Orange, Color::Blue, Color::Blue],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Blue, Color::Green, Color::Orange, Color::Red, Color::Red, Color::Orange, Color::White],
        cube.orange
    );
    assert_eq!(
        vec![Color::Yellow, Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Green, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Blue, Color::Orange, Color::White, Color::White, Color::White, Color::Green, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_orange_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_orange_cw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Green, Color::Red, Color::Yellow, Color::White, Color::White, Color::Blue, Color::Orange],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Red, Color::Orange, Color::Red, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Yellow, Color::Blue, Color::Green, Color::Green, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Green, Color::Blue, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow, Color::White, Color::Red, Color::Blue],
        cube.orange
    );
    assert_eq!(
        vec![Color::Red, Color::White, Color::Red, Color::Orange, Color::Blue, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Green, Color::White, Color::White, Color::Green, Color::Blue, Color::White, Color::Yellow],
        cube.white
    );
}

#[test]
fn it_can_rotate_orange_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_orange_ccw();
    assert_eq!(
        vec![Color::Green, Color::Red, Color::Green, Color::Red, Color::Yellow, Color::Green, Color::White, Color::Blue, Color::Yellow],
        cube.yellow
    );
    assert_eq!(
        vec![Color::White, Color::Yellow, Color::Blue, Color::Blue, Color::Red, Color::Orange, Color::Red, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Orange, Color::Blue, Color::Green, Color::Orange, Color::Blue, Color::Yellow, Color::Red],
        cube.green
    );
    assert_eq!(
        vec![Color::Blue, Color::Red, Color::White, Color::Yellow, Color::Orange, Color::Orange, Color::Yellow, Color::Blue, Color::Green],
        cube.orange
    );
    assert_eq!(
        vec![Color::Orange, Color::White, Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Yellow, Color::Orange, Color::Yellow],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Green, Color::White, Color::White, Color::White, Color::Blue, Color::White, Color::Orange],
        cube.white
    );
}

#[test]
fn it_can_rotate_blue_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_blue_cw();
    assert_eq!(
        vec![Color::Blue, Color::Red, Color::White, Color::Red, Color::Yellow, Color::Orange, Color::White, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Orange, Color::Yellow, Color::Blue, Color::Red, Color::Red, Color::Orange, Color::Green, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Green, Color::White, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Orange, Color::Blue, Color::Orange, Color::White, Color::Green, Color::Orange, Color::Blue],
        cube.orange
    );
    assert_eq!(
        vec![Color::Green, Color::Green, Color::Yellow, Color::Orange, Color::Blue, Color::White, Color::Yellow, Color::Yellow, Color::Red],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Yellow, Color::White, Color::White, Color::Green, Color::White, Color::Blue, Color::Red],
        cube.white
    );
}

#[test]
fn it_can_rotate_blue_counter_clockwise() {
    let mut cube = Cube::new_with_test_pattern();
    cube.rotate_blue_ccw();
    assert_eq!(
        vec![Color::Red, Color::Blue, Color::White, Color::Red, Color::Yellow, Color::Orange, Color::White, Color::Blue, Color::Red],
        cube.yellow
    );
    assert_eq!(
        vec![Color::Blue, Color::Yellow, Color::Blue, Color::White, Color::Red, Color::Orange, Color::Orange, Color::Green, Color::White],
        cube.red
    );
    assert_eq!(
        vec![Color::Orange, Color::Red, Color::Green, Color::Blue, Color::Green, Color::White, Color::Blue, Color::Yellow, Color::Orange],
        cube.green
    );
    assert_eq!(
        vec![Color::Yellow, Color::Yellow, Color::Green, Color::Blue, Color::Orange, Color::Red, Color::Green, Color::Orange, Color::Orange],
        cube.orange
    );
    assert_eq!(
        vec![Color::Red, Color::Yellow, Color::Yellow, Color::White, Color::Blue, Color::Orange, Color::Yellow, Color::Green, Color::Green],
        cube.blue
    );
    assert_eq!(
        vec![Color::Red, Color::Green, Color::Yellow, Color::White, Color::White, Color::Green, Color::White, Color::Red, Color::Blue],
        cube.white
    );
}

#[test]
fn it_can_scramble() {
    let cube1 = Cube::new();
    let mut cube2 = Cube::new();
    cube2.scramble();
    let mut cube3 = cube2.clone();
    cube3.scramble();
    assert_ne!(cube1, cube2);
    assert_ne!(cube2, cube3);
}
