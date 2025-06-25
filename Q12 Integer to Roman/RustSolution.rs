struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_numerals = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::with_capacity(15); // Pre-allocate enough space for Roman numeral
        let mut temp_num = num;  // Create a mutable copy of num

        // Iterate over the Roman numeral mappings
        for &(value, symbol) in roman_numerals.iter() {
            let count = temp_num / value;  // Determine how many times we can use this Roman symbol
            if count > 0 {
                result.push_str(&symbol.repeat(count as usize));  // Repeat the symbol 'count' times
                temp_num -= value * count;  // Decrease temp_num by the total value we have processed
            }
        }

        result
    }
}
