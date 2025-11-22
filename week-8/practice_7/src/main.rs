fn main() {
   //initialisation of tuple with data type
   let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
   println!("Tuple contents = {:?}",datatype_tuple );

   //intialising of tuple w/o data type 
   let nodatatype_tuple = ("Rust", "fun", 100);
   println!("Tuple contents = {:?}",nodatatype_tuple );

   //accessing tuple elements at index 0
   println!("Value at index 0 = {}",datatype_tuple.0 );

   //accessing tuple element atindex 1
   println!("Value at index 1 = {}",datatype_tuple.1 );

   //accessing at index 2
   println!("Value at index 2 = {}",datatype_tuple.2 );
}
