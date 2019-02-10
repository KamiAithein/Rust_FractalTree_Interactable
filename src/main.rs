extern crate rand;
extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

mod fractal;
use crate::fractal::*;

mod tree;
use crate::tree::*;

use std::f64;


fn main(){
    let (width, height) = (1000, 900);
    let mut window: PistonWindow = WindowSettings::new(
        "fractal",
        [
            width,
            height,
        ],
    ).exit_on_esc(true).build().unwrap();
    
    let mut current = 4;
    let mut length = 10;
    let (mut x1, mut y1, mut x2, mut y2) = (500, 10, 500, length + 10);
    let mut theta = 1.5*(f64::consts::PI);
    let mut dt = (f64::consts::PI)/6.0;
    let mut dp = 0.0;

    let mut anim_rate = 100.0;

    let mut tree = Tree::new(current, length, x1, y1, x2, y2, theta, dt);
    let speed = 5;
    while let Some(event) = window.next(){
        let mut valid_key = true;
        if let Some(Button::Keyboard(key)) = event.press_args(){
            match key{
                Key::Left => {
                    current -= 1;
                },
                Key::Right => {
                    current += 1;
                },
                Key::Up =>{
                    length += 1;
                },
                Key::Down =>{
                    length -= 1;

                },
                Key::W =>{
                    y1 -= speed;
                    y2 -= speed;
                },
                Key::A =>{
                    x1 -= speed;
                    x2 -= speed;
                },
                Key::S =>{
                    y1 += speed;
                    y2 += speed;
                },
                Key::D =>{
                    x1 += speed;
                    x2 += speed;
                },
                Key::E =>{
                    theta += (f64::consts::PI)/10.0;
                }
                Key::Q =>{
                    theta -= (f64::consts::PI)/10.0;
                }
                Key::F=>{
                    dt -= (f64::consts::PI)/500.0;
                }
                Key::G=>{
                    dt += (f64::consts::PI)/500.0;
                }
                Key::Plus=>{
                    anim_rate += 10.0;
                }
                Key::Minus=>{
                    anim_rate -= 10.0;
                }
                _ => {
                    valid_key = false;
                }
            };
            if(valid_key){
                tree = Tree::new(current, length, x1, y1, x2, y2, theta, dt);
            }
        }
        window.draw_2d(&event, |c, g|{
            dp -= (f64::consts::PI)/(anim_rate);
            clear([0.0,0.0,0.0,0.0], g);
            run(&c, g, &mut tree, dp as f32);
        });
        event.update(|args|{

        });
    }
}