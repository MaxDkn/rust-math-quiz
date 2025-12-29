pub mod function {
    use num_traits::PrimInt;

    pub fn format_answers<T, F>(values: &[T], fmt: F) -> [String; 4]
    where
        F: Fn(&T) -> String,
    {
        assert_eq!(values.len(), 4);

        values
            .iter()
            .map(fmt)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub fn fill_unique_random<T, F>(
        values: &mut Vec<T>,
        target_len: usize,
        mut generator: F,
    ) where
        T: PartialEq,
        F: FnMut() -> T,
    {
        while values.len() < target_len {
            let v = generator();
            if !values.contains(&v) {
                values.push(v);
            }
        }
    }

    pub fn gcd<T>(mut a: T, mut b: T) -> T where T: PrimInt {
        while b != T::zero() {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    pub fn lcm<T>(a: T, b: T) -> T where T: PrimInt {
        (a * b) / gcd(a, b)
    }
}