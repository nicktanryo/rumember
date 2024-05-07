pub mod backup {
    use crate::{args::args::Args, notification::notification};
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

        check_backup_limit(&args, &target_directory);
        initiate_backup(&args.source_path, &target_path);

        if args.allow_notification {
            notification::send_notification(
                "Rumember Backup",
                format!("Initiate backup for {}", &args.source_path).as_str(),
            );
        }
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

    fn check_backup_limit(args: &Args, target_directory: &str) {
        let backup_directory = Path::new(target_directory);
        if !backup_directory.exists() {
            return;
        }

        let paths = fs::read_dir(target_directory).unwrap();
        if paths.count() < args.max_count as usize {
            return;
        }

        log::info!(
            "Path contain more backups than limit {}, removing oldest backup",
            args.max_count
        );
        let oldest_file = fs::read_dir(target_directory)
            .unwrap()
            .min_by_key(|path| path.as_ref().unwrap().file_name());
        let oldest_file_path = format!(
            "{}/{}",
            target_directory,
            oldest_file
                .unwrap()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .as_str()
        );

        remove_backup(&oldest_file_path);
    }

    fn remove_backup(path: &str) {
        if !Path::new(path).exists() {
            log::warn!("Path {} is invalid, not removing backup", path);
        }

        log::info!("Removing backup in path {}", path);
        std::process::Command::new("rm")
            .arg("-rf")
            .arg(path)
            .spawn()
            .expect("Something is wrong");
    }
}
