
use std::io;

fn main() {
    
    println!("What is the temperature?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let temp:f32 = input.trim().parse().expect("Invalid input");


    //converting to f
    let f = (9.0 / 5.0) * temp +32.0;
    println!("The temperature in Fahrenheit is {}F",f);

    // converting to kelvin
    let k = temp + 273.15;
    println!("The temperature in kelvin is {}k",k);

    // classification
    if temp < 0.0 {
        println!("freezing point");
    }
    else if temp >=0.0 && temp <= 30.0{
        println!("normal range");
    }
    else if temp > 30.0{
        println!("Hot temperature");
    }
}
    
