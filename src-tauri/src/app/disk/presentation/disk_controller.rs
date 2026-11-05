use tauri::State;

use crate::shared::container::Container;
use crate::shared::errors::AppResult;

use super::disk_dto::DiskDto;

#[tauri::command]
pub async fn get_disk(container: State<'_, Container>) -> AppResult<DiskDto> {
    let disk = container.disk.usage()?;
    Ok(DiskDto::from(&disk))
}
