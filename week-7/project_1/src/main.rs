use std::io;

// Function for trapezium area
fn area_trapezium(h: f32, b1: f32, b2: f32) -> f32 {
    h * 0.5 * (b1 + b2)
}

// Function for rhombus area
fn area_rhombus(d1: f32, d2: f32) -> f32 {
    0.5 * d1 * d2
}

// Function for parallelogram area
fn area_parallelogram(base: f32, height: f32) -> f32 {
    base * height
}

// Function for cube area
fn area_cube(side: f32) -> f32 {
    6.0 * side.powi(2)
}

// Function for cylinder volume
fn volume_cylinder(radius: f32, height: f32) -> f32 {
    std::f32::consts::PI * radius.powi(2) * height
}

// Helper: get input
fn get_input(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse().expect("Invalid number")
}

fn main() {
    println!("--- Shape Calculator ---");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed");
    let choice: u32 = choice.trim().parse().expect("Invalid choice");

    match choice {
        1 => {
            let h = get_input("Enter height:");
            let b1 = get_input("Enter base1:");
            let b2 = get_input("Enter base2:");
            println!("Area of Trapezium = {}", area_trapezium(h, b1, b2));
        }
        2 => {
            let d1 = get_input("Enter diagonal1:");
            let d2 = get_input("Enter diagonal2:");
            println!("Area of Rhombus = {}", area_rhombus(d1, d2));
        }
        3 => {
            let base = get_input("Enter base:");
            let height = get_input("Enter height:");
            println!("Area of Parallelogram = {}", area_parallelogram(base, height));
        }
        4 => {
            let side = get_input("Enter length of a side:");
            println!("Area of Cube = {}", area_cube(side));
        }
        5 => {
            let radius = get_input("Enter radius:");
            let height = get_input("Enter height:");
            println!("Volume of Cylinder = {}", volume_cylinder(radius, height));
        }
        _ => println!("Invalid choice!"),
    }
}
