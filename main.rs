fn main() {
   let is_concert = true;
    let is_event = is_concert; // No movement of ownership as this data type implements the Copy Trait.
    println!("{}", is_concert);
    println!("{}", is_event);

    let sushi = "Salmon";
    let dinner = sushi; // No movement of ownership as this data type implements the Copy Trait.
    println!("{}", sushi);
    println!("{}", dinner);

    let sushi = String::from("Salmon");
    println!("{}", sushi); // This is valid here as the ownership hasn't been transferred yet.
    let dinner = sushi; // The ownership is transferred here in this line because in Rust, the String type doesnot implement the Copy Trait.
    // println!("{}", sushi); This is no more valid in this line as the ownership has been transferred in the previous line.
    println!("{}", dinner);

    let food = eat_meal(dinner); // The ownership of "Salmon" is transferred from dinner variable to food variable in this line.
    println!("{}", food);
}
fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
