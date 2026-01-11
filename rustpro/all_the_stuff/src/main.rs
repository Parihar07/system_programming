fn main() {
    println!("Hello, understanding scope thing in rust programming!");

    // this is not accessible because it is in differnt scope  
    // print!("{k}");
    understanding_strings();
    moving_ownership();
    dropping();
    clone_stuff();

    mu_references();
}

fn understanding_strings(){
    let text = "hello".to_string();

    let candy = String::from("this is again different string shitt");

    println!("This is som wiered way of having string rust : {text}");
    println!("This is som wiered way of having string rust : {candy}");

    let mut strio = String::new();
    strio.push_str("hello stuff adding here ");

    println!(" Trying oout string push back : {strio}");

    refernces();
}

fn moving_ownership(){
    let person = String::from("somethig shitt");
    let tmp = person;

    println!("{tmp}");
}

fn dropping(){
    let person=String::from("Parihar");

    drop(person);
}

fn clone_stuff(){
    let person = String::from("This is string form clone stuff");
    let tmp = person.clone();

    println!("Person {}, tmp : {}", person,tmp);

}

fn refernces(){
    let va = 32;
    let ref_val = &va;

    println!("{}", *ref_val);

}

fn mu_references(){
    let car = String::from("Some Car");
    let ref1 = &car;
    let ref2 = &car;

    println!("{ref1} and {ref2}");
}