
pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let c : T = *a;
    *a = *b;
    *b = c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
