use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FileDownload<'a> {
    pub off: u64,
    pub name: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct FileDownloadResponse {
    pub off: u64,
    pub data: Vec<u8>,
    pub len: Option<u64>,
}

impl<'a> super::McuMgrRequest for FileDownload<'a> {
    type Response = FileDownloadResponse;

    const WRITE_OPERATION: bool = false;
    const GROUP_ID: u16 = 8;
    const COMMAND_ID: u8 = 0;
}

#[derive(Debug, Serialize)]
pub struct FileUpload<'a, 'b> {
    pub off: u64,
    pub data: &'a [u8],
    pub name: &'b str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub off: u64,
}

impl<'a, 'b> super::McuMgrRequest for FileUpload<'a, 'b> {
    type Response = FileUploadResponse;

    const WRITE_OPERATION: bool = true;
    const GROUP_ID: u16 = 8;
    const COMMAND_ID: u8 = 0;
}
