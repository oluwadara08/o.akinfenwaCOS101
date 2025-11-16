fn main(){

    //aray with data type 
    let arr1:[i32;4] = [10,20,30,40];
    println!("\narray with data type");
    println!("array is {:?}",arr1 );
    println!("arraysize is: {:?}",arr1.len() );


    //array w/o data type
    let arr2 = [10.4,20.7,30.7,40.9,51.2,72.2];
    println!("\nArray without data type");
    println!("array is {:?}",arr2 );
    println!("array sixe is:{:?}",arr2.len() );

    //array w default values that creates and initialises all its elements with a default value of -1
    let arr3:[i32;2] = [-1,8];
    println!("\nArray with default values");
    println!("array is {:?}",arr3 );
    println!("array size is:{:?}",arr3.len() );
}