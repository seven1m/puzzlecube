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
        // save values
        let y0 = self.yellow[0];
        let y1 = self.yellow[1];
        let y2 = self.yellow[2];
        let y3 = self.yellow[3];
        let y4 = self.yellow[4];
        let y5 = self.yellow[5];
        let y6 = self.yellow[6];
        let y7 = self.yellow[7];
        let y8 = self.yellow[8];
        let g0 = self.green[0];
        let g1 = self.green[1];
        let g2 = self.green[2];
        let o0 = self.orange[0];
        let o1 = self.orange[1];
        let o2 = self.orange[2];
        let r0 = self.red[0];
        let r1 = self.red[1];
        let r2 = self.red[2];
        let b0 = self.blue[0];
        let b1 = self.blue[1];
        let b2 = self.blue[2];
        // yellow
        self.yellow[0] = y6;
        self.yellow[1] = y3;
        self.yellow[2] = y0;
        self.yellow[3] = y7;
        self.yellow[5] = y1;
        self.yellow[6] = y8;
        self.yellow[7] = y5;
        self.yellow[8] = y2;
        // red
        self.red[0] = g0;
        self.red[1] = g1;
        self.red[2] = g2;
        // green
        self.green[0] = o0;
        self.green[1] = o1;
        self.green[2] = o2;
        // blue
        self.blue[0] = r0;
        self.blue[1] = r1;
        self.blue[2] = r2;
        // orange
        self.orange[0] = b0;
        self.orange[1] = b1;
        self.orange[2] = b2;
    }
}
