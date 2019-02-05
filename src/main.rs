mod archive;

use azul::prelude::*;

struct MyDataModel {
    current_page: usize,
    page: String,
    second_page: String,
    source: Vec<self::archive::ImagePage>,
    pages: Vec<String>,
    page_layout: PageLayout,
}

impl Default for MyDataModel {
    fn default() -> Self {
        MyDataModel {
            current_page: 0,
            page: "".to_string(),
            second_page: "".to_string(),
            source: vec![],
            pages: vec![],
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

fn update_keyboard(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    let keyboard = app_state.windows[event.window_id]
        .state
        .get_keyboard_state();

    let mut data = app_state.data.lock().ok()?;
    let next = match keyboard.current_char? {
        't' => {
            if let PageLayout::Page = data.page_layout {
                data.current_page = data.current_page - (data.current_page % 2);
            }

            data.page_layout = data.page_layout.toggle();
            Redraw
        }
        'n' | '\u{f702}' => {
            match data.page_layout {
                PageLayout::Page => {
                    if data.current_page < data.pages.len() {
                        data.current_page = data.current_page + 1;
                    }
                }
                PageLayout::Book => {
                    if data.current_page + 1 < data.pages.len() {
                        data.current_page = data.current_page + 2;
                    }
                }
            }

            Redraw
        }
        'p' | '\u{f703}' => {
            match data.page_layout {
                PageLayout::Page => {
                    if data.current_page > 0 {
                        data.current_page = data.current_page - 1;
                    }
                }
                PageLayout::Book => {
                    data.current_page = data.current_page - 2;

                    if data.current_page < usize::min_value() {
                        data.current_page = 0;
                    }
                }
            }

            Redraw
        }
        'q' => {
            if keyboard.super_down {
                println!("Quitting …");
                std::process::exit(1);
            }

            DontRedraw
        }
        _ => DontRedraw,
    };

    let image: &self::archive::ImagePage = data.source.get(data.current_page).unwrap();
    if !app_state.resources.has_image(image.filename.to_string()) {
        app_state
            .resources
            .add_image(
                image.filename.to_string(),
                &mut image.content.as_slice(),
                ImageType::GuessImageFormat,
            )
            .ok();
    };
    data.page = image.filename.to_string();

    if let PageLayout::Book = data.page_layout {
        let image: &self::archive::ImagePage = data.source.get(data.current_page).unwrap();
        if !app_state.resources.has_image(image.filename.to_string()) {
            app_state
                .resources
                .add_image(
                    image.filename.to_string(),
                    &mut image.content.as_slice(),
                    ImageType::GuessImageFormat,
                )
                .ok();
        };
        data.second_page = image.filename.to_string();
    }

    next
}

impl Layout for MyDataModel {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::new(NodeType::Div)
            .with_callback(
                EventFilter::Window(WindowEventFilter::VirtualKeyUp),
                Callback(update_keyboard),
            )
            .with_class("comic-book");

        match self.page_layout {
            PageLayout::Book => {
                dom.add_child(
                    Dom::new(NodeType::Div)
                        .with_class("page")
                        .with_css_override(
                            "my_image",
                            CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
                                self.page.to_string(),
                            ))),
                        ),
                );
            }
            _ => {}
        };

        dom.add_child(
            Dom::new(NodeType::Div)
                .with_class("page")
                .with_css_override(
                    "my_image",
                    CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
                        self.second_page.to_string(),
                    ))),
                ),
        );

        match self.page_layout {
            PageLayout::Page => {
                info.window.state.title = format!("{}", self.pages.get(self.current_page).unwrap())
            }
            PageLayout::Book => {
                info.window.state.title = format!(
                    "{} - {}",
                    self.pages.get(self.current_page + 1).unwrap(),
                    self.pages.get(self.current_page).unwrap()
                )
            }
        }

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

    let mut data = MyDataModel::default();
    let mut images: Vec<self::archive::ImagePage> = vec![];
    println!("Loading images …");
    self::archive::load_images("cromartie.cbr".to_string(), &mut images);
    println!("Loading images ✓");

    for image in &images {
        data.pages.push(image.filename.to_string());
    }
    data.source = images;

    let app = App::new(data, AppConfig::default());
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Crusty".into();
    let window = Window::new(window_options, css).unwrap();

    println!("Running app");
    app.run(window).unwrap();
}
