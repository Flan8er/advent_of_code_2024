use crate::utils::utils::read_file_data;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    println!("Running day one part one.")
}

fn part_two() {
    println!("Running day one part two.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::utils::read_file_data;

    #[test]
    fn test_data() {
        let test_path = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data = read_file_data(test_path);

        assert_eq!(test_data, String::from("poopie"));
    }
}
