use std::io;

fn main(){

    loop{
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Invalid Name");

    println!("Hours worked?");
    let mut hours = String::new();
    io::stdin().read_line(&mut hours).expect("Error");
    let hours:i32 = hours.trim().parse().expect("Error");

    if name =="no"{
    break;}
    
    if hours <= 40{
        let gross = 3_000;
        if gross > 100_000{
        let _gross = gross - 2_000;
    }
        println!("Your pay is {}",gross);
    }
    else if hours > 40{
        let gross = 4_500;
        if gross > 100_000{
        let _gross = gross - 2_000;
    }
        println!("Your pay is {}",gross);
    }

}
}