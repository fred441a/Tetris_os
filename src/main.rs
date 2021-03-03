extern crate termion;

use std::io::{stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color};

use rand::Rng;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

//globals
const emptyblock: Block = Block {
    isFilled: false,
    red: 0,
    green: 0,
    blue: 0,
};
const emptybrick: Brick = Brick {
    shape: [[emptyblock; 4]; 4],
    position: Pos { x: 0, y: 0 },
};

const redblock: Block = Block {
    isFilled: true,
    red: 255,
    green: 0,
    blue: 0,
};

const blueblock: Block = Block {
    isFilled: true,
    red: 0,
    green: 0,
    blue: 255,
};

const greenblock: Block = Block {
    isFilled: true,
    red: 0,
    green: 255,
    blue: 0,
};

const yellowblock: Block = Block {
    isFilled: true,
    red: 255,
    green: 255,
    blue: 0,
};

const Purpleblock: Block = Block {
    isFilled: true,
    red: 255,
    green: 0,
    blue: 255,
};

const tealblock: Block = Block {
    isFilled: true,
    red: 0,
    green: 255,
    blue: 255,
};

const orangeblock: Block = Block {
    isFilled: true,
    red: 247,
    green: 153,
    blue: 0,
};

const Bricks: [Brick; 7] = [
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, orangeblock, orangeblock],
            [emptyblock, emptyblock, orangeblock, orangeblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, emptyblock, blueblock],
            [emptyblock, emptyblock, emptyblock, blueblock],
            [emptyblock, emptyblock, blueblock, blueblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, yellowblock, yellowblock],
            [emptyblock, yellowblock, yellowblock, emptyblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, Purpleblock, Purpleblock, emptyblock],
            [emptyblock, emptyblock, Purpleblock, Purpleblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, tealblock, emptyblock],
            [emptyblock, tealblock, tealblock, tealblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
    Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, emptyblock],
            [emptyblock, emptyblock, greenblock, emptyblock],
            [emptyblock, emptyblock, greenblock, emptyblock],
            [emptyblock, emptyblock, greenblock, greenblock],
        ],
        position: Pos { x: 0, y: 0 },
    },
];

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut Tetris = Game {
        map: [[emptyblock; 10]; 20],
        onMapBrick: Bricks[3],
        points: 0,
        nextBrick: Bricks[5],
        savedBrick: emptybrick,
    };

    let mut Time = SystemTime::now();

    loop {
        let ElapsedTime = Time.elapsed().unwrap();

        match stdin.next() {
            Some(Ok(b'q')) => break,
            Some(Ok(b'a')) | Some(Ok(68)) => {
                Tetris.moveBrick(Pos { x: 0, y: -1 });
                Tetris.print();
            }
            Some(Ok(b'd')) | Some(Ok(67)) => {
                Tetris.moveBrick(Pos { x: 0, y: 1 });
                Tetris.print();
            }
            Some(Ok(b's')) | Some(Ok(66)) => {
                if (Tetris.bottomCheck()) {
                    Tetris.moveBrick(Pos { x: 1, y: 0 });
                }
                Tetris.print();
            }
            Some(Ok(b'w')) | Some(Ok(65)) => {
                Tetris.rotate();
                Tetris.print();
            }
            None => (),
            _ => (),
        }

        if ElapsedTime.as_secs() >= 1 {
            if Tetris.bottomCheck() {
                if !Tetris.moveBrick(Pos { x: 1, y: 0 }) {
                    Tetris.addBrickToMap();
                    Tetris.point();
                    Tetris.onMapBrick = Tetris.nextBrick;
                    Tetris.nextBrick = Tetris.newBrick();
                }
            } else {
                Tetris.addBrickToMap();
                Tetris.point();
                Tetris.onMapBrick = Tetris.nextBrick;
                Tetris.nextBrick = Tetris.newBrick();
            }
            Tetris.print();
            Time = SystemTime::now();
        }
    }
}

/// this is the blocks that make up bricks
#[derive(Copy, Clone, PartialEq)]
struct Block {
    isFilled: bool,
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Copy, Clone)]
struct Pos {
    x: i8,
    y: i8,
}

/// this is the overall "Class" for all "subclasses" of tetris bricks
#[derive(Copy, Clone)]
struct Brick {
    ///shape is a 2d array of booleans symbolizing the shape of the brick
    shape: [[Block; 4]; 4], //todo add color variable
    position: Pos,
}

