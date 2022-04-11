pub mod front_of_house;
use front_of_house::hosting;
pub mod test {
    pub use crate::front_of_house::hosting;
    fn func() {
        hosting::add_to_waitlist();
    }
}

fn main() {
    println!("Hello, world!");
    test::hosting::add_to_waitlist();
}
