// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.

fn two_sum(tab : Vec<i64>, target : i64) -> (i64, i64) {
    for i in 0..tab.len() - 1 {
        for j in i + 1..tab.len() {
            if tab[i] + tab[j] == target {
                return (tab[i], tab[j]);
            }
        }
    }
    (0, 0)
}

fn main() {
    let v = vec![0, 2, 4, 6];
    println!("{:?}", two_sum(v, 10));
}
