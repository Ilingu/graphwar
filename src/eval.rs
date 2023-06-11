use std::f64::consts::{E, LN_2, PI, SQRT_2, TAU};

use evalexpr::{eval_number, EvalexprError};

pub struct MathExpression {
    expr: String,
}

impl MathExpression {
    pub fn new(raw_expr: &str) -> Result<Self, EvalexprError> {
        let evaluable = Self::convert_to_evaluable(raw_expr.to_string());
        eval_number(
            Self::replace_with_value(&evaluable, 0.0 /* no matter which value I put here */)
                .as_str(),
        )?;

        // TODO: won't catch thing like: exp(1) for example
        if !evaluable.contains('x') {
            return Err(EvalexprError::CustomMessage(
                "this line is constant, which is forbidden 😊".to_string(),
            ));
        }

        let valid_expr = Self { expr: evaluable };
        Ok(valid_expr)
    }

    pub fn compute(&self, x: f64) -> Result<f64, EvalexprError> {
        eval_number(Self::replace_with_value(&self.expr, x).as_str())
    }

    fn replace_with_value(expr: &str, x: f64) -> String {
        expr.replace("exp", "ewp")
            .replace('x', format!("({x})").as_str())
            .replace("ewp", "exp")
    }

    fn convert_to_evaluable(expr: String) -> String {
        let mut math_keywords = vec![];

        let (mut current_word, mut current_word_start) = (String::new(), 0);
        for (i, ch) in expr.chars().enumerate() {
            match ch.is_ascii_alphabetic() {
                true => match current_word == *"" {
                    true => {
                        current_word = format!("{ch}");
                        current_word_start = i
                    }
                    false => current_word.push(ch),
                },
                false => {
                    match current_word.as_str() {
                        "x" | "" => (),
                        _ => math_keywords.push((current_word_start, i - 1, current_word.clone())),
                    }
                    current_word = String::new()
                }
            }
        }
        match current_word.as_str() {
            "x" | "" => (),
            _ => math_keywords.push((current_word_start, expr.len() - 1, current_word.clone())),
        }

        let mut evaluable_expr = expr;
        for (start_idx, end_idx, content) in math_keywords.iter().rev() {
            let indexable = evaluable_expr.as_str();

            match content.to_lowercase().as_str() {
                // some math constant
                "pi" => {
                    evaluable_expr = format!(
                        "{}{}{}",
                        &indexable[..*start_idx],
                        PI,
                        &indexable[*end_idx + 1..]
                    )
                }
                "tau" => {
                    evaluable_expr = format!(
                        "{}{}{}",
                        &indexable[..*start_idx],
                        TAU,
                        &indexable[*end_idx + 1..]
                    )
                }
                "sqrttwo" => {
                    evaluable_expr = format!(
                        "{}{}{}",
                        &indexable[..*start_idx],
                        SQRT_2,
                        &indexable[*end_idx + 1..]
                    )
                }
                "e" => {
                    evaluable_expr = format!(
                        "{}{}{}",
                        &indexable[..*start_idx],
                        E,
                        &indexable[*end_idx + 1..]
                    )
                }
                "lntwo" => {
                    evaluable_expr = format!(
                        "{}{}{}",
                        &indexable[..*start_idx],
                        LN_2,
                        &indexable[*end_idx + 1..]
                    )
                }
                // math function
                _ => {
                    evaluable_expr = format!(
                        "{}math::{}",
                        &indexable[..*start_idx],
                        &indexable[*start_idx..]
                    )
                }
            }
        }
        evaluable_expr
    }
}
