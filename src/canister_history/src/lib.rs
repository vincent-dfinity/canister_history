use ic_cdk::api::management_canister::main::{
    canister_info,
    CanisterInfoRequest, CanisterInfoResponse,
};

/// Returns canister info with all available canister changes for a canister characterized by a given principal.
/// Traps if the canister_info management call is rejected (in particular, if the principal does not characterize a canister).
#[ic_cdk::update]
async fn canister_history(info_request: CanisterInfoRequest) -> CanisterInfoResponse {
    canister_info(info_request).await.unwrap().0
}

// Enable Candid export
ic_cdk::export_candid!();
