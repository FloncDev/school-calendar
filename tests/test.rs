use school_calander::{self, models::Homework};

#[test]
fn homework_shorthand() {
    let hw = Homework::from_shorthand(String::from("CS 20/09 15 The rest of this is a summary"));

    println!("{}", hw.summary);
}