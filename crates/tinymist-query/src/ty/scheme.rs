#![allow(unused)]

use comemo::Track;
use typst::{
    engine::Engine,
    foundations::{func::Repr, Context, Value},
};

use super::Ty;
use crate::{analysis::term_value, ty::TypeInfo};

pub struct TySchemeWorker<'a> {
    engine: Engine<'a>,
    scheme: &'a mut TypeInfo,
}

impl TySchemeWorker<'_> {
    pub fn define(&mut self, v: &Value) -> Ty {
        if let Value::Func(f) = v {
            match f.inner() {
                Repr::Element(..) | Repr::Native(..) | Repr::Plugin(..) | Repr::With(..) => {
                    self.define_value(v)
                }
                Repr::Closure(..) => {
                    let ctx = Context::default();
                    let values = Vec::<Value>::default();
                    let v = f.call(&mut self.engine, ctx.track(), values).unwrap();
                    self.define_value(&v)
                }
            }
        } else {
            self.define_value(v)
        }
    }

    fn define_value(&self, v: &Value) -> Ty {
        term_value(v)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    use tinymist_project::WorldProvider;
    use tinymist_world::ShadowApi;
    use typlite::CompileOnceArgs;
    use typst::{
        engine::{Route, Sink, Traced},
        foundations::Bytes,
        introspection::Introspector,
        World,
    };

    use crate::{tests::*, ty::TypeInfo};

    #[test]
    fn test_builitin() {
        let args = CompileOnceArgs {
            input: Some("/builtin.typ".into()),
            ..CompileOnceArgs::default()
        };
        let mut verse = args.resolve().expect("verse");

        verse
            .map_shadow(
                Path::new("/typings.typ"),
                Bytes::from_string(include_str!("typings.typ")),
            )
            .unwrap();
        verse
            .map_shadow(
                Path::new("/builtin.typ"),
                Bytes::from_string(include_str!("builtin.typ")),
            )
            .unwrap();

        let world = verse.snapshot();
        let source = world.source(world.main()).unwrap();
        let module = typst_shim::eval::eval_compat(&world, &source).unwrap();

        let route = Route::default();
        let mut sink = Sink::default();
        let introspector = Introspector::default();
        let traced = Traced::default();
        let engine = Engine {
            routines: &typst::ROUTINES,
            world: ((&world) as &dyn World).track(),
            introspector: introspector.track(),
            traced: traced.track(),
            sink: sink.track_mut(),
            route,
        };

        let mut scheme = TypeInfo::default();
        let mut w = TySchemeWorker {
            engine,
            scheme: &mut scheme,
        };
        for (k, v) in module.scope().iter() {
            let fid = v.span().id().unwrap();
            if fid != source.id() {
                continue;
            }

            let ty = w.define(v.read());
            w.scheme.exports.insert(k.into(), ty);
        }

        let result = format!("{:#?}", TypeCheckSnapshot(&source, &scheme));

        println!("{result}");
        // assert_snapshot!(result);
    }
}
