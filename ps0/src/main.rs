use factorial::Factorial;

fn main() {
    let mut a: Vec<u32> = vec![];
    for i in 0..=4 {
        let value = i + 5.factorial() / (i.factorial() * (5 - i).factorial());
        a.push(value);
    }
    println!("a = {:?}", a);

    let mut b: Vec<u32> = vec![];
    for i in [1, 2, 4, 5] {
        b.push(3 * i);
    }
    println!("b = {:?}", b);
}
