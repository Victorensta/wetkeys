// use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::{query, update};
use vtk_backend::*;
use vtk_backend::api::UploadFileAtomicRequest;
use vtk_backend::api::DeleteFileResult;
use candid::Principal;
use vtk_backend::api::{RegisterFileRequest, RegisterFileResponse};

#[update]
fn upload_file_atomic(request: UploadFileAtomicRequest) -> u64 {
    let caller = ic_cdk::caller();
    with_state_mut(|s| vtk_backend::api::upload_file_atomic(caller, request, s))
}

#[update]
fn upload_file_continue(request: UploadFileContinueRequest) -> Result<(), UploadFileError> {
    let caller = ic_cdk::caller();
    with_state_mut(|s| vtk_backend::api::upload_file_continue(caller, request, s))
}

#[update]
fn register_file(request: RegisterFileRequest) -> RegisterFileResponse {
    vtk_backend::api::register_file(request)
}

#[query]
fn download_file(file_id: u64, chunk_id: u64) -> FileDownloadResponse {
    with_state(|s| vtk_backend::api::download_file(s, file_id, chunk_id))
}

#[update]
fn delete_file(file_id: u64) -> DeleteFileResult {
    let caller = ic_cdk::caller();
    with_state_mut(|s| vtk_backend::api::delete_file(s, caller, file_id))
}


#[query]
fn list_files() -> Vec<PublicFileMetadata> {
    let caller = ic_cdk::caller();
    with_state(|s| {
        // If caller is anonymous, return empty list
        if caller == Principal::anonymous() {
            panic!("Not authenticated");
        }

        // Get files owned by the caller
        let empty_vec = Vec::new();
        let owned_files = s.file_owners.get(&caller).unwrap_or(&empty_vec);
        
        owned_files.iter().filter_map(|&file_id| {
            s.file_data.get(&file_id).map(|file| {
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
                    file_id,
                    file_name: file.metadata.file_name.clone(),
                    group_name: "".to_string(),          // Fill this if you use groups
                    group_alias: None,                   // Or Some(...) if available
                    file_status,
                }
            })
        }).collect()
    })
}


#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


fn main() {}