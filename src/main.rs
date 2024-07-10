use std::io;

fn main() {
    let mut todo_list: Vec<Todo> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<String> = input.split_whitespace().map(str::to_string).collect();
        match inputs[0].as_str() {
            "add" => {
                for v in inputs {
                    let todo = Todo {
                        title: v,
                        is_completed: false,
                    };
                    todo_list.push(todo);
                }
            },
            "list" => {
                for todo in &todo_list {
                    println!("{}", todo.title);
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

enum TODO_CASES {
    
}