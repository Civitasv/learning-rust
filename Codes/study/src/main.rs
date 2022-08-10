use colored::*;

fn main() {
    println!(
        "{}, {} !",
        "it".green(),
        "works😀".truecolor(0, 0, 0).bold()
    );
}
