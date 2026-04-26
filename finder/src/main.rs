use img_finder::{
    finder,
    logger::{LogMsg, LoggerHandler},
};

fn logger(msg: &LogMsg) {
    match msg {
        LogMsg::Info(s) => println!("[INFO]: {}", s),
        LogMsg::Hash(s) => println!("[HASHING] \"{}\"", s),
        LogMsg::Decoding(s) => println!("[DECODING] \"{}\"", s),
        LogMsg::Finished(s) => println!("   [FINISHED] \"{}\"", s),
        LogMsg::ImageTotal(n) => println!("[TOTAL] \"{}\"", n),
        LogMsg::Error(err) => println!("[ERR] {}", err),
    }
}

fn test_downloads() {
    let (handle, sender) = LoggerHandler::new();
    let logger_thread = std::thread::spawn(|| handle.blocking_run(logger));

    let dir = "/home/mualice/Downloads/";
    let mut finder = finder::HammingClustererFinder::new(sender.clone());

    finder.scan_directory(dir);
    let best_matches = finder.get_clustered_duplicates(10);
    println!(
        "best matches found in Downloads folder ({:#?})",
        best_matches
    );

    drop(sender);
    drop(finder);
    logger_thread.join().unwrap();
}

fn main() {
    test_downloads();
}
