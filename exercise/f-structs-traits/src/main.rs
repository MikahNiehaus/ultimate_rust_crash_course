
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Mikah {
    name:  String,
    age: 	i32,
}

trait Age {
    fn age(self: &mut Self);
}

impl Age for Mikah {
    fn age(self: &mut Self){
        self.age += 1;
    }
}
fn main() {
    
    let mut _mikah = Mikah {
        name: String::from("Mikah Niehaus"),
        age: 20,
    };

    _mikah.age();
    println!("Happy Bday! {}'s age is now {:?}", _mikah.name, _mikah.age);

}


