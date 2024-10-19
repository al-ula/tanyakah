use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::debug;
use crate::config::CONFIG;


pub fn get_component(name: &str) -> String{
    let components = CONFIG.get().unwrap().components.clone();
    components.join(name).with_extension("hbs").to_str().unwrap().to_string()
}

pub async fn render_layout(name: &str, contents: HashMap<String, String>, data: &Value) -> String {
    let mut reg = handlebars::Handlebars::new();
    debug!("Rendering {}", name);
    for (key, value) in contents {
        reg.register_template_file(&key, &value).unwrap();
    }
    reg.render(name, data).unwrap()
}
