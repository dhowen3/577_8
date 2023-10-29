use std::io;
use std::cmp::max;
    
type InputNumType = u16;
type TwoTuple = (InputNumType, InputNumType);
type ResultNumType = u32;

fn get_one_num() -> InputNumType {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<InputNumType>().unwrap()
}

fn get_two_num() -> TwoTuple {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut split = line.trim().split_whitespace();
    let num1 = split.next().unwrap().parse::<InputNumType>().unwrap();
    let num2 = split.next().unwrap().parse::<InputNumType>().unwrap();
    (num1, num2)
}

fn get_result(instance_vec: &Vec<TwoTuple>,
	      num_items_and_capacity: &TwoTuple) -> ResultNumType {
    let num_items = num_items_and_capacity.0 as usize;
    let capacity = num_items_and_capacity.1 as usize;
    // set up dp array
    let mut dp : Vec<Vec<ResultNumType>> = Vec::new();
    for _row_index in 0..(num_items + 1) {
	let mut row_vec : Vec<ResultNumType> = Vec::new();
	for _col_index in 0..(capacity + 1) {
	    row_vec.push(0);
	}
	dp.push(row_vec);
    }
    // fill in dp array
    for row_index in 1..(num_items + 1) {
	for col_index in 1..(capacity + 1) {
	    let item_weight_value = instance_vec.get(row_index - 1).unwrap();
	    let former = dp[row_index - 1][col_index];
	    let mut latter = 0;
	    let weight_offset : i32 = (col_index as i32) - (item_weight_value.0 as i32);
	    if weight_offset >= 0 {
		latter = dp[row_index - 1][weight_offset as usize] + (item_weight_value.1 as ResultNumType);
	    }
	    dp[row_index][col_index] = max(former, latter);
	    }
    }
    dp[num_items][capacity]
}

fn main() {
    let num_instances = get_one_num(); 
    let mut outer_counter : InputNumType = 0;
    loop {
	if outer_counter == num_instances {
	    break;
	}
	let num_items_and_capacity = get_two_num();
	let mut inner_counter = 0;
	let mut vec_c : Vec<TwoTuple> = Vec::new();
    loop {
	if inner_counter == num_items_and_capacity.0 {
	    break;
	}
	let item_weight_value = get_two_num();
	vec_c.push(item_weight_value);
	inner_counter += 1;
    }
    let result = get_result(&vec_c, &num_items_and_capacity);
    println!("{}", result);
    outer_counter += 1;
    }
}
