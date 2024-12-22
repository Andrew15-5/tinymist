use crate::analysis::func_signature;

use super::*;

pub fn is_plain_value(value: &Value) -> bool {
    matches!(
        value,
        Value::Label(..)
            | Value::None
            | Value::Auto
            | Value::Bool(..)
            | Value::Int(..)
            | Value::Float(..)
            | Value::Decimal(..)
            | Value::Length(..)
            | Value::Angle(..)
            | Value::Ratio(..)
            | Value::Relative(..)
            | Value::Fraction(..)
            | Value::Color(..)
            | Value::Gradient(..)
            | Value::Pattern(..)
            | Value::Symbol(..)
            | Value::Version(..)
            | Value::Str(..)
            | Value::Bytes(..)
            | Value::Datetime(..)
            | Value::Duration(..)
            | Value::Content(..)
            | Value::Styles(..)
    )
}

/// Gets the type of a value.
#[comemo::memoize]
pub fn term_value(value: &Value) -> Ty {
    match value {
        Value::Array(a) => {
            let values = a
                .iter()
                .map(|v| term_value_rec(v, Span::detached()))
                .collect::<Vec<_>>();
            Ty::Tuple(values.into())
        }
        // todo: term arguments
        Value::Args(..) => Ty::Lit(LitTy::Args),
        Value::Plugin(plugin) => {
            // todo: create infer variables for plugin functions
            let values = plugin
                .iter()
                .map(|method| (method.as_str().into(), Ty::Func(SigTy::any())))
                .collect();
            Ty::Dict(RecordTy::new(values))
        }
        Value::Dict(dict) => {
            let values = dict
                .iter()
                .map(|(k, v)| (k.as_str().into(), term_value_rec(v, Span::detached())))
                .collect();
            Ty::Dict(RecordTy::new(values))
        }
        Value::Module(module) => {
            let values = module
                .scope()
                .iter()
                .map(|(k, v, s)| (k.into(), term_value_rec(v, s)))
                .collect();
            Ty::Dict(RecordTy::new(values))
        }
        Value::Type(ty) => Ty::Lit(LitTy::TypeType(*ty)),
        Value::Dyn(dyn_val) => Ty::Lit(LitTy::Type(dyn_val.ty())),
        Value::Func(func) => Ty::Func(func_signature(func.clone()).type_sig()),
        _ if is_plain_value(value) => Ty::Value(InsTy::new(value.clone())),
        _ => Ty::Any,
    }
}

pub fn term_value_rec(value: &Value, s: Span) -> Ty {
    match value {
        Value::Type(ty) => Ty::Lit(LitTy::TypeType(*ty)),
        Value::Dyn(v) => Ty::Lit(LitTy::Type(v.ty())),
        Value::None
        | Value::Auto
        | Value::Array(..)
        | Value::Args(..)
        | Value::Plugin(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Func(..)
        | Value::Label(..)
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Float(..)
        | Value::Decimal(..)
        | Value::Length(..)
        | Value::Angle(..)
        | Value::Ratio(..)
        | Value::Relative(..)
        | Value::Fraction(..)
        | Value::Color(..)
        | Value::Gradient(..)
        | Value::Pattern(..)
        | Value::Symbol(..)
        | Value::Version(..)
        | Value::Str(..)
        | Value::Bytes(..)
        | Value::Datetime(..)
        | Value::Duration(..)
        | Value::Content(..)
        | Value::Styles(..) => {
            if !s.is_detached() {
                Ty::Value(InsTy::new_at(value.clone(), s))
            } else {
                Ty::Value(InsTy::new(value.clone()))
            }
        }
    }
}
