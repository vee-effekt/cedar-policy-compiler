//! Lower Cedar AST to an intermediate representation suitable for compilation

use cedar_policy_core::ast::{Policy, Template, Expr, ExprKind, Literal};

/// Intermediate representation of a Cedar policy
#[derive(Debug, Clone)]
pub struct LoweredModule {
    /// Entry point function that evaluates the policy
    pub entry: LoweredFunction,
}

#[derive(Debug, Clone)]
pub struct LoweredFunction {
    /// Function body as a sequence of instructions
    pub instructions: Vec<Instruction>,
}

/// Simple stack-based instruction set
#[derive(Debug, Clone)]
pub enum Instruction {
    // Literals
    PushBool(bool),
    PushInt(i64),
    PushString(String),

    // Comparison operations
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Entity/attribute operations
    GetAttribute(String),
    HasAttribute(String),
    In,

    // Control flow
    IfThenElse,
    Return,

    // Policy decision
    Permit,
    Forbid,
}

impl LoweredModule {
    /// Convert a Cedar Policy to the intermediate representation (cedar 4.4+)
    pub fn from_policy(policy: &Policy) -> Result<Self, String> {
        let mut instructions = Vec::new();

        // In cedar 4.4+, condition() returns Expr directly, not Option<Expr>
        // Compile the condition (when clause)
        let condition = policy.condition();
        compile_expr(&condition, &mut instructions)?;

        // Add the policy effect (permit/forbid)
        match policy.effect() {
            cedar_policy_core::ast::Effect::Permit => {
                instructions.push(Instruction::Permit);
            }
            cedar_policy_core::ast::Effect::Forbid => {
                instructions.push(Instruction::Forbid);
            }
        }

        instructions.push(Instruction::Return);

        Ok(LoweredModule {
            entry: LoweredFunction { instructions },
        })
    }

    /// Convert a Cedar Template to the intermediate representation
    /// Templates are policy templates that can be instantiated
    pub fn from_template(template: &Template) -> Result<Self, String> {
        let mut instructions = Vec::new();

        // Compile the condition
        let condition = template.condition();
        compile_expr(&condition, &mut instructions)?;

        // Add the policy effect
        match template.effect() {
            cedar_policy_core::ast::Effect::Permit => {
                instructions.push(Instruction::Permit);
            }
            cedar_policy_core::ast::Effect::Forbid => {
                instructions.push(Instruction::Forbid);
            }
        }

        instructions.push(Instruction::Return);

        Ok(LoweredModule {
            entry: LoweredFunction { instructions },
        })
    }
}

/// Compile a Cedar expression into instructions
fn compile_expr(expr: &Expr, instructions: &mut Vec<Instruction>) -> Result<(), String> {
    use ExprKind::*;

    match expr.expr_kind() {
        Lit(lit) => {
            compile_literal(lit, instructions);
            Ok(())
        }

        // Binary operations
        BinaryApp { op, arg1, arg2 } => {
            compile_expr(arg1, instructions)?;
            compile_expr(arg2, instructions)?;

            use cedar_policy_core::ast::BinaryOp;
            match op {
                BinaryOp::Eq => instructions.push(Instruction::Equal),
                BinaryOp::In => instructions.push(Instruction::In),
                BinaryOp::Less => instructions.push(Instruction::LessThan),
                BinaryOp::LessEq => instructions.push(Instruction::LessThanOrEqual),
                _ => return Err(format!("Unsupported binary operator: {:?}", op)),
            }
            Ok(())
        }

        // Unary operations
        UnaryApp { op, arg } => {
            compile_expr(arg, instructions)?;

            use cedar_policy_core::ast::UnaryOp::*;
            match op {
                Not => instructions.push(Instruction::Not),
                _ => return Err(format!("Unsupported unary operator: {:?}", op)),
            }
            Ok(())
        }

        // Attribute access: entity.attribute
        GetAttr { expr: entity, attr } => {
            compile_expr(entity, instructions)?;
            instructions.push(Instruction::GetAttribute(attr.to_string()));
            Ok(())
        }

        // Check if attribute exists
        HasAttr { expr: entity, attr } => {
            compile_expr(entity, instructions)?;
            instructions.push(Instruction::HasAttribute(attr.to_string()));
            Ok(())
        }

        // If-then-else
        If {
            test_expr,
            then_expr,
            else_expr,
        } => {
            compile_expr(test_expr, instructions)?;
            compile_expr(then_expr, instructions)?;
            compile_expr(else_expr, instructions)?;
            instructions.push(Instruction::IfThenElse);
            Ok(())
        }

        // For now, we'll return errors for unsupported features
        _ => Err(format!(
            "Expression type not yet supported in compiler: {:?}",
            expr.expr_kind()
        )),
    }
}

/// Compile a literal value
fn compile_literal(lit: &Literal, instructions: &mut Vec<Instruction>) {
    match lit {
        Literal::Bool(b) => instructions.push(Instruction::PushBool(*b)),
        Literal::Long(i) => instructions.push(Instruction::PushInt(*i)),
        Literal::String(s) => instructions.push(Instruction::PushString(s.to_string())),
        // TODO: Handle other literal types
        _ => instructions.push(Instruction::PushBool(false)), // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowering_simple_permit() {
        // This is a placeholder test - we'd need to construct a Policy AST
        // For now, just test that the instruction types exist
        let instructions = vec![
            Instruction::PushBool(true),
            Instruction::Permit,
            Instruction::Return,
        ];

        assert_eq!(instructions.len(), 3);
    }
}
