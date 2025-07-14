use std::fs;



pub fn list_images()  -> Vec<String> {
    let mut Images: Vec<String> = Vec::new();
    let entries = fs::read_dir("assets/Images").expect("Could not read Images directory");
    for entry_result in entries {

        let entry = entry_result.expect("Could not read directory entry");
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "jpg" || ext == "png" || ext == "jpeg" || ext == "gif" {
                if let Some(stem) = path.file_stem() {
                    if let Some(name) = stem.to_str() {
                        Images.push(name.to_string());
                    }
                }
            }
        }
    }

    Images
}
