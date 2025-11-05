fn main() {
    let fname = "Chidubem John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str("and Technology");

    println!("My name is: {}",fname);
    //check length
    println!("The length my fullname is: {}",fname.len());
    println!("I am a student of {} department",department );
    println!("{}",school );
    println!("{}",uni);
}