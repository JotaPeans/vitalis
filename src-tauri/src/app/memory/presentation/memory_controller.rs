use tauri::State;

use crate::shared::container::Container;
use crate::shared::errors::AppResult;

use super::memory_dto::MemoryDto;

#[tauri::command]
pub async fn get_memory(container: State<'_, Container>) -> AppResult<MemoryDto> {
    let memory = container.memory.snapshot()?;
    Ok(MemoryDto::from(&memory))
}
