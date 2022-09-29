enum OperationType {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    POW,
    FIRST,
    SECOND,
    IDLE
}

enum ExpressionType {
    Expr(Expression),
    N(f64)
}

struct Expression {
    // an arg can either exist or not exist. If it exists, then it can either be an Expression or an f64.
    arg1: Option<Box<ExpressionType>>,
    arg2: Option<Box<ExpressionType>>,
    operation: OperationType
}

impl Expression {

    fn blank() -> Expression {
        Expression {
            arg1: Option::None,
            arg2: Option::None,
            operation: OperationType::IDLE
        }
    }

    fn wrap(n: f64) -> Expression {
        Expression {
            arg1: Some(Box::new(ExpressionType::N(n))),
            arg2: None,
            operation: OperationType::FIRST
        }
    }

    fn evaluate(&self) -> Option<f64> {

        let blank = Box::new(ExpressionType::N(0.0));

        let arg1 = match &self.arg1 { Some(x) => x, None => &blank };
        let arg2 = match &self.arg2 { Some(x) => x, None => &blank };

        let arg1 = match &**arg1 {
            ExpressionType::Expr(expr) => expr.evaluate(),
            ExpressionType::N(n) => Some(*n)
        };

        let arg2 = match &**arg2 {
            ExpressionType::Expr(expr) => expr.evaluate(),
            ExpressionType::N(n) => Some(*n)
        };

        let arg1 = match arg1 { Some(x) => x, _ => 0.0 };
        let arg2 = match arg2 { Some(x) => x, _ => 0.0 };

        match self.operation {
            OperationType::ADDITION => Some(arg1 + arg2),
            OperationType::SUBTRACTION => Some(arg1 - arg2),
            OperationType::MULTIPLICATION => Some(arg1 * arg2),
            OperationType::DIVISION => Some(arg1 / arg2),
            OperationType::POW => Some(arg1.powf(arg2)),
            OperationType::FIRST => Some(arg1),
            OperationType::SECOND => Some(arg2),
            _ => None
        }
    }
}

fn main() {

    let expr1 = Expression {
        arg1: Some(Box::new(ExpressionType::Expr(Expression::wrap(2.0)))),
        arg2: Some(Box::new(ExpressionType::Expr(Expression::wrap(3.0)))),
        operation: OperationType::POW
    };
    let expr2 = Expression::wrap(10.0);
    let expr3 = Expression {
        arg1: Some(Box::new(ExpressionType::Expr(expr1))),
        arg2: Some(Box::new(ExpressionType::Expr(expr2))),
        operation: OperationType::ADDITION
    };

    println!("{}", expr3.evaluate().unwrap());
}

fn evaluate(line: &String) -> Expression {
    let result = line.parse::<f64>();
    match result {
        Ok(number) => return Expression::wrap(number),
        _ => {}
    };

    let mut tokens: Vec<String> = vec![String::new()];

    //&String::from("3 + (x + 5) + ((3x + 2) + 3)")

    split_by_scope(line, &mut tokens);

    let tokens = tokens.iter().filter(|x| { !(***x).is_empty() }).collect::<Vec<&String>>();

    let tokens = tokens.iter().map(|x| { evaluate(x) });
    
    Expression::blank()
}

fn split_by_scope(raw: & String, tokens: &mut Vec<String>) {

    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];
    let ops = ['+', '-', '*', '/', '^'];

    let mut depth = 0;

    for character in raw.chars() {
        if character.is_whitespace() {
            continue;
        }
        if openings.contains(&character) {
            if depth == 0 {
                tokens.push(String::new());
            } else {
                let current = tokens.len() - 1;
                (&mut tokens[current]).push(character);
            } depth += 1;
        } else if closings.contains(&character) {
            depth -= 1;
            if depth == 0 {
                tokens.push(String::new());
            } else {
                let current = tokens.len() - 1;
                (&mut tokens[current]).push(character);
            }
        } else {
            if depth == 0 && ops.contains(&character) {
                tokens.push(String::from(character));
                tokens.push(String::new());
            } else {
                let current = tokens.len() - 1;
                (&mut tokens[current]).push(character);
            }
        }
    }

}
