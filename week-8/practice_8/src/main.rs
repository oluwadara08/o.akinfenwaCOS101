fn main() {
   //initilae mutable tuples
   let mut mount_heights = ("Everest", 8848, "Fishtail", 6993);

   println!("Original tuple = {:?}",mount_heights );

   //change 3rd and 4th element of the tuple
   mount_heights.2 = "Lhotse";
   mount_heights.3 = 8516;

   println!("Changed tuple = {:?}",mount_heights );
}
