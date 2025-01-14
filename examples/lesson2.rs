const fn inc(x: &mut i32) {
    *x += 1;
}

const C: i32 = {
    let mut c = 41;
    inc(&mut c);
    c
};

fn main() {
    println!("1 / C = {}", C);
    println!("2 / C = {}", C);
    println!("3 / C = {}", C);
    inc(&mut C);
    println!("4 / C = {}", C);
}
