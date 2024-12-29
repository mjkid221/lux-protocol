use proc_macro::TokenStream;
use quote::{quote, ToTokens};

use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Fields, Token,
};

struct SessionArgs {
    signer: syn::ExprAssign,
    authority: syn::ExprAssign,
}
impl Parse for SessionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let signer = input.parse()?;

        input.parse::<Token![,]>()?;

        let authority = input.parse()?;
        Ok(SessionArgs { signer, authority })
    }
}

fn is_session(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("session")
}

// #[proc_macro_derive(Session, attributes(session))]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let input_parsed = parse_macro_input!(input as DeriveInput);

//     let fields = match input_parsed.data {
//         Data::Struct(data) => match data.fields {
//             Fields::Named(fields) => fields,
//             _ => panic!("Session trait can only be derived for structs with named fields"),
//         },
//         _ => panic!("Session trait can only be derived for structs"),
//     };

//     // Ensure that the struct has a session_token field
//     let session_token_field = fields
//         .named
//         .iter()
//         .find(|field| field.ident.as_ref().unwrap().to_string() == "session_token")
//         .expect("Session trait can only be derived for structs with a session_token field");
//     {
//         let session_token_type = &session_token_field.ty;
//         let session_token_type_string = quote! { #session_token_type }.to_string();
//         assert!(
//         session_token_type_string == "Option < Account < 'info, SessionToken > >",
//         "Session trait can only be derived for structs with a session_token field of type Option<Account<'info, SessionToken>>"
//         );
//     };

//     // Session Token field must have the #[session] attribute
//     let session_attr = session_token_field
//      .attrs
//      .iter()
//      .find(|attr| is_session(attr))
//      .expect("Session trait can only be derived for structs with a session_token field with the #[session] attribute");

//     let session_args = session_attr.parse_args::<SessionArgs>().unwrap();
//     let session_signer = session_args.signer.right.into_token_stream();

//     // Session Authority
//     let session_authority = session_args.authority.right.into_token_stream();

//     let struct_name = &input_parsed.ident;
//     let (impl_generics, ty_generics, where_clause) = input_parsed.generics.split_for_impl();

//     let output = quote! {

//         #[automatically_derived]
//         impl #impl_generics Session #ty_generics for #struct_name #ty_generics #where_clause {

//             // Target Program
//             fn target_program(&self) -> Pubkey {
//                 crate::id()
//             }

//             // Session Token
//             fn session_token(&self) -> Option<Account<'info, SessionToken>> {
//                 self.session_token.clone()
//             }

//             // Session Authority
//             fn session_authority(&self) -> Pubkey {
//                 self.#session_authority
//             }

//             // Session Signer
//             fn session_signer(&self) -> Signer<'info> {
//                 self.#session_signer.clone()
//             }

//         }
//     };

//     output.into()
// }
#[proc_macro_derive(LightSession, attributes(session))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let name = &input_parsed.ident;

    let fields = match input_parsed.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("Session trait can only be derived for structs with named fields"),
        },
        _ => panic!("Session trait can only be derived for structs"),
    };

    let session_token_field = fields
        .named
        .iter()
        .find(|field| field.ident.as_ref().unwrap().to_string() == "session_token")
        .expect("Session trait can only be derived for structs with a session_token field");
    {
        let session_token_type = &session_token_field.ty;
        let session_token_type_string = quote! { #session_token_type }.to_string();
        assert!(
        session_token_type_string == "LightAccount < SessionToken >",
        "Session trait can only be derived for structs with a session_token field of type Option<Account<'info, SessionToken>>"
        );
    };

    // Session Token field must have the #[session] attribute
    let session_attr = session_token_field
     .attrs
     .iter()
     .find(|attr| is_session(attr))
     .expect("Session trait can only be derived for structs with a session_token field with the #[session] attribute");

    // Generate the implementation
    let expanded = quote! {
        impl<'info> #name<'info> {
            pub fn validate(&self) -> Result<()> {
                // Add any session-specific validation logic here
                Ok(())
            }
        }

        // Implement any additional session-related traits or methods
        // impl<'info> crate::SessionAccount<'info> for #name<'info> {
        //     fn get_session_token(&self) -> &LightAccount<SessionToken> {
        //         &self.session_token
        //     }

        //     fn get_authority(&self) -> &Signer<'info> {
        //         &self.payer
        //     }
        // }
    };

    TokenStream::from(expanded)
}

struct SessionAuthArgs(syn::Expr, syn::Expr);

impl Parse for SessionAuthArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let auth_expr = input.parse()?;
        let error_ty = input.parse()?;
        Ok(Self(auth_expr, error_ty))
    }
}

#[proc_macro_attribute]
pub fn session_auth_or(attr: TokenStream, item: TokenStream) -> TokenStream {
    let SessionAuthArgs(auth_expr, error_ty) = parse_macro_input!(attr);

    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let input_fn_name = input_fn.sig.ident;
    let input_fn_vis = input_fn.vis;
    let input_fn_block = input_fn.block;
    let input_fn_inputs = input_fn.sig.inputs;
    let input_fn_output = input_fn.sig.output;

    let output = quote! {
        // #input_fn_vis fn #input_fn_name(#input_fn_inputs) #input_fn_output {
        //     let session_token = ctx.accounts.session_token;
        //     if let Some(token) = session_token {

        //     }
        // }
    };
    output.into()
}
