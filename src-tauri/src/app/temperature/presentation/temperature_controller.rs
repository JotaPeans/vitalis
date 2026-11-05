use tauri::State;

use crate::shared::container::Container;
use crate::shared::errors::AppResult;

use super::temperature_dto::TemperatureGroupDto;

#[tauri::command]
pub async fn get_temperatures(container: State<'_, Container>) -> AppResult<Vec<TemperatureGroupDto>> {
    let groups = container.temperature.list_groups()?;
    Ok(groups.iter().map(TemperatureGroupDto::from).collect())
}
