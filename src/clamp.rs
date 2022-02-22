/// clamp from a recovered version 0.1.0
pub fn clamp<T: PartialOrd>(low: T, value: T, high: T) -> T {
    debug_assert!(low < high, "low is bigger than high!");
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[test]
fn tests() {
    assert_eq!(clamp(1.0, 0.5, 2.0), 1.0);
    assert_eq!(clamp(1.0, 1.5, 2.0), 1.5);
    assert_eq!(clamp(1.0, 3.0, 2.0), 2.0);

    let a = 1.0;
    let b = 2.0;
    let c = 3.0;
    assert_eq!(clamp(&a, &b, &c), &b);
}
