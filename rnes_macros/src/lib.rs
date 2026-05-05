use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, ItemFn, Result, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

struct OpcodeArgs {
    code: Expr,
    cycles: Expr,
    mode: Expr,
    cycle_penalty: Expr,
}

fn parse_keys(input: ParseStream) -> Result<HashMap<String, Expr>> {
    let mut map = HashMap::new();

    while !input.is_empty() {
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let expr: Expr = input.parse()?;

        map.insert(ident.to_string(), expr);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(map)
}

impl Parse for OpcodeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let code = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut map = parse_keys(input)?;

        let cycles = map
            .remove("cycles")
            .ok_or_else(|| input.error("missing `cycles`"))?;

        let mode = map
            .remove("mode")
            .ok_or_else(|| input.error("missing `mode`"))?;

        let cycle_penalty = map.remove("penalty").unwrap_or(syn::parse_quote!(None));

        Ok(OpcodeArgs {
            code,
            cycles,
            mode,
            cycle_penalty,
        })
    }
}

#[proc_macro_attribute]
pub fn opcode(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as OpcodeArgs);
    let item_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &item_fn.sig.ident;
    let code = args.code;
    let cycle = args.cycles;
    let mode = args.mode;
    let penalty = args.cycle_penalty;

    let expanded = quote! {
        #item_fn

        inventory::submit! {
            Opcode {
                code: #code,
                cycles: #cycle,
                instruction: #fn_name,
                mode: #mode,
                cycle_penalty: #penalty
            }
        }
    };

    expanded.into()
}
