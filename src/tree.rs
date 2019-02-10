use std::collections::LinkedList;
use piston_window::{line, Context, G2d};
use piston_window::types::Color;
use std::f64;

extern crate rand;
use rand::Rng;

const PI: f64 = (f64::consts::PI);

pub struct Tree{
    root: Node, 
    pub is_generated: bool,
    number: i32,
    theta: f64,
    dt: f64,
}
impl Tree{
    pub fn new(number: i32, length: i32, x1: i32, y1: i32, x2: i32, y2: i32, theta: f64, dt: f64) -> Tree{
        Tree{
            root: Node::new(
                LinkedList::new(),
                Point::new(x1,y1), //p1
                Point::new(x2,y2),  //p2
                length,
            ),
            is_generated: false,
            number,
            theta,
            dt
        }
    }
    pub fn generate(&mut self){
        self.root.step(0, self.theta - PI/2.0, self.number, self.dt);
        self.is_generated = true;
    }
    pub fn draw(&mut self, con: &Context, g: &mut G2d, dp: f32){
        self.root.draw(con, g, 0, dp);
    }
    
}
#[derive(Clone)]
struct Point{
    x: i32,
    y: i32,
}
impl Point{
    pub fn new(x: i32, y: i32) -> Point{
        Point{
            x,
            y,
        }
    }
}
#[derive(Clone)]
struct Node{
    children: LinkedList<Node>,
    point1: Point, 
    point2: Point,
    length: i32,
}
impl Node{
    pub fn new(children: LinkedList<Node>, point1: Point, point2: Point, length: i32) -> Node{
        Node{
            children,
            point1,
            point2,
            length,
        }
    }
    fn clone_point1(&self) -> Point{
        self.point1.clone()
    }
    fn clone_point2(&self) -> Point{
        self.point2.clone()
    }
    fn add(&mut self, child: Node){
        self.children.push_back(child);
    }
    fn step(&mut self, mut level: i32, mut theta: f64, number: i32, dt:f64){
        level += 1;

        

        let point2 = self.clone_point2();

        let child1_x =  point2.x + (self.length as f64 * (theta + dt).sin()) as i32;
        let child1_y = point2.y - (self.length as f64 * (theta + dt).cos())  as i32;
        
        let child2_x =  point2.x + (self.length as f64 *(theta - dt).sin()) as i32;
        let child2_y = point2.y - (self.length as f64 * (theta - dt).cos()) as i32;

        let mut child1 = Node::new(LinkedList::new(), point2.clone(), Point::new(child1_x, child1_y), self.length);
        let mut child2 = Node::new(LinkedList::new(), point2.clone(), Point::new(child2_x, child2_y), self.length);

        if level < number {
            child1.step(level, theta + dt, number, dt);
            child2.step(level, theta - dt, number, dt);
        }
        
        self.add(child1);
        self.add(child2);
    }
    pub fn draw(&mut self, con: &Context, g: &mut G2d, level: i32, dp: f32){
        if self.point1.x < 1000 && self.point1.x > 0 &&
        self.point2.x < 1000 && self.point2.x > 0 &&
        self.point1.y < 1000 && self.point1.y > 0 &&
        self.point2.y < 1000 && self.point2.y > 0
        {
            let sin = |x: f32, p: f32| -> f32{
                0.5*((x*0.25 + p + dp).sin()) + 0.5
            };
            let dr = sin(level as f32, (3.0*PI/2.0) as f32);
            let dg = sin(level as f32, (PI/2.0) as f32);
            let db = sin(level as f32, 0.0);
            line(
                [1.0*dr,1.0*dg,1.0*db,1.0],
                1.0,
                [
                    self.point1.x as f64,
                    self.point1.y as f64,
                    self.point2.x as f64,
                    self.point2.y as f64,
                ],
                con.transform,
                g,
            );
            for n in &mut self.children.clone(){
                n.draw(con, g, level +1, dp);
            }
        }

    }

}

