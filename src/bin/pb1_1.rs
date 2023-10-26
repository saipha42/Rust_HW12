use std::f32::consts::PI;



#[derive(Debug)]
enum Shape {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
    Triangle((i32, i32), (i32, i32), (i32, i32))
}


impl Shape {
    
    fn rep_string(&self)->String {
        match self {
            
            Shape::Circle(x,y , r )=> {
                format!("<Circle: {}, {}, {}>",x, y, r)
            },
            Shape::Rectangle(x, y, w, h )=> {
                format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h)
            },
            Shape::Triangle(p1, p2, p3) => {
                format!("<Triangle : {:?}, {:?}, {:?}>", p1, p2, p3)
            }
        }
    }

    fn area(&self)-> f32 {

        match self {
            
            Shape::Circle(_x,_y , r )=> {
                format!("{:.2}",(PI * ((r*r) as f32))).parse::<f32>().unwrap()
            },
            Shape::Rectangle(_x, _y, w, h )=> {
                format!("{:.2}",((w*h) as f32)).parse::<f32>().unwrap()
            },
            Shape::Triangle((x1, y1), (x2, y2), (x3, y3)) => {
                0.5 * ((x1 - x3)*(y2 - y1) - (x1 - x2)*(y3 - y1)) as f32
            }
        }

    }
}

#[test]
fn test_shapes() {
    const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1), Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20), Shape::Rectangle(10, 40, 15, 10),
    Shape::Triangle((0,0), (10,0), (5,10)), Shape::Triangle((0,0), (20,0), (15,10))
    ];
    const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle : (0, 0), (10, 0), (5, 10)>, area: 50.00",
    "<Triangle : (0, 0), (20, 0), (15, 10)>, area: 100.00",

    ];

    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}


fn main() {

    let circle = Shape::Circle(0, 0, 5);
    println!("{}, area : {}", circle.rep_string(), circle.area());

    let rect = Shape::Rectangle(0,0,10,10);
    println!("{}, area : {}", rect.rep_string(), rect.area());

    let triangle = Shape::Triangle((0,0), (10,0), (5,10));
    println!("{}, area : {}", triangle.rep_string(), triangle.area());
}

