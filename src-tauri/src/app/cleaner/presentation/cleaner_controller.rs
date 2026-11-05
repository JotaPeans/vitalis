use tauri::State;

use crate::shared::container::Container;
use crate::shared::errors::{AppError, AppResult};

use super::cleaner_dto::{CleanCandidateDto, QuitAppsDto, QuitResultDto};

#[tauri::command]
pub async fn get_clean_candidates(container: State<'_, Container>) -> AppResult<Vec<CleanCandidateDto>> {
    let cleaner = container.cleaner.clone();
    let candidates = tauri::async_runtime::spawn_blocking(move || cleaner.candidates())
        .await
        .map_err(|e| AppError::Platform(e.to_string()))??;
    Ok(candidates.iter().map(CleanCandidateDto::from).collect())
}

#[tauri::command]
pub async fn quit_apps(container: State<'_, Container>, body: QuitAppsDto) -> AppResult<QuitResultDto> {
    run_quit(&container, body, false).await
}

#[tauri::command]
pub async fn force_quit_apps(container: State<'_, Container>, body: QuitAppsDto) -> AppResult<QuitResultDto> {
    run_quit(&container, body, true).await
}

async fn run_quit(container: &Container, body: QuitAppsDto, force: bool) -> AppResult<QuitResultDto> {
    let pids = body.validate()?;
    let cleaner = container.cleaner.clone();
    let result = tauri::async_runtime::spawn_blocking(move || cleaner.quit(&pids, force))
        .await
        .map_err(|e| AppError::Platform(e.to_string()))??;
    Ok(QuitResultDto::from(&result))
}
