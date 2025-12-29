pub mod function {
    use num_traits::PrimInt;
    use rand::seq::SliceRandom;
    use rand::distr::uniform::SampleRange;

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

    pub fn fill_unique_random<R>(
        values: &mut Vec<usize>,
        target_len: usize,
        rng: &mut impl rand::Rng,
        range: R,
    ) where R: SampleRange<usize> + Clone,
    {
        while values.len() < target_len {
            let v = rng.random_range(range.clone());
            if !values.contains(&v) {
                values.push(v);
            }
        }
        values.shuffle(rng);
    }

    pub fn gcd<T>(
        mut a: T,
        mut b: T
    ) -> T where
        T: PrimInt
    {
        while b != T::zero() {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}