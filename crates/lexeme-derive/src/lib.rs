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

    let mut lexeme_name = vec![];
    let mut lexeme_regex = vec![];

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
            lexeme_name.push(variant.ident);
            let mut regex = regex;
            regex.insert_str(0, r"\A");
            lexeme_regex.push(regex);
        }
    }

    let lexeme_count = lexeme_regex.len();
    let lexeme_ix = (0..lexeme_count).collect::<Vec<_>>();

    quote! {
        impl #impl_generics #lexeme::Lexeme for #ty #ty_generics #where_clause {
            fn lex(input: &str) -> Option<(Self, usize)> {
                static REGEX: #lexeme::OnceCell::<(
                    #lexeme::RegexSet,
                    [#lexeme::Regex; #lexeme_count],
                )> = #lexeme::OnceCell::new();

                let (regex_set, regex) = REGEX
                    .get_or_try_init(|| Ok::<_, #lexeme::RegexError>((
                        #lexeme::RegexSet::new(&[#(#lexeme_regex),*])?,
                        [#(#lexeme::Regex::new(#lexeme_regex)?),*],
                    )))
                    .expect("lexeme regex failed to build");

                for ix in regex_set.matches(input) {
                    let re = &regex[ix];
                    let mat = re.find(input).unwrap();
                    return match ix {
                        #(#lexeme_ix => Some((Self::#lexeme_name, mat.end())),)*
                        _ => unreachable!()
                    }
                }

                None
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
