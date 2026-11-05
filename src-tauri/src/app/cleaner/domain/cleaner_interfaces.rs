use std::collections::HashSet;

pub trait IWindowRepository: Send + Sync {
    fn pids_with_visible_windows(&self) -> HashSet<i32>;
}

pub trait IAppTerminator: Send + Sync {
    fn terminate(&self, pid: i32, force: bool) -> bool;
}
