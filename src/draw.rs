use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x: f64 = to_coord(x);
    let gui_y: f64 = to_coord(y);

    rectangle(      
        color,      //颜色
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],     //画一个正方形需要的数据
        con.transform,          //上下文转换成了什么？
        g,          //图像二维缓冲区
    );
}

pub fn draw_rectangle(  //为什么这里新建一个函数？和上面有什么区别？
    color: Color,           //这里可以调节画出来的矩阵大小，通过width和h
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x :f64 = to_coord(x);
    let y :f64 = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}