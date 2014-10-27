#![crate_name = "env_reader"]
#![crate_type = "dylib"]
#![feature(phase)]
#![feature(globs)]
#![feature(plugin_registrar)]
#![feature(slicing_syntax)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::codemap::Span;
use syntax::ext::base::*;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use std::os;
use syntax::parse::token;

pub fn expand_env_opt_str(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
                         -> Box<MacResult + 'static> {
    let env_var = match get_single_str_from_tts(cx, sp, tts, "env_opt_str!") {
        Some(var) => var,
        None => return DummyResult::expr(sp),
    };
    match os::getenv(env_var[]) {
        Some(val) => {
            let interned = token::intern_and_get_ident(val[]);
            MacExpr::new(cx.expr_some(sp, cx.expr_str(sp, interned)))
        },
        None => MacExpr::new(cx.expr_none(sp)),
    }
}

pub fn expand_env_str(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
                      -> Box<MacResult + 'static> {
    let env_var = match get_single_str_from_tts(cx, sp, tts, "env_str!") {
        Some(var) => var,
        None => return DummyResult::expr(sp),
    };
    let env_val = match os::getenv(env_var[]) {
        Some(val) => val,
        None => {
            cx.span_err(sp, format!("couldn't find env {}", env_var).as_slice());
            return DummyResult::expr(sp);
        },
    };
    let interned = token::intern_and_get_ident(env_val.as_slice());
    MacExpr::new(cx.expr_str(sp, interned))
}

pub fn expand_env_opt_uint(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
                          -> Box<MacResult + 'static> {
    let env_var = match get_single_str_from_tts(cx, sp, tts, "env_str!") {
        Some(var) => var,
        None => return DummyResult::expr(sp),
    };
    let env_val = match os::getenv(env_var[]) {
        Some(val) => val,
        None => return MacExpr::new(cx.expr_none(sp)),
    };
    match from_str(env_val[].trim()) {
        Some(val) => MacExpr::new(cx.expr_some(sp, cx.expr_uint(sp, val))),
        None => {
            cx.span_err(sp, format!("could not parse uint from {}", env_var)[]);
            return DummyResult::expr(sp);
        },
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("env_str", expand_env_str);
    reg.register_macro("env_opt_str", expand_env_opt_str);
    reg.register_macro("env_opt_uint", expand_env_opt_uint);
}

