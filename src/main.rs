mod utils;

fn main() {
    let result_array = [utils::add(55, 66), utils::sub(55, 66)];
    println!("{:?}", result_array);
}