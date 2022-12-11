#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn do_imports(_item: TokenStream) -> TokenStream {
    let mut imports = Vec::new();
    for day in 1..26 {
        imports.push(format!("mod day{day}; use crate::day{day}::*;", day = day));
    }
    imports.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn do_function_call_array(_item: TokenStream) -> TokenStream {
    let mut function_call_array = Vec::new();
    function_call_array.push(
        "const CALL: [(&dyn Fn(&String) -> String, &dyn Fn(&String) -> String); 25] = ["
            .to_string(),
    );
    for day in 1..26 {
        function_call_array.push(format!("(&day{day}_a, &day{day}_b),", day = day));
    }
    function_call_array.push("];".to_string());
    function_call_array.join("\n").parse().unwrap()
}
