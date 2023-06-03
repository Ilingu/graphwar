use evalexpr::{eval_number, EvalexprError};

pub struct MathExpression {
    expr: String,
}

impl MathExpression {
    pub fn new(raw_expr: &str) -> Result<Self, EvalexprError> {
        let evaluable = Self::convert_to_evaluable(raw_expr.to_string());
        eval_number(Self::replace_with_value(&evaluable, 1.0).as_str())?;

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
        expr.replace("ln(", "math::ln(")
            .replace("log(", "math::log(")
            .replace("log2(", "math::log2(")
            .replace("log10(", "math::log10(")
            .replace("exp(", "math::exp(")
            .replace("exp2(", "math::exp2(")
            .replace("pow(", "math::pow(")
            .replace("cos(", "math::cos(")
            .replace("acos(", "math::acos(")
            .replace("cosh(", "math::cosh(")
            .replace("acosh(", "math::acosh(")
            .replace("sin(", "math::sin(")
            .replace("asin(", "math::asin(")
            .replace("sinh(", "math::sinh(")
            .replace("asinh(", "math::asinh(")
            .replace("tan(", "math::tan(")
            .replace("atan(", "math::atan(")
            .replace("atan2(", "math::atan2(")
            .replace("tanh(", "math::tanh(")
            .replace("atanh(", "math::atanh(")
            .replace("sqrt(", "math::sqrt(")
            .replace("cbrt(", "math::cbrt(")
            .replace("hypot(", "math::hypot(")
            .replace("abs(", "math::abs(")
    }
}
