use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::info;

const COMPONENTS_PATH: &str = "components";

pub fn get_component(name: &str) -> String {
    format!("{}/{}.hbs", COMPONENTS_PATH, name)
}

pub async fn render_layout(name: &str, contents: HashMap<String, String>, data: &Value) -> String {
    let mut reg = handlebars::Handlebars::new();
    info!("Rendering {}", name);
    for (key, value) in contents {
        reg.register_template_file(&key, &value).unwrap();
    }
    reg.render(name, data).unwrap()
}
