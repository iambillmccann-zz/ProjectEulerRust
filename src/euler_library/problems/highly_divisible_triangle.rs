/// <summary>
/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
/// Let us list the factors of the first seven triangle numbers:
/// 1: 1
/// 3: 1,3
/// 6: 1,2,3,6
/// 10: 1,2,5,10
/// 15: 1,3,5,15
/// 21: 1,3,7,21
/// 28: 1,2,4,7,14,28
/// We can see that 28 is the first triangle number to have over five divisors.
/// What is the value of the first triangle number to have over five hundred divisors?
/// </summary>
/// <returns></returns>
pub fn compute() -> String {
    let mut done: bool = false;
    let mut sequence: u64 = 1;
    let mut triangle: u64 = 0;
    let minimum: usize = 500;

    while !done {
        triangle += sequence;
        let divisors: Vec<u64> = ::euler_library::math_library::get_divisors(triangle);
        if divisors.len() > minimum { done = true }

        sequence += 1
    }
    triangle.to_string()
}
