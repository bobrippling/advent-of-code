use std::fs;

type N = isize;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_part1() {
    assert_eq!(part1(&"2 * 3 + (4 * 5)"), 26);
    assert_eq!(part1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(part1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(part1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&"2 * 3 + (4 * 5)"), 46);
    assert_eq!(part2(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(part2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(part2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(part2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
}

fn part1(s: &str) -> N {
    let context = Context { precedence: PrecedenceMode::Same };

    s.lines()
        .map(|line| parse(line, &context).unwrap())
        //.inspect(|tree| dump(&*tree, 0))
        .map(|tree| eval(&*tree))
        .sum()
}

fn part2(s: &str) -> N {
    let context = Context { precedence: PrecedenceMode::TighterAdd };

    s.lines()
        .map(|line| parse(line, &context).unwrap())
        //.inspect(|tree| dump(&*tree, 0))
        .map(|tree| eval(&*tree))
        .sum()
}

enum Tree {
    Mul(BTree, BTree),
    Add(BTree, BTree),
    Num(N),
}

type BTree = Box<Tree>;

fn eval(t: &Tree) -> N {
    match t {
        Tree::Mul(a, b) => eval(a) * eval(b),
        Tree::Add(a, b) => eval(a) + eval(b),
        Tree::Num(n) => *n,
    }
}

#[allow(dead_code)]
fn dump(t: &Tree, indent: usize) {
    fn print_indent(n: usize) {
        for _ in 0..n {
            print!("  ");
        }
    }

    print_indent(indent);

    match t {
        Tree::Mul(a, b) => {
            println!("*");
            dump(a, indent + 1);
            dump(b, indent + 1);
        }
        Tree::Add(a, b) => {
            println!("+");
            dump(a, indent + 1);
            dump(b, indent + 1);
        }
        Tree::Num(n) => {
            println!("{}", n);
        }
    }
}

fn parse<'a>(s: &'a str, context: &Context) -> Result<BTree, &'a str> {
    let mut parser = Parser::new(s);
    let tree = op(&mut parser, context);

    match parser.next() {
        Token::Eof => Ok(tree),
        _ => Err(parser.remaining_str())
    }
}

fn op(p: &mut Parser, context: &Context) -> BTree {
    let tighter_add = match context.precedence {
        PrecedenceMode::Same => false,
        PrecedenceMode::TighterAdd => true,
    };
    let next_parser = if tighter_add { op_add } else { term };
    let mut lhs = next_parser(p, context);

    loop {
        lhs = match p.next() {
            Token::Mul => {
                let rhs = next_parser(p, context);

                BTree::new(Tree::Mul(lhs, rhs))
            }
            Token::Add if !tighter_add => {
                let rhs = next_parser(p, context);

                BTree::new(Tree::Add(lhs, rhs))
            }
            t => {
                p.unget(t);
                break lhs;
            },
        }
    }
}

fn op_add(p: &mut Parser, context: &Context) -> BTree {
    let lhs = term(p, context);

    match p.next() {
        Token::Add => {
            let rhs = op_add(p, context);

            BTree::new(Tree::Add(lhs, rhs))
        }
        t => {
            p.unget(t);
            lhs
        }
    }
}

fn term(p: &mut Parser, context: &Context) -> BTree {
    match p.next() {
        Token::Num(n) => {
            Box::new(Tree::Num(n))
        }
        Token::OpenParen => {
            let sub = op(p, context);
            let chomped = p.chomp(Token::CloseParen);
            assert!(chomped);
            sub
        }
        t => panic!("unexpected token {:?}", t)
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Token {
    Mul,
    Add,
    Num(N),
    OpenParen,
    CloseParen,
    Eof,
}

enum PrecedenceMode {
    Same,
    TighterAdd,
}

struct Context {
    precedence: PrecedenceMode,
}

#[derive(Debug)]
struct Parser<'a> {
    s: &'a str,
    i: usize,
    unget: Option<Token>,
}

impl<'s> Parser<'s> {
    fn new(s: &'s str) -> Self {
        Parser {
            s,
            i: 0,
            unget: None,
        }
    }

    fn next(&mut self) -> Token {
        if let Some(t) = self.unget.take() {
            return t;
        }

        let mut i = self.i;
        let chars = self.s.chars().collect::<Vec<_>>();
        let debug = false;

        if debug {
            println!("parser next token, chars: {:?}", chars.iter().collect::<String>());
        }

        while let Some(ch) = chars.get(i) {
            if debug {
                println!("  ch {}", ch);
            }
            if ch.is_whitespace() {
                i += 1;
                continue;

            }

            if debug {
                println!("  matching {}", ch);
            }
            let token = match ch {
                '*' => Token::Mul,
                '+' => Token::Add,
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                mut c if c.is_ascii_digit() => {
                    let mut n: N = 0;

                    loop {
                        n = n * 10 + (*c as N) - ('0' as N);
                        c = match chars.get(i + 1) {
                            Some(c) if c.is_ascii_digit() => {
                                i += 1;
                                c
                            }
                            _ => break,
                        };
                    }

                    Token::Num(n)
                }
                _ => panic!("lex error")
            };
            self.i = i + 1;
            if debug {
                println!("  found token {:?}, leaving chars {:?}", token, chars.iter().skip(self.i).collect::<String>());
            }
            return token;
        }

        Token::Eof
    }

    fn chomp(&mut self, expected: Token) -> bool {
        self.next() == expected
    }

    fn unget(&mut self, t: Token) {
        assert!(self.unget.is_none());
        self.unget = Some(t);
    }

    fn remaining_str(&self) -> &'s str {
        &self.s[self.i..]
    }
}
