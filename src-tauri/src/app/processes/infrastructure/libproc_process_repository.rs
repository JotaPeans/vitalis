use crate::app::processes::domain::app_usage_entity::{ProcessDetailEntity, ProcessSampleEntity};
use crate::app::processes::domain::processes_interfaces::IProcessRepository;
use crate::lib::ffi::libproc;

pub struct LibprocProcessRepository;

impl IProcessRepository for LibprocProcessRepository {
    fn list_samples(&self) -> Vec<ProcessSampleEntity> {
        libproc::all_pids()
            .into_iter()
            .filter_map(|pid| {
                let footprint_bytes = libproc::phys_footprint(pid)?;
                Some(ProcessSampleEntity {
                    pid,
                    responsible_pid: libproc::responsible_pid(pid),
                    name: process_name(pid),
                    footprint_bytes,
                })
            })
            .collect()
    }

    fn describe(&self, pid: i32) -> ProcessDetailEntity {
        ProcessDetailEntity {
            path: libproc::path(pid),
            arguments: libproc::arguments(pid),
        }
    }

    fn is_alive(&self, pid: i32) -> bool {
        libproc::is_alive(pid)
    }
}

fn process_name(pid: i32) -> String {
    libproc::name(pid)
        .or_else(|| {
            libproc::path(pid).and_then(|p| p.rsplit('/').next().map(str::to_string))
        })
        .unwrap_or_else(|| format!("pid {pid}"))
}
