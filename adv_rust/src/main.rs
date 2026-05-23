#[derive(Debug)]
struct Rectangle {
    length:u8,
    breadth:u8,
}

impl Rectangle {
    fn new(length:u8, breadth:u8) -> Self{
        Rectangle { length:length, breadth:breadth }
    }

    fn area(&self) -> u8{
        self.length * self.breadth
    }
}

// fn calculate_area(dim:&Rectangle) -> u8{
//     return dim.length * dim.breadth;
// }

fn main(){
    // let rec1:Rectangle = Rectangle { length: 2, breadth: 5 };
    // let rec2:Rectangle = Rectangle { length: 1, breadth: 4 };

    let rec1 = Rectangle::new(8, 10);
    println!("Rectangle : {:?}", rec1);
    let a1 = rec1.area();
    println!("Area : {}", a1);

    // let a1:u8 = calculate_area(&rec1);
    // let a2  = calculate_area(&rec2);
    // println!("R1 : {:?} , R2 : {:?}", rec1, rec2);
    // println!("Area 1 : {} and Area 2 : {}", a1, a2);
}