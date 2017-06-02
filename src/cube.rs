use rand;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Yellow,
    White,
    Green,
    Blue,
    Red,
    Orange
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Color::Yellow => write!(f, "Y"),
            &Color::White  => write!(f, "W"),
            &Color::Green  => write!(f, "G"),
            &Color::Blue   => write!(f, "B"),
            &Color::Red    => write!(f, "R"),
            &Color::Orange => write!(f, "O"),
        }
    }
}

static COLORS: [Color; 6] = [
    Color::Yellow,
    Color::White,
    Color::Green,
    Color::Blue,
    Color::Red,
    Color::Orange,
];

// Unfold the cube like this:
//
//     +---+
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
        self.red[0] = clone.yellow[2];
        self.red[3] = clone.yellow[1];
        self.red[6] = clone.yellow[0];
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

    pub fn scramble(&mut self) {
        self.scramble_with_debug(false);
    }

    pub fn scramble_with_debug(&mut self, debug: bool) {
        let mut last_move: (Color, u8) = (Color::White, 100);
        let mut color: Color;
        let mut direction: u8;
        for _ in 0..20 {
            loop {
                let turn = random_turn();
                color = turn.0;
                direction = turn.1;
                if last_move.0 != color || last_move.1 == direction || last_move.1 == 100 { break }
            }
            last_move = (color, direction);
            if debug { debug_turn(color, direction) }
            self.turn(color, direction);
        }
        println!();
    }

    pub fn turn(&mut self, color: Color, direction: u8) {
        match color {
            Color::Yellow => { if direction == 0 { self.rotate_yellow_cw() } else { self.rotate_yellow_ccw() } },
            Color::White  => { if direction == 0 { self.rotate_white_cw()  } else { self.rotate_white_ccw()  } },
            Color::Green  => { if direction == 0 { self.rotate_green_cw()  } else { self.rotate_green_ccw()  } },
            Color::Blue   => { if direction == 0 { self.rotate_blue_cw()   } else { self.rotate_blue_ccw()   } },
            Color::Red    => { if direction == 0 { self.rotate_red_cw()    } else { self.rotate_red_ccw()    } },
            Color::Orange => { if direction == 0 { self.rotate_orange_cw() } else { self.rotate_orange_ccw() } },
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n").unwrap();
        write!(f, "               +-------+\n").unwrap();
        write!(f, "               | {} {} {} |\n", self.yellow[0], self.yellow[1], self.yellow[2]).unwrap();
        write!(f, "               | {} {} {} |\n", self.yellow[3], self.yellow[4], self.yellow[5]).unwrap();
        write!(f, "               | {} {} {} |\n", self.yellow[6], self.yellow[7], self.yellow[8]).unwrap();
        write!(f, "       +-------+-------+-------+-------+\n").unwrap();
        write!(f, "       | {} {} {} |", self.red[0], self.red[1], self.red[2]).unwrap();
        write!(f, " {} {} {} |", self.green[0], self.green[1], self.green[2]).unwrap();
        write!(f, " {} {} {} |", self.orange[0], self.orange[1], self.orange[2]).unwrap();
        write!(f, " {} {} {} |\n", self.blue[0], self.blue[1], self.blue[2]).unwrap();
        write!(f, "       | {} {} {} |", self.red[3], self.red[4], self.red[5]).unwrap();
        write!(f, " {} {} {} |", self.green[3], self.green[4], self.green[5]).unwrap();
        write!(f, " {} {} {} |", self.orange[3], self.orange[4], self.orange[5]).unwrap();
        write!(f, " {} {} {} |\n", self.blue[3], self.blue[4], self.blue[5]).unwrap();
        write!(f, "       | {} {} {} |", self.red[6], self.red[7], self.red[8]).unwrap();
        write!(f, " {} {} {} |", self.green[6], self.green[7], self.green[8]).unwrap();
        write!(f, " {} {} {} |", self.orange[6], self.orange[7], self.orange[8]).unwrap();
        write!(f, " {} {} {} |\n", self.blue[6], self.blue[7], self.blue[8]).unwrap();
        write!(f, "       +-------+-------+-------+-------+\n").unwrap();
        write!(f, "               | {} {} {} |\n", self.white[0], self.white[1], self.white[2]).unwrap();
        write!(f, "               | {} {} {} |\n", self.white[3], self.white[4], self.white[5]).unwrap();
        write!(f, "               | {} {} {} |\n", self.white[6], self.white[7], self.white[8]).unwrap();
        write!(f, "               +-------+").unwrap();
        write!(f, "\n\n").unwrap();
        write!(f, "     ________                    ________\n").unwrap();
        write!(f, "   / {} {} {}  / \\                / {} {} {}  / \\\n", self.yellow[0], self.yellow[1], self.yellow[2], self.white[2], self.white[5], self.white[8]).unwrap();
        write!(f, "  / {} {} {}  / {} \\              / {} {} {}  / {} \\\n", self.yellow[3], self.yellow[4], self.yellow[5], self.orange[2], self.white[1], self.white[4], self.white[7], self.blue[6]).unwrap();
        write!(f, " / {} {} {}  / {} {} \\            / {} {} {}  / {} {} \\\n", self.yellow[6], self.yellow[7], self.yellow[8], self.orange[1], self.orange[5], self.white[0], self.white[3], self.white[6], self.blue[7], self.blue[3]).unwrap();
        write!(f, "/________/ {} {} {} `          /________/ {} {} {} `\n", self.orange[0], self.orange[4], self.orange[8], self.blue[8], self.blue[4], self.blue[0]).unwrap();
        write!(f, "\\ {} {} {}  \\  {} {} /           \\ {} {} {}  \\  {} {} /\n", self.green[0], self.green[1], self.green[2], self.orange[3], self.orange[7], self.red[8], self.red[7], self.red[6], self.blue[5], self.blue[1]).unwrap();
        write!(f, " \\ {} {} {}  \\  {} /             \\ {} {} {}  \\  {} /\n", self.green[3], self.green[4], self.green[5], self.orange[6], self.red[5], self.red[4], self.red[3], self.blue[2]).unwrap();
        write!(f, "  \\ {} {} {}  \\  /               \\ {} {} {}  \\  /\n", self.green[6], self.green[7], self.green[8], self.red[2], self.red[1], self.red[0]).unwrap();
        write!(f, "   \\________\\/                 \\________\\/")
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

fn random_turn() -> (Color, u8) {
    let mut rng = rand::thread_rng();
    let color = rand::sample(&mut rng, COLORS.iter(), 1)[0];
    let direction = rand::sample(&mut rng, 0..2, 1)[0];
    (*color, direction)
}

fn debug_turn(color: Color, direction: u8) {
    let face = match color {
        Color::Yellow => 'U',
        Color::White  => 'D',
        Color::Green  => 'F',
        Color::Blue   => 'B',
        Color::Red    => 'L',
        Color::Orange => 'R',
    };
    if direction == 0 {
        print!("{} ", face);
    } else {
        print!("{}' ", face);
    }
}
