use crate::CalcMode::{
    ADDITION, DIVISION, EXPONENT, LPAREN, MULTIPLICATION, NUMBER, RPAREN, SUBTRACTION,
};
use std::env;
use std::process::exit;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum CalcMode {
    NUMBER = 5,
    ADDITION = 1,
    SUBTRACTION = 0,
    MULTIPLICATION = 3,
    DIVISION = 2,
    EXPONENT = 4,
    RPAREN = 6,
    LPAREN = 7,
}

#[derive(Clone)]
struct CalcTok {
    mode: CalcMode,
    value: Option<f32>,
    left_tok: Option<Box<CalcTok>>,
    right_tok: Option<Box<CalcTok>>,
}

fn get_precedence(mode: CalcMode) -> i32 {
    match mode {
        ADDITION => 1,
        SUBTRACTION => 1,
        MULTIPLICATION => 2,
        DIVISION => 2,
        EXPONENT => 3,
        _ => -1,
    }
}

fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '^'
}

fn parse_expression(expression: &mut Vec<char>) -> Vec<CalcTok> {
    expression.retain(|c| !c.is_whitespace());
    let mut tokens: Vec<CalcTok> = Vec::new();
    let mut i = 0;
    while i < expression.len() {
        let c: char = expression[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() || c == '.' || c == '-' && (i == 0 || is_operator(expression[i - 1]))
        {
            let num_start = i;
            let mut dots_found: u8 = 0;
            if c == '-' {
                i += 1;
            }

            while i < expression.len() && (expression[i].is_ascii_digit() || expression[i] == '.') {
                if expression[i] == '.' {
                    dots_found += 1;
                    if dots_found > 1 {
                        break;
                    }
                }
                i += 1;
            }
            let num_str: String = expression[num_start..i].iter().collect();
            let num: f32 = num_str.parse().unwrap();
            tokens.push(CalcTok {
                mode: NUMBER,
                value: Some(num),
                left_tok: None,
                right_tok: None,
            });
        } else {
            match c {
                '+' => tokens.push(CalcTok {
                    mode: ADDITION,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                '-' => tokens.push(CalcTok {
                    mode: SUBTRACTION,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                '*' => tokens.push(CalcTok {
                    mode: MULTIPLICATION,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                '/' => tokens.push(CalcTok {
                    mode: DIVISION,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                '^' => tokens.push(CalcTok {
                    mode: EXPONENT,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                '(' => tokens.push(CalcTok {
                    mode: LPAREN,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                ')' => tokens.push(CalcTok {
                    mode: RPAREN,
                    value: None,
                    left_tok: None,
                    right_tok: None,
                }),
                _ => {
                    println!("Unknown token: {}", c);
                    exit(1)
                }
            }
            i += 1;
        }
    }
    tokens
}

fn make_tree(tokens: &[CalcTok]) -> CalcTok {
    if tokens.len() == 1 {
        return tokens[0].clone();
    }

    let mut top_level_index: Option<usize> = None;
    let mut top_level_precedence: i32 = -1;
    let mut current_paren_scope: usize = 0;

    for (i, token) in tokens.iter().enumerate() {
        let mode_val: i32 = token.mode as i32;
        let precedence: i32 = get_precedence(token.mode);
        if token.mode == LPAREN {
            current_paren_scope += 1;
        } else if token.mode == RPAREN {
            current_paren_scope -= 1;
        } else if current_paren_scope == 0
            && (0..=4).contains(&mode_val)
            && (top_level_index.is_none() || precedence <= top_level_precedence)
        {
            top_level_index = Some(i);
            top_level_precedence = precedence;
        }
    }

    let idx: usize = top_level_index.expect("No operators found!");
    let mut top: CalcTok = tokens[idx].clone();

    let left_slice: &[CalcTok] = strip_outer_parens(&tokens[..idx]);
    let right_slice: &[CalcTok] = strip_outer_parens(&tokens[idx + 1..]);

    top.left_tok = Some(Box::new(make_tree(left_slice)));
    top.right_tok = Some(Box::new(make_tree(right_slice)));
    top
}

fn strip_outer_parens(tokens: &[CalcTok]) -> &[CalcTok] {
    if tokens.len() >= 2 && tokens[0].mode == LPAREN && tokens[tokens.len() - 1].mode == RPAREN {
        &tokens[1..tokens.len() - 1]
    } else {
        tokens
    }
}

fn evaluate(tok: &CalcTok) -> f32 {
    let left_value: f32 = if tok.left_tok.as_ref().unwrap().mode != NUMBER {
        evaluate(tok.left_tok.as_ref().unwrap())
    } else {
        tok.left_tok.as_ref().unwrap().value.unwrap()
    };

    let right_value: f32 = if tok.right_tok.as_ref().unwrap().mode != NUMBER {
        evaluate(tok.right_tok.as_ref().unwrap())
    } else {
        tok.right_tok.as_ref().unwrap().value.unwrap()
    };

    match tok.mode {
        ADDITION => left_value + right_value,
        SUBTRACTION => left_value - right_value,
        MULTIPLICATION => left_value * right_value,
        DIVISION => left_value / right_value,
        EXPONENT => left_value.powf(right_value),
        _ => 0.0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument but got {}!", args.len() - 1);
        return;
    }

    let tokens: Vec<CalcTok> = parse_expression(&mut args[1].chars().collect());
    let top_level_token: CalcTok = make_tree(&tokens);
    println!(" = {}", evaluate(&top_level_token));
}
