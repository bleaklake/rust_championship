#[cfg(test)]
mod tests {
    use championship::create_tournement;

    #[test]
    fn even() {
        let input = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
            "Dan".to_string(),
        ];

        if let [_a, _b, _c, _d] = input.iter().map(|x| x).collect::<Vec<&String>>()[..] {
            let expected = vec![
                vec![(_a, _b), (_c, _d)],
                vec![(_b, _c), (_d, _a)],
                vec![(_c, _a), (_b, _d)],
                vec![(_b, _a), (_d, _c)],
                vec![(_c, _b), (_a, _d)],
                vec![(_a, _c), (_d, _b)],
            ];

            let actual = create_tournement(&input, 2);

            assert_eq!(expected, actual);
        } else {
            panic!("Coudn't destructure array of pointers");
        };
    }

    #[test]
    fn odd() {
        let input = vec![0, 1, 2, 3, 4];

        if let [_0, _1, _2, _3, _4] = input.iter().map(|x| x).collect::<Vec<&i32>>()[..] {
            let expected = vec![
                vec![(_0, _3), (_2, _1)],
                vec![(_1, _4), (_3, _2)],
                vec![(_2, _0), (_4, _3)],
                vec![(_3, _1), (_0, _4)],
                vec![(_4, _2), (_1, _0)],
            ];

            let actual = create_tournement(&input, 1);

            assert_eq!(expected, actual);
        } else {
            panic!("Coudn't destructure array of pointers");
        };
    }
}
