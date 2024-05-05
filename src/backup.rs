pub mod backup {
    use crate::args::args::Args;
    use std::{fs, path::Path};

    pub fn create_backup(args: &Args) {
        let source_path_instance = validate_path(&args.source_path).unwrap();

        validate_backup_directory(args);

        let target_directory = get_or_create_target_directory(args);
        check_or_create_directory(&target_directory);

        let source_name = source_path_instance.file_name().unwrap();
        let target_name = if source_path_instance.is_dir() {
            format!(
                "{}_{}",
                source_name.to_str().unwrap(),
                &get_current_time("%Y-%m-%d_%H:%M:%S")
            )
        } else {
            format!(
                "{}_{}.{}",
                source_path_instance.file_stem().unwrap().to_str().unwrap(),
                &get_current_time("%Y-%m-%d_%H:%M:%S"),
                source_path_instance.extension().unwrap().to_str().unwrap()
            )
        };

        let target_path = format!("{}/{}", target_directory, target_name);

        initiate_backup(&args.source_path, &target_path);
    }

    fn initiate_backup(source_path: &str, target_path: &str) {
        log::info!("Initiate backup from {} to {}", source_path, target_path);
        std::process::Command::new("cp")
            .arg("-a")
            .arg(source_path)
            .arg(target_path)
            .spawn()
            .expect("Something is wrong");
    }

    fn get_backup_directory() -> String {
        let mut cache_path = dirs::cache_dir().unwrap().to_str().unwrap().to_string();
        cache_path.push_str("/rumember");
        cache_path
    }

    fn validate_backup_directory(_: &Args) {
        let cache_path = get_backup_directory();
        check_or_create_directory(&cache_path);
    }

    fn check_or_create_directory(path: &str) {
        if !Path::new(path).exists() {
            log::info!("No directory found, creating directory: {}", path);
            fs::create_dir(path).unwrap();
        } else {
            log::info!("Path {} exists, not creating new directory", path);
        }
    }

    fn validate_path(path: &str) -> Result<&Path, &str> {
        let path_instance = Path::new(path);
        if path_instance.exists() {
            Ok(path_instance)
        } else {
            log::error!("Path {} does not exists", path);
            Err("Path does not exists")
        }
    }

    fn get_current_time(format: &str) -> String {
        chrono::Local::now().format(format).to_string()
    }

    fn get_or_create_target_directory(args: &Args) -> String {
        let mut backup_directory = get_backup_directory();
        let directory = match &args.target_path {
            Some(path) => path.clone(),
            _ => Path::new(&args.source_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        };

        backup_directory.push_str(format!("/{}", directory.as_str()).as_str());
        backup_directory
    }
}
