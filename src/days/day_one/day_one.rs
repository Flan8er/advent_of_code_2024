use crate::utils::utils::read_file_data;

pub fn main() {
    let path = String::from("./src/days/day_one/assets/data.txt");
    let data = read_file_data(path);

    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data.clone()));
}

fn part_one(input: String) -> u64 {
    let (left_values, right_values) = collect_both_lists(input);

    calculate_combined_distance(left_values, right_values)
}

fn part_two(input: String) -> u64 {
    let (left_values, right_values) = collect_both_lists(input);

    let mut similarity_score = 0;

    for value in left_values {
        let duplicates = how_many_duplicates(&value, &right_values);

        similarity_score += duplicates * value;
    }

    similarity_score
}

fn how_many_duplicates(search_value: &u64, search_vector: &Vec<u64>) -> u64 {
    let mut number_of_duplicates = 0;

    for location_id in search_vector {
        if location_id == search_value {
            number_of_duplicates += 1;
        }
    }

    number_of_duplicates
}

fn calculate_combined_distance(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let mut total_distance = 0;

    if left.len() != right.len() {
        panic!("Arrays have different lengths!");
    }

    for index in 0..left.len() {
        total_distance += left[index].abs_diff(right[index]);
    }

    total_distance
}

fn collect_both_lists(input: String) -> (Vec<u64>, Vec<u64>) {
    let split_values = input.split("\n").collect::<Vec<&str>>();

    let mut left_values: Vec<u64> = Vec::new();
    let mut right_values: Vec<u64> = Vec::new();

    for pair in split_values {
        let value_pair = pair.split("   ").collect::<Vec<&str>>();

        let left_value = value_pair[0];
        let right_value = value_pair[1];

        // Parse the string slices into numbers\
        left_values.push(left_value.parse::<u64>().unwrap());
        right_values.push(right_value.parse::<u64>().unwrap());
    }

    // Sort the vectors
    left_values.sort();
    right_values.sort();

    (left_values, right_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::utils::read_file_data;

    #[test]
    fn test_data_part1() {
        let test_path = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data = read_file_data(test_path);

        let test_result = part_one(test_data);
        assert_eq!(test_result, 11);
    }

    #[test]
    fn test_data_part2() {
        let test_path = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data = read_file_data(test_path);

        let test_result = part_two(test_data);
        assert_eq!(test_result, 31);
    }
}
