extern crate termion;

use std::io::{stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color};

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

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let redblock = Block {
        isFilled: true,
        red: 255,
        green: 0,
        blue: 0,
    };

    let mut IBlock = Brick {
        shape: [
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
            [emptyblock, emptyblock, emptyblock, redblock],
        ],
        position: Pos { x: 0, y: 0 },
    };

    let mut Tetris = Game {
        map: [[emptyblock; 10]; 20],
        onMapBrick: IBlock,
        points: 0,
        nextBrick: emptybrick,
        savedBrick: emptybrick,
    };

    let mut Time = SystemTime::now();

    loop {
        let ElapsedTime = Time.elapsed().unwrap();

        match stdin.next() {
            Some(Ok(b'q')) => break,
            Some(Ok(b'a')) => Tetris.moveBrick(Pos{x:0,y:-1}),
            Some(Ok(b'd')) => Tetris.moveBrick(Pos{x:0,y:1}),
            Some(Ok(b's')) => Tetris.moveBrick(Pos{x:1,y:0}),
            Some(Ok(b'w')) => Tetris.rotate(),
            _ => (),
        }

        if ElapsedTime.as_secs() >= 1 {
            Tetris.moveBrick(Pos { x: 1, y: 0 });
            Tetris.print();
            stdout.flush();
            Time = SystemTime::now();
        }
    }
}

/// this is the blocks that make up bricks
#[derive(Copy, Clone)]
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
    }

    ///prints the shape of the block to the terminal(mostly used for debug)
    //make print work with raw mode
    fn print(&self) {
        for i in self.shape.iter() {
            for j in i.iter() {
                if (*j).isFilled {
                    print!(
                        "{}#",
                        color::Fg(color::Rgb((*j).red, (*j).green, (*j).blue))
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
                    fused_map[i + self.onMapBrick.position.x as usize]
                        [j + self.onMapBrick.position.y as usize] = *onMapBlock;
                }
            }
        }
        return fused_map;
    }

    /// adds the onmapbrick permenently to the map for when the brick cannot move anymore.
    fn addBrickToMap() {}

    ///saves the onMapBrick to savedBrick
    fn saveBrick(&mut self) {
        self.savedBrick = self.onMapBrick;
        self.nextBrick.position = self.onMapBrick.position;
        self.onMapBrick = self.nextBrick;
        self.nextBrick = self.newBrick();
    }

    ///Generates a new brick from the standart 7 pieces
    fn newBrick(&self) -> Brick {
        emptybrick
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
                    if self.map[i + tempBrick.position.x as usize]
                        [j + tempBrick.position.y as usize]
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
                let x = self.onMapBrick.position.x + 3;
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
                    if self.map[i + tempBrick.position.x as usize]
                        [j + tempBrick.position.y as usize]
                        .isFilled
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// moves the brick on the map from it's current position
    fn moveBrick(&mut self, Position: Pos) {
        if self.moveCheck(Pos {
            x: self.onMapBrick.position.x + Position.x,
            y: self.onMapBrick.position.y + Position.y,
        }) {
            self.onMapBrick.position.x += Position.x;
            self.onMapBrick.position.y += Position.y;
        }
    }

    /// calculates points and removes lines that are full
    fn point() {}
}
