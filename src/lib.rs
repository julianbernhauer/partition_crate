pub fn partition(mut vector: Vec<usize>, mut start: usize, stop: usize, pivot_value: usize) -> Result<usize, &'static str> {
    if start > vector.len() || stop > vector.len() {
        return Err("Invalid arguments");
    }
    if !vector.contains(&pivot_value) {
        return Err("Invalid pivot value");
    }

    let mut left = start;
    let mut right = stop;

    while left <= right {
        while left <= stop && vector[left] < pivot_value {
            left += 1;
        }

        while right >= start && vector[right] >= pivot_value {
            right -= 1;
        }

        if left <= right {
            vector.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    Ok(left)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_partition_invalid_args() {
        let vector = vec![4, 2, 7, 1, 5, 3];
        let start = 0;
        let stop = 5; // Valid stop index
        let pivot_value = 9; // Invalid pivot value

        let result = partition(vector.clone(), start, stop, pivot_value);
        assert_eq!(result, Err("Invalid pivot value"));
    }

    #[test]
    fn test_partition_valid() {
        let vector = vec![156, 335, 289, 405, 136, 331, 41, 292, 113, 349, 346, 97, 106, 77, 148, 253, 498, 30, 465, 166, 489, 380, 84, 89, 335, 228, 135, 143, 231, 344, 453, 355, 207, 45, 190, 292, 131, 470, 228, 180, 455, 286, 18, 286, 442, 261, 361, 190, 118, 66, 431, 268, 407, 286, 345, 384, 20, 384, 352, 477, 32, 219, 219, 175, 315, 440, 494, 105, 285, 135, 334, 242, 237, 308, 178, 447, 109, 286, 460, 282, 464, 481, 83, 460, 83, 24, 428, 207, 141, 67, 173, 312, 36, 232, 256, 424, 93, 195, 147, 418];
        let start = 0;
        let stop = 99;
        let pivot_value = 300;

        let result = partition(vector.clone(), start, stop, pivot_value);
        println!("{:?}", vector);
        println!("{:?}", result);
    }

    #[test]
    fn test_partition_valid_partition() {
        let mut vector = vec![5, 3, 8, 1, 7, 2, 6, 4];
        let start = 0;
        let stop = vector.len() - 1;
        let pivot_value = 4; // Pivot value is chosen such that it partitions the vector into two halves: [3, 1, 2] and [5, 8, 7, 6]

        let result = partition(vector.clone(), start, stop, pivot_value);

        match result {
            Ok(pivot_position) => {
                let left_half = &vector[start..pivot_position];
                let right_half = &vector[pivot_position..=stop];

                println!("Left Half: {:?}", left_half);
                println!("Right Half: {:?}", right_half);
                println!("Pivot Position: {}", pivot_position);

                for &element in left_half {
                    assert!(element < pivot_value);
                }

                for &element in right_half {
                    assert!(element >= pivot_value);
                }
            },
            Err(_) => {
                // If the partitioning fails, the test should fail.
                assert!(false, "Partitioning failed");
            }
        }
    }

}