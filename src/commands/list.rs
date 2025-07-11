pub fn list_templates_fn(){

    let mut templates: Vec<String> = crate::template::list_templates();

    templates.sort();

    for name in templates {
        println!("{}", name);
    }

}