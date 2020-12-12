use ::day_11;

fn main () {
    let input = include_str!("input.txt");
    let mut room = day_11::WaitingRoom::new(&input);
    let mut prev_state = String::new();
    loop {
        prev_state = room.get_state();
        room.update();
        room.print_state();
        if room.get_state() == prev_state {
            println!("{:#?}", room);
            println!("{}", room.count_occupied());
            break
        }
        prev_state = room.get_state();
    }
}