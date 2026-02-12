
// The core trait of the animal (that it speaks)
trait Animal {
    fn speak(&self) -> String;
}

// A structure to define the dog type
struct Dog {
    name: String,
}

// A direct implementation for the trait
impl Animal for Dog {
    fn speak(&self) -> String {
        return format!("{} says woof!", self.name);
    }
}

struct Cat {
    name: String
}

impl Animal for Cat {
    fn speak(&self) -> String {
        return format!("{} says meow!", self.name);
    }
}

// "T must implement the Animal trait, which the above do"
fn print_animal_sound<T: Animal>(animal: &T) {
    println!("   {}", animal.speak());
}

pub fn playing_with_traits(){
    println!("=== Traits and Attributes in Rust ===\n");
    let dog = Dog {
        name: String::from("Dog A"),
    };

    let cat = Cat {
        name: String::from("Cat B")
    };

    print_animal_sound(&dog);
    print_animal_sound(&cat);

}
