use parser::parse_statements;

mod ast;
mod parser;

fn main() {
    let input = r#"
        val pi = 3;
        var x = 4;
        x := 10;
        write(x);
    "#;

    match parse_statements(input) {
        Ok((_remaining, statements)) => {
            for statement in statements {
                println!("{:?}", statement);
            }
        }
        Err(e) => eprintln!("Error parsing input: {:?}", e),
    }
}
