fn main() {

    //creating vectors
    let v : Vec<i64> = Vec::new();

    //printing the vector
    println!("the size of vec::new is{:?}",v.len() );

    //using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing
    println!("The length of vec macro is {:?}",v.len() ); 
}