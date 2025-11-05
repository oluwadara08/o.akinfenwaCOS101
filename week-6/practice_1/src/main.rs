fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "PAU";
    let addr:&str = "Km 52 Lekki-Ekpe Expressway, Ibeju-Lekki, lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAdress: {}",uni,addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "school of science and technology";
    println!("Department: {}, \nSchool: {}",department,school);
}