
	fn main() {
    // Staff positions and APS levels
    let staff = vec![
        ("Intern", 1, 2),
        ("Administrator", 3, 5),
        ("Associate", 5, 8),
        ("Director", 10, 13),
        ("CEO", 14, 20),
    ];

    let position = "Associate";
    let years = 6;
    let mut found = false;

    // Check the staff level
    for person in &staff {
        if person.0 == position
            && years >= person.1
            && years <= person.2
        {
            println!("Position: {}", person.0);
            println!("Years of experience: {}", years);
            println!("APS Level: APS 5-8");
            found = true;
        }
    }

    if found == false {
        println!("Staff level not found");
    }
}
}