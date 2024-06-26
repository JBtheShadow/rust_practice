use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Macros must be declared or brought into scope before they can be used
// Unlike functions which can just show anywhere

// Declarative macros
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural macros are trickier as they must reside on their own crate with a special crate type
// They can be custom derive, attribute-like and function-like

// A custom derive procedural macro, look at hello_macro for implementation
// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

#[derive(HelloMacro)]
struct Pancakes;

// Attribute-like macros generate code for attributes other than derive
// While derive only works for structs and enums, these macros can be applied to other
// items as well, such as functions

// One example is the route macro, often found in web application frameworks
// #[route(GET, "/")]
// fn index() {

// Their implementation would have a signature like this
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// Function-like macros define macros that look like function calls, similarly to macro_rules! macros
// One example is a macro from a database access framework to run sql queries
// Where macro_rules! uses match syntax, function-like macros manipulate the tokenstream directly
// let sql = sql!(SELECT * FROM posts WHERE id=1);

// A macro like that would have this signature
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {

fn main() {
    let _v = myvec![1, 2, 3];

    Pancakes::hello_macro();
}
