struct Person
{
    name: String
}

struct Company<'z>
{
    name: String,
    ceo: &'z Person
}

fn main()
{
    let p = Person { name: "John".to_string() };
    let c = Company { name: "Acme".to_string(), ceo: &p };
    println!("{} is the CEO of {}", c.ceo.name, c.name);
}

