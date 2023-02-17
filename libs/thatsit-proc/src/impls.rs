use proc_macro2::{TokenStream, Span};
use quote::{quote, format_ident};
use syn::{parse::Parse, parse2 as parse, ItemImpl, Generics, Type, braced, Path};

pub fn literate (attrs: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: render source file as comments-first HTML
    // println!("{input:?}");
    input
}

pub fn widget (attrs: TokenStream, input: TokenStream) -> TokenStream {
    input

    //// Path to struct for which we will be implementing Widget
    //let path = parse::<Path>(input);

    //// TODO: Any additional non-Widget generics

    //// TODO: where clause

    //// The braced body of the implementation
    //let brace_token = braced!(content in input);

    //// TODO: Any attributes before the `render` or `handle` clause

    //// TODO: Expect `render` or `handle` clause

    //// TODO: Expect arguments to clause

    //// TODO: Expect body of clause

    //let render_in   = None;
    //let render_out  = None;
    //let render_impl = None;
    //let render = quote! {
        //impl Render<#render_in, #render_out> for #path {
            //fn render (&self, context: #render_in) -> Result<#render_out> {
                //#render_impl
            //}
        //}
    //};

    //let handle_in   = None;
    //let handle_out  = None;
    //let handle_impl = None;
    //let handle = quote! {
        //impl Handle<#handle_in, #handle_out> for #path {
            //fn handle (&mut self, context: #handle_in) -> Result<#handle_out> {
                //#handle_impl
            //}
        //}
    //};

    //quote! {
        //impl Widget for Path {
            //#render
            //#handle
        //}
    //}

}

struct Widget {
    path: Path,
    
}
