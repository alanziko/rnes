use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, ItemFn, Result, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

struct OpcodeArgs {
    code: Expr,
    cycles: Expr,
    mode: Expr,
}

// TODO: Rewrite

impl Parse for OpcodeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let code = input.parse()?;
        input.parse::<Token![,]>()?;

        let ident: Ident = input.parse()?;
        if ident != "cycles" {
            return Err(input.error("expected `cycles`"));
        }

        input.parse::<Token![=]>()?;
        let cycles: Expr = input.parse()?;

        input.parse::<Token![,]>()?;

        let ident: Ident = input.parse()?;
        if ident != "mode" {
            return Err(input.error("expected `mode`"));
        }
        input.parse::<Token![=]>()?;
        let mode: Expr = input.parse()?;

        Ok(OpcodeArgs { code, cycles, mode })
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

    let expanded = quote! {
        #item_fn

        inventory::submit! {
            Opcode {
                code: #code,
                cycles: #cycle,
                instruction: #fn_name,
                mode: #mode
            }
        }
    };

    expanded.into()
}