impl Brick {
    ///this function rotates the brick within the shape matrix.
    fn rotate(&mut self) {
        ///the temperary shape to caluclate the new rotated shape into
        let mut temp_shape: [[Block; 4]; 4] = [[emptyblock; 4]; 4];
        /// the translated axis, so that the brick rotates around the center piece.
        let mut x: i8;

        for (i, bolarr) in self.shape.iter().enumerate() {
            for (j, block) in bolarr.iter().enumerate() {
                x = (i as i8) - 1;
                x = x * -1;
                temp_shape[j][(x + 2) as usize] = *block;
            }
        }

        self.shape = temp_shape;
        let mut temp_shape: [[Block; 4]; 4] = [[emptyblock; 4]; 4];

        let mut trans_pos: Pos = Pos { x: 0, y: 0 };
        let mut temp_trans_point: Pos = Pos { x: 0, y: 0 };

        for (i, bolarr) in self.shape.iter().enumerate() {
            for (j, block) in bolarr.iter().enumerate() {
                if block.isFilled {
                    if i > temp_trans_point.x as usize {
                        temp_trans_point.x = i as i8;
                    }
                    if j > temp_trans_point.y as usize {
                        temp_trans_point.y = j as i8;
                    }
                }
                if temp_trans_point.x != 0 {
                    trans_pos.x = 3 - temp_trans_point.x;
                }
                if temp_trans_point.y != 0 {
                    trans_pos.y = 3 - temp_trans_point.y;
                }
            }
        }

        for (i, bolarr) in self.shape.iter().enumerate() {
            for (j, block) in bolarr.iter().enumerate() {
                if block.isFilled {
                    temp_shape[i + trans_pos.x as usize][j + trans_pos.y as usize] = *block;
                }
            }
        }
        self.shape = temp_shape;
    }

    ///prints the shape of the block to the terminal(mostly used for debug)
    //make print work with raw mode
    fn print(&self) {
        for i in self.shape.iter() {
            for j in i.iter() {
                if (*j).isFilled {
                    print!(
                        "{}#{}",
                        color::Fg(color::Rgb((*j).red, (*j).green, (*j).blue)),
                        color::Fg(color::Reset)
                    )
                } else {
                    print!(" ");
                }
            }
            print!("\n\r");
        }
    }
}

/// this struct defines a  of tetris
struct Game {
    /// tetris needs a map
    map: [[Block; 10]; 20],
    ///the brick that is currently falling down the map
    onMapBrick: Brick,
    ///the next brick to go on the block
    nextBrick: Brick,
    /// a brick saved for later usage
    savedBrick: Brick,
    /// the points in the game
    points: u64,
}

impl Game {
    /// prints the ui of the game.
    fn print(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        print!("points {}", self.points);
        print!("{}", termion::cursor::Goto(1, 2));
        self.nextBrick.print();
        print!("{}", termion::cursor::Goto(1, 6));
        for i in self.BrickMap().iter() {
            for j in i.iter() {
                if (*j).isFilled {
                    print!(
                        "{}#{}",
                        color::Fg(color::Rgb((*j).red, (*j).green, (*j).blue)),
                        color::Fg(color::Reset)
                    )
                } else {
                    print!(".");
                }
            }
            print!("\n\r");
        }
    }

    /// adds the onmapBrick to the map temporarily so that it can be displayed.
    fn BrickMap(&self) -> [[Block; 10]; 20] {
        let mut fused_map = self.map;
        for (i, Brickarr) in self.onMapBrick.shape.iter().enumerate() {
            for (j, onMapBlock) in Brickarr.iter().enumerate() {
                if (*onMapBlock).isFilled {
                    fused_map[(i as i8 + self.onMapBrick.position.x) as usize]
                        [(j as i8 + self.onMapBrick.position.y) as usize] = *onMapBlock;
                }
            }
        }
        return fused_map;
    }

    /// adds the onmapbrick permenently to the map for when the brick cannot move anymore.
    fn addBrickToMap(&mut self) {
        for (i, Brickarr) in self.onMapBrick.shape.iter().enumerate() {
            for (j, onMapBlock) in Brickarr.iter().enumerate() {
                if (*onMapBlock).isFilled {
                    self.map[(i as i8 + self.onMapBrick.position.x) as usize]
                        [(j as i8 + self.onMapBrick.position.y) as usize] = *onMapBlock;
                }
            }
        }
    }

