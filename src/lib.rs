mod generator;
pub fn print_random_number() {
    println!("Random usize: {}", generator::gen_ran());
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
