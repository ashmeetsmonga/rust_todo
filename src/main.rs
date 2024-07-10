use std::io;

fn main() {
    let mut todo_list: Vec<Todo> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<String> = input.split_whitespace().map(str::to_string).collect();
        match inputs[0].as_str() {
            "add" => {
                for i in 1..inputs.len() {
                    let todo = Todo {
                        title: inputs[i].clone(),
                        is_completed: false,
                    };
                    todo_list.push(todo);
                }
            },
            "list" => {
                for i in 0..todo_list.len() {
                    println!("{}: {}", i + 1, todo_list[i].title);
                }
            },
            _ => println!("Match not found"),
        }
    }
}

struct Todo {
    title: String,
    is_completed: bool,
}