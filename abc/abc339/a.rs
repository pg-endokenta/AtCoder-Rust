use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let after_dot: String = s.split('.').last().unwrap_or("").to_string();
    println!("{}", after_dot);
}