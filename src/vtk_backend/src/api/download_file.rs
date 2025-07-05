// pub use crate::ceil_division;
use crate::{FileContent, FileData, FileDownloadResponse, State};
// use ic_cdk::export::candid::Principal;
// use candid::Principal;
// use ic_cdk::println;

pub fn download_file(s: &State, file_id: u64, chunk_id: u64) -> FileDownloadResponse {
    match s.file_data.get(&file_id) {
        None => FileDownloadResponse::NotFoundFile,
        Some(file) => match &file.content {
            FileContent::Uploaded { file_type, num_chunks } => {
                match s.file_contents.get(&(file_id, chunk_id)) {
                    Some(contents) => FileDownloadResponse::FoundFile(FileData {
                        contents: contents.clone(),
                        file_type: file_type.clone(),
                        num_chunks: *num_chunks,
                    }),
                    None => FileDownloadResponse::NotFoundFile,
                }
            }
            _ => FileDownloadResponse::NotUploadedFile,
        },
    }
}



// fn get_file_data(s: &State, file_id: u64, chunk_id: u64) -> FileDownloadResponse {
//     // unwrap is safe because we already know the file exists
//     let this_file = s.file_data.get(&file_id).unwrap();
//     match &this_file.content {
//         FileContent::Pending { .. } | FileContent::PartiallyUploaded { .. } => {
//             FileDownloadResponse::NotUploadedFile
//         }
//         FileContent::Uploaded {
//             file_type,
//             // Remove owner_key as it's no longer needed
//             // Instead, we'll just store the file type and metadata
//             // No need for shared_keys map either
//             // owner_key,
//             // shared_keys: _,
//             num_chunks,
//         } => FileDownloadResponse::FoundFile(FileData {
//             contents: s.file_contents.get(&(file_id, chunk_id)).unwrap(),
//             file_type: file_type.clone(),
//             // No need to store an encrypted key
//             // owner_key: owner_key.clone(),
//             num_chunks: *num_chunks,
//         }),
//     }
// }

// fn get_shared_file_data(
//     s: &State,
//     file_id: u64,
//     chunk_id: u64,
//     _user: Principal,
// ) -> FileDownloadResponse {
//     // unwrap is safe because we already know the file exists
//     let this_file = s.file_data.get(&file_id).unwrap();
//     match &this_file.content {
//         FileContent::Pending { .. } | FileContent::PartiallyUploaded { .. } => {
//             FileDownloadResponse::NotUploadedFile
//         }
//         FileContent::Uploaded {
//             file_type,
//             // owner_key: _,
//             // shared_keys,
//             num_chunks,
//         } => FileDownloadResponse::FoundFile(FileData {
//             contents: s.file_contents.get(&(file_id, chunk_id)).unwrap(),
//             file_type: file_type.clone(),
//             // owner_key: shared_keys.get(&user).unwrap().clone(),
//             num_chunks: *num_chunks,
//         }),
//     }
// }
// pub fn download_file(
//     s: &State,
//     file_id: u64,
//     chunk_id: u64,
//     caller: Principal,
// ) -> FileDownloadResponse {
//     match s.file_owners.get(&caller) {
//         // This is the case where the files is owned by this user.
//         Some(files) => match files.contains(&file_id) {
//             true => get_file_data(s, file_id, chunk_id),
//             false => {
//                 if is_file_shared_with_me(s, file_id, caller) {
//                     get_shared_file_data(s, file_id, chunk_id, caller)
//                 } else {
//                     FileDownloadResponse::PermissionError
//                 }
//             }
//         },
//         // But it could also be the case that the file is shared with this user.
//         None => {
//             if is_file_shared_with_me(s, file_id, caller) {
//                 get_shared_file_data(s, file_id, chunk_id, caller)
//             } else {
//                 FileDownloadResponse::PermissionError
//             }
//         }
//     }
// }

// fn is_file_shared_with_me(s: &State, file_id: u64, caller: Principal) -> bool {
//     match s.file_shares.get(&caller) {
//         None => false,
//         Some(arr) => arr.contains(&file_id),
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::{File, FileContent, FileData, FileMetadata, State};

    #[test]
    fn download_existing_uploaded_file() {
        let mut state = State::default();
        state.file_data.insert(
            0,
            File {
                metadata: FileMetadata {
                    file_name: "test_file.txt".to_string(),
                    requested_at: 12345,
                    uploaded_at: Some(12345),
                },
                content: FileContent::Uploaded {
                    num_chunks: 1,
                    file_type: "txt".to_string(),
                },
            },
        );
        state.file_contents.insert((0, 0), vec![1, 2, 3]);

        let result = download_file(&state, 0, 0);
        assert_eq!(
            result,
            FileDownloadResponse::FoundFile(FileData {
                contents: vec![1, 2, 3],
                file_type: "txt".to_string(),
                num_chunks: 1,
            })
        );
    }

    #[test]
    fn download_nonexistent_file() {
        let state = State::default();
        let result = download_file(&state, 42, 0);
        assert_eq!(result, FileDownloadResponse::NotFoundFile);
    }

    #[test]
    fn download_not_uploaded_file() {
        let mut state = State::default();
        state.file_data.insert(
            0,
            File {
                metadata: FileMetadata {
                    file_name: "test_file.txt".to_string(),
                    requested_at: 12345,
                    uploaded_at: None,
                },
                content: FileContent::Pending { alias: "abc".to_string() },
            },
        );
        let result = download_file(&state, 0, 0);
        assert_eq!(result, FileDownloadResponse::NotUploadedFile);
    }
}
