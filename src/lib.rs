/// A parser that takes a string and computes its numerical value using the given rules.
/// Operators are applied in order of precedence from left to right.
/// An exception to this is brackets, which are used to explicitly denote precedence
/// by grouping parts of an expression that should be evaluated first.
/// Rules: a = ‘+’, b = ‘-’, c = ‘*’, d = ‘/’, e = ‘(’, f = ‘)’
pub fn parse(string: &str) -> i32 {
    // Shunting Yard Algorithm to produce a Reverse Polish Notation (RPN) expression.
    let mut output_queue = Vec::<NumberOrOperator>::new();
    let mut operator_stack = Vec::<Operator>::new();
    let mut current_number = String::new();

    // Helper macro to avoid code duplication;
    // flush the current number and push it to the output queue.
    macro_rules! flush_current_number {
        () => {
            if !current_number.is_empty() {
                output_queue.push(current_number.parse::<i32>().unwrap().into());
                current_number.clear();
            }
        };
    }

    for c in string.chars() {
        match c {
            '0'..='9' => current_number.push(c),
            'a'..='d' => {
                flush_current_number!();
                // While there is an operator token, o2, at the top of the operator stack
                // which is not a left parenthesis, pop o2 off the operator stack, onto the output queue.
                let o1 = Operator::from(c);
                while !operator_stack.is_empty()
                    && *operator_stack.last().unwrap() != Operator::LBra
                {
                    output_queue.push(operator_stack.pop().unwrap().into());
                }
                // At the end of iteration push o1 onto the operator stack.
                operator_stack.push(o1);
            }
            'e' => operator_stack.push(Operator::from(c).into()),
            'f' => {
                flush_current_number!();
                // Until the token at the top of the stack is a left parenthesis,
                // pop operators off the stack onto the output queue.
                while !operator_stack.is_empty()
                    && *operator_stack.last().unwrap() != Operator::LBra
                {
                    output_queue.push(operator_stack.pop().unwrap().into());
                }
                if operator_stack.is_empty() {
                    panic!("Invalid expression, missing left parenthesis");
                }
                // Pop the left parenthesis from the stack, but not onto the output queue.
                operator_stack.pop();
            }
            _ => panic!("Invalid character"),
        }
    }

    // We need to flush the last number of the operation, if any.
    flush_current_number!();

    // When there are no more tokens to read, while there are still operator tokens in the stack:
    // if the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses.
    while !operator_stack.is_empty() {
        let ope = operator_stack.pop().unwrap();
        if ope == Operator::LBra {
            panic!("Invalid expression, missing right parenthesis");
        }
        output_queue.push(ope.into());
    }

    // Now the output queue is in RPN, we can evaluate it.
    let mut output_stack = Vec::<i32>::new();
    while !output_queue.is_empty() {
        match output_queue.remove(0) {
            NumberOrOperator::Number(n) => output_stack.push(n),
            NumberOrOperator::Operator(o) => {
                let n2 = output_stack.pop().unwrap();
                let n1 = output_stack.pop().unwrap();
                let result = match o {
                    Operator::Sum => n1 + n2,
                    Operator::Sub => n1 - n2,
                    Operator::Mul => n1 * n2,
                    Operator::Div => n1 / n2,
                    _ => panic!("Invalid operator"),
                };
                output_stack.push(result);
            }
        }
    }

    output_stack.pop().unwrap_or_default()
}

pub enum NumberOrOperator {
    Number(i32),
    Operator(Operator),
}

impl From<i32> for NumberOrOperator {
    fn from(value: i32) -> Self {
        NumberOrOperator::Number(value)
    }
}

impl From<Operator> for NumberOrOperator {
    fn from(value: Operator) -> Self {
        NumberOrOperator::Operator(value)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Sum,
    Sub,
    Mul,
    Div,
    LBra,
    RBra,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            'a' => Operator::Sum,
            'b' => Operator::Sub,
            'c' => Operator::Mul,
            'd' => Operator::Div,
            'e' => Operator::LBra,
            'f' => Operator::RBra,
            _ => panic!("Invalid character"),
        }
    }
}

#[test]
fn given_tests() {
    assert_eq!(parse("3a2c4"), 20);
    assert_eq!(parse("32a2d2"), 17);
    assert_eq!(parse("500a10b66c32"), 14208);
    assert_eq!(parse("3ae4c66fb32"), 235);
    assert_eq!(parse("3c4d2aee2a4c41fc4f"), 990);
}

#[test]
fn custom_tests() {
    assert_eq!(parse(""), 0);
    assert_eq!(parse("1"), 1);
    assert_eq!(parse("123ae2d2f"), 124);
}

#[test]
#[should_panic]
fn invalid_character() {
    parse("abcdefg");
}

#[test]
#[should_panic]
fn missing_right_parenthesis_test() {
    parse("123ae2d2");
}
