use championship::create_tournement;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let list = vec![
        User {
            name: "Alice".to_string(),
            age: 20,
        },
        User {
            name: "Bob".to_string(),
            age: 21,
        },
        User {
            name: "Carol".to_string(),
            age: 22,
        },
        User {
            name: "Dan".to_string(),
            age: 23,
        },
    ];

    let res = create_tournement(&list, 2);

    println!("{:#?}", res);
    /*
        [
        [
            (
                User {
                    name: "Alice",
                    age: 20,
                },
                User {
                    name: "Bob",
                    age: 21,
                },
            ),
            (
                User {
                    name: "Carol",
                    age: 22,
                },
                User {
                    name: "Dan",
                    age: 23,
                },
            ),
        ],
        [
            (
                User {
                    name: "Bob",
                    age: 21,
                },
                User {
                    name: "Carol",
                    age: 22,
                },
            ),
            (
                User {
                    name: "Dan",
                    age: 23,
                },
                User {
                    name: "Alice",
                    age: 20,
                },
            ),
        ],
        [
            (
                User {
                    name: "Carol",
                    age: 22,
                },
                User {
                    name: "Alice",
                    age: 20,
                },
            ),
            (
                User {
                    name: "Bob",
                    age: 21,
                },
                User {
                    name: "Dan",
                    age: 23,
                },
            ),
        ],
        [
            (
                User {
                    name: "Bob",
                    age: 21,
                },
                User {
                    name: "Alice",
                    age: 20,
                },
            ),
            (
                User {
                    name: "Dan",
                    age: 23,
                },
                User {
                    name: "Carol",
                    age: 22,
                },
            ),
        ],
        [
            (
                User {
                    name: "Carol",
                    age: 22,
                },
                User {
                    name: "Bob",
                    age: 21,
                },
            ),
            (
                User {
                    name: "Alice",
                    age: 20,
                },
                User {
                    name: "Dan",
                    age: 23,
                },
            ),
        ],
        [
            (
                User {
                    name: "Alice",
                    age: 20,
                },
                User {
                    name: "Carol",
                    age: 22,
                },
            ),
            (
                User {
                    name: "Dan",
                    age: 23,
                },
                User {
                    name: "Bob",
                    age: 21,
                },
            ),
        ],
    ]
        */
}
