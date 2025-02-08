#[test]
fn test_memory_order() {
    let test_value = -1.0;
    let value = -f32::copysign(2.0, test_value);
    println!("value is: {value}");
}
