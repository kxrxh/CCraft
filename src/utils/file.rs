use std::{
    fs::{self, File},
    io::{Error, Write},
    path::Path,
};

pub(crate) fn create_file(folder_path: &str, file_name: &str, content: &str) -> Result<(), Error> {
    let path = Path::new(folder_path).join(file_name);
    let mut file = File::create(&path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

// Function to remove a file
// pub(crate) fn remove_file(folder_path: &str, file_name: &str) -> Result<(), Error> {
//     let path = Path::new(folder_path).join(file_name);
//     fs::remove_file(path)?;
//     Ok(())
// }

pub(crate) fn is_file_exist(folder_path: &str, file_name: &str) -> bool {
    let path = Path::new(folder_path).join(file_name);
    path.exists()
}

pub(crate) fn search_files(extensions: &[&str], path: &str) -> Vec<String> {
    let mut files = vec![];
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.extend(search_files(extensions, &path.to_str().unwrap()));
            continue;
        }
        let ext = path.extension().unwrap().to_str().unwrap();
        if extensions.contains(&ext) {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}
