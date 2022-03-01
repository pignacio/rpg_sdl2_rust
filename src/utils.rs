pub fn positive_mod<A: TryInto<i32>, B: TryInto<i32>>(value: A, modulo: B) -> usize {
    let signed_modulo = modulo.try_into().unwrap_or_else(|x| panic!("Not a i32"));
    let result = value.try_into().unwrap_or_else(|x| panic!("Not a i32")) % signed_modulo;
    if result < 0 {
        (result + signed_modulo) as usize
    } else {
        result as usize
    }
}