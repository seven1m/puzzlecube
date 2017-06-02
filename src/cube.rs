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
// Cubelets are numbered starting from the top-left,
// reading from right-to-left, top-to-bottom, like this:
//
// +-+-+-+
// |0 1 2|
// |3 4 5|
// |6 7 8|
// +-+-+-+
//
// Of course, the center cubelet (no. 4), never changes color.

#[derive(Debug, PartialEq, Clone)]
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

    pub fn rotate_red_cw(&mut self) {
        rotate_face_cw(&mut self.red);
        let clone = self.clone();
        self.yellow[0] = clone.blue[8];
        self.yellow[3] = clone.blue[5];
        self.yellow[6] = clone.blue[2];
        self.green[0] = clone.yellow[0];
        self.green[3] = clone.yellow[3];
        self.green[6] = clone.yellow[6];
        self.white[0] = clone.green[0];
        self.white[3] = clone.green[3];
        self.white[6] = clone.green[6];
        self.blue[2] = clone.white[6];
        self.blue[5] = clone.white[3];
        self.blue[8] = clone.white[0];
    }

    pub fn rotate_red_ccw(&mut self) {
        rotate_face_ccw(&mut self.red);
        let clone = self.clone();
        self.yellow[0] = clone.green[0];
        self.yellow[3] = clone.green[3];
        self.yellow[6] = clone.green[6];
        self.green[0] = clone.white[0];
        self.green[3] = clone.white[3];
        self.green[6] = clone.white[6];
        self.white[0] = clone.blue[8];
        self.white[3] = clone.blue[5];
        self.white[6] = clone.blue[2];
        self.blue[2] = clone.yellow[6];
        self.blue[5] = clone.yellow[3];
        self.blue[8] = clone.yellow[0];
    }

    pub fn rotate_green_cw(&mut self) {
        rotate_face_cw(&mut self.green);
        let clone = self.clone();
        self.yellow[6] = clone.red[8];
        self.yellow[7] = clone.red[5];
        self.yellow[8] = clone.red[2];
        self.red[2] = clone.white[0];
        self.red[5] = clone.white[1];
        self.red[8] = clone.white[2];
        self.orange[0] = clone.yellow[6];
        self.orange[3] = clone.yellow[7];
        self.orange[6] = clone.yellow[8];
        self.white[0] = clone.orange[6];
        self.white[1] = clone.orange[3];
        self.white[2] = clone.orange[0];
    }

    pub fn rotate_green_ccw(&mut self) {
        rotate_face_ccw(&mut self.green);
        let clone = self.clone();
        self.yellow[6] = clone.orange[0];
        self.yellow[7] = clone.orange[3];
        self.yellow[8] = clone.orange[6];
        self.red[2] = clone.yellow[8];
        self.red[5] = clone.yellow[7];
        self.red[8] = clone.yellow[6];
        self.orange[0] = clone.white[2];
        self.orange[3] = clone.white[2];
        self.orange[6] = clone.white[0];
        self.white[0] = clone.red[2];
        self.white[1] = clone.red[5];
        self.white[2] = clone.red[8];
    }

    pub fn rotate_orange_cw(&mut self) {
        rotate_face_cw(&mut self.orange);
        let clone = self.clone();
        self.yellow[2] = clone.green[2];
        self.yellow[5] = clone.green[5];
        self.yellow[8] = clone.green[8];
        self.green[2] = clone.white[2];
        self.green[5] = clone.white[5];
        self.green[8] = clone.white[8];
        self.white[2] = clone.blue[6];
        self.white[5] = clone.blue[3];
        self.white[8] = clone.blue[0];
        self.blue[6] = clone.yellow[2];
        self.blue[3] = clone.yellow[5];
        self.blue[0] = clone.yellow[8];
    }

    pub fn rotate_orange_ccw(&mut self) {
        rotate_face_ccw(&mut self.orange);
        let clone = self.clone();
        self.yellow[2] = clone.blue[6];
        self.yellow[5] = clone.blue[3];
        self.yellow[8] = clone.blue[0];
        self.green[2] = clone.yellow[2];
        self.green[5] = clone.yellow[5];
        self.green[8] = clone.yellow[8];
        self.white[2] = clone.green[2];
        self.white[5] = clone.green[5];
        self.white[8] = clone.green[8];
        self.blue[6] = clone.white[2];
        self.blue[3] = clone.white[5];
        self.blue[0] = clone.white[8];
    }

    pub fn rotate_blue_cw(&mut self) {
        rotate_face_cw(&mut self.blue);
        let clone = self.clone();
        self.yellow[0] = clone.orange[2];
        self.yellow[1] = clone.orange[5];
        self.yellow[2] = clone.orange[8];
        self.orange[2] = clone.white[8];
        self.orange[5] = clone.white[7];
        self.orange[8] = clone.white[6];
        self.white[6] = clone.red[0];
        self.white[7] = clone.red[3];
        self.white[8] = clone.red[6];
        self.red[0] = clone.yellow[0];
        self.red[3] = clone.yellow[1];
        self.red[6] = clone.yellow[2];
    }

    pub fn rotate_blue_ccw(&mut self) {
        rotate_face_ccw(&mut self.blue);
        let clone = self.clone();
        self.yellow[0] = clone.red[6];
        self.yellow[1] = clone.red[3];
        self.yellow[2] = clone.red[0];
        self.orange[2] = clone.yellow[0];
        self.orange[5] = clone.yellow[1];
        self.orange[8] = clone.yellow[2];
        self.white[6] = clone.orange[8];
        self.white[7] = clone.orange[5];
        self.white[8] = clone.orange[2];
        self.red[0] = clone.white[6];
        self.red[3] = clone.white[7];
        self.red[6] = clone.white[8];
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
