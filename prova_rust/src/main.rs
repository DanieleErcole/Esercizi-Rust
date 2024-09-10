
trait Printable {
    fn to_my_string(&self) -> String;
}

struct PrintableVector {
    elements: Vec<Box<dyn Printable>>
}

impl PrintableVector {

    fn new() -> Self {
        return PrintableVector { 
            elements: vec![]
        }
    }

    fn add(&mut self, el: Box<dyn Printable>) {
        self.elements.push(el);
    }

}

impl Printable for PrintableVector {

    fn to_my_string(&self) -> String {
        let mut str: String = String::new();
        for el in self.elements.iter() {
            str.push_str(&el.to_my_string());
            str.push_str(" - ");
        }
        return str;
    }

}


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {

    fn new(w: i32, h:i32) -> Self {
        return Rectangle {
            width: w,
            height: h
        }
    }

    fn new_from(other: &Rectangle) -> Self {
        return Self {
            width: other.width,
            height: other.height
        }
    }

}

impl Printable for Rectangle {

    fn to_my_string(&self) -> String {
        let mut str = String::new();
        str.push_str("w: ");
        str.push_str(&self.width.to_string());
        str.push_str(" h: ");
        str.push_str(&self.height.to_string());
        return str;
    }

}

fn main() {
    let r1: Rectangle = Rectangle::new(200, 300);
    let r2: Rectangle = Rectangle::new(400, 200);
    let r3: Rectangle = Rectangle::new_from(&r2);

    println!("R1 - w:{:?} h:{:?}", r1.width, r1.height);
    println!("R2 - w:{:?} h:{:?}", r2.width, r2.height);
    println!("R3 - w:{:?} h:{:?}", r3.width, r3.height);
    println!("---- Vector");

    let mut v = PrintableVector::new();
    v.add(Box::new(r1));
    v.add(Box::new(r2));
    v.add(Box::new(r3));
    println!("{:?}", v.to_my_string());
}
