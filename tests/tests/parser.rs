use char_lex::prelude::*;

#[token]
#[derive(Debug, PartialEq)]
enum Token {
    Whitespace = [' ', '\t', '\r', '\n'],

    Digit(Digit),
    Symbol(Symbol),
}

#[token]
#[derive(Debug, PartialEq)]
enum Digit {
    Zero = '0',
    One = '1',
    Two = '2',
    Three = '3',
    Four = '4',
    Five = '5',
    Six = '6',
    Seven = '7',
    Eight = '8',
    Nine = '9',
}

#[token]
#[derive(Debug, PartialEq)]
enum Symbol {
    Plus = '+',
    Semicolon = ';',

    Parenthesis(Parenthesis),
}

#[token]
#[derive(Debug, PartialEq)]
enum Parenthesis {
    Left = '(',
    Right = ')',
}

#[test]
fn parser_test() {
    let mut lexer: Lexer<Token, Token> = Lexer::new("1 + (3 + 6);");

    let state = statement(&mut lexer);

    assert_eq!(
        Statement::Expression(Expression::Plus(
            Box::new(Expression::Digit(Digit::One)),
            Box::new(Expression::Parenthesis(Box::new(Expression::Plus(
                Box::new(Expression::Digit(Digit::Three)),
                Box::new(Expression::Digit(Digit::Six))
            ))))
        )),
        state.unwrap()
    );
}

#[derive(Debug, PartialEq)]
enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
enum Expression {
    Digit(Digit),
    Parenthesis(Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
}

fn statement(lexer: &mut Lexer<Token, Token>) -> Option<Statement> {
    let cursor = lexer.get_cursor();

    if let Some(expr) = expression(lexer) {
        if let Some(Token::Symbol(Symbol::Semicolon)) = lexer.next_ignored(Token::Whitespace) {
            return Some(Statement::Expression(expr));
        }
    }
    lexer.set_cursor(cursor);
    None
}

fn expression(lexer: &mut Lexer<Token, Token>) -> Option<Expression> {
    if let Some(e) = parenthesis_expr(lexer) {
        Some(e)
    } else if let Some(e) = plus_expr(lexer) {
        Some(e)
    } else if let Some(e) = digit_expr(lexer) {
        Some(e)
    } else {
        None
    }
}

fn parenthesis_expr(lexer: &mut Lexer<Token, Token>) -> Option<Expression> {
    let cursor = lexer.get_cursor();

    if let Some(Token::Symbol(Symbol::Parenthesis(Parenthesis::Left))) =
        lexer.next_ignored(Token::Whitespace)
    {
        if let Some(expr) = expression(lexer) {
            if let Some(Token::Symbol(Symbol::Parenthesis(Parenthesis::Right))) =
                lexer.next_ignored(Token::Whitespace)
            {
                return Some(Expression::Parenthesis(Box::new(expr)));
            }
        }
    }
    lexer.set_cursor(cursor);
    None
}

fn plus_expr(lexer: &mut Lexer<Token, Token>) -> Option<Expression> {
    let cursor = lexer.get_cursor();

    if let Some(expr1) = digit_expr(lexer) {
        if let Some(Token::Symbol(Symbol::Plus)) = lexer.next_ignored(Token::Whitespace) {
            if let Some(expr2) = expression(lexer) {
                return Some(Expression::Plus(Box::new(expr1), Box::new(expr2)));
            }
        }
    }
    lexer.set_cursor(cursor);
    None
}

fn digit_expr(lexer: &mut Lexer<Token, Token>) -> Option<Expression> {
    let cursor = lexer.get_cursor();

    if let Some(Token::Digit(d)) = lexer.next_ignored(Token::Whitespace) {
        Some(Expression::Digit(d))
    } else {
        lexer.set_cursor(cursor);
        None
    }
}
