use std::io;

fn main() {
    loop {
        println!("      Menu                     Price      
            P = Poundo Yam/Edinkaiko         N3,500
            F = Fried Rice & Chicken         N3,000
            A = Amala & Ewedu Soup           N2,500
            E = Eba & Egusi Soup             N2,000
            W = white Rice & Stew            N2,500");


    println!("Enter food code: ");
    let mut fcode = String::new();
    io::stdin().read_line(&mut fcode).expect("failed to read input");
    let fcode:String = fcode.trim().parse().expect("Error");


    println!("Enter quantity: ");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("failed to read input");
    let qty:f32 = qty.trim().parse().expect("error");

    if fcode == "p"{
        let price = 3_500.00;
        let pay = price * qty;
        if pay > 10_000.00{
            let _pay = pay * 0.95;
            println!("Your total is {}",_pay);
        }
    }
    else if fcode == "f"{
        let price = 3_000.00;
        let pay = price * qty;
        if pay > 10_000.00{
            let _pay = pay * 0.95;
            println!("Your total is {}",_pay);
        }
    }
    else if fcode == "a"{
        let price = 2_500.00;
        let pay = price * qty;
        if pay > 10_000.00{
            let _pay = pay * 0.95;
            println!("Your total is {}",_pay);
        }
    }
    else if fcode == "e"{
        let price = 2_000.00;
        let pay = price * qty;
        if pay > 10_000.00{
            let _pay = pay * 0.95;
            println!("Your total is {}",_pay);
        }
    }
     else if fcode == "e"{
        let price = 2_000.00;
        let pay = price * qty;
        if pay > 10_000.00{
            let _pay = pay * 0.95;
            println!("Your total is {}",_pay);
        }
    }
    else if fcode == "cancel"{
        break;
    }
    }


        


}
