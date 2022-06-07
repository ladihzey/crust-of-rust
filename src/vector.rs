#[cfg(test)]
mod test {
    #[test]
    fn should_create_vector() {
        let mut v1: Vec<i32> = Vec::new();
        v1.push(1);
        v1.push(2);
        v1.push(3);

        let v2 = vec![1, 2, 3];

        assert_eq!(v1, v2);
    }

    #[test]
    fn should_iterate_over_vector() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        assert_eq!(v, vec![150, 82, 107]);
    }
}
