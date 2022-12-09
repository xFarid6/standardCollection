use rand::Rng;
use std::io::stdin;

fn main()
{
    // generate a number between 1 and 100
    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess the number: ");
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                // parsed should be a u32
                let parsed = buffer.trim().parse::<u32>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Please enter a number between 1 and 100");
                            continue;
                        } else if guess < number {
                            println!("Too low");
                            continue;
                        } else if guess > number {
                            println!("Too high");
                            continue;
                        } else {
                            println!("You win!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error parsing: {}", e); // TODO: remove
                        println!("Please enter a number");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line
{
    fn len(&self) -> f64
    {
        let x_diff = self.start.x - self.end.x;
        let y_diff = self.start.y - self.end.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
}

fn methods()
{
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 12.0 };
    let my_line = Line { start: p1, end: p2 };

    println!("Length of line: {}", my_line.len());
}

/////////////////////////////////////

fn say_hello() { println!("Hello!"); }

fn closures()
{
    // way 1: plain function call
    say_hello();

    // way 2: assign the function to a variable
    let sh = say_hello;
    sh();

    // way 3: pass the function as an argument
    let plus_one = |x: i32| -> i32 {x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    let plus_two = |x|
    {
        let mut z = x;
        z += two;
        return z
    };
    // let borrow_two = &mut two; // error: cannot borrow `two`, it is inside the closure now
    println!("{} + 2 = {}", a, plus_two(a));

    // pass arguments by value
    let mut num = 5;
    {
        let add_num = move |x: &mut i32| *x += num;
        add_num(&mut num);
    }

    println!("{} + {} = {}", a, num, plus_two(a));
}

/////////////////////////////////////

fn is_even(x: u32) -> bool { x % 2 == 0 }

fn greater_than(limit: u32) -> impl Fn(u32) -> bool
{
    move |y| y > limit
}

fn higher_order_functions()
{
    // function that take functions
    // f(g) { let x = g(); f(x) }

    // function that return functions
    // f() -> g { g() } ; also called generators


    // sum of all even squares that are less than limit
    let limit = 500;
    let mut sum = 0;

    let above_limit = |x| x > limit; // closure that captures limit

    // imperative style
    for i in 0.. {
        let i_squared = i * i;
        // if i_squared > limit { break; }
        if above_limit(i_squared) { break; }
        else if i_squared % 2 == 0 { sum += i_squared; }
    }

    println!("imperative style, loop sum is: {}", sum);

    // functional style
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x); // fold is like reduce in JS
                                                    // fold(initial_value, function)
                                                    // fold(0, |sum, x| sum + x) is the same as
                                                    // fold the sequence in a signle value
                                                    // performing a pairwise operation
                                                    // specify the function to apply to the accumulator

    println!("functional style, loop sum is: {}", sum2);
}

/////////////////////////////////////

trait Animal // when defining a class we can choose to implement this trait
{
    fn create(name: &'static str) -> Self; // Self is the type of the class that implements this trait

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str,
}

struct Cat
{
    name: &'static str,
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human { name: name }
    }

    fn name(&self) -> &'static str { self.name }

    fn talk(&self)
    {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat { name: name }
    }

    fn name(&self) -> &'static str { self.name }

    fn talk(&self)
    {
        println!("{} says meow", self.name);
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut sum = 0;
        for i in self { sum += *i; }
        return sum;
    }
}

fn traits()
{
    // traits are like interfaces in other languages
    // traits are used to define shared behavior in an abstract way
    // traits can be implemented for any type
    // traits can be used as bounds to specify that a generic type may be any type that implements a trait

    let person = Human { name: "John" };
    person.talk();

    let cat = Cat { name: "Misty" };
    cat.talk();

    let human2: Human = Animal::create("Bob");
    human2.talk();

    let cat2: Cat = Animal::create("Misty");
    cat2.talk();

    // you can add traits to type you don't own, which you haven't created
    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum()); // sum is a trait method
}

/////////////////////////////////////

use std::fmt::Debug;

#[derive(Debug)] // this is a macro that implements the Debug trait
struct Circle { radius: f64 }

#[derive(Debug)]
struct Square { side: f64 }

trait Shape { fn area(&self) -> f64; }

impl Shape for Circle
{
    fn area(&self) -> f64 { std::f64::consts::PI * (self.radius * self.radius) }
}

impl Shape for Square
{
    fn area(&self) -> f64 { self.side * self.side }
}

// there are 3 ways of specifying a trait passed as a parameter

// 1. Trait as a parameter
fn print_info(shape: impl Shape + Debug) 
{
    println!("{:?}", shape); // since there is this line it requires to impl the Debug Trait
    println!("This shape has an area of {}", shape.area());
}

