use crate::app::memory::domain::memory_entity::RawMemoryStatsEntity;
use crate::app::memory::domain::memory_interfaces::IMemoryRepository;
use crate::lib::ffi::mach::{self, SwapUsage};
use crate::shared::errors::{AppError, AppResult};

pub struct MachMemoryRepository;

impl IMemoryRepository for MachMemoryRepository {
    fn read_stats(&self) -> AppResult<RawMemoryStatsEntity> {
        let vm = mach::vm_statistics()
            .ok_or_else(|| AppError::Platform("Falha ao ler estatísticas de memória".into()))?;
        let total_bytes = mach::sysctl_value::<u64>("hw.memsize")
            .ok_or_else(|| AppError::Platform("Falha ao ler memória total".into()))?;
        let swap = mach::sysctl_value::<SwapUsage>("vm.swapusage").unwrap_or_default();
        Ok(RawMemoryStatsEntity {
            page_size: mach::page_size(),
            total_bytes,
            internal_pages: vm.internal_page_count.into(),
            external_pages: vm.external_page_count.into(),
            purgeable_pages: vm.purgeable_count.into(),
            wired_pages: vm.wire_count.into(),
            compressor_pages: vm.compressor_page_count.into(),
            swap_used_bytes: swap.used,
            swap_total_bytes: swap.total,
            pressure_level: mach::sysctl_value::<i32>("kern.memorystatus_vm_pressure_level").unwrap_or(1),
        })
    }
}
