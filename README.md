# Canister history

A canister which calls the [canister_info](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister-info) management API to get the canister history.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

You can run below command to get the canister history for a given canister.

```bash
dfx canister call canister_history_backend canister_history '(record {canister_id=principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; num_requested_changes=opt 3})'
```

Here `bkyz2-fmaaa-aaaaa-qaaaq-cai` is used as an example, you may want to replace with your own canister id.

Please make sure you're checking a canister with the correct network. For example, if you want to check a canister on the IC mainnet, please add `--ic` flag.
