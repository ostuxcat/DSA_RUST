pub fn find_char_case(ch: char) -> String {
    if ch >= 'A' && ch <= 'Z' {
        return "Uppercase!".to_string();
    } else if ch >= 'a' && ch <= 'z' {
        return "Lowercase!".to_string();
    } else {
        return "Invalid Character!".to_string();
    }
}
pub fn while_loop_numbers(n: u16) {
    let mut count: u16 = 1;
    while count <= n {
        println!("{}", count);
        count += 1;
    }
}
pub fn for_loop_numbers(n: u16) {
    for count in 1..=n {
        println!("{}", count);
    }
}
pub fn sum_n(n: u16) -> u16 {
    let mut sum: u16 = 0;
    for i in 1..=n {
        sum += i;
    }
    return sum;
}
pub fn sum_odd_n(n: u16) -> u16 {
    let mut sum: u16 = 0;
    for i in 1..=n {
        if i % 2 != 0 {
            sum += i;
        }
    }
    return sum;
}
