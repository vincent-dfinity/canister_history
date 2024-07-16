# Canister History

This is a canister which calls the [canister_info](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister-info) management API to get the canister history.

## Running locally

If you want to test this project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

You can run below command to get the canister history for a given canister.

```bash
dfx canister call canister_history canister_history '(record {canister_id=principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; num_requested_changes=opt 3})'
```

Here `bkyz2-fmaaa-aaaaa-qaaaq-cai` is used as an example, you may want to replace with your own canister id.

Please make sure you're checking a canister with the correct network. For example, if you want to check a canister on the IC mainnet, please add `--ic` flag.

## Use as pullable

This canister has been deployed to IC mainnet and also set to be `pullable`. Please check the [dfx.json](/dfx.json#L7) for details.

You can refer to the [dfx deps document](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-deps), along with [dfx deps blog post](https://internetcomputer.org/blog/features/dfx-deps) to learn how to use `dfx deps`.

Please refer to the [example](./example) about how to use this pullable canister. Basically, you need to

1. Add the pullable canister to the [dfx.json](./example/dfx.json#L8).
2. Run `dfx deps pull` to pull the dependent canister, and generate `deps/pulled.json`.
3. Run `dfx deps init` to generate the `deps/init.json`.  
   You have to do this even there's no init argument for this canister.
4. Run `dfx deps deploy` to deploy the pulled canister locally.
