extern crate azul;
extern crate glob;
extern crate tempfile;
extern crate unrar;
extern crate walkdir;
extern crate zip;

// use azul::prelude::*;

// struct MyDataModel {}

// impl Layout for MyDataModel {
//     fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
//         Dom::new(NodeType::Div)
//     }
// }

// fn main() {
//     let app = App::new(MyDataModel {}, AppConfig::default());
//     let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
//     app.run(window).unwrap();
// }

fn zip_archive() {
    let fname = std::path::Path::new("Girigiri.cbz");
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        println!("{} - {:?}", file.name(), file.size())
    }
}

fn rar_archive() {
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

    println!(
        "{:?}",
        glob::glob(dir.path().join("**/*").to_str().unwrap())
            .unwrap()
            .count()
    );

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

        if !entry.file_type().is_dir() {
            count = count + 1;
            println!("{}", entry.path().display());
        }
    }

    println!("{}", count);
}

fn main() {
    // zip_archive();
    rar_archive();
}
