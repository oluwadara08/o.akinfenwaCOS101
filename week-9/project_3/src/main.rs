use std::io::Write;
use std::fs::File;
use std::io::Read;

fn main(){
    let dataset_num = vec!["1","2","3","4","5"];
    let dataset_name = vec!["Adegoke Alabi Dada","Murtala Afeez Bendu","Okorocha Calistus Ogbonna", "Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let dataset_mnstry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let dataset_gpz = vec!["South West","North East","South South","South West","South East"];

    let mut file = File::create("EFCC_DATA.txt").expect("create failed");

    writeln!(file, "S/N").unwrap();
    for num in &dataset_num{
        writeln!(file, "{}", num).unwrap()
    }
    writeln!(file).unwrap();

    writeln!(file, "Name").unwrap();
    for name in &dataset_name{
        writeln!(file, "{}", name).unwrap()
    }
    writeln!(file).unwrap();

    writeln!(file, "Ministry").unwrap();
    for mini in &dataset_mnstry{
        writeln!(file, "{}", mini).unwrap()
    }
    writeln!(file).unwrap();

    writeln!(file, "Geopolitical Zone").unwrap();
    for zone in &dataset_gpz{
        writeln!(file, "{}", zone).unwrap()
    }
    writeln!(file).unwrap();

    let mut file = std::fs::File::open("EFCC_DATA.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    print!("{}",contents );
     
    print!("success");



}