//! Rust mirror van Core's AST-types.
//!
//! Bron van waarheid blijft `_Core/src/ast/types.ts` (P-QBE-01). Deze module dupliceert
//! een minimale subset voor decompiler-output. Bij Core-AST-wijzigingen: handmatige
//! synchronisatie totdat we in fase-2 (v0.7.x+) een code-gen-pipeline introduceren.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Statement {
    Print { args: Vec<Expression> },
    Let { target: String, value: Expression },
    Goto { target: String },
    Gosub { target: String },
    Return,
    End,
    Comment { text: String },
    /// Placeholder voor decompiler-stub waar we de exacte statement niet kunnen reconstrueren.
    Unknown { raw_offset: u32, bytes: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Expression {
    NumberLiteral { value: f64 },
    StringLiteral { value: String },
    Variable { name: String },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub dialect: String,
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            dialect: "qb45".to_string(),
            statements: Vec::new(),
        }
    }

    /// Render to a `.bas`-style source string. Variable names get auto-generated `V001` form
    /// since originals are lost during compilation (P-QBE-05 erkend).
    pub fn to_bas(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            match stmt {
                Statement::Print { args } => {
                    out.push_str("PRINT ");
                    for (i, e) in args.iter().enumerate() {
                        if i > 0 {
                            out.push_str("; ");
                        }
                        out.push_str(&render_expr(e));
                    }
                    out.push('\n');
                }
                Statement::Let { target, value } => {
                    out.push_str(&format!("{} = {}\n", target, render_expr(value)));
                }
                Statement::Goto { target } => out.push_str(&format!("GOTO {}\n", target)),
                Statement::Gosub { target } => out.push_str(&format!("GOSUB {}\n", target)),
                Statement::Return => out.push_str("RETURN\n"),
                Statement::End => out.push_str("END\n"),
                Statement::Comment { text } => out.push_str(&format!("' {}\n", text)),
                Statement::Unknown { raw_offset, bytes } => out.push_str(&format!(
                    "' [unrecognized statement at offset {:#x}, {} bytes]\n",
                    raw_offset, bytes
                )),
            }
        }
        out
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

fn render_expr(e: &Expression) -> String {
    match e {
        Expression::NumberLiteral { value } => format!("{}", value),
        Expression::StringLiteral { value } => format!("\"{}\"", value),
        Expression::Variable { name } => name.clone(),
        Expression::Unknown => "?".to_string(),
    }
}
