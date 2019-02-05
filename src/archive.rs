pub fn zip_archive() {
    let fname = std::path::Path::new("Girigiri.cbz");
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        // println!("{} - {:?}", file.name(), file.size())
    }
}

pub fn rar_archive(archive_path: String) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();

    unrar::Archive::new(archive_path)
        .extract_to(dir.path().to_str().unwrap().to_string())
        .unwrap()
        .process()
        .unwrap();

    let mut count = 0;
    let entries = walkdir::WalkDir::new(dir.path());

    for entry in entries {
        let entry = entry.unwrap();

        if !entry.file_type().is_dir() {
            count = count + 1;
        }
    }

    dir
}

pub fn expand(archive_path: String) -> tempfile::TempDir {
    rar_archive(archive_path)
}

pub struct ImagePage {
    pub path: String,
    pub filename: String,
    pub content: Vec<u8>,
}

pub fn images(dir: &std::path::Path, imgs: &mut Vec<ImagePage>) {
    let mut b = true;
    let entries = walkdir::WalkDir::new(dir).sort_by(|a, b| a.path().cmp(b.path()));

    for entry in entries {
        let entry = entry.unwrap();

        if !entry.file_type().is_dir() {
            if b {
                b = false
            }
            // println!("{:?}", entry);

            let img = std::fs::read(entry.path()).unwrap();
            let image_page = ImagePage {
                path: entry.path().display().to_string(),
                filename: entry.file_name().to_str().unwrap().to_string(),
                content: img,
            };
            imgs.push(image_page);
        }
    }
}

pub fn load_images(archive_path: String, imgs: &mut Vec<ImagePage>) {
    let expanded = expand(archive_path);
    let dir = expanded.path();

    images(&dir, imgs)
}
