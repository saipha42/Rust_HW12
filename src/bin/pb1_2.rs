use std::{f32::consts::PI, i32};


#[derive(Debug)]
struct Circle {

    x: i32,
    y: i32,
    r: i32
}

#[derive(Debug)]
struct Rectangle {

    x: i32,
    y: i32,
    w: i32,
    h : i32
}


// Added triangle struct
#[derive(Debug)]
struct Triangle {

    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32)
}




trait Shape {

    fn rep_string(&self) -> String;
    fn area(&self) -> f32;
    fn cpy(&self)-> Box<dyn Shape>;
}

impl  Clone for Box<dyn Shape> {
    
    fn clone(&self) -> Box<dyn Shape> {
        self.cpy()
    }
}



impl Shape for Circle {

    fn rep_string(&self)->String {
        
        format!("<Circle: {}, {}, {}>",self.x, self.y, self.r)
        
    }

    fn area(&self)-> f32 {
        format!("{:.2}",PI * self.r as f32 * self.r as f32).parse::<f32>().unwrap()
    }

    fn cpy(&self)-> Box<dyn Shape> {
        Box::new(
            Circle {
                x: self.x,
                y: self.y,
                r: self.r
            }
        )
    }
}

impl Circle {
    
    fn new(x : i32, y: i32, r: i32)-> Box<dyn Shape>
     {
        Box::new(
            Circle { x : x, y: y, r : r}
        )
    }
}


impl Shape for Rectangle {

    fn rep_string(&self)->String {
        
        format!("<Rectangle: {}, {}, {}, {}>",self.x, self.y, self.w, self.h)
        
    }

    fn area(&self)-> f32 {
        format!("{:.2}",(self.w * self.h) as f32).parse::<f32>().unwrap()
    }


    fn cpy(&self)-> Box<dyn Shape> {
        Box::new(
            Rectangle {
                x: self.x,
                y: self.y,
                w: self.w,
                h: self.h
            }
        )
    }

}

impl Rectangle {
    
    fn new(x : i32, y: i32, w: i32, h: i32)-> Box<dyn Shape> {
        Box::new(
            Rectangle {x, y , w, h}
        )
    }
}




// Implementation for triangle struct

impl Shape for Triangle {

    fn rep_string(&self)->String {
        
        format!("<Triangle: {:?}, {:?}, {:?}>",self.p1, self.p2, self.p3)
    }

    fn area(&self)-> f32 {
        let (x1, y1) = self.p1;
        let (x2, y2) = self.p2;
        let (x3, y3) = self.p3;
        0.5 * ((x1 - x3)*(y2 - y1) - (x1 - x2)*(y3 - y1)) as f32
    }

    fn cpy(&self)-> Box<dyn Shape> {
        Box::new(
            Triangle {
                p1: self.p1,
                p2: self.p2,
                p3: self.p3
            }
        )
    }

}

impl Triangle {
    
    fn new(p1: (i32, i32), p2 : (i32, i32), p3 : (i32, i32))-> Box<dyn Shape> {
        Box::new(
            Triangle {
                p1,
                p2,
                p3
            }
        )
    }
}





#[test]
fn test_shapes_001() {

    fn input_shape_list() -> Vec<Box<dyn Shape>> {
        vec![
            Circle::new(0, 0, 1), Circle::new(50, 50, 15),
            Rectangle::new(40, 40, 20, 20), Rectangle::new(10, 40, 15, 10),
            Triangle::new((0,0), (10,0), (5,10)), Triangle::new((0,0), (20,0), (15,10))
        ]
    }

    const EXPECTED_001: &[&str] = &[
        "<Circle: 0, 0, 1>", "<Circle: 50, 50, 15>",
        "<Rectangle: 40, 40, 20, 20>", "<Rectangle: 10, 40, 15, 10>",
        "<Triangle: (0, 0), (10, 0), (5, 10)>",
        "<Triangle: (0, 0), (20, 0), (15, 10)>",
    ];


    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}


#[test]
fn test_shapes_002() {
    fn input_shape_list() -> Vec<Box<dyn Shape>> {
        vec![
            Circle::new(0, 0, 1), Circle::new(50, 50, 15),
            Rectangle::new(40, 40, 20, 20), Rectangle::new(10, 40, 15, 10),
            Triangle::new((0,0), (10,0), (5,10)), Triangle::new((0,0), (20,0), (15,10))
        ]
    }

    let shape_list = input_shape_list();
    const EXPECTED_002: &[&str] = &[
        "<Circle: 0, 0, 1>, area: 3.14",
        "<Circle: 50, 50, 15>, area: 706.86",
        "<Rectangle: 40, 40, 20, 20>, area: 400.00",
        "<Rectangle: 10, 40, 15, 10>, area: 150.00",
        "<Triangle: (0, 0), (10, 0), (5, 10)>, area: 50.00",
        "<Triangle: (0, 0), (20, 0), (15, 10)>, area: 100.00",
        ];
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}



#[test]
fn test_shapes_003() {

    fn input_shape_list() -> Vec<Box<dyn Shape>> {
        vec![
            Circle::new(0, 0, 1), Circle::new(50, 50, 15),
            Rectangle::new(40, 40, 20, 20), Rectangle::new(10, 40, 15, 10),
            Triangle::new((0,0), (10,0), (5,10)), Triangle::new((0,0), (20,0), (15,10))
        ]
    }

    const EXPECTED_002: &[&str] = &[
        "<Circle: 0, 0, 1>, area: 3.14",
        "<Circle: 50, 50, 15>, area: 706.86",
        "<Rectangle: 40, 40, 20, 20>, area: 400.00",
        "<Rectangle: 10, 40, 15, 10>, area: 150.00",
        "<Triangle: (0, 0), (10, 0), (5, 10)>, area: 50.00",
        "<Triangle: (0, 0), (20, 0), (15, 10)>, area: 100.00",
        ];

    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}



fn main() {

    
    let circle = Circle::new(0, 0, 10);
    let cloned_circle: Box<dyn Shape> = circle.clone();
    println!("{}, Area: {}", cloned_circle.rep_string(), cloned_circle.area());

    let rect = Rectangle::new(0, 0, 10, 10);
    let cloned_rect = rect.clone();
    println!("{}, Area: {}", cloned_rect.rep_string(), cloned_rect.area());

    let triangle = Triangle::new((0, 0), (10, 0), (5, 10));
    let cloned_triangle = triangle.clone();
    println!("{}, Area: {}", cloned_triangle.rep_string(), cloned_triangle.area())
}