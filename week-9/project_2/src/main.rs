use std::io::Write;
use std::fs::File;
use std::io::Read;

fn main(){
    let student_names = vec!["Oluchi Mordi","Adams","Shanla Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_num = vec!["ACC10211111","ECO10101011","CSC10328828","EE1102020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec!["300","100","200","200","100"];

    let mut file = File::create("PAU_SMIS.xlsx").expect("Create failed");
    
    writeln!(file, "student_names").unwrap();
    for names in &student_names{
        writeln!(file, "{}", names).unwrap()
    }
    writeln!(file).unwrap();

    writeln!(file, "matric_num").unwrap();
    for num in &matric_num{
        writeln!(file, "{}", num).unwrap()
    }
    writeln!(file).unwrap();

    writeln!(file, "department").unwrap();
    for dept in &department{
        writeln!(file, "{}", dept).unwrap()
    }
    writeln!(file, "level").unwrap();
    for lvl in &level{
        writeln!(file, "{}", lvl).unwrap()
    }
    writeln!(file).unwrap();
    
    let mut file = std::fs::File::open("PAU_SMIS.xlsx").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    print!("{}",contents );
     
    print!("success");

}
