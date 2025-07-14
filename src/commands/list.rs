pub fn list_images_fn(){

    let mut Images: Vec<String> = crate::image::list_images();

    Images.sort();

    for name in Images {
        println!("{}", name);
    }

}