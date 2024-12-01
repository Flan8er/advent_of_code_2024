use crate::utils::utils::read_file_data;

pub fn main() {
    let path = String::from("./src/days/day_one/assets/data.txt");
    let data = read_file_data(path);

    // println!("Part one answer: {}", part_one(data.clone()));
    // println!("Part two answer: {}", part_two(data.clone()));
}

fn part_one(input: String) {}

fn part_two(input: String) {}

mod tests {
    use super::*;
    use crate::utils::utils::read_file_data;

    #[test]
    fn test_data_part1() {
        let test_path = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data = read_file_data(test_path);

        // let test_result = part_one(test_data);
        // assert_eq!(test_result, 11);
    }

    #[test]
    fn test_data_part2() {
        let test_path = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data = read_file_data(test_path);

        // let test_result = part_two(test_data);
        // assert_eq!(test_result, 31);
    }
}
