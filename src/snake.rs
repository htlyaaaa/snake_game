use std::collections::LinkedList;
use piston_window::{Context, G2d};//上下文和缓冲区
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.20, 0.80, 0.10, 1.0];
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//将方向反过来
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x+2,
            y,
        });
        body.push_back(Block {
            x: x+1,
            y,
        });
        body.push_back(Block {
            x: x,
            y,
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None, 
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }//这里的 draw 方法只需要对 self.body 进行读取操作，因此使用不可变引用 &self.body 更为合适
    }   

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
       //( (*head_block).x  , (*head_block).y )
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },//向上走时y轴反而减一，这看似颠倒的逻辑，其实是因为坐标原点处在屏幕的左上方，和平常的认知相反
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);//蛇在移动时，本质是移除最后一个块，增加了正面的块
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction//实现clone的trait后，实际上是克隆了一个方向 并移走
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,  //用传进来的新方向，替代蛇原本的方向
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
    //恢复尾巴，实际上是蛇增长的方法
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return  true;   //蛇头与蛇身重复，返回真
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;//漏掉不检测尾巴的块，目的是
            }//某个模棱两可的时刻：衔尾蛇，蛇头过去了，蛇尾还没移走
        }
        return false;
    }
}

