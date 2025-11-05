fn main() {
    let fname = "Pan-Atlantic University";
    println!();
    println!("Name: {}",fname);
    println!("Before trime");
    println!("length is {}",fname.len());
    println!();
    println!("After trim");
    println!("length is {}",fname.trim().len() );
}