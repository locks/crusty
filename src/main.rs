mod archive;

use azul::prelude::*;

struct MyDataModel {
    // current_page: ImageId,
    current_index: u32,
    page_layout: PageLayout,
}

impl Default for MyDataModel {
    fn default() -> Self {
        MyDataModel {
            current_index: 0,
            page_layout: PageLayout::Book,
        }
    }
}

enum PageLayout {
    Page,
    Book,
}

impl PageLayout {
    fn toggle(&self) -> PageLayout {
        use PageLayout::*;

        match *self {
            Book => Page,
            Page => Book,
        }
    }
}

// fn toggle_page_layout(app_state: &mut AppState<MyDataModel>) {
//     println!("Toggling");
//     let data = app_state.data.lock().ok()?;
//     let mut layout = data.page_layout;

//     match layout {
//         PageLayout::Page => {
//             layout = PageLayout::Book;
//         }
//         PageLayout::Book => {
//             layout = PageLayout::Page;
//         }
//     };
// }

fn update_keyboard(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    let current_key: char = app_state.windows[event.window_id]
        .state
        .get_keyboard_state()
        .current_char?;

    match current_key {
        't' => {
            let mut data = app_state.data.lock().ok()?;
            data.page_layout = data.page_layout.toggle();

            Redraw
        }
        _ => {
            println!("no match: {:?}", current_key);
            DontRedraw
        }
    }
}

impl Layout for MyDataModel {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        // let images = info.resources.images;
        // let images = info.resources.get_image("0")
        let mut dom = Dom::new(NodeType::Div)
            .with_callback(
                EventFilter::Window(WindowEventFilter::VirtualKeyUp),
                Callback(update_keyboard),
            )
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
            );

        match self.page_layout {
            PageLayout::Book => {
                dom.add_child(
                    Dom::new(NodeType::Div)
                        .with_class("page")
                        .with_css_override(
                            "my_image",
                            CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
                                "1".to_string(),
                            ))),
                        ),
                );
            }
            _ => {}
        };

        return dom;
    }
}

fn main() {
    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.css")
        };
    }

    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let mut app = App::new(MyDataModel::default(), AppConfig::default());
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Crusty".into();
    let window = Window::new(window_options, css).unwrap();

    let mut images: Vec<self::archive::ImagePage> = vec![];
    let mut count = 0;
    self::archive::load_images("cromartie.cbr".to_string(), &mut images);

    for image in images {
        // println!("{}", image.filename);
        app.add_image(
            // image.filename,
            count.to_string(),
            &mut image.content.as_slice(),
            ImageType::GuessImageFormat,
        )
        .unwrap();
        count = count + 1;
    }

    app.run(window).unwrap();
}
