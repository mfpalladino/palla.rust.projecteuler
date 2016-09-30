/* 
 Multiples of 3 and 5
 If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 The sum of these multiples is 23.
 Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn main() {
    let mut sum_of_multiples = 0;

    for i in 1..1000 {
        let multiple_of_3 = i % 3 == 0;
        let multiple_of_5 = i % 5 == 0;
        if multiple_of_3 || multiple_of_5 {
            sum_of_multiples += i;
        }
    }

    println!("{:?}", sum_of_multiples);
}
