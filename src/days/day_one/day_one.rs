use crate::utils::utils::read_file_data;

pub fn main() {
    let path: String = String::from("./src/days/day_one/assets/data.txt");
    let data: String = read_file_data(path).trim().to_string();

    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data.clone()));
}

fn part_one(input: String) -> usize {
    let (mut left_values, mut right_values) = collect_both_lists(input);

    // Sort the vectors
    left_values.sort();
    right_values.sort();

    calculate_combined_distance(left_values, right_values)
}

fn part_two(input: String) -> usize {
    let (left_values, right_values) = collect_both_lists(input);

    calculate_similarity_score(left_values, right_values)
}

fn calculate_similarity_score(left_values: Vec<usize>, right_values: Vec<usize>) -> usize {
    let mut similarity_score: usize = 0;

    for value in left_values {
        let duplicates: usize = right_values.iter().filter(|&n| *n == value).count().into();

        similarity_score += duplicates * value;
    }

    similarity_score.into()
}

fn calculate_combined_distance(left: Vec<usize>, right: Vec<usize>) -> usize {
    let mut total_distance: usize = 0;

    if left.len() != right.len() {
        panic!("Arrays have different lengths!");
    }

    for index in 0..left.len() {
        total_distance += left[index].abs_diff(right[index]);
    }

    total_distance
}

fn collect_both_lists(input: String) -> (Vec<usize>, Vec<usize>) {
    let split_values: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let mut left_values: Vec<usize> = Vec::new();
    let mut right_values: Vec<usize> = Vec::new();

    for pair in split_values {
        let value_pair: Vec<&str> = pair.split("   ").collect::<Vec<&str>>();

        let left_value: &str = value_pair[0];
        let right_value: &str = value_pair[1];

        // Parse the string slices into numbers\
        left_values.push(
            left_value
                .parse::<usize>()
                .expect("Failed to parse string to usize"),
        );
        right_values.push(
            right_value
                .parse::<usize>()
                .expect("Failed to parse string to usize"),
        );
    }

    (left_values, right_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::utils::read_file_data;

    #[test]
    fn test_data_part1() {
        let test_path: String = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data: String = read_file_data(test_path).trim().to_string();

        let test_result: usize = part_one(test_data);
        assert_eq!(test_result, 11);
    }

    #[test]
    fn test_data_part2() {
        let test_path: String = String::from("./src/days/day_one/assets/test_data.txt");
        let test_data: String = read_file_data(test_path).trim().to_string();

        let test_result: usize = part_two(test_data);
        assert_eq!(test_result, 31);
    }
}
