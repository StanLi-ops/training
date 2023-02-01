enum Movement {
    // Variants
    Up,
    Down,
    Lift,
    Right
}
fn move_avatar(m: Movement){
    // Perform action depending on info 
    match m {
        Movement::Up => println!("Acatar moving up "),
        Movement::Down => println!("Acatar moving down "),
        Movement::Lift => println!("Acatar moving left "),
        Movement::Right => println!("Acatar moving right "),
    }
}

pub fn run(){
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Lift;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}