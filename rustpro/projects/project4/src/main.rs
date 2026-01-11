/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)

Derive a Debug trait implementation for the Flight struct.

Define a `new` constructor function that returns a new
instance of a Flight.

Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.

Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.

Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.

Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/

#[derive(Debug)]
struct Flight {
    origin: String,
    destinition: String,
    price: f64,
    #[allow(dead_code)]
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destinition: String, price: f64, passengers: u32) -> Flight {
        Self { // you can write here Flight also but in future if you want to perform change sthis would help not changing everywhere
            origin,
            destinition,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, destinition: String) -> &mut Self {
        self.destinition = destinition;
        self
    }

    fn increase_price(&mut self, price: f64) -> &mut Self{
        self.price += price;
        self
    }

    fn itenary(&mut self) {
        println!("{} -> {}", self.origin, self.destinition);
    }
}

fn main() {
    println!("Hello, thi sproject four dealing iwth struct concepts and more!");

    let mut flight = Flight::new(String::from("hyderbad"), String::from("Kolkata"), 6000.0, 2);

    println!("{flight:#?}");
    flight
        .change_destination(String::from("Chennai"))
        .increase_price(3003.0);

    // flight.change_destination(String::from("Chennai"))
    // flight.increase_price(3003.0);
    flight.itenary();

    println!("{flight:#?}");

}
