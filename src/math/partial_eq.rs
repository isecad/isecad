pub fn eq<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

pub fn ne<T: PartialEq>(a: T, b: T) -> bool {
    a != b
}
