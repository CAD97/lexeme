<!-- Edit note: this is the crate documentation for the lexeme crate! -->

This crate provides a crate to define a simple [lexer] using [regex]. This is
similar to [Logos]; however, lexeme uses the regex crate directly, rather than
implementing its own parsing routines. This makes lexeme much simpler to
maintain than Logos, but sacrifices performance for the simplicity.

lexeme also provides purely the minimal lexing API to the consumer:
`fn Lexeme::lex(&str) -> Option<(YourTokenKind, /*len:*/ usize)>`.
This is deliberate: lexing is theoretically a pure operation, so the API should
enforce this. How your parser wants to manage lexing and state is up to you;
lexeme merely facilitates the implementation.

## Token Disambiguation

*This is the biggest* (current) *downside to using* lexeme. If multiple tokens
are possible matches, lexeme inherits the [regex `|` behavior][alt], that is,
that the *prior* arm (for lexeme, the lexically earlier variant) is preferred.
**This is different from most tokenizers**, which have a "greedy" behavior,
where the longest token is the one produced.

This means that lexeme is not sufficient for lexing where you want to separate
reserved words (keywords) from unreserved words (identifiers) at this stage.
Where one token is strictly longer than the other (e.g. operators, `==` & `=`),
the longer one can be ordered first to get priority.

## How fast?

Please note; these numbers are for comparison only, and the author is not a
statistician nor a software benchmarking expert.

These benchmarks give lexeme a throughput of about 2%-5% the throughput
of logos for regex-heavy workloads, with up to 64% throughput when literal
optimizations can carry performance. However, it should be noted that this
doesn't make lexeme slow by any means (15+ MiB/s is nothing to scoff at!);
logos is just ludicrously fast.

```text
Identifiers/Logos       time:   [866.76 ns 868.10 ns 869.58 ns]
                        thrpt:  [854.34 MiB/s 855.79 MiB/s 857.12 MiB/s]
Identifiers/Lexeme      time:   [40.459 us 40.494 us 40.534 us]
                        thrpt:  [18.328 MiB/s 18.346 MiB/s 18.362 MiB/s]

Keywords, Operators, and Punctators/Logos
                        time:   [2.5958 us 2.5998 us 2.6041 us]
                        thrpt:  [780.43 MiB/s 781.71 MiB/s 782.93 MiB/s]
Keywords, Operators, and Punctators/Lexeme
                        time:   [4.0701 us 4.0771 us 4.0860 us]
                        thrpt:  [497.38 MiB/s 498.46 MiB/s 499.32 MiB/s]

Strings/Logos           time:   [712.22 ns 713.65 ns 715.32 ns]
                        thrpt:  [1.1340 GiB/s 1.1367 GiB/s 1.1390 GiB/s]
Strings/Lexeme          time:   [16.401 us 16.414 us 16.427 us]
                        thrpt:  [50.567 MiB/s 50.606 MiB/s 50.645 MiB/s]
```

  [alt]: <https://docs.rs/regex/latest/regex/index.html#composites>
  [lexer]: <https://en.wikipedia.org/wiki/Lexical_analysis>
  [Logos]: <https://lib.rs/crates/logos>
  [regex]: <https://lib.rs/crates/regex>
