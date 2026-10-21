use std::io::Write;

fn main() {
    let lager = "Lager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star";
    let stout = "Stout: Legend, Turbo King, Williams";
    let non_alcoholic = "Non-Alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz";

    let mut file = std::fs::File::create("breweries.txt").unwrap();

    file.write_all(lager.as_bytes()).unwrap();
    file.write_all(stout.as_bytes()).unwrap();
    file.write_all(non_alcoholic.as_bytes()).unwrap();

    println!("File created successfully");
}