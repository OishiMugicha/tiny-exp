pub mod ast;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;

use wasm_bindgen::prelude::*;

/// JSへは表示用の文字列のみを渡します。i64をJSのNumberに変換しません。
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct RunOutput {
    pub tokens: String,
    pub ast: String,
    pub value: String,
    pub error: String,
}

#[wasm_bindgen]
pub fn run(source: &str) -> RunOutput {
    let mut output = RunOutput::default();
    let result = (|| {
        let tokens = lexer::lex(source)?;
        output.tokens = format!("{tokens:#?}");
        let ast = parser::parse(&tokens)?;
        output.ast = format!("{ast:#?}");
        let value = eval::eval(&ast)?;
        output.value = match value {
            ast::Value::Int(i) => i.to_string(),
            ast::Value::Bool(b) => b.to_string(),
        };
        Ok::<(), error::Error>(())
    })();
    if let Err(error) = result {
        output.error = error.to_string();
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runner_returns_successful_stages() {
        let output = run("1 + 2 * 3");
        assert!(!output.tokens.is_empty());
        assert!(!output.ast.is_empty());
        assert_eq!(output.value, "7");
        assert!(output.error.is_empty());
    }

    #[test]
    fn runner_preserves_completed_stages_on_failure() {
        for (source, tokens_complete, ast_complete, error_prefix) in [
            ("@", false, false, "字句解析エラー:"),
            ("1 +", true, false, "構文エラー:"),
            ("true + 1", true, true, "型エラー:"),
            (
                "9223372036854775807 + 1",
                true,
                true,
                "整数の範囲を超えました",
            ),
        ] {
            let output = run(source);
            assert_eq!(!output.tokens.is_empty(), tokens_complete, "{source}");
            assert_eq!(!output.ast.is_empty(), ast_complete, "{source}");
            assert!(output.value.is_empty(), "{source}");
            assert!(
                output.error.starts_with(error_prefix),
                "{source}: {}",
                output.error
            );
        }
    }
}
