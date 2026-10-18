fn main() {
    // Names and years of experience
    let developers = vec![
        ("Joana", 3),
        ("Maryam", 5),
        ("Praise", 8),
        ("Daniel", 4),
        ("Ummu", 10),
    ];

    let mut highest = developers[0];

    // Find the developer with the highest experience
    for person in &developers {
        if person.1 > highest.1 {
            highest = *person;
        }
    }

    println!("Name: {}", highest.0);
    println!("Years of experience: {}", highest.1);
}