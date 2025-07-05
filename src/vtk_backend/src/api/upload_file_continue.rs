use crate::{FileContent, State, UploadFileContinueRequest, UploadFileError};
use candid::Principal;

pub fn upload_file_continue(
    caller: Principal,
    request: UploadFileContinueRequest,
    state: &mut State,
) -> Result<(), UploadFileError> {
    // Check if caller is authenticated (not anonymous)
    if caller == Principal::anonymous() {
        return Err(UploadFileError::NotAuthenticated);
    }

    let file_id = request.file_id;
    let chunk_id = request.chunk_id;

    let updated_file_data = match state.file_data.remove(&file_id) {
        Some(mut file) => {
            // Check if the caller is the owner of this file
            if file.metadata.requester_principal != caller {
                return Err(UploadFileError::NotAuthenticated);
            }

            let updated_contents = match file.content {
                FileContent::PartiallyUploaded { num_chunks, file_type } => {
                    assert!(chunk_id < num_chunks, "invalid chunk id");
                    assert!(
                        !state.file_contents.contains_key(&(file_id, chunk_id)),
                        "chunk already uploaded"
                    );
                    state.file_contents.insert((file_id, chunk_id), request.contents);
                    if state.file_contents
                        .range((file_id, 0)..=(file_id, num_chunks - 1))
                        .count() as u64
                        == num_chunks
                    {
                        FileContent::Uploaded {
                            num_chunks,
                            file_type,
                        }
                    } else {
                        FileContent::PartiallyUploaded {
                            num_chunks,
                            file_type,
                        }
                    }
                }
                f => panic!("expected a partially uploaded file. Found: {f:?}"),
            };
            file.content = updated_contents;
            file
        }
        None => panic!("file doesn't exist"),
    };
    assert_eq!(state.file_data.insert(file_id, updated_file_data), None);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{File, FileContent, FileMetadata, State};
    use crate::api::UploadFileAtomicRequest;

    fn make_atomic_request(name: &str, content: Vec<u8>, file_type: &str, num_chunks: u64) -> UploadFileAtomicRequest {
        UploadFileAtomicRequest {
            name: name.to_string(),
            content,
            file_type: file_type.to_string(),
            num_chunks,
        }
    }

    #[test]
    fn upload_file_continue_transitions() {
        let mut state = State::default();
        let test_principal = Principal::from_text("2vxsx-fae").unwrap();
        // First chunk (atomic)
        let req = make_atomic_request("bigfile.bin", vec![1, 2, 3], "bin", 3);
        let file_id = crate::api::upload_file_atomic(test_principal, req, &mut state).unwrap();
        // Should be PartiallyUploaded
        let file = state.file_data.get(&file_id).unwrap();
        assert!(matches!(&file.content, FileContent::PartiallyUploaded { num_chunks: 3, file_type: ref ft } if ft == "bin"));
        // Second chunk
        upload_file_continue(
            test_principal,
            UploadFileContinueRequest {
                file_id,
                chunk_id: 1,
                contents: vec![4, 5, 6],
            },
            &mut state,
        ).unwrap();
        let file = state.file_data.get(&file_id).unwrap();
        assert!(matches!(&file.content, FileContent::PartiallyUploaded { num_chunks: 3, file_type: ref ft } if ft == "bin"));
        // Third chunk
        upload_file_continue(
            test_principal,
            UploadFileContinueRequest {
                file_id,
                chunk_id: 2,
                contents: vec![7, 8, 9],
            },
            &mut state,
        ).unwrap();
        let file = state.file_data.get(&file_id).unwrap();
        assert!(matches!(&file.content, FileContent::Uploaded { num_chunks: 3, file_type: ref ft } if ft == "bin"));
        // Check chunk data
        assert_eq!(state.file_contents.get(&(file_id, 0)).map(|v| v.clone()), Some(vec![1, 2, 3]));
        assert_eq!(state.file_contents.get(&(file_id, 1)).map(|v| v.clone()), Some(vec![4, 5, 6]));
        assert_eq!(state.file_contents.get(&(file_id, 2)).map(|v| v.clone()), Some(vec![7, 8, 9]));
    }

    #[test]
    fn anonymous_user_cannot_continue_upload() {
        let mut state = State::default();
        let test_principal = Principal::from_text("2vxsx-fae").unwrap();
        // First create a file with authenticated user
        let req = make_atomic_request("bigfile.bin", vec![1, 2, 3], "bin", 3);
        let file_id = crate::api::upload_file_atomic(test_principal, req, &mut state).unwrap();
        
        // Try to continue upload with anonymous user
        let result = upload_file_continue(
            Principal::anonymous(),
            UploadFileContinueRequest {
                file_id,
                chunk_id: 1,
                contents: vec![4, 5, 6],
            },
            &mut state,
        );
        assert!(matches!(result, Err(UploadFileError::NotAuthenticated)));
    }

    #[test]
    fn wrong_user_cannot_continue_upload() {
        let mut state = State::default();
        let test_principal1 = Principal::from_text("2vxsx-fae").unwrap();
        let test_principal2 = Principal::from_text("3vxsx-fae").unwrap(); // Different principal
        // First create a file with authenticated user
        let req = make_atomic_request("bigfile.bin", vec![1, 2, 3], "bin", 3);
        let file_id = crate::api::upload_file_atomic(test_principal1, req, &mut state).unwrap();
        
        // Try to continue upload with different user
        let result = upload_file_continue(
            test_principal2,
            UploadFileContinueRequest {
                file_id,
                chunk_id: 1,
                contents: vec![4, 5, 6],
            },
            &mut state,
        );
        assert!(matches!(result, Err(UploadFileError::NotAuthenticated)));
    }
}