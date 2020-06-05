pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn lt<T: PartialOrd>(a: T, b: T) -> bool {
    a < b
}

pub fn gt<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

pub fn le<T: PartialOrd>(a: T, b: T) -> bool {
    a <= b
}

pub fn ge<T: PartialOrd>(a: T, b: T) -> bool {
    a >= b
}
