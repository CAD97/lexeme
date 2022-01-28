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

These benchmarks show lexeme as two-to-three orders of magnitude slower than
logos. However, it should be noted that this doesn't make lexeme slow by any
means (10+ MiB/s is nothing to scoff at!); logos is just ludicrously fast.

```text
Identifiers/Logos       time:   [446.71 ns 447.32 ns 447.98 ns]
                        thrpt:  [1.6195 GiB/s 1.6219 GiB/s 1.6241 GiB/s]
Identifiers/Lexeme      time:   [106.71 us 106.86 us 107.05 us]
                        thrpt:  [6.9400 MiB/s 6.9521 MiB/s 6.9620 MiB/s]

Keywords, Operators, and Punctators/Logos
                        time:   [1.5550 us 1.5592 us 1.5635 us]
                        thrpt:  [1.2694 GiB/s 1.2729 GiB/s 1.2763 GiB/s]
Keywords, Operators, and Punctators/Lexeme
                        time:   [28.311 us 28.325 us 28.338 us]
                        thrpt:  [71.715 MiB/s 71.749 MiB/s 71.784 MiB/s]

Strings/Logos           time:   [453.70 ns 456.27 ns 458.70 ns]
                        thrpt:  [1.7684 GiB/s 1.7779 GiB/s 1.7879 GiB/s]
Strings/Lexeme          time:   [66.062 us 66.107 us 66.152 us]
                        thrpt:  [12.557 MiB/s 12.565 MiB/s 12.574 MiB/s]
```

  [alt]: <https://docs.rs/regex/latest/regex/index.html#composites>
  [lexer]: <https://en.wikipedia.org/wiki/Lexical_analysis>
  [Logos]: <https://lib.rs/crates/logos>
  [regex]: <https://lib.rs/crates/regex>
