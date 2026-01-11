/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch
 
Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.
 
Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.
 
Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.
 
Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.
 
Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String.
 
Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.
 
Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).
 
Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

fn main() {
    println!("Hello, This is project 3 for understanding slices!");

    let cereals = [String::from("Cookie Crisp"), String::from("Cinnamon Toast Crunch"), String::from("Frosted Flakes"), String::from("Cocoa Puffs"), String::from("Captain Crunch")];
    let first_two = &cereals[..2];
    println!("{first_two:?}");

    let mid_three = &cereals[1..4];
    println!("{mid_three:?}");

    let last_three = &cereals[2..];
    println!("{last_three:?}");

    let cookie_crisp = &cereals[0];
    println!("{cookie_crisp}");

    let cookie = &cereals[0][..6];
    println!("{cookie}");

    let cocoa_puffs = &cereals[3];
    println!("{cocoa_puffs}");

    let puffs = &cereals[3][6..];
    println!("{puffs}");

}
