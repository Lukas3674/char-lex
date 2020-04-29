use char_lex::prelude::*;

#[token]
#[derive(Debug, PartialEq)]
enum Token {
    Whitespace = [' ', '\t', '\r', '\n'],

    Digit(Digit),
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

#[test]
fn number_test() {
    let mut lexer: Lexer<Token, Token> = Lexer::new(" 150");

    let n = number(&mut lexer);

    assert_eq!(Some(vec![Digit::One, Digit::Five, Digit::Zero]), n);
    assert_eq!(&LexErr::EndOfFile, lexer.get_error().unwrap());
}

fn number(lexer: &mut Lexer<Token, Token>) -> Option<Vec<Digit>> {
    let mut n = Vec::new();
    while let Some(Token::Digit(d)) = lexer.next_ignored(Token::Whitespace) {
        n.push(d);
    }

    if n.is_empty() {
        None
    } else {
        Some(n)
    }
}
