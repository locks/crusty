mod archive;

use azul::prelude::*;

struct MyDataModel {
    current_page: usize,
    first_page: String,
    second_page: String,
    source: Vec<self::archive::ImagePage>,
    pages: Vec<String>,
    page_layout: PageLayout,
    show_help: bool,
}

impl Default for MyDataModel {
    fn default() -> Self {
        MyDataModel {
            current_page: 0,
            first_page: "".to_string(),
            second_page: "".to_string(),
            source: vec![],
            pages: vec![],
            page_layout: PageLayout::Book,
            show_help: false,
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
                    if data.current_page != usize::min_value() {
                        data.current_page = data.current_page - 2;
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
        'h' => {
            data.show_help = !data.show_help;

            Redraw
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
    data.first_page = image.filename.to_string();

    if let PageLayout::Book = data.page_layout {
        let second_image: &self::archive::ImagePage =
            data.source.get(data.current_page + 1).unwrap();
        if !app_state
            .resources
            .has_image(second_image.filename.to_string())
        {
            app_state
                .resources
                .add_image(
                    second_image.filename.to_string(),
                    &mut second_image.content.as_slice(),
                    ImageType::GuessImageFormat,
                )
                .ok();
        };
        data.second_page = second_image.filename.to_string();
    }

    next
}

impl Layout for MyDataModel {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::new(NodeType::Div)
            .with_class("comic-book")
            .with_callback(
                EventFilter::Window(WindowEventFilter::VirtualKeyUp),
                Callback(update_keyboard),
            )
            .with_child(
                Dom::new(NodeType::Div)
                    .with_class("page")
                    .with_css_override(
                        "my_image",
                        CssProperty::Background(azul::css::StyleBackground::Image(CssImageId(
                            self.first_page.to_string(),
                        ))),
                    ),
            );

        if self.show_help {
            dom.add_child(
                Dom::div()
                    .with_class("help")
                    .with_child(Dom::label("Shortcuts:\n- n, left arrow: next page(s)\n- p, right arrow: previous page(s)\n- h: help screen"))
            )
        }

        if let PageLayout::Book = self.page_layout {
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
        };

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
    let args: Vec<String> = std::env::args().collect();

    let mut archive_name: String;
    if args.len() > 1 {
        match args[1].parse() {
            Ok(file) => archive_name = file,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0);
            }
        }
    } else {
        eprintln!("No argument passed.");
        std::process::exit(0);
    }

    let mut data = MyDataModel::default();
    let mut images: Vec<self::archive::ImagePage> = vec![];
    println!("Archive: {}", archive_name);
    println!("Loading images …");
    self::archive::load_images(archive_name, &mut images);
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
