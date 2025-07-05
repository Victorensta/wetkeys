pub mod api;
mod memory;

use candid::CandidType;
// use candid::Principal;
use ic_stable_structures::{
    // memory_manager::MemoryId,
    // storable::Storable, // Import Bound from storable submodule
    StableBTreeMap,
};

use memory::Memory; // Only Memory is needed for file storage
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
// use std::ops::Bound::{Included, Excluded};

thread_local! {
    /// Initialize the state randomness with the current time.
    static STATE: RefCell<State> = RefCell::new(State::new(&get_randomness_seed()[..]));
}

type FileId = u64;
type ChunkId = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub file_id: u64,
    pub file_name: String,
    pub alias: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileMetadata {
    pub file_name: String,
    // pub user_public_key: Vec<u8>,
    // pub requester_principal: Principal,
    pub requested_at: u64,
    pub uploaded_at: Option<u64>,
    pub storage_provider: String, // "icp" or "walrus"
    pub blob_id: Option<String>,  // Only for Walrus files
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FileStatus {
    #[serde(rename = "pending")]
    Pending { alias: String, requested_at: u64 },
    #[serde(rename = "partially_uploaded")]
    PartiallyUploaded,
    #[serde(rename = "uploaded")]
    Uploaded {
        uploaded_at: u64,
        // No document_key needed here as we moved to vertkeys
        // document_key: Vec<u8>,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PublicFileMetadata {
    pub file_id: u64,
    pub file_name: String,
    pub group_name: String,
    pub group_alias: Option<String>,
    pub file_status: FileStatus,
    // pub shared_with: Vec<PublicUser>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    pub metadata: FileMetadata,
    pub content: FileContent,
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileContent {
    Pending {
        alias: String,
    },
    Uploaded {
        num_chunks: u64,
        file_type: String,
        // owner_key: Vec<u8>, // VetKD public key
        // No need for shared_keys map as we are moving to vetkeys
        // shared_keys: BTreeMap<Principal, Vec<u8>>,
    },
    PartiallyUploaded {
        num_chunks: u64,
        file_type: String,
        // owner_key: Vec<u8>, // VetKD public key
        // No need for shared_keys map as we are moving to vetkeys
        // shared_keys: BTreeMap<Principal, Vec<u8>>,
    },
}


#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq)]
pub struct FileData {
    pub contents: Vec<u8>,
    pub file_type: String,
    pub num_chunks: u64,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Debug)]
pub enum FileDownloadResponse {
    #[serde(rename = "not_found_file")]
    NotFoundFile,
    #[serde(rename = "not_uploaded_file")]
    NotUploadedFile,
    #[serde(rename = "permission_error")]
    PermissionError,
    #[serde(rename = "found_file")]
    FoundFile(FileData),
}

#[derive(Debug, CandidType, Serialize, Deserialize)]
pub enum UploadFileError {
    #[serde(rename = "not_requested")]
    NotRequested,
    #[serde(rename = "already_uploaded")]
    AlreadyUploaded,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq)]
pub enum FileSharingResponse {
    #[serde(rename = "pending_error")]
    PendingError,
    #[serde(rename = "permission_error")]
    PermissionError,
    #[serde(rename = "ok")]
    Ok,
}


#[derive(Serialize, Deserialize)]
pub struct State {
    // Keeps track of how many files have been requested so far
    // and is used to assign IDs to newly requested files.
    file_count: u64,

    // Keeps track of usernames vs. their principals.
    // pub users: BTreeMap<Principal, User>,

    /// Mapping between file IDs and file information.
    pub file_data: BTreeMap<u64, File>,


    // Mapping between a user's principal and the list of files that are owned by the user.
    // pub file_owners: BTreeMap<Principal, Vec<u64>>,

    // Mapping between a user's principal and the list of files that are shared with them.
    // pub file_shares: BTreeMap<Principal, Vec<u64>>,

    /// The contents of the file (stored in stable memory).
    #[serde(skip, default = "init_file_contents")]
    pub file_contents: StableBTreeMap<(FileId, ChunkId), Vec<u8>, Memory>,

}

impl State {
    pub(crate) fn generate_file_id(&mut self) -> u64 {

        let file_id = self.file_count;
        self.file_count += 1;
        file_id
    }

    fn new(_rand_seed: &[u8]) -> Self {
        Self {
            file_count: 0,
            // users: BTreeMap::new(),
            file_data: BTreeMap::new(),
            file_contents: init_file_contents(),
        }
    }

    // pub(crate) fn num_chunks_uploaded(&self, file_id: u64) -> u64 {
    //     self.file_contents
    //         .range((Included((file_id, 0u64)), Excluded(((file_id + 1), 0u64))))
    //         .count() as u64
    // }
}

// This is a standard Rust pattern for initializing the state.
impl Default for State {
    fn default() -> Self {
        State::new(vec![0; 32].as_slice())
    }
}

/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UploadFileRequest {
    pub file_id: u64,
    pub file_content: Vec<u8>,
    pub file_type: String,
    // Not needed for VetKD
    // pub owner_key: Vec<u8>,
    pub num_chunks: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UploadFileContinueRequest {
    pub file_id: u64,
    pub chunk_id: u64,
    pub contents: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
pub fn get_time() -> u64 {
    ic_cdk::api::time()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> u64 {
    // This is used only in tests and we need a fixed value we can test against.
    12345
}

fn get_randomness_seed() -> Vec<u8> {
    // this is an array of u8 of length 8.
    let time_seed = ic_cdk::api::time().to_be_bytes();
    // we need to extend this to an array of size 32 by adding to it an array of size 24 full of 0s.
    let zeroes_arr: [u8; 24] = [0; 24];
    [&time_seed[..], &zeroes_arr[..]].concat()
}


pub fn ceil_division(dividend: usize, divisor: usize) -> usize {
    if dividend % divisor == 0 {
        dividend / divisor
    } else {
        dividend / divisor + 1
    }
}

fn init_file_contents() -> StableBTreeMap<(FileId, ChunkId), Vec<u8>, Memory> {
    StableBTreeMap::init(crate::memory::get_file_contents_memory())
}