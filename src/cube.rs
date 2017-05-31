#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Yellow,
    White,
    Green,
    Blue,
    Red,
    Orange
}

// Unfold the cube like this:
//
//     +===+
//     | Y |
// +---+---+---+---+
// | R | G | O | B |
// +---+---+---+---+
//     | W |
//     +---+
//
// Cubelets are numbered startin from the top-left,
// reading from right-to-left, top-to-bottom, like this:
//
// +-+-+-+
// |0 1 2|
// |3 4 5|
// |6 7 8|
// +-+-+-+
//
// Of course, the center cubelet (no. 5), never changes color.

#[derive(Debug, PartialEq)]
pub struct Cube {
    pub yellow: Vec<Color>,
    pub white:  Vec<Color>,
    pub green:  Vec<Color>,
    pub blue:   Vec<Color>,
    pub red:    Vec<Color>,
    pub orange: Vec<Color>,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            yellow: vec![
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
            ],
            white: vec![
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
            ],
            green: vec![
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
            ],
            blue: vec![
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
            ],
            red: vec![
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Red,
            ],
            orange: vec![
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
                Color::Orange,
            ],
        }
    }

    pub fn rotate_yellow_cw(&mut self) {
        rotate_face_cw(&mut self.yellow);
        move_row(&mut self.red, &mut self.green, &mut self.orange, &mut self.blue, 0);
    }

    pub fn rotate_yellow_ccw(&mut self) {
        rotate_face_ccw(&mut self.yellow);
        move_row(&mut self.red, &mut self.blue, &mut self.orange, &mut self.green, 0);
    }

    pub fn rotate_white_cw(&mut self) {
        rotate_face_cw(&mut self.white);
        move_row(&mut self.red, &mut self.blue, &mut self.orange, &mut self.green, 6);
    }

    pub fn rotate_white_ccw(&mut self) {
        rotate_face_ccw(&mut self.white);
        move_row(&mut self.red, &mut self.green, &mut self.orange, &mut self.blue, 6);
    }
}

fn rotate_face_cw(face: &mut Vec<Color>) {
    let c0 = face[0];
    let c1 = face[1];
    let c2 = face[2];
    let c3 = face[3];
    let c5 = face[5];
    let c6 = face[6];
    let c7 = face[7];
    let c8 = face[8];
    face[0] = c6;
    face[1] = c3;
    face[2] = c0;
    face[3] = c7;
    face[5] = c1;
    face[6] = c8;
    face[7] = c5;
    face[8] = c2;
}

fn rotate_face_ccw(face: &mut Vec<Color>) {
    let c0 = face[0];
    let c1 = face[1];
    let c2 = face[2];
    let c3 = face[3];
    let c5 = face[5];
    let c6 = face[6];
    let c7 = face[7];
    let c8 = face[8];
    face[0] = c2;
    face[1] = c5;
    face[2] = c8;
    face[3] = c1;
    face[5] = c7;
    face[6] = c0;
    face[7] = c3;
    face[8] = c6;
}

fn move_row(face1: &mut Vec<Color>, face2: &mut Vec<Color>, face3: &mut Vec<Color>, face4: &mut Vec<Color>, start_index: usize) {
    // save values
    let f1_0 = face1[start_index];
    let f1_1 = face1[start_index + 1];
    let f1_2 = face1[start_index + 2];
    let f2_0 = face2[start_index];
    let f2_1 = face2[start_index + 1];
    let f2_2 = face2[start_index + 2];
    let f3_0 = face3[start_index];
    let f3_1 = face3[start_index + 1];
    let f3_2 = face3[start_index + 2];
    let f4_0 = face4[start_index];
    let f4_1 = face4[start_index + 1];
    let f4_2 = face4[start_index + 2];
    // place values
    face1[start_index] = f2_0;
    face1[start_index + 1] = f2_1;
    face1[start_index + 2] = f2_2;
    face2[start_index] = f3_0;
    face2[start_index + 1] = f3_1;
    face2[start_index + 2] = f3_2;
    face3[start_index] = f4_0;
    face3[start_index + 1] = f4_1;
    face3[start_index + 2] = f4_2;
    face4[start_index] = f1_0;
    face4[start_index + 1] = f1_1;
    face4[start_index + 2] = f1_2;
}
