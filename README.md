# CHAR-LEX

[`char_lex`] is a crate for easely creating a [`char`] based lexer from multiple custom enums!

## Example

```rust
use char_lex::prelude::*;

#[token]
#[derive(PartialEq)]
enum Token {
    Whitespace = [' ', '\t', '\r', '\n'],

    Digit(Digit),
}

#[token]
#[derive(PartialEq)]
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

fn main() {
    let mut lexer: Lexer<Token, Token> = Lexer::new("1 \r\n 8 9");

    let mut tokens = Vec::new();
    while let Ok(t) = lexer.poll(Some(Token::Whitespace)) {
        tokens.push(t);
    }
    
    assert_eq!(Err(LexErr::EndOfFile), lexer.poll(None));
    assert_eq!(vec![Token::Digit(Digit::One), Token::Digit(Digit::Eight), Token::Digit(Digit::Nine)], tokens);
}
```

`Tokens` can also be wrapped in anything that implements the [`TokenWrapper<T>`] trait!

## Example

```rust
#[derive(PartialEq)]
struct Wrapper<Token> {
    token: Token,
    character: char,
}

impl TokenWrapper<Token> for Wrapper {
    fn wrap(token: T, context: Context) -> Self {
        Self { token, character: context.character }
    }
}

fn main() {
    let mut lexer: Lexer<Token, Wrapper> = Lexer::new("1");

    let mut tokens = Vec::new();
    while let Ok(t) = lexer.poll(Some(Token::Whitespace)) {
        tokens.push(t);
    }
    
    assert_eq!(Err(LexErr::EndOfFile), lexer.poll(None));
    assert_eq!(vec![Wrapper { token: Token::Digit(Digit::One), character: '1' }], tokens);
}
```

[`char_lex`]: https://doc.rust-lang.org/
[`char`]: https://doc.rust-lang.org/std/primitive.char.html
[`TokenWrapper<T>`]: https://doc.rust-lang.org/