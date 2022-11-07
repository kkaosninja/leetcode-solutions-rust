// Problem => https://leetcode.com/problems/maximum-69-number/
// Submission => https://leetcode.com/submissions/detail/838615249/

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        // Handle case of all digits being 9. No change necessary
        if all_digits_same(num, 9) {
            return num;
        }

        /* In the general case, the biggest number you will get, is the one that will
           result when the first 6 (counted from the left) is changed to 9
        */

        let digits_count = count_digits(num);

        let mut position_to_flip: u32 = digits_count;

        const TEN: i32 = 10;

        let mut dividend = num;

        while position_to_flip != 0 {
            // Get the leftmost digit
            let current_digit = dividend / TEN.pow(position_to_flip - 1);
            
            // Find the first 6, and flip it and return immediately
            if current_digit == 6 {
                return num + 3 * TEN.pow(position_to_flip - 1);
            }

            dividend = dividend - (current_digit * TEN.pow(position_to_flip - 1));
            position_to_flip -= 1;
        }

        return num;
    }
    
}

fn all_digits_same(num: i32, digit: i32) -> bool {
    let mut dividend = num;

    while dividend != 0 {
        if dividend % 10 != digit {
            return false;
        }

        dividend /= 10;
    }

    return true;
}

fn count_digits(num: i32) -> u32 {
    let mut count = 0;
    let mut input = num;

    while input != 0 {
        input /= 10;
        count += 1
    }

    return count;
}
