pub fn rename_sequence() -> Option<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let folder = std::path::Path::new(&args[0]);
    let new_name = args[1].clone();
    let mut files = std::fs::read_dir(folder)
        .ok()?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect::<Vec<_>>();
    files.sort_by_key(|file| file.path());
    for (index, file) in files.iter().enumerate() {
        let old_path = file.path();
        let old_name = old_path.file_name()?.to_str()?.split('.').next()?;
        let new_name = format!("{}_{:03}.png", new_name, index);
        let new_path = std::path::Path::new(folder).join(new_name);
        std::fs::rename(&old_path, &new_path).ok()?;
    }
    Some(())
}
