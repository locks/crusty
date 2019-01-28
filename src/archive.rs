
fn zip_archive() {
    let fname = std::path::Path::new("Girigiri.cbz");
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        println!("{} - {:?}", file.name(), file.size())
    }
}

pub fn rar_archive() -> tempfile::TempDir {
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

    // println!(
    //     "{:?}",
    //     walkdir::WalkDir::new(dir.path()).into_iter().count()
    // );

    let mut count = 0;
    let entries = walkdir::WalkDir::new(dir.path())
        // .into_iter()
        // .take(10)
        ;

    for entry in entries {
        let entry = entry.unwrap();
        // dbg!(&entry.file_name());

        // if count == 1 {
        //     println!("{:?}", entry.path())
        // }

        if !entry.file_type().is_dir() {
            count = count + 1;
            // println!("{}", entry.path().display());
        }
    }

    dir
}

pub fn load_images(dir: &std::path::Path, imgs: &mut Vec<Vec<u8>>) {
    let mut b = true;
    let entries = walkdir::WalkDir::new(dir)
            .sort_by(|a,b| {
            // println!("{:?}-{:?}", a, b);
            a.path().cmp(b.path())
        })
;

    for entry in entries {
        let entry = entry.unwrap();

        if !entry.file_type().is_dir() {
            if b {
                b = false
            }
                println!("{:?}", entry);

            let img = std::fs::read(entry.path()).unwrap();
            imgs.push(img);
        }
    }
}
