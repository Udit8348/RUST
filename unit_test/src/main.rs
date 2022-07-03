// Lesson #26

/*
    technically, this is un-allowed
    main can only return types that are std::process::Termination
*/
// fn main() -> usize {
//     24
// }

fn main() -> f32 {
    24.5
}

fn util() -> usize {
    24
}
  
#[cfg(test)]            // configure "test"
mod tests {
    use crate::main;    // bring the function into the test scope

    // #[test]             // tests which functions should be functions
    // fn main_returns_24() {
    //     assert_eq!(main(), 24);
    // }

    #[test]
    #[should_panic]
    fn main_panics_with_i() {
        assert_eq!(main() as usize as f32, main() as f32);
    }

    #[test]
    fn main_returns_f() {
        assert_eq!(main() as f32, 24.5);
    }

    use crate::util;
    #[test]
    fn util_returns_24() {
        assert_eq!(util(), 24);
    }
}
