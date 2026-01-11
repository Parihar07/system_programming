fn main() {
    println!("Hello, Understanding the structs!");

    struct Something{
        name:String,
        price:f32,
        is_hot:bool,
    }

    let something = Something{
        name: String::from("Coffee stuff"),
        price: 45.7,
        is_hot: true,
    };

    println!("{}, {}, {}",something.name,something.price,something.is_hot);
}
