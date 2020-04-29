# CHAR-LEX

`Char-Lex` is a crate for easely creating a `char` based lexer from multiple custom enums!

#### [GitHub](https://github.com/Lukas3674/char-lex)
#### [Crates.io](https://crates.io/crates/char-lex)
#### [Docs.rs](https://docs.rs/char-lex/)

## Example

```rust
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

fn main() {
    let lexer: Lexer<Token, Token> = Lexer::new("189");
    let tokens: Vec<Token> = lexer.collect();

    assert_eq!(
        vec![
            Token::Digit(Digit::One),
            Token::Digit(Digit::Eight),
            Token::Digit(Digit::Nine)
        ],
        tokens
    );
}
```

`Tokens` with the `TokenTrait` can also be wrapped in anything that implements the `TokenWrapper<T>` trait!

## Example

```rust
#[derive(Debug, PartialEq)]
struct Wrapper {
    token: Token,
    character: char,
}

impl TokenWrapper<Token> for Wrapper {
    fn wrap(token: Token, context: Context) -> Self {
        Self {
            token,
            character: context.character,
        }
    }
}

fn main() {
    let lexer: Lexer<Token, Wrapper> = Lexer::new("1");
    let tokens: Vec<Wrapper> = lexer.collect();

    assert_eq!(
        vec![Wrapper {
            token: Token::Digit(Digit::One),
            character: '1'
        }],
        tokens
    );
}
```
