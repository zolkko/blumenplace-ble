#![feature(rustc_private, plugin_registrar)]

extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

use syntax::ast;
use syntax::codemap;
use syntax::codemap::Span;
use syntax::ext::base;
use syntax::ext::base::*;
use syntax::feature_gate;
use syntax::parse::token::intern;
use syntax::parse::{token};
use syntax::ptr::P;
use syntax::print;
use rustc_plugin::Registry;

enum State {
    SvcNumber,
    Argument,
    Separator
}

impl State {
    fn next(&self) -> State {
        match *self {
            State::SvcNumber => State::Separator,
            State::Separator => State::Argument,
            State::Argument  => State::Separator
        }
    }
}

fn expand_svc_call(cx: &mut ExtCtxt, sp: Span, args: &[ast::TokenTree])
        -> Box<MacResult + 'static> {

    if !cx.ecfg.enable_asm() {
        feature_gate::emit_feature_err(
            &cx.parse_sess.span_diagnostic, "asm", sp,
            feature_gate::GateIssue::Language,
            feature_gate::EXPLAIN_ASM);
        return DummyResult::expr(sp);
    }

    let expn_id = cx.codemap().record_expansion(codemap::ExpnInfo {
        call_site: sp,
        callee: codemap::NameAndSpan {
            format: codemap::MacroBang(intern("svc_call")),
            span: None,
            allow_internal_unstable: false,
        },
    });

    println!("{}", print::pprust::tts_to_string(args));

    let sep_pos = args.iter().position(|tt| {
        match *tt {
            ast::TokenTree::Token(_, token::Comma) => true,
            _ => false
        }
    }).unwrap_or(args.len());

    println!("Separator position {}", sep_pos);

    let mut state = State::SvcNumber;
    let mut parser = cx.new_parser_from_tts(args);

    match state {
        SvcNumber => {
        }
    }

    let mut asm = token::InternedString::new("nop\n\tnop\n\tnop\n\tnop\n\t");
    let mut outputs = Vec::new();
    let mut inputs = Vec::new();
    let mut clobs = Vec::new();

    let inline_asm = ast::ExprInlineAsm(ast::InlineAsm {
        asm: token::intern_and_get_ident(&asm),
        asm_str_style: ast::StrStyle::CookedStr,
        outputs: outputs,
        inputs: inputs,
        clobbers: clobs,
        volatile: true,
        alignstack: false,
        dialect: ast::AsmDialect::Att,
        expn_id: expn_id,
    });
    let inline_asm_expr = Some(P(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: inline_asm,
        span: sp,
        attrs: None,
    }));
    let unsafe_expr = ast::ExprBlock(P(ast::Block {
        stmts: Vec::new(),
        expr: inline_asm_expr,
        id: ast::DUMMY_NODE_ID,
        rules: ast::UnsafeBlock(ast::UserProvided),
        span: sp
    }));

    MacEager::expr(P(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: unsafe_expr,
        span: sp,
        attrs: None,
    }))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("svc_call", expand_svc_call);
}

