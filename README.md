# Rusty-Lexer
A lexical analyzer for C/C++ written in Rust using deterministic finite automata and regular expressions. 

To run, use cargo manager Rust and point to your source file in `main.rs`:

```
cargo run
```

## Input
```C
#include <stdio.h>
#include <stdlib.h>
#include <mmap.h>
#include <pthread.h>

int main(int argc, char **argv)
{
  int x = 420;
  int y = 69;
  y += 290;
  int j = 0;
  j += 1;
  bool nine = false;
  int x <= 5;
  x << 1;
  bool that = true;
  char that = 'v';
  char *string = "The meaning of life";
  printf("%s", "Hello World!");
  if (n > 56)
  {
    printf("Bonkers");
  }
  int counter;
  char c = 'c';
  if (c >= 2)
  {
    printf("Ather");
  }
  for (counter = 0; counter <= 20; counter++)
  {
    printf("Hi");
  }
  return;
}
```
## Output
```
[KEYWORD("#include"), LESSTHAN("<"), KEYWORD("stdio.h"), GREATERTHAN(">"), KEYWORD("#include"), LESSTHAN("<"), KEYWORD("stdlib.h"), GREATERTHAN(">"), KEYWORD("#include"), LESSTHAN("<"), KEYWORD("mmap.h"), GREATERTHAN(">"), KEYWORD("#include"), LESSTHAN("<"), KEYWORD("pthread.h"), GREATERTHAN(">"), KEYWORD("int"), IDENTIFIER("main"), KEYWORD("int"), IDENTIFIER("argc"), KEYWORD("char"), STARTSTAR("**"), IDENTIFIER("argv"), STARTBRACES("{"), KEYWORD("int"), IDENTIFIER("x"), ASSIGNMENT("="), NUMBER("420"), SEMICOLON(";"), KEYWORD("int"), IDENTIFIER("y"), ASSIGNMENT("="), NUMBER("69"), SEMICOLON(";"), IDENTIFIER("y"), PLUSEQUALS("+="), NUMBER("290"), SEMICOLON(";"), KEYWORD("int"), IDENTIFIER("j"), ASSIGNMENT("="), NUMBER("0"), SEMICOLON(";"), IDENTIFIER("j"), PLUSEQUALS("+="), NUMBER("1"), SEMICOLON(";"), IDENTIFIER("bool"), IDENTIFIER("nine"), ASSIGNMENT("="), IDENTIFIER("false"), KEYWORD("int"), IDENTIFIER("x"), LESSTHANEQUALS("<="), NUMBER("5"), SEMICOLON(";"), IDENTIFIER("x"), SHIFTLEFT("<<"), NUMBER("1"), SEMICOLON(";"), IDENTIFIER("bool"), IDENTIFIER("that"), ASSIGNMENT("="), IDENTIFIER("true"), KEYWORD("char"), IDENTIFIER("that"), ASSIGNMENT("="), APOSTROPHE("\'"), IDENTIFIER("v"), SEMICOLON(";"), KEYWORD("char"), MULTIPLY("*"), IDENTIFIER("tring"), ASSIGNMENT("="), INVERTEDCOMMAS("\""), STRING("The meaning of life"), INVERTEDCOMMAS("\""), SEMICOLON(";"), KEYWORD("printf"), INVERTEDCOMMAS("\""), STRING("%s"), INVERTEDCOMMAS("\""), COMMA(","), INVERTEDCOMMAS("\""), STRING("Hello World!"), INVERTEDCOMMAS("\""), CLOSEPARENTHESES(")"), SEMICOLON(";"), KEYWORD("if"), OPENPARENTHESES("("), IDENTIFIER("n"), GREATERTHAN(">"), NUMBER("56"), STARTBRACES("{"), KEYWORD("printf"), INVERTEDCOMMAS("\""), STRING("Bonkers"), INVERTEDCOMMAS("\""), CLOSEPARENTHESES(")"), SEMICOLON(";"), ENDBRACES("}"), KEYWORD("int"), IDENTIFIER("counter"), KEYWORD("char"), IDENTIFIER("c"), ASSIGNMENT("="), APOSTROPHE("\'"), IDENTIFIER("c"), SEMICOLON(";"), KEYWORD("if"), OPENPARENTHESES("("), IDENTIFIER("c"), GREATERTHANEQUALS(">="), NUMBER("2"), STARTBRACES("{"), KEYWORD("printf"), INVERTEDCOMMAS("\""), STRING("Ather"), INVERTEDCOMMAS("\""), CLOSEPARENTHESES(")"), SEMICOLON(";"), ENDBRACES("}"), KEYWORD("for"), OPENPARENTHESES("("), IDENTIFIER("counter"), ASSIGNMENT("="), NUMBER("0"), SEMICOLON(";"), IDENTIFIER("counter"), LESSTHANEQUALS("<="), NUMBER("20"), SEMICOLON(";"), IDENTIFIER("counter"), PLUS("+"), STARTBRACES("{"), KEYWORD("printf"), INVERTEDCOMMAS("\""), STRING("Hi"), INVERTEDCOMMAS("\""), CLOSEPARENTHESES(")"), SEMICOLON(";"), ENDBRACES("}"), KEYWORD("return"), ENDBRACES("}")]
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
