use {proc_macro_error::proc_macro_error, quote::quote};

/// Derive macro for the [`Lexeme`](trait.Lexeme.html) trait.
///
/// See the trait documentation for how to use the derive.
#[proc_macro_error]
#[proc_macro_derive(Lexeme, attributes(lexeme))]
pub fn derive_lexeme(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    impl_lexeme_for(input).into()
}

fn impl_lexeme_for(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut crate_attrs = input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("lexeme"))
        .map(|attr| attr.parse_args::<CrateAlias>())
        .flat_map(|attr| {
            attr.map_err(proc_macro_error::Diagnostic::from)
                .map_err(proc_macro_error::Diagnostic::emit)
        });
    let lexeme = crate_attrs
        .next()
        .map(|alias| alias.path)
        .map(|krate| quote!(#krate))
        .unwrap_or_else(|| quote!(::lexeme));

    for attr in crate_attrs {
        proc_macro_error::emit_error!(attr, "duplicate attribute");
    }

    proc_macro_error::set_dummy(quote! {
        impl #impl_generics #lexeme::Lexeme for #ty #ty_generics #where_clause {
            fn lex(_input: &str) -> Option<(Self, usize)> {
                unimplemented!()
            }
        }
    });

    input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("lexeme"))
        .map(|attr| attr.parse_args::<CrateAlias>())
        .flat_map(Result::err)
        .map(proc_macro_error::Diagnostic::from)
        .for_each(proc_macro_error::Diagnostic::emit);

    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        syn::Data::Union(data) => {
            proc_macro_error::abort!(data.union_token, "Lexeme derive only works for enums")
        },
        syn::Data::Struct(data) => {
            proc_macro_error::abort!(data.struct_token, "Lexeme derive only works for enums")
        },
    };

    let mut lexemes = Vec::<(syn::Ident, String)>::with_capacity(variants.len());
    for variant in variants {
        let mut regex_attrs = variant
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("lexeme"))
            .map(|attr| attr.parse_args::<LexemeRegex>())
            .flat_map(|attr| {
                attr.map_err(proc_macro_error::Diagnostic::from)
                    .map_err(proc_macro_error::Diagnostic::emit)
            });

        let (regex, regex_attr) = match regex_attrs.next() {
            Some(attr) => (attr.regex_lit.value(), attr),
            _ => continue, // ignore variants without annotation
        };

        let mut should_skip = false;

        for attr in regex_attrs {
            proc_macro_error::emit_error!(attr, "duplicate attribute");
        }

        match regex_syntax::Parser::new().parse(&regex) {
            Ok(_) => (),
            Err(syntax_error) => {
                proc_macro_error::emit_error!(regex_attr, syntax_error);
                should_skip = true;
            },
        }

        // FUTURE: support data-holding enum variants
        if !matches!(variant.fields, syn::Fields::Unit) {
            proc_macro_error::emit_error!(
                variant.fields,
                "lexeme only supports lexing unit enum kinds"
            );
            should_skip = true;
        }

        if should_skip {
            continue; // skip this lexeme to avoid derived errors, but continue to other lexemes
        } else {
            lexemes.push((variant.ident, regex));
        }
    }

    let mut mega_regex = String::from(r"\A(?:");
    let mut lexeme_ix = Vec::<usize>::with_capacity(lexemes.len());
    let mut lexeme_variant = Vec::<syn::Ident>::with_capacity(lexemes.len());

    for (ix, (name, regex)) in lexemes.into_iter().enumerate() {
        use std::fmt::Write;
        lexeme_ix.push(ix);
        if ix != 0 {
            mega_regex.push('|');
        }
        write!(mega_regex, "(?P<{name}>{regex})").expect("infallible");
        lexeme_variant.push(name);
    }
    mega_regex.push(')');

    quote! {
        impl #impl_generics #lexeme::Lexeme for #ty #ty_generics #where_clause {
            fn lex(input: &str) -> Option<(Self, usize)> {
                static REGEX: #lexeme::OnceCell::<#lexeme::Regex> = #lexeme::OnceCell::new();
                let regex = REGEX
                    .get_or_try_init(|| #lexeme::Regex::new(#mega_regex))
                    .expect("lexeme combined regex failed to build");
                let caps = regex.captures(input)?;
                if cfg!(debug_assertions) {
                    let matched_lexemes = caps
                        .iter()
                        .skip(1) // whole match
                        .enumerate()
                        .flat_map(|(ix, mat)| mat.map(|_| [#(stringify!(#lexeme_variant)),*][ix]))
                        .collect::<Vec<_>>();
                    assert!(
                        matched_lexemes.len() == 1,
                        "lexeme matched more than one capture group ({:?}); this should not be possible",
                        matched_lexemes);
                }
                #(
                    if let Some(cap) = caps.get(#lexeme_ix + 1) {
                        debug_assert_eq!(cap.start(), 0, "lexeme matched not at start");
                        return Some((Self::#lexeme_variant, cap.end()));
                    }
                )*
                unreachable!()
            }
        }
    }
}

struct CrateAlias {
    #[allow(dead_code)]
    kw: syn::Token![crate],
    #[allow(dead_code)]
    eq: syn::Token![=],
    path: syn::Path,
}

impl syn::parse::Parse for CrateAlias {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CrateAlias {
            kw: input.parse()?,
            eq: input.parse()?,
            path: input.parse()?,
        })
    }
}

impl quote::ToTokens for CrateAlias {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.kw.to_tokens(tokens);
        self.eq.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}

struct LexemeRegex {
    regex_lit: syn::LitStr,
}

impl syn::parse::Parse for LexemeRegex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(LexemeRegex {
            regex_lit: input.parse()?,
        })
    }
}

impl quote::ToTokens for LexemeRegex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.regex_lit.to_tokens(tokens);
    }
}
