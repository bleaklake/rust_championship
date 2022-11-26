use championship::create_tournement;

fn main() {
    let list = vec![0, 1, 2, 3, 4];

    let res = create_tournement(&list, 1);

    println!("{:#?}", res);
    /*
    [
        [
            (
                0,
                3,
            ),
            (
                2,
                1,
            ),
        ],
        [
            (
                1,
                4,
            ),
            (
                3,
                2,
            ),
        ],
        [
            (
                2,
                0,
            ),
            (
                4,
                3,
            ),
        ],
        [
            (
                3,
                1,
            ),
            (
                0,
                4,
            ),
        ],
        [
            (
                4,
                2,
            ),
            (
                1,
                0,
            ),
        ],
    ]
        */
}
