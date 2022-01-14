// @2
// D=A
// @3
// D=D+A
// @0
// M=D
// this picture shows, this image depicts, this is a photograph of 

fn main() {
    let source = "Hello, world!\n2nd line";
    let parser = Parser::new(source);
    parser.print_rows();
    parser.advance();
    println!("{}", parser.current_row());
}

enum CommandType {
    A,
    C,
    L,
}

struct Parser<'a> {
    rows: Vec<&'a str>,
    index: usize,
}

impl Parser<'_> {
    fn new(source: &str) -> Parser {
        return Parser {
            rows: source.split("\n").collect(),
            index: 0,
        };
    }

    fn has_more_commands(&self) -> bool {
        return self.rows.len() > self.index + 1;
    }

    fn advance(mut self) {
        self.index += 1
    }

    fn command_type(&self) -> CommandType {
        let current = self.current_row();
        if current.starts_with('@') {
            return CommandType::A;
        }
        if self.dest() == self.comp() {
            return CommandType::C;
        }
        return CommandType::L;
    }

    fn symbol(&self) -> &str {
        return "";
    }

    fn dest(&self) -> &str {
        return "";
    }

    fn comp(&self) -> &str {
        return "";
    }

    fn jump(&self) -> &str {
        return "";
    }

    fn print_rows(&self) {
        for (i, val) in self.rows.iter().enumerate() {
            println!("{}: {}", i, val);
        }
    }

    fn current_row(&self) -> &str {
        return self.rows[self.index];
    }
}