// 2. Trait bound syntax, more concise for when you have multiple parameters
fn print_info2<T: Shape + Debug>(shape: T, shape2: T)
{
    println!("{:?}", shape);
    println!("This shape has an area of {}", shape.area());
}

// 3. where clause, more concise when it implements more traits
fn print_info3<T>(shape: T) where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("This shape has an area of {}", shape.area());
}

/////////////////////////////////////

struct Person
{
    name: String,
}

impl Person
{
    // this is the FACTORY PATTERN
    fn new(name: &str) -> Person
    {
        Person { name: name.to_string() }
    }

    // implement the new function again
    // using the Trait builder and the Into trait
    // which allows automatic conversion when possible
    fn new2<S: Into<String>>(name: S) -> Person
    {
        Person { name: name.into() }
    }

    // another way of specifying the same thiing is to use the Where clause
    fn new<S>(name: S) -> Person where S: Into<String>
    {
        Person { name: name.into() }
    }
}

// fn main()
// {
    // let jhon = Person::new("Jhon");
    // let name: String = "Jane".to_string();
    // let jane = Person::new(&name/*.as_ref()*/);
// }

/////////////////////////////////////

struct Creature
{
    name: String,
    attack: i32,
    defense: i32,
}

impl Creature {
    fn new(name: &str, attack: i32, defense: i32) -> Creature
    {
        println!("{} has been created", name);
        Creature { name: name.to_string(), attack, defense }
    }
}

impl Drop for Creature
{
    fn drop(&mut self)
    {
        println!("{} has been destroyed", self.name);
    }
}

fn main()
{
    let goblin = Creature::new("Goblin", 10, 5);
    println!("{} has {} attack and {} defense", goblin.name, goblin.attack, goblin.defense);
    drop(goblin); // drop is a function that calls the drop method of the object
    // when the object is dropped the drop method is called
    // the drop method is called when the object goes out of scope
    // if you call it yourself it is deterministic finalization
    // you control when the object is destroyed
}

/////////////////////////////////////

use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Complex<T>
{
    re: T,
    im: T,
}

impl<T> Complex<T>
{
    fn new(re: T, im: T) -> Complex<T>
    {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
where T: Add<Output = T>
{
    // You have to specify this thype because the compiler can't infer it
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output
    {
        Complex::<i32> { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> AddAssign for Complex<T>
where T: AddAssign
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
where T: Neg<Output = T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output
    {
        Complex::<T> { re: -self.re, im: -self.im }
    }
}

// partial eq
// full eq x = x
// NAN = not a number, for floating point numbers 0/0 inf/inf
// NAN == NAN -> false

impl<T> PartialEq<T> for Complex<T> 
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool
    {
        self.re == rhs && self.im == rhs.im
    }

    fn ne(&self, rhs: &Self) -> bool
    {
        self.re != rhs && self.im != rhs.im
    }
}

// to implement full eq is not necessary if you already have partialeq, as it is deduced
impl<T: Eq> Eq for Complex<T> where T: Eq {}

fn main()
{
    let a = Complex::new(1, 2);
    let b = Complex::new(3, 4);
    let c = a + b;
    println!("{:?}", c);

    let d = Complex::new(1.0, 2.0);
    let e = Complex::new(3.0, 4.0);
    let f = d + e;
    println!("{:?}", f);

    let mut g = Complex::new(1, 2);
    let h = Complex::new(3, 4);
    g += h;
    println!("{:?}", g);

    let i = Complex::new(1, 2);
    let j = -i;
    println!("{:?}", j);
}

/////////////////////////////////////

trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("String: {}", *self)
    }
}

fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format());
}

fn main()
{
    let a = 123;
    let b = "hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());

    print_it(a);
    print_it(b);
}

/////////////////////////////////////

fn print_it_too(z: &Printable)
{
    println!("{}", z.format());
}

fn main()
{
    let a = 123;
    let b = "hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());

    print_it_too(&a);
    print_it_too(&b);
}

/////////////////////////////////////

trait Animal {
    // static: called as Animal::create()
    // returns the type of the implementor
    // fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str,
}

impl Animal for Human
{
    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }
}

struct Cat
{
    name: &'static str,
}

impl Animal for Cat
{
    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main()
{
    let mut creatures: Vec<Creature> = Vec::new();
    // creatures.push(Human { name: "John" });
    // creatures.push(Cat { name: "Misty" }); // panic! vec is of humans now
    creatures.push(Creature::Human(
        Human { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Misty" }
    ));

    for c: Creature in creatures: Vec<Creature>
    {
        match c
        {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    // but you can just initialize the vector with the Trait
    // need to have the Sized trait to be able to push
    // as can't push objects of unknkown size
    let mut animals: Vec<Box<Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "John" }));
    animals.push(Box::new(Cat { name: "Misty" }));

    for a: Box<Animal> in animals.iter()
    {
        a.talk();
    }
}

/////////////////////////////////////