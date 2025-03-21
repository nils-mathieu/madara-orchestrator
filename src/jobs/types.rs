use mongodb::bson::serde_helpers::uuid_1_as_binary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JobType {
    /// Submitting DA data to the DA layer
    DataSubmission,
    /// Getting a proof from the proving service
    ProofCreation,
    /// Verifying the proof on the base layer
    ProofVerification,
    /// Updaing the state root on the base layer
    StateUpdation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JobStatus {
    /// An acknowledgement that the job has been received by the
    /// orchestrator and is waiting to be processed
    Created,
    /// Some system has taken a lock over the job for processing and no
    /// other system to process the job
    LockedForProcessing,
    /// The job has been processed and is pending verification
    PendingVerification,
    /// The job has been processed and verified. No other actions needs to be taken
    Completed,
    /// The job was processed but the was unable to be verified under the given time
    VerificationTimeout,
    /// The job failed processing
    VerificationFailed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobItem {
    /// an uuid to identify a job
    #[serde(with = "uuid_1_as_binary")]
    pub id: Uuid,
    /// a meaningful id used to track a job internally, ex: block_no, txn_hash
    pub internal_id: String,
    /// the type of job
    pub job_type: JobType,
    /// the status of the job
    pub status: JobStatus,
    /// external id to track the status of the job. for ex, txn hash for blob inclusion
    /// or job_id from SHARP
    pub external_id: String,
    /// additional field to store values related to the job
    pub metadata: HashMap<String, String>,
    /// helps to keep track of the version of the item for optimistic locking
    pub version: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JobVerificationStatus {
    #[allow(dead_code)]
    Pending,
    #[allow(dead_code)]
    Verified,
    #[allow(dead_code)]
    Rejected,
}
