fn vectors()
{
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:?}", v);
    v.push(9);
    println!("v = {:?}", v);
    println!("v[0] = {}", v[0]);

    // usize, isize
    let idx: usize = 0; // 0..=usize::MAX
    println!("v[idx] = {}", v[idx]); // can't arbitrarly choose index type here. Must be that of the system

    // let does_not_exist = &v[100]; // panic, only way to crash in Rust 

    // so you should use, for vectors, get() method
    // which return an Option<&T>
    match v.get(100) {
        Some(x) => println!("v[100] = {}", x),
        None => println!("There is no element 100"),
    }

    // iterating over the values in a vector
    for x in &v { println!("{}", x); }

    // iterating and mutating vector in the same loop
    for x in &mut v { *x += 50; }

    // deleting the last element
    let last = v.pop(); // returns an Option<T>
    println!("last = {:?}", last);

    // deleting the first element
    let first = v.remove(0); // returns T

    // deleting all elements
    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    // this part is about the ownership of the vector
    // and the elements it contains
    let v = vec![1, 2, 3, 4];

    // here the third element is borrowed immutably
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    // the vec! macro can be used to initialize a vector
    // with a given set of values
    // is different from Vec::new() because it can infer the type of the vector
    // from the type of the elements it contains
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v = {:?}", v);
}

//////////////////////////////////////////////////////

use std::collections::HashMap;

fn hash_map()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);
    
    // if you use a key that doesn't exist, you get a panic
    // println!("a circle has {} sides", shapes["circle".into()]);

    // you can iterate a hashmap with a for loop, but it is slow
    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    // you can overwrite a value
    shapes.insert(String::from("square"), 5);
    println!("a square has {} sides", shapes["square".into()]);

    // you can only insert a value if the key has no value
    shapes.entry(String::from("circle")).or_insert(1);

    // separate scope because we'll be copying the entire hashmap as a mutable one
    { 
        let actual = shapes
            .entry("circle".into());
            .or_insert(1);
        *actual = 0;
    }   
}

fn main() {
    // vectors();
    hash_map();
}

//////////////////////////////////////////////////////

use std::thread;
use std::time;
use std::collections::HashSet;

fn main()
{
    let mut greeks = HashSet::new();
    greeks.insert("alpha");
    greeks.insert("beta");
    println!("greeks = {:?}", greeks);

    greeks.insert("gamma");
    greeks.insert("delta");
    println!("greeks = {:?}", greeks);

    // adding values that are already in the set has no effect
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega");
    }

    // contains method to check if a value is in the set
    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    // remove a value
    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta");
    }

    // example sets
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("1_5 is a subset of 1_10? -> {}", _1_5.is_subset(&_1_10));

    // disjoint = no common elements
    println!("1_5 and 6_10 are disjoint? -> {}", _1_5.is_disjoint(&_6_10));

    // union = all elements in both sets
    println!("1_5 union 6_10 = {:?}", _1_5.union(&_6_10).collect::<Vec<_>>());

    // intersection = elments in the first set that are also in the second set
    println!("1_10 intersection 2_8 = {:?}", _1_10.intersection(&_2_8).collect::<Vec<_>>());

    // difference = elements in the first set that are not in the second set
    println!("1_10 difference 2_8 = {:?}", _1_10.difference(&_2_8).collect::<Vec<_>>());

    // symmetric difference = elements in either set but not both
    println!("1_10 symmetric difference 2_8 = {:?}", _1_10.symmetric_difference(&_2_8).collect::<Vec<_>>());
}

//////////////////////////////////////////////////////

pub fn main() 
{
    let mut v = vec![1, 2, 3, 4, 5];

    // iterating over the values in a vector
    for x in v.iter_mut() {
        println!("x = {}", *x);
    }
    println!("v = {:?}", v);

    // printing all values in reverse order
    for x in v.iter().rev() {
        println!("x = {}", *x);
    }

    // into_iter = consumes the vector, move operation that transform the collection into an iterator
    // iter = borrows the vector, non-consuming iterator
    // iter_mut = borrows the vector mutably, non-consuming iterator

    let mut vec = vec![1, 2, 3, 4, 5];
    let mut vec2 = vec![1, 2, 3, 4, 5];
    // let it = vec.into_iter();
    vec2.extend(vec);
    println!("vec2 = {:?}", vec2);
    // println!("vec = {:?}", vec); // vec is consumed by the extend method, so it is no longer available
}

//////////////////////////////////////////////////////

fn strings()
{
    // strings are UTF-8 encoded, bytes
    let s: &'static str = "hello"; // static lifetime, &str is a string slice
    // on a string slice you cant reassing it or change single values
    // you can only iterate over it
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // for flexibility, use String type
    // is a growable, mutable, owned, UTF-8 encoded string type
    // heap allocated
    // String is a wrapper over a Vec<u8>
    let mut letters = String::new();    
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }

    println!("{}", letters);

    // conversion between &str and String
    let u: &str = &letters;

    // concatenation
    let z = letters + "abc";
    println!("{}", z);

    let mut abc = "abc".to_string();
    abc.remove(0);
    abc.push_str("def");
    println!("{}", abc.replace("bc", "12"));

    // format macro
    let x = format!("{}, world!", letters);
    println!("{}", x);

    let hello = "Здравствуйте";
    let rust = "Rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    // indexing into a string
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1} {0}!", run, forest);
    println!("{}", rfr);

    // giving names to variables
    let info = format!(
        "The name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format! ( // compiler will check for the correct number of variables
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);

}