#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl - indicates we're providing implementation of behaviours for Rectangle
// there can be multiple impl blocks for a struct
impl Rectangle {
    // Methods on a struct must have 'self' as the first parameter, this can be 
    //  A) a mutable reference (&mut self)
    //  B) an immutable reference (&self)
    //  C) a reference that takes ownership of the struct (self)
    //      this is rare, but can be useful if the method transforms self into something else,
    //      and therefore we don't want anyone to be able to access the old struct.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width >= other_rectangle.width && self.height >= other_rectangle.height
    }

    // Associated functions for structs do not take a 'self' parameter, a
    // common use case is in defining constructors, that return an instance 
    // of the struct.
    fn square(dimension: u32) -> Rectangle {
        Rectangle {
            width: dimension,
            height: dimension
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60
    };

    println!("Rectangle 1 can hold Rectangle 2? {}", rect1.can_hold(&rect2));
    println!("Rectangle 2 can hold Rectangle 1? {}", rect2.can_hold(&rect1));    

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let square = Rectangle::square(2);
    println!("We have just used an associated function to make a square: \n{:#?}", square);
}