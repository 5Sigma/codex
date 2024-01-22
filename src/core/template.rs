use std::sync::{Arc, Mutex};

use crate::Result;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

#[derive(Default, Clone)]
struct IdHelper {
    ids: Arc<Mutex<Vec<String>>>,
}

impl HelperDef for IdHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _h: &handlebars::Helper<'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc handlebars::Context,
        _rc: &mut handlebars::RenderContext<'reg, 'rc>,
        out: &mut dyn handlebars::Output,
    ) -> handlebars::HelperResult {
        let mut ids = self.ids.lock().unwrap();
        if ids.is_empty() {
            ids.push(nanoid::nanoid!());
        }
        let _ = out.write(ids.first().unwrap());
        Ok(())
    }
}

fn mul(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let v1 = h.param(0).and_then(|v| v.value().as_i64()).unwrap_or(0);
    let v2 = h.param(1).and_then(|v| v.value().as_i64()).unwrap_or(0);
    let r = v1 * v2;
    let _ = out.write(&r.to_string());
    Ok(())
}

pub fn render_template<T>(data: T, template: &str) -> Result<String>
where
    T: serde::Serialize,
{
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("id", Box::<IdHelper>::default());
    handlebars.register_helper("mul", Box::new(mul));
    handlebars.register_template_string("template", template)?;
    handlebars.unregister_escape_fn();
    let res = handlebars.render("template", &data)?;
    Ok(res)
}
