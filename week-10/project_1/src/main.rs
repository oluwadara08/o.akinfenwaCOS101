struct Laptopbrand {
    name:String,
    price:u32,
    quantity:u32

}

fn main() {
    println!("HP---------650000
IBM----------755000
Toshiba----------555000
Dell-----------850000");


    let hp = Laptopbrand{
        name:String::from("HP"),
        quantity:10,
        price:650000
    };
    let ibm = Laptopbrand{
        name:String::from("IBM"),
        quantity:6,
        price:755000
    };
    let toshiba = Laptopbrand{
        name:String::from("Toshiba"),
        quantity:10,
        price:555000
    };
    let dell = Laptopbrand{
        name:String::from("DELL"),
        quantity:4,
        price:850000
    }; 

    let purchase = 3;

    let total_cost = (hp.price*purchase) + (ibm.price*purchase) + (toshiba.price*purchase) + (dell.price*purchase);

println!("Your total for 3 HPs, 3 IBMs, 3 Toshibas, 3 Dells is N{}",total_cost );

}
