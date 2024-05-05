pub mod args {
    use clap::Parser;

    const DEFAUTL_SCHEDULE: &str = "0 * * * * *";

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[arg(short, long, help = "directory/file to back up")]
        pub source_path: String,

        #[arg(
            short,
            long,
            help = "directory/file name in cache directory (no '/'), default to directory/file name from source"
        )]
        pub target_path: Option<String>,

        #[arg(long, default_value = DEFAUTL_SCHEDULE, help = "Format \"SECOND MINUTE HOUR DAY_OF_MONTH MONTH DAY_OF_WEEK YEAR[OPTIONAL]\"")]
        pub schedule: String,

        #[arg(short, long, help = "cache directory, default to system")]
        pub backup_path: Option<String>,
    }
}
