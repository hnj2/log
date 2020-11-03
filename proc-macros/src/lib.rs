#![feature(str_split_once)]

use quote::*;
use proc_macro::*;

#[proc_macro]
pub fn parse_env_filters(_: TokenStream) -> TokenStream {

    let filter_raw = std::env::var("RUST_LOG_FILTERS").unwrap_or("".to_string());

    //println!("filter_raw: \"{}\"", filter_raw);

    let filter_parts = filter_raw.split_terminator(";").map(|p|p.trim()).filter(|p|!p.is_empty());

    let (prefix_filters, defaults) : (Vec<&str>, Vec<&str>) = filter_parts.partition(|p| p.contains("="));

    let default_filter = format_ident!("{}", match defaults.len() {
        0 => "Trace",
        1 => defaults[0],
        _ => panic!("Multiple default filters found (found {:?} in RUST_LOG_FILTERS=\"{}\")!",
                defaults, filter_raw),
    });

    //println!("default_filter: \"{}\"", default_filter);

    let mut prefix_filters: Vec<_> = prefix_filters.iter().map(|p| {
        let (prefix, filter) = p.split_once("=").unwrap();
        (prefix, format_ident!("{}", filter))
    }).collect();

    prefix_filters.sort_by_key(|pair| -(pair.0.len() as isize));

    let (prefixes, filters): (Vec<_>, Vec<_>) = prefix_filters.iter().cloned().unzip().into();

    //for (prefix, filter) in &prefix_filters {
    //    println!("\"{}\"-filter: \"{}\"", prefix, filter);
    //}

    let results = quote::quote! {
        #(if (StartsWith::< #prefixes , MODULE_PATH>::VALUE) {
            return #filters;
        })*
        return #default_filter;
    };

    //println!("{}", results);

    TokenStream::from(results)
}
