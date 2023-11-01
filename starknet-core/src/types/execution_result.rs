use alloc::string::String;

use serde::{Deserialize, Serialize};

use super::TransactionExecutionStatus;

/// A more idiomatic way to access `execution_status` and `revert_reason`.
#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Succeeded,
    Reverted { reason: String },
}

impl ExecutionResult {
    pub fn status(&self) -> TransactionExecutionStatus {
        match self {
            ExecutionResult::Succeeded => TransactionExecutionStatus::Succeeded,
            ExecutionResult::Reverted { .. } => TransactionExecutionStatus::Reverted,
        }
    }

    /// Returns `None` if execution status is not `Reverted`.
    ///
    /// A more idiomatic way of accessing the revert reason is to match the `Reverted` enum
    /// variant.
    pub fn revert_reason(&self) -> Option<&str> {
        match self {
            ExecutionResult::Succeeded => None,
            ExecutionResult::Reverted { reason } => Some(reason),
        }
    }
}

impl Serialize for ExecutionResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Raw<'a> {
            execution_status: &'a TransactionExecutionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            revert_reason: &'a Option<&'a str>,
        }

        let raw = Raw {
            execution_status: &self.status(),
            revert_reason: &self.revert_reason(),
        };

        Raw::serialize(&raw, serializer)
    }
}

impl<'de> Deserialize<'de> for ExecutionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            execution_status: TransactionExecutionStatus,
            #[serde(default)]
            revert_reason: String,
        }

        let raw = Raw::deserialize(deserializer)?;

        match raw.execution_status {
            TransactionExecutionStatus::Succeeded => Ok(Self::Succeeded),
            TransactionExecutionStatus::Reverted => Ok(Self::Reverted {
                reason: raw.revert_reason,
            }),
        }
    }
}
