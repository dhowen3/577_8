use std::io;

let type InputNumType = u16;

fn get_one_num() -> InputNumType {
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    line.trim().parse::<InputNumType>().unwrap()
}

fn get_two_num() -> (InputNumType, InputNumType) {
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    let split = line.trim().split_whitespace();
    let num1 = split.next().unwrap().parse::<InputNumType>().unwrap();
    let num2 = split.next().unwrap().parse::<InputNumType>().unwrap();
    (num1, num2)
}

fn main() {
    let num_instances = get_one_num(); 
    let mut outer_counter : IndexNumType = 0;
    loop {
	if outer_counter == num_instances {
	    break;
	}
	let num_items_and_capacity = get_two_num();
	let inner_counter = 0;
	let mut vec_c : Vec<(InputNumType, InputNumType)> = Vec::new();
	loop {
	    if inner_counter == num_items_and_capacity.0 {
		break;
	    }
	    let item_weight_value = get_two_num();
	    vec_c.push(item_weight_value);
	    inner_counter += 1;
	}
	outer_counter += 1;
    }
}
