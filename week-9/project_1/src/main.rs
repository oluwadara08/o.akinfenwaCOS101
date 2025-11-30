use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desparados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo Kings","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = File::create("Nigerian Breweries.txt").expect("create failed");

    writeln!(file, "Lager").unwrap();
    for drink in &lager{
        writeln!(file, "{}", drink).expect("Failed to write");
    }
    writeln!(file, "Stout").unwrap();
    for drink in &stout{
        writeln!(file, "{}", drink).expect("Failed to write");
    }
    writeln!(file, "Non-alcoholic").unwrap();
    for drink in &non_alcoholic{
        writeln!(file, "{}", drink).expect("Failed to write");
    }
    writeln!(file).expect("Failed to write");

    print!("Success");
}
