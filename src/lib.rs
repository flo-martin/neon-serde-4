#![allow(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(clippy)]
#![deny(clippy::pedantic)]
#![allow(stutter)]
#![recursion_limit = "128"]

//!
//! Neon-serde
//! ==========
//!
//! This crate is a utility to easily convert values between
//!
//! A `Handle<JsValue>` from the `neon` crate
//! and any value implementing `serde::{Serialize, Deserialize}`
//!
//! ## Usage
//!
//! #### `neon_serde::from_value`
//! Convert a `Handle<js::JsValue>` to
//! a type implementing `serde::Deserialize`
//!
//! #### `neon_serde::to_value`
//! Convert a value implementing `serde::Serialize` to
//! a `Handle<JsValue>`

pub mod de;
pub mod errors;
pub mod ser;

mod macros;

extern crate neon;
extern crate serde;
extern crate num;

#[cfg(test)]
mod tests {
    use neon::{prelude::{FunctionContext, Handle, Context}, result::JsResult, types::JsValue};

    use crate::de::{from_value, from_value_opt};
    use crate::ser::{to_value};


    #[test]
    fn test_it_compiles() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Handle<'j, JsValue> = cx.argument::<JsValue>(0)?;
                let () = from_value(&mut cx, arg)
                    .or_else(|e| cx.throw_error(e.to_string()))
                    .unwrap();
                ()
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();
            Ok(result)
        }

        let _ = check;
    }

    #[test]
    fn test_it_compiles_2() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Option<Handle<'j, JsValue>> = cx.argument_opt(0);
                let () = from_value_opt(&mut cx, arg)
                    .or_else(|e| cx.throw_error(e.to_string()))
                    .unwrap();
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();
            Ok(result)
        }

        let _ = check;
    }
}