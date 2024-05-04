pub mod claims;
pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;
pub mod state_vanilla;

pub use crate::error::ContractError;

pub use msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, GetAllEpochResponse, GetEpochResponse, ProofMsg,
};
pub use state::{
    Config, Witness, Epoch, 
};
