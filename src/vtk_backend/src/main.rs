// use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::{query, update};
use vtk_backend::*;
use vtk_backend::api::UploadFileAtomicRequest;
use vtk_backend::api::DeleteFileResult;

#[update]
fn upload_file_atomic(request: UploadFileAtomicRequest) -> u64 {
    with_state_mut(|s| vtk_backend::api::upload_file_atomic(request, s))
}

#[update]
fn upload_file_continue(request: UploadFileContinueRequest) {
    with_state_mut(|s| vtk_backend::api::upload_file_continue(request, s))
}

#[query]
fn download_file(file_id: u64, chunk_id: u64) -> FileDownloadResponse {
    with_state(|s| vtk_backend::api::download_file(s, file_id, chunk_id))
}

#[update]
fn delete_file(file_id: u64) -> DeleteFileResult {
    with_state_mut(|s| vtk_backend::api::delete_file(s, file_id))
}


#[query]
fn list_files() -> Vec<PublicFileMetadata> {
    with_state(|s| {
        s.file_data.iter().map(|(file_id, file)| {
            let file_status = match &file.content {
                FileContent::Pending { alias } => FileStatus::Pending {
                    alias: alias.clone(),
                    requested_at: file.metadata.requested_at,
                },
                FileContent::PartiallyUploaded { .. } => FileStatus::PartiallyUploaded,
                FileContent::Uploaded { .. } => FileStatus::Uploaded {
                    uploaded_at: file.metadata.uploaded_at.unwrap_or(file.metadata.requested_at),
                },
            };

            PublicFileMetadata {
                file_id: *file_id,
                file_name: file.metadata.file_name.clone(),
                group_name: "".to_string(),          // Fill this if you use groups
                group_alias: None,                   // Or Some(...) if available
                file_status,
            }
        }).collect()
    })
}


#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


fn main() {}