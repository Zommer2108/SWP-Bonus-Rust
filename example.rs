
///////////////////////////////
// Ownership + Borrowing

fn example1(x : i32) -> i32 {
   let y = x;  // Local type inference.

   // y = 1;   // Variables are immutable by default.

  return x+y;
}


fn example2(x : i32) -> i32 {
   let mut y = x;  // Mutable variables must be declared explicitly.

   let z = y;

   y = 1;

  return x+y+z;
}


fn example3(x : String) -> String {
   let y = x.clone();

   println!("{}",y);

   return x;
}

#[derive(Clone,Copy)]
struct Rectangle {
    x : i32,
    y : i32
}

fn example4() {
    let p = Rectangle{x : 1, y : 2};
    let q = p;

   println!("{} {}",q.x, p.x);

}

struct Square {
    x : i32
}

fn example5() {
    let p = Square{x : 1};
    let q = &p;

    println!("{} {}",q.x, p.x);

}

fn example6() {
    let p = Square{x : 1};
    let q = &p;

    println!("{} {}",q.x, p.x);

}

/*
fn example7() {
    let mut p = Square{x : 1};
    let q = &p;

    p.x = 2;
    println!("{} {}",q.x, p.x);
}
*/

fn example8() {
    let mut p = Square{x : 1};
    {
    let q = &p;

        println!("{} {}",q.x, p.x);
    }

    p.x = 2;

    println!("{}",p.x);

}

//////////////////////////
// Traits

trait Shape {
    fn area(s : &Self) -> i32;
}

impl Shape for Rectangle {
    fn area(r : &Rectangle) -> i32 {
        return r.x * r.y;
    }
}

impl Shape for Square {
    fn area(s : &Square) -> i32 {
     return s.x * s.x;
    }
}

fn sum_area<A:Shape,B:Shape>(x : &A, y : &B) -> i32 {
   return Shape::area(x) + Shape::area(y);
}


fn example9() {
    let r = Rectangle{x : 1, y : 2};
    let s = Square{x : 3};

    println!("{}",sum_area(&r,&s));

}

// Method notation known from OO.

trait Shape2 {
    fn area2(&self) -> i32;
}

impl Shape2 for Rectangle {
    fn area2(&self) -> i32 {
        return self.x * self.y;
    }
}

impl Shape2 for Square {
    fn area2(&self) -> i32 {
     return self.x * self.x;
    }
}


fn sum_area2<A:Shape2,B:Shape2>(x : &A, y : &B) -> i32 {
   return x.area2() + y.area2();
}


fn example10() {
    let r = Rectangle{x : 1, y : 2};
    let s = Square{x : 3};

    println!("{}",sum_area2(&r,&s));

}


fn sum_area3(x : Box<dyn Shape2>, y : Box<dyn Shape2>) -> i32 {
    return x.area2() + y.area2();
}


fn example11() {
    let r = Box::new(Rectangle{x : 1, y : 2});
    let s = Box::new(Square{x : 3});

    println!("{}",sum_area3(r,s));

}

////////////////////////////////////////////////
// Data types and pattern matching in Rust

pub enum Exp {
    Int {
        val: i32
    },
    Plus {
        left: Box<Exp>,
        right: Box<Exp>
    },
    Mult{
        left: Box<Exp>,
        right: Box<Exp>
    },
}

fn eval(e : &Exp) -> i32 {
    return match e {
        Exp::Int { val } => *val,
        Exp::Plus { left, right } => eval(left) + eval(right),
        Exp::Mult { left, right } => eval(left) * eval(right),
    }
}

fn example12() {
   {
       let e = Exp::Int { val : 1 };
       println!("{}", eval(&e));
   }

   {
       let e = Exp::Plus{left : Box::new(Exp::Int { val : 1 }), right : Box::new(Exp::Int { val : 2})};
       println!("{}", eval(&e));
   }
}

fn main() {

    println!("Hello {} {}", 5, "World!");  //  Type of placeholders inferred.

    println!("{}", example1(5));

    println!("{}", example2(5));

    println!("{}", example3(String::from("Hallo")));

    example4();

    example5();

    example6();

    example8();

    example9();

    example10();

    example11();

    example12();

}
