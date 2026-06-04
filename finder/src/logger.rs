use std::sync::mpsc;

#[derive(Debug)]
pub enum LogMsg {
    Info(String),
    Starting,
    Error(String),
    Decoding(String),
    Hash(String),
    Finished(String),
    FileError(String, String),
    ImageTotal(usize),
}

#[derive(Clone)]
pub struct LoggerSender {
    tx: mpsc::Sender<LogMsg>,
}

impl LoggerSender {
    pub fn start(&self) {
        let _ = self.tx.send(LogMsg::Starting);
    }
    pub fn info(&self, details: impl Into<String>) {
        let _ = self.tx.send(LogMsg::Info(details.into()));
    }
    pub fn hash(&self, path: impl Into<String>) {
        let _ = self.tx.send(LogMsg::Hash(path.into()));
    }
    pub fn decoding(&self, path: impl Into<String>) {
        let _ = self.tx.send(LogMsg::Decoding(path.into()));
    }
    pub fn finished(&self, path: impl Into<String>) {
        let _ = self.tx.send(LogMsg::Finished(path.into()));
    }
    pub fn total(&self, total: impl Into<usize>) {
        let _ = self.tx.send(LogMsg::ImageTotal(total.into()));
    }
    pub fn file_fail(&self, path: impl Into<String>, err: String) {
        let _ = self.tx.send(LogMsg::FileError(path.into(), err));
    }

    pub fn warn(&self, msg: impl Into<String>) {
        let _ = self.tx.send(LogMsg::Error(msg.into()));
    }
}

pub struct LoggerHandler {
    rx: mpsc::Receiver<LogMsg>,
}

impl LoggerHandler {
    pub fn new() -> (Self, LoggerSender) {
        let (tx, rx) = mpsc::channel::<LogMsg>();
        (Self { rx }, LoggerSender { tx })
    }
    pub fn blocking_run(self, func: impl Fn(&LogMsg)) {
        for ref msg in self.rx {
            func(msg);
        }
    }
}
