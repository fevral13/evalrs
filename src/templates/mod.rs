use crate::templates::index::INDEX_TEMPLATE;

mod index;

pub fn init_templates() -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.add_raw_template("index", INDEX_TEMPLATE).unwrap();
    tera
}