    ///saves the onMapBrick to savedBrick
    fn saveBrick(&mut self) {
        self.savedBrick = self.onMapBrick;
        self.nextBrick.position = self.onMapBrick.position;
        self.onMapBrick = self.nextBrick;
        self.nextBrick = self.newBrick();
    }

    ///Generates a new brick from the standart 7 pieces
    fn newBrick(&self) -> Brick {
        let mut rng = rand::thread_rng();
        return Bricks[rng.gen_range(0..7)];
    }

    ///makes the onMapBrick the savedBrick and empties the savedBrick
    fn useSaved() {}

    ///cheks if the brick can rotate at it's current position on the map
    fn rotateCheck(&self) -> bool {
        let mut tempBrick = self.onMapBrick;
        tempBrick.rotate();
        for (i, blockarr) in tempBrick.shape.iter().enumerate() {
            for (j, tempBlock) in blockarr.iter().enumerate() {
                if tempBlock.isFilled {
                    if self.map[(i as i8 + tempBrick.position.x) as usize]
                        [(j as i8 + tempBrick.position.y) as usize]
                        .isFilled
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn rotate(&mut self) {
        if self.rotateCheck() {
            self.onMapBrick.rotate();
        }
    }

    ///checks if onMapBrick has hit the bottom of the map (or another brick) returns false if it cannot move.
    fn bottomCheck(&self) -> bool {
        if self.onMapBrick.position.x == 16 {
            return false;
        }

        for (i, tempBlock) in self.onMapBrick.shape[3].iter().enumerate() {
            if tempBlock.isFilled {
                let x = self.onMapBrick.position.x + 4;
                let y = self.onMapBrick.position.y + i as i8;
                if self.map[x as usize][y as usize].isFilled {
                    return false;
                }
            }
        }
        return true;
    }

    ///Checks if the onMapBrick can move anywhere.
    fn moveCheck(&self, place: Pos) -> bool {
        let mut tempBrick = self.onMapBrick;
        tempBrick.position = place;

        for (i, blockarr) in tempBrick.shape.iter().enumerate() {
            for (j, tempBlock) in blockarr.iter().enumerate() {
                if tempBlock.isFilled {
                    //print! ("{}",(tempBrick.position.x));
                    if (j as i8 + tempBrick.position.y) as usize >= 10 {
                        return false;
                    } else {
                        if self.map[(i as i8 + tempBrick.position.x) as usize]
                            [(j as i8 + tempBrick.position.y) as usize]
                            .isFilled
                        {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }

    /// moves the brick on the map from it's current position
    fn moveBrick(&mut self, Position: Pos) -> bool {
        if self.moveCheck(Pos {
            x: self.onMapBrick.position.x + Position.x,
            y: self.onMapBrick.position.y + Position.y,
        }) {
            self.onMapBrick.position.x += Position.x;
            self.onMapBrick.position.y += Position.y;
            return true;
        }
        return false;
    }

    /// calculates points and removes lines that are full
    fn point(&mut self) {
        let mut lines: u64 = 0;
        let mut temp_map = self.map.clone();
        //What the fuck am i doing! 2 TEMP MAPS THATS TOO MANY MAAANNNN!!!
        let mut temp_temp_map = self.map.clone();
        for (i, t) in self.map.iter().enumerate() {
            if !self.map[i].contains(&emptyblock) {
                lines += 1;
                temp_map[i]= [emptyblock;10];
                for (j,l) in self.map.iter().enumerate(){
                    if j <= i {
                        if j+1 != 20{
                            temp_map[j+1] = temp_temp_map[j]
                        }
                        
                    }else{
                        temp_map[j] = temp_temp_map[j]
                    }
                }
                temp_temp_map = temp_map;
            }
        }
        if lines > 0 {
            self.points += lines.pow(2);
            self.map = temp_map;
        }
        //fix
    }

    //this is poopy
    fn MapMovedown(&mut self, lines: usize) {
        let mut temp_map: [[Block; 10]; 20] = [[emptyblock; 10]; 20];
        for (i, t) in self.map.iter().enumerate() {
            for (j, l) in t.iter().enumerate() {
                if l.isFilled {
                    temp_map[i - lines][j] = self.map[i][j];
                }
            }
        }
    }
}
