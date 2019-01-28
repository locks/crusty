extern crate azul;
extern crate tempfile;
extern crate unrar;
extern crate walkdir;
extern crate zip;

use azul::prelude::*;

// struct MyDataModel {
//     current_page: ImageId,
// }
struct MyDataModel;

impl Layout for MyDataModel {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
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
                    .with_css_override(
                        "my_image",
                        CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
                            "0".to_string(),
                        ))),
                    ),
            )
            // .with_child(
            //     Dom::new(NodeType::Div)
            //         .with_class("page")
            //         .with_css_override(
            //             "my_image",
            //             CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
            //                 "1".to_string(),
            //             ))),
            //         ),
            // )
        // .with_child(label)
        // .with_child(button)
        // .with_child(
        //     image.with_css_override("page_height", CssProperty::Height(LayoutHeight::px(w))),
        // )
    }
}

fn main() {
    // zip_archive();
    let archive = archive::rar_archive();
    let dir = archive.path();

    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.css")
        };
    }

    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let mut app = App::new(MyDataModel, AppConfig::default());
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Crusty".into();
    let window = Window::new(window_options, css).unwrap();

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
