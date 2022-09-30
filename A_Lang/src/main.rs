use std::ops;

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

impl OperationType {
    fn to_operation_type(raw: &str) -> OperationType {
        match raw {
            "+" => OperationType::ADDITION,
            "-" => OperationType::SUBTRACTION,
            "*" => OperationType::MULTIPLICATION,
            "/" => OperationType::DIVISION,
            "^" => OperationType::POW,
            _ => OperationType::IDLE
        }
    }
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

impl ops::Add<Expression> for Expression {
    type Output = Option<f64>;

    fn add(self, other: Expression) -> Option<f64> {
        Expression::new(
            ExpressionType::N(match self.evaluate() { Some(x) => x, None => 0.0 }), // TODO: refactor this code so that it properly handles improper inputs.
            ExpressionType::N(match other.evaluate() { Some(x) => x, None => 0.0 }),
            OperationType::ADDITION
        ).evaluate()
    }
}

impl Expression {

    fn blank() -> Expression {
        Expression {
            arg1: Option::None,
            arg2: Option::None,
            operation: OperationType::IDLE
        }
    }

    fn zero() -> Expression {
        Expression {
            arg1: Some(Box::new(ExpressionType::N(0.0))),
            arg2: None,
            operation: OperationType::FIRST
        }
    }

    fn wrap(n: f64) -> Expression {
        Expression {
            arg1: Some(Box::new(ExpressionType::N(n))),
            arg2: None,
            operation: OperationType::FIRST
        }
    }

    fn new(arg1: ExpressionType, arg2: ExpressionType, op: OperationType) -> Expression {
        Expression {
            arg1: Some(Box::new(arg1)),
            arg2: Some(Box::new(arg2)),
            operation: op
        }
    }

    fn equals(&self, other: Expression) -> bool {
        false
    }

    fn to_expression(raw: String) -> Expression {

        let args = raw.split(" ").map(|x| { String::from(x) }).collect::<Vec<String>>(); // use split by scope

        let ops: Vec<(&str, &str)> = vec![("*", "/"), ("+", "-")];
        let mut acc = Expression::zero();

        for (i, arg) in args.iter().enumerate() {
            for op in &ops {
                if arg.eq(op.0) || arg.eq(op.1) {
                    let arg2 = &args[i + 1];

                    if acc.evaluate().unwrap() == 0.0 {

                    }
                }
            }
        }

        Expression::zero()
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

    let ops: Vec<(&str, &str)> = vec![("*", "/"), ("+", "-")];

    let mut tokens: Vec<String> = vec![String::new()];

    //&String::from("3 + (x + 5) + ((3x + 2) + 3)")

    split_by_scope(line, &mut tokens);

    let tokens = tokens.iter().filter(|x| { !(***x).is_empty() }).collect::<Vec<&String>>();

    let mut acc = Expression::zero();

    for (i, token) in tokens.iter().enumerate() {
        for op in &ops {
            if (*token).eq(op.0) || (*token).eq(op.1) {
                // 5 + 3 + 8 => (5 + 3) + 8
                acc = Expression::new(
                    ExpressionType::Expr(acc),
                    ExpressionType::Expr(
                        Expression::zero()
                    ),
                    OperationType::FIRST
                );
            }
        }
    }


    
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