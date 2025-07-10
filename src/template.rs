use std::fs;



pub fn list_templates()  -> Vec<String> {
    let mut templates: Vec<String> = Vec::new();
    let entries = fs::read_dir("assets/templates").expect("Could not read templates directory");
    for entry_result in entries {

        let entry = entry_result.expect("Could not read directory entry");
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "jpg" || ext == "png" || ext == "jpeg" || ext == "gif" {
                if let Some(stem) = path.file_stem() {
                    if let Some(name) = stem.to_str() {
                        templates.push(name.to_string());
                    }
                }
            }
        }
    }

    templates
}
