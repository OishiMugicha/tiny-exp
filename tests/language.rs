//! 式言語の構文と評価の統合テスト。
use tiny_exp::{
    ast::{Exp, Prim, Value},
    error::Error,
    eval::eval,
    lexer::{Token, lex},
    parser::parse,
};

fn int(i: i64) -> Exp {
    Exp::Literal(Value::Int(i))
}
fn execute(source: &str) -> Result<Value, Error> {
    eval(&parse(&lex(source)?)?)
}

#[test]
fn lexer_literals_and_symbols() {
    assert_eq!(
        lex(" \n if true then (12-3)*2 else false < 4 + 1").unwrap(),
        vec![
            Token::If,
            Token::Bool(true),
            Token::Then,
            Token::LeftParen,
            Token::Integer("12".into()),
            Token::Minus,
            Token::Integer("3".into()),
            Token::RightParen,
            Token::Star,
            Token::Integer("2".into()),
            Token::Else,
            Token::Bool(false),
            Token::Less,
            Token::Integer("4".into()),
            Token::Plus,
            Token::Integer("1".into()),
        ]
    );
    assert!(lex(" \n").unwrap().is_empty());
    for source in ["@", "truex", "foo", "1 / 2"] {
        assert!(matches!(lex(source), Err(Error::Lex(_))), "{source}");
    }
}

#[test]
fn parser_literals() {
    assert_eq!(parse(&[Token::Integer("42".into())]).unwrap(), int(42));
    assert_eq!(
        parse(&[Token::Bool(false)]).unwrap(),
        Exp::Literal(Value::Bool(false))
    );
    assert_eq!(
        parse(&[Token::Minus, Token::Integer("9223372036854775808".into())]).unwrap(),
        int(i64::MIN)
    );
}

#[test]
fn eval_literals() {
    assert_eq!(eval(&int(42)).unwrap(), Value::Int(42));
    assert_eq!(
        eval(&Exp::Literal(Value::Bool(true))).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn parser_precedence() {
    assert_eq!(
        parse(&lex("1 + 2 * 3").unwrap()).unwrap(),
        Exp::Binary(
            Box::new(int(1)),
            Prim::Add,
            Box::new(Exp::Binary(Box::new(int(2)), Prim::Mul, Box::new(int(3))))
        )
    );
}

#[test]
fn eval_arithmetic() {
    for (op, expected) in [
        (Prim::Add, Value::Int(5)),
        (Prim::Sub, Value::Int(1)),
        (Prim::Mul, Value::Int(6)),
        (Prim::Lt, Value::Bool(false)),
    ] {
        assert_eq!(
            eval(&Exp::Binary(Box::new(int(3)), op, Box::new(int(2)))).unwrap(),
            expected
        );
    }
}

#[test]
fn arithmetic_end_to_end() {
    for (source, expected) in [
        ("1 + 2 * 3", 7),
        ("(1 + 2) * 3", 9),
        ("10 - 3 - 2", 5),
        ("-123", -123),
        ("1--2", 3),
        ("- 2 * 3", -6),
    ] {
        assert_eq!(execute(source).unwrap(), Value::Int(expected), "{source}");
    }
    assert_eq!(execute("1 + 2 < 4").unwrap(), Value::Bool(true));
}

#[test]
fn conditionals() {
    for (source, expected) in [
        ("if 1 < 2 then 42 else 0", 42),
        ("if true then if false then 0 else 1 else 2", 1),
        ("if false then 0 else if true then 2 else 3", 2),
        ("if true then 1 else true + 2", 1),
        ("if false then 9223372036854775807 + 1 else 2", 2),
        ("1 + (if true then 2 else 3)", 3),
    ] {
        assert_eq!(execute(source).unwrap(), Value::Int(expected), "{source}");
    }
}

#[test]
fn syntax_errors() {
    for source in [
        "",
        "1 2",
        "(1 + 2",
        "1 +",
        "if true then 1",
        "1 < 2 < 3",
        "-(1)",
        "--2",
        "-true",
        "if true then 1 else (",
    ] {
        assert!(
            matches!(parse(&lex(source).unwrap()), Err(Error::Parse(_))),
            "{source}"
        );
    }
}

#[test]
fn type_and_range_errors() {
    for source in [
        "true + 1",
        "false < true",
        "if 1 then 2 else 3",
        "(1 < 2) < 3",
    ] {
        assert!(matches!(execute(source), Err(Error::Type(_))), "{source}");
    }
    for source in [
        "9223372036854775808",
        "-9223372036854775809",
        "9223372036854775807 + 1",
        "-9223372036854775808 - 1",
        "9223372036854775807 * 2",
    ] {
        assert!(matches!(execute(source), Err(Error::Overflow)), "{source}");
    }
}

#[test]
fn whitespace_and_unknown_characters() {
    for source in ["", " ", "\t\r\n", "　"] {
        assert_eq!(lex(source).unwrap(), vec![]);
    }
    for source in ["1 ", " 1\n", "　1　"] {
        assert_eq!(execute(source).unwrap(), Value::Int(1));
    }
    for source in ["あ", "1あ", "trueあ", "🙂", "true1", "if_", "falsex"] {
        assert!(matches!(lex(source), Err(Error::Lex(_))), "{source}");
    }
    assert!(matches!(execute(" "), Err(Error::Parse(_))));
}

#[test]
fn integer_boundaries() {
    for (source, expected) in [
        ("9223372036854775807", i64::MAX),
        ("-9223372036854775808", i64::MIN),
        ("- 9223372036854775808", i64::MIN),
        ("00042", 42),
        ("-00042", -42),
        ("-0", 0),
        ("-9223372036854775808 + 1", i64::MIN + 1),
    ] {
        assert_eq!(execute(source).unwrap(), Value::Int(expected), "{source}");
    }
    for source in [
        "-9223372036854775808 + -1",
        "9223372036854775807 - -1",
        "-9223372036854775808 * -1",
        "-9223372036854775808 * 2",
    ] {
        assert_eq!(execute(source), Err(Error::Overflow), "{source}");
    }
}
