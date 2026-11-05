use crate::app::cleaner::domain::cleaner_interfaces::IAppTerminator;
use crate::lib::ffi::appkit;

pub struct NsWorkspaceTerminatorRepository;

impl IAppTerminator for NsWorkspaceTerminatorRepository {
    fn terminate(&self, pid: i32, force: bool) -> bool {
        let requested = appkit::terminate(pid, force);
        if !requested {
            tracing::warn!(pid, force, "terminate request rejected");
        }
        requested
    }
}
