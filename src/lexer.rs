use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // 符号は含めず、数字列のまま保持します。i64への変換はパーサーの役割です。
    // これにより i64::MIN の絶対値も字句解析できます。
    Integer(String),
    Bool(bool),
    Plus,
    Minus,
    Star,
    Less,
    If,
    Then,
    Else,
    LeftParen,
    RightParen,
}

/// 空白を除き、数字列・キーワード・記号をトークンに変換します。
/// 符号は独立したトークンとして扱います。
pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    let mut chars = source.char_indices().peekable();
    let mut tokens = Vec::new();
    while let Some((start, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        let token = match c {
            '0'..='9' => {
                let mut end = start + c.len_utf8();
                while let Some(&(position, next)) = chars.peek() {
                    if !next.is_ascii_digit() {
                        break;
                    }
                    end = position + next.len_utf8();
                    chars.next();
                }
                Token::Integer(source[start..end].to_owned())
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end = start + c.len_utf8();
                while let Some(&(position, next)) = chars.peek() {
                    if !(next.is_alphanumeric() || next == '_') {
                        break;
                    }
                    end = position + next.len_utf8();
                    chars.next();
                }
                match &source[start..end] {
                    "if" => Token::If,
                    "then" => Token::Then,
                    "else" => Token::Else,
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    word => return Err(Error::Lex(format!("未知の単語: {word}"))),
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '<' => Token::Less,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => return Err(Error::Lex(format!("未知の文字: {c}"))),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
