use criterion::*;

mod lexeme;
mod logos;

static SOURCE: &str = "
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
";

static IDENTIFIERS: &str = "It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton";

static STRINGS: &str = r#""tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree.""#;

fn lex_all_logos(src: &str) {
    use ::logos::Logos;
    for token in logos::Token::lexer(src) {
        black_box(token);
    }
}

fn lex_all_lexeme(mut src: &str) {
    use ::lexeme::Lexeme;
    while let Some((token, len)) = lexeme::Token::lex(src) {
        black_box(token);
        // NB: this technically includes an unnecessary bounds check. Rather
        // than complicate the API to avoid this, we consider this okay!
        src = &src[len..];
    }
}

fn benchmarks(c: &mut Criterion) {
    let mut g = c.benchmark_group("Identifiers");
    g.throughput(criterion::Throughput::Bytes(IDENTIFIERS.len() as u64));
    g.bench_with_input("Logos", IDENTIFIERS, |b, i| b.iter(|| lex_all_logos(i)));
    g.bench_with_input("Lexeme", IDENTIFIERS, |b, i| b.iter(|| lex_all_lexeme(i)));
    g.finish();

    let mut g = c.benchmark_group("Keywords, Operators, and Punctators");
    g.throughput(criterion::Throughput::Bytes(SOURCE.len() as u64));
    g.bench_with_input("Logos", SOURCE, |b, i| b.iter(|| lex_all_logos(i)));
    g.bench_with_input("Lexeme", SOURCE, |b, i| b.iter(|| lex_all_lexeme(i)));
    g.finish();

    let mut g = c.benchmark_group("Strings");
    g.throughput(criterion::Throughput::Bytes(STRINGS.len() as u64));
    g.bench_with_input("Logos", STRINGS, |b, i| b.iter(|| lex_all_logos(i)));
    g.bench_with_input("Lexeme", STRINGS, |b, i| b.iter(|| lex_all_lexeme(i)));
    g.finish();
}

criterion_group!(shootout, benchmarks);
criterion_main!(shootout);
