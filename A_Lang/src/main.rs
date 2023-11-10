enum OperationType {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
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
            arg2: Some(Box::new(ExpressionType::N(1))),
            operation: OperationType::MULTIPLICATION
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
            OperationType::FIRST => Some(arg1),
            OperationType::SECOND => Some(arg2),
            _ => None
        }
    }

    fn to_expression(raw: String) -> Expression {

        let expr = Expression::blank();

        expr
    }
}

fn main() {
    let expr1 = Expression::wrap(5.0);
    let expr2 = Expression::wrap(10.0);
    let expr3 = Expression {
        arg1: Some(Box::new(ExpressionType::Expr(expr1))),
        arg2: Some(Box::new(ExpressionType::Expr(expr2))),
        operation: OperationType::ADDITION
    };

    println!("{}", expr3.evaluate().unwrap());
}

fn evaluate(line: &String) -> f64 {
    let result = line.parse::<f64>();
    match result {
        Ok(number) => return number,
        _ => {}
    };

    // Check for scopes

    let by_scope = split_by_scope(line);

    // Evaluate scopes

    for scoped_expression in &by_scope {
        evaluate(scoped_expression);
    }

    // Check for exponents



    // Check for Multiplication + Division

    // Check for Addition + Subtraction

    0.0
}

fn split_by_scope(raw: &String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![String::new()];

    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];

    let mut depth = 0;

    for character in raw.chars() {
        if openings.contains(&character) {
            tokens.push(String::new());
        } else if closings.contains(&character) {
            tokens.push(String::new());
        } else {
            let current = tokens.len() - 1;
            (&mut tokens[current]).push(character);
        }
    }

    tokens
}
