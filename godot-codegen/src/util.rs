/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

pub fn to_module_name(class_name: &str) -> String {
    // Remove underscores and make peekable
    let mut class_chars = class_name.bytes().filter(|&ch| ch != b'_').peekable();

    // 2-lookbehind
    let mut previous: [Option<u8>; 2] = [None, None]; // previous-previous, previous

    // None is not upper or number
    #[inline(always)]
    fn is_upper_or_num<T>(ch: T) -> bool
    where
        T: Into<Option<u8>>,
    {
        let ch = ch.into();
        match ch {
            Some(ch) => ch.is_ascii_digit() || ch.is_ascii_uppercase(),
            None => false,
        }
    }

    // None is lowercase
    #[inline(always)]
    fn is_lower_or<'a, T>(ch: T, default: bool) -> bool
    where
        T: Into<Option<&'a u8>>,
    {
        let ch = ch.into();
        match ch {
            Some(ch) => ch.is_ascii_lowercase(),
            None => default,
        }
    }

    let mut result = Vec::with_capacity(class_name.len());
    while let Some(current) = class_chars.next() {
        let next = class_chars.peek();

        let [two_prev, one_prev] = previous;

        // See tests for cases covered
        let caps_to_lowercase = is_upper_or_num(one_prev)
            && is_upper_or_num(current)
            && is_lower_or(next, false)
            && !is_lower_or(&two_prev, true);

        // Add an underscore for Lowercase followed by Uppercase|Num
        // Node2D => node_2d (numbers are considered uppercase)
        let lower_to_uppercase = is_lower_or(&one_prev, false) && is_upper_or_num(current);

        if caps_to_lowercase || lower_to_uppercase {
            result.push(b'_');
        }
        result.push(current.to_ascii_lowercase());

        // Update the look-behind
        previous = [previous[1], Some(current)];
    }

    let mut result = String::from_utf8(result).unwrap();

    // There are a few cases where the conversions do not work:
    // * VisualShaderNodeVec3Uniform => visual_shader_node_vec_3_uniform
    // * VisualShaderNodeVec3Constant => visual_shader_node_vec_3_constant
    if let Some(range) = result.find("_vec_3").map(|i| i..i + 6) {
        result.replace_range(range, "_vec3_")
    }
    if let Some(range) = result.find("gd_native").map(|i| i..i + 9) {
        result.replace_range(range, "gdnative")
    }
    if let Some(range) = result.find("gd_script").map(|i| i..i + 9) {
        result.replace_range(range, "gdscript")
    }

    // To prevent clobbering `gdnative` during a glob import we rename it to `gdnative_`
    if result == "gdnative" {
        return "gdnative_".into();
    }

    result
}

pub fn ident(s: &str) -> Ident {
    format_ident!("{}", s)
}

#[rustfmt::skip]
pub fn safe_ident(s: &str) -> Ident {
    // See also: https://doc.rust-lang.org/reference/keywords.html
    match s {
        // Lexer
        | "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while"

        // Lexer 2018+
        | "async" | "await" | "dyn"

        // Reserved
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield"

        // Reserved 2018+
        | "try"
           => format_ident!("{}_", s),

         _ => ident(s)
    }
}

pub fn ident_escaped(s: &str) -> Ident {
    // note: could also use Ident::parse(s) from syn, but currently this crate doesn't depend on it

    let transformed = match s {
        "type" => "type_",
        s => s,
    };

    ident(transformed)
}

pub fn c_str(s: &str) -> TokenStream {
    let s = Literal::string(&format!("{}\0", s));
    quote! {
        #s.as_ptr() as *const i8
    }
}

pub fn strlit(s: &str) -> Literal {
    Literal::string(s)
}