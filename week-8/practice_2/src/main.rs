fn main() {

    let v = vec!['c','o','m','p','u','t','e','r'];

    let mut input1 = String::new();

    println!("enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");

    let index:usize = input1.trim().parse().expect("invalid input");

    let ch: char = v[index];

    print!("{} is the character for index [{}]\n",ch, index );
}