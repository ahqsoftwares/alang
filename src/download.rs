use downloader::Downloader;
use std::fs::{self, create_dir_all};
use std::path::Path;

// Define a custom progress reporter:
#[allow(dead_code)]
struct SimpleReporterPrivate {
    last_update: std::time::Instant,
    max_progress: Option<u64>,
    message: String,
}
struct SimpleReporter {
    private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
    logger: fn(u64, u64) -> (),
}

impl SimpleReporter {
    #[cfg(not(feature = "tui"))]
    fn create(logger: fn(u64, u64) -> ()) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
            logger,
        })
    }
}

impl downloader::progress::Reporter for SimpleReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = SimpleReporterPrivate {
            last_update: std::time::Instant::now(),
            max_progress,
            message: message.to_owned(),
        };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            let max_bytes = match p.max_progress {
                Some(bytes) => format!("{:?}", bytes),
                None => "{unknown}".to_owned(),
            };

            let max_bytes: u64 = max_bytes.parse().unwrap_or(0);
            (self.logger)(current, max_bytes);
        }
    }

    fn set_message(&self, _message: &str) {}

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}

pub fn download(url: String, folder: String, file_name: String, logger: fn(u64, u64) -> ()) -> u8 {
    let file_parsed = format!("{}/{}", &folder, &file_name);
    let file = std::path::Path::new(&file_parsed);

    let datas = create_dir_all(folder.clone());

    fs::remove_file(file.clone()).unwrap_or(());

    match datas {
        Err(_daras) => return 1, //println!("{}", daras.to_string()),
        Ok(()) => {}             //println!("Created Dir for files"),
    };

    let mut downloader = Downloader::builder()
        .download_folder(Path::new(&folder.clone()))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new(&url).file_name(file);

    #[cfg(not(feature = "tui"))]
    let dl = dl.progress(SimpleReporter::create(logger));

    let result = downloader.download(&[dl]).unwrap();

    let mut status = 0;

    for r in result {
        match r {
            Err(_e) => {
                //println!("Error: {}", &e);
                status = 1;
            }
            Ok(_s) => {
                //println!("Success: {}", &s);
                status = 0;
            }
        };
    }
    status
}
