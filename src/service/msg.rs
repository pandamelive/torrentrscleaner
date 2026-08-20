use crate::models::ScanResult;

#[derive(Debug)]
pub enum ScanCommand {
    ManualScan { force_refresh: bool },
    ReloadRules,
}

#[derive(Debug)]
pub enum ScanEvent {
    TaskStarted(u64),
    TaskFinished(ScanResult),
    TaskError(u64, String),
}