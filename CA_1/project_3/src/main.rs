use std::io;

fn main(){
    println!("Code      Book Title          Price(N)");
    println!("R        Rust for Beginners    15 000   ");
    println!("A          AI basics           12 500");
    println!("D     Data Structures in Rust  20 000");
    println!("N      Networking Essentials   18 000");


    println!("What is the book code?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let input:String = input.trim().parse().expect("Error");

    println!("how many do you need?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let _num:i32 = input2.trim().parse().expect("error");

    if input == "r"{
        let price = 15_000;
        let pay = price * _num;
        println!("Your total is {}",pay);
    }
    else if input == "a"{
        let price = 12_500;
        let pay = price * _num;
        println!("Your total is {}",pay);
    }
    else if input == "d"{
        let price = 20_000;
        let pay = price * _num;
        println!("Your total is {}",pay);
    }
    else if input == "n"{
        let price = 18_000;
        let pay = price * _num;
        println!("Your total is {}",pay);
    }


}