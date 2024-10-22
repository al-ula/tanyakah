pub(crate) mod sections;

use crate::config::CONFIG;
use serde_json::Value;
use std::collections::HashMap;

pub fn get_component(name: &str) -> String {
    let components = CONFIG.get().unwrap().components.clone();
    components
        .join(name)
        .with_extension("hbs")
        .to_str()
        .unwrap()
        .to_string()
}

pub async fn render_layout(
    name: &str,
    content_file: HashMap<String, String>,
    content_string: HashMap<String, String>,
    data: &Value,
) -> String {
    let mut reg = handlebars::Handlebars::new();
    for (key, value) in content_file {
        reg.register_template_file(&key, &value).unwrap();
    }
    for (key, value) in content_string {
        reg.register_template_string(&key, &value).unwrap();
    }
    reg.render(name, data).unwrap()
}
