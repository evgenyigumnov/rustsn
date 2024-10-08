pub fn explore_files(
    path: &str,
    include_ext: &Vec<String>,
    exclude_path: &Vec<String>,
) -> Vec<String> {
    let mut files = Vec::new();
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            files.append(&mut explore_files(
                path.to_str().unwrap(),
                include_ext,
                exclude_path,
            ));
        } else {
            let path = path.to_str().unwrap();
            if include_ext.iter().any(|ext| {
                let file_ext = format!(".{}", ext);
                path.ends_with(file_ext.as_str())
            }) {
                if exclude_path.iter().any(|exclude| {
                    // TODO: generate full path not only folder name
                    path.contains(exclude)
                }) {
                    continue;
                }
                files.push(path.to_string());
            }
        }
    }
    files
}
