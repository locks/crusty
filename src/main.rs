mod archive;

use azul::prelude::*;

struct MyDataModel {
    current_page: usize,
    pages: Vec<String>,
    page_layout: PageLayout,
}

impl Default for MyDataModel {
    fn default() -> Self {
        MyDataModel {
            current_page: 0,
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
    let current_key: char = app_state.windows[event.window_id]
        .state
        .get_keyboard_state()
        .current_char?;

    match current_key {
        't' => {
            let mut data = app_state.data.lock().ok()?;

            if let PageLayout::Page = data.page_layout {
                data.current_page = data.current_page - (data.current_page % 2);
            }

            data.page_layout = data.page_layout.toggle();
            Redraw
        }
        'n' => {
            let mut data = app_state.data.lock().ok()?;

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
        'p' => {
            let mut data = app_state.data.lock().ok()?;

            match data.page_layout {
                PageLayout::Page => {
                    if data.current_page > 0 {
                        data.current_page = data.current_page - 1;
                    }
                }
                PageLayout::Book => {
                    if data.current_page - 1 > 0 {
                        data.current_page = data.current_page - 2;
                    }
                }
            }

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
                                self.pages
                                    .get(self.current_page + 1)
                                    .unwrap_or(&"0".to_string())
                                    .to_string(),
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
                        self.pages.get(self.current_page).unwrap().to_string(),
                    ))),
                ),
        );

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
    self::archive::load_images("cromartie.cbr".to_string(), &mut images);

    for image in &images {
        data.pages.push(image.filename.to_string());
    }

    let mut app = App::new(data, AppConfig::default());
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Crusty".into();
    let window = Window::new(window_options, css).unwrap();

    let mut count = 0;
    for image in &images {
        // println!("{}", image.filename);
        // app.app_state.data.current_page = Option::Some(image.filename);
        app.add_image(
            image.filename.to_string(),
            &mut image.content.as_slice(),
            ImageType::GuessImageFormat,
        )
        .unwrap();
        count = count + 1;
    }

    app.run(window).unwrap();
}
