extern crate azul;
extern crate tempfile;
extern crate unrar;
extern crate walkdir;
extern crate zip;

use azul::prelude::*;

struct MyDataModel {
    // current_page: ImageId,
}

impl Layout for MyDataModel {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        // let w = info.window.state.size.dimensions.width as f32 * 0.75;
        // info.window.state.title = "NO".to_string();
        // println!("{:?} - {:?}", w, info.window.state.size);

        // Dom::new(NodeType::Div)
        // .with_child(
        //     Dom::new(NodeType::Image(info.resources.get_image("0").unwrap())).with_id("my-page"),
        // )

        // let label = azul::widgets::label::Label::new(format!("{}", 3)).dom();
        // let button = azul::widgets::button::Button::with_label("Update counter").dom();
        // let image = Dom::new(NodeType::Image(info.resources.get_image("0").unwrap()))
        //     .with_id("my-page")
        //     .with_css_override("page_height", CssProperty::Height(LayoutHeight::px(w)));
        // .dom()
        // .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_class("comic-book")
            .with_child(
                Dom::new(NodeType::Div)
                    .with_class("page")
                    // .with_id("my-page-1")
                    .with_css_override("my_image_id", CssProperty::Background(azul::css::StyleBackground::Image(CssImageId("0".to_string())))),
            )
            .with_child(
                Dom::new(NodeType::Div)
                    .with_class("page")
                    .with_id("my-page-2")
                    // .with_css_override("my_image_id", "0"),
            )
        // .with_child(label)
        // .with_child(button)
        // .with_child(
        //     image.with_css_override("page_height", CssProperty::Height(LayoutHeight::px(w))),
        // )
    }
}

fn zip_archive() {
    let fname = std::path::Path::new("Girigiri.cbz");
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        println!("{} - {:?}", file.name(), file.size())
    }
}

fn rar_archive() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();

    let archive = unrar::Archive::new("cromartie.cbr".into()).list().unwrap();
    for entry in archive {
        let e = entry.unwrap();

        if e.is_directory() {
            continue;
        }
        // println!("{}", e.filename);
    }

    println!("--- NEW BEGINNINGS ---");

    unrar::Archive::new("cromartie.cbr".into())
        .extract_to(dir.path().to_str().unwrap().to_string())
        .unwrap()
        .process()
        .unwrap();

    // println!("{:?}", dir.path().join("**"));

    // println!(
    //     "{:?}",
    //     glob::glob(dir.path().join("**/*").to_str().unwrap()) // NOT "/**/*"
    //         .unwrap()
    //         .count()
    // );

    // for entry in
    //     glob::glob(dir.path().join("**/*").to_str().unwrap()).expect("Failed to read glob pattern")
    // {
    //     match entry {
    //         Ok(path) => println!("{:?}", path.display()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }

    println!(
        "{:?}",
        walkdir::WalkDir::new(dir.path()).into_iter().count()
    );

    let mut count = 0;

    for entry in walkdir::WalkDir::new(dir.path()) {
        let entry = entry.unwrap();

        if count == 1 {
            println!("{:?}", entry.path())
        }

        if !entry.file_type().is_dir() {
            count = count + 1;
            // println!("{}", entry.path().display());
        }
    }

    dir
}

fn load_images(dir: &std::path::Path, imgs: &mut Vec<Vec<u8>>) {
    let mut b = true;

    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();

        if !entry.file_type().is_dir() {
            if b {
                println!("{:?}", entry);
                b = false
            }

            let img = std::fs::read(entry.path()).unwrap();
            imgs.push(img);
        }
    }
}

fn main() {
    // zip_archive();
    let archive = rar_archive();
    let dir = archive.path();

    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.css")
        };
    }

    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let mut app = App::new(MyDataModel {}, AppConfig::default());
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(WindowCreateOptions::default(), css).unwrap();

    let mut images = vec![];
    let mut count = 0;
    load_images(&dir, &mut images);

    for image in images {
        app.add_image(
            count.to_string(),
            &mut image.as_slice(),
            ImageType::GuessImageFormat,
        )
        .unwrap();
        count = count + 1;
    }

    app.run(window).unwrap();
}
