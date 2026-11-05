use std::collections::HashSet;

use crate::app::cleaner::domain::cleaner_interfaces::IWindowRepository;
use crate::lib::ffi::cgwindow;

pub struct CgWindowRepository;

impl IWindowRepository for CgWindowRepository {
    fn pids_with_visible_windows(&self) -> HashSet<i32> {
        cgwindow::pids_with_windows()
    }
}
