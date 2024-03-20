use database::structs::request_status::RequestFail;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct SignAndSendTransactionResolveEvent {
    pub session_id: String,
    pub request_id: String,
    pub network: String,
    pub tx_hash: Option<String>,
    #[ts(optional)]
    pub failure_reason: Option<RequestFail>,
}
