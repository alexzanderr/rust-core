

use std::collections::HashMap;

pub fn gcd(x: i32, y:i32) {

}

pub fn two_sum(
    nums_vector: Vec<i32>,
    target_sum: i32
) -> Vec<i32> {
    if nums_vector.is_empty() {
        return vec![];
    }
    {
        let minus_ten_to_minus_9 = i32::pow(-10, 9);
        let ten_to_minus_9 = i32::pow(10, 9);

        if target_sum < minus_ten_to_minus_9
        || target_sum > ten_to_minus_9 {
            panic!("target must be between (-10^9, 10^9)")
        }

        if nums_vector.len() < 2
        || nums_vector.len() as i32 > i32::pow(10, 4) {
            panic!("len or arr must be between (2, 10^4)")
        }

        for value in nums_vector.iter() {
            if value < &i32::pow(-10, 9)
            || value > &i32::pow(10, 9) {
                panic!("every element of the array
                    must be between (-10^9, 10^9)")
            }
        }
    }

    let mut pos_table: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums_vector.iter().enumerate() {
        let diff = target_sum - value;
        if pos_table.contains_key(&diff) {
            return vec![*pos_table.get(&diff).unwrap(), index as i32];
        } else {
            pos_table.insert(*value, index as i32);
        }
    }
    return vec![];
}
