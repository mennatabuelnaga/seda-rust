# Command-Line Help for `seda`

This document contains the help content for the `seda` command-line program.

**Command Overview:**

* [`seda`↴](#seda)
* [`seda document`↴](#seda-document)
* [`seda generate`↴](#seda-generate)
* [`seda run`↴](#seda-run)
* [`seda node`↴](#seda-node)
* [`seda node bridge`↴](#seda-node-bridge)
* [`seda node get`↴](#seda-node-get)
* [`seda node get-nodes`↴](#seda-node-get-nodes)
* [`seda node register`↴](#seda-node-register)
* [`seda node update`↴](#seda-node-update)
* [`seda node update accept-ownership`↴](#seda-node-update-accept-ownership)
* [`seda node update set-pending-owner`↴](#seda-node-update-set-pending-owner)
* [`seda node update set-socket-address`↴](#seda-node-update-set-socket-address)
* [`seda node unregister`↴](#seda-node-unregister)
* [`seda node peers`↴](#seda-node-peers)
* [`seda node peers add`↴](#seda-node-peers-add)
* [`seda node peers list`↴](#seda-node-peers-list)
* [`seda node peers remove`↴](#seda-node-peers-remove)
* [`seda sub-chain`↴](#seda-sub-chain)
* [`seda sub-chain call`↴](#seda-sub-chain-call)
* [`seda sub-chain view`↴](#seda-sub-chain-view)

## `seda`

For interacting with the SEDA protocol.

**Usage:** `seda [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `document` — Debug command for helping to generate our CLI.md file
* `generate` — Generates an auto-completion file content for the specified shell
* `run` — Runs the SEDA node
* `node` — Commands to interact with the SEDA node
* `sub-chain` — Debug commands to help interact with sub-chains

###### **Options:**

* `--log-file-path <LOG_FILE_PATH>` — The path where you want the log file to write to



## `seda document`

Debug command for helping to generate our CLI.md file

**Usage:** `seda document`



## `seda generate`

Generates an auto-completion file content for the specified shell

**Usage:** `seda generate <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell to generate the auto-completion for

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `seda run`

Runs the SEDA node

**Usage:** `seda run [OPTIONS]`

###### **Options:**

* `-d`, `--deposit <DEPOSIT>` — An option to override the node deposit config value
* `-g`, `--gas <GAS>` — An option to override the node gas config value
* `--secret-key <SECRET_KEY>` — An option to override the node secret key config value
* `--signer-account-id <SIGNER_ACCOUNT_ID>` — An option to override the node signer account ID config value
* `--contract-account-id <CONTRACT_ACCOUNT_ID>` — An option to override the node contract account ID config value
* `--public-key <PUBLIC_KEY>` — An option to override the node public key config value
* `--job-manager-interval-ms <JOB_MANAGER_INTERVAL_MS>` — An option to override the node job manager interval(ms) config value
* `--runtime-worker-threads <RUNTIME_WORKER_THREADS>` — An option to override the node runtime worker threads config value
* `--p2p-server-address <P2P_SERVER_ADDRESS>` — An option to override the node p2p server address config value
* `--p2p-known-peers <P2P_KNOWN_PEERS>` — An option to override the node p2p known peers config value
* `--chain-rpc-url <CHAIN_RPC_URL>` — An option to override the Near chain rpc url config value



## `seda node`

Commands to interact with the SEDA node

**Usage:** `seda node [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `bridge` — Run a view method on the specified chain with the args and post it to the main chain
* `get` — Get a node from a given node ID if it exists
* `get-nodes` — Get a list of nodes limited by the given size from an offset
* `register` — Register a node from the given deposit and socket address
* `update` — Update a node by either accepting ownership, setting the pending owner, or changing the socket address
* `unregister` — Unregister a node from the given node ID
* `peers` — Commands for interacting with the p2p peers

###### **Options:**

* `--chain-rpc-url <CHAIN_RPC_URL>` — An option to override the Near chain rpc url config value



## `seda node bridge`

Run a view method on the specified chain with the args and post it to the main chain

**Usage:** `seda node bridge --chain <CHAIN> --sub-chain-contract-id <SUB_CHAIN_CONTRACT_ID> --sub-chain-method-name <SUB_CHAIN_METHOD_NAME> --bridge-deposit <BRIDGE_DEPOSIT> --args <ARGS>`

###### **Options:**

* `-c`, `--chain <CHAIN>`

  Possible values: `another`, `near`

* `--sub-chain-contract-id <SUB_CHAIN_CONTRACT_ID>`
* `--sub-chain-method-name <SUB_CHAIN_METHOD_NAME>`
* `--bridge-deposit <BRIDGE_DEPOSIT>`
* `-a`, `--args <ARGS>`



## `seda node get`

Get a node from a given node ID if it exists

**Usage:** `seda node get [OPTIONS] --node-id <NODE_ID>`

###### **Options:**

* `-n`, `--node-id <NODE_ID>`
* `-c`, `--contract-id <CONTRACT_ID>`



## `seda node get-nodes`

Get a list of nodes limited by the given size from an offset

**Usage:** `seda node get-nodes [OPTIONS]`

###### **Options:**

* `-l`, `--limit <LIMIT>`

  Default value: `10`
* `-o`, `--offset <OFFSET>`

  Default value: `0`
* `-c`, `--contract-id <CONTRACT_ID>`



## `seda node register`

Register a node from the given deposit and socket address

**Usage:** `seda node register [OPTIONS] --register-deposit <REGISTER_DEPOSIT> --socket-address <SOCKET_ADDRESS>`

###### **Options:**

* `-r`, `--register-deposit <REGISTER_DEPOSIT>`
* `-s`, `--socket-address <SOCKET_ADDRESS>`
* `-d`, `--deposit <DEPOSIT>` — An option to override the node deposit config value
* `-g`, `--gas <GAS>` — An option to override the node gas config value
* `--secret-key <SECRET_KEY>` — An option to override the node secret key config value
* `--signer-account-id <SIGNER_ACCOUNT_ID>` — An option to override the node signer account ID config value
* `--contract-account-id <CONTRACT_ACCOUNT_ID>` — An option to override the node contract account ID config value
* `--public-key <PUBLIC_KEY>` — An option to override the node public key config value
* `--job-manager-interval-ms <JOB_MANAGER_INTERVAL_MS>` — An option to override the node job manager interval(ms) config value
* `--runtime-worker-threads <RUNTIME_WORKER_THREADS>` — An option to override the node runtime worker threads config value
* `--p2p-server-address <P2P_SERVER_ADDRESS>` — An option to override the node p2p server address config value
* `--p2p-known-peers <P2P_KNOWN_PEERS>` — An option to override the node p2p known peers config value



## `seda node update`

Update a node by either accepting ownership, setting the pending owner, or changing the socket address

**Usage:** `seda node update [OPTIONS] --node-id <NODE_ID> <COMMAND>`

###### **Subcommands:**

* `accept-ownership` — 
* `set-pending-owner` — 
* `set-socket-address` — 

###### **Options:**

* `-n`, `--node-id <NODE_ID>`
* `-d`, `--deposit <DEPOSIT>` — An option to override the node deposit config value
* `-g`, `--gas <GAS>` — An option to override the node gas config value
* `--secret-key <SECRET_KEY>` — An option to override the node secret key config value
* `--signer-account-id <SIGNER_ACCOUNT_ID>` — An option to override the node signer account ID config value
* `--contract-account-id <CONTRACT_ACCOUNT_ID>` — An option to override the node contract account ID config value
* `--public-key <PUBLIC_KEY>` — An option to override the node public key config value
* `--job-manager-interval-ms <JOB_MANAGER_INTERVAL_MS>` — An option to override the node job manager interval(ms) config value
* `--runtime-worker-threads <RUNTIME_WORKER_THREADS>` — An option to override the node runtime worker threads config value
* `--p2p-server-address <P2P_SERVER_ADDRESS>` — An option to override the node p2p server address config value
* `--p2p-known-peers <P2P_KNOWN_PEERS>` — An option to override the node p2p known peers config value



## `seda node update accept-ownership`

**Usage:** `seda node update accept-ownership`



## `seda node update set-pending-owner`

**Usage:** `seda node update set-pending-owner <OWNER>`

###### **Arguments:**

* `<OWNER>`



## `seda node update set-socket-address`

**Usage:** `seda node update set-socket-address <ADDRESS>`

###### **Arguments:**

* `<ADDRESS>`



## `seda node unregister`

Unregister a node from the given node ID

**Usage:** `seda node unregister [OPTIONS] --node-id <NODE_ID>`

###### **Options:**

* `-n`, `--node-id <NODE_ID>`
* `-d`, `--deposit <DEPOSIT>` — An option to override the node deposit config value
* `-g`, `--gas <GAS>` — An option to override the node gas config value
* `--secret-key <SECRET_KEY>` — An option to override the node secret key config value
* `--signer-account-id <SIGNER_ACCOUNT_ID>` — An option to override the node signer account ID config value
* `--contract-account-id <CONTRACT_ACCOUNT_ID>` — An option to override the node contract account ID config value
* `--public-key <PUBLIC_KEY>` — An option to override the node public key config value
* `--job-manager-interval-ms <JOB_MANAGER_INTERVAL_MS>` — An option to override the node job manager interval(ms) config value
* `--runtime-worker-threads <RUNTIME_WORKER_THREADS>` — An option to override the node runtime worker threads config value
* `--p2p-server-address <P2P_SERVER_ADDRESS>` — An option to override the node p2p server address config value
* `--p2p-known-peers <P2P_KNOWN_PEERS>` — An option to override the node p2p known peers config value



## `seda node peers`

Commands for interacting with the p2p peers

**Usage:** `seda node peers <COMMAND>`

###### **Subcommands:**

* `add` — Adds a peer to a running node
* `list` — Lists all currently connected peers
* `remove` — Removes a connected peer



## `seda node peers add`

Adds a peer to a running node

**Usage:** `seda node peers add <MULTI_ADDR>`

###### **Arguments:**

* `<MULTI_ADDR>` — A libp2p compatible address (ex. /ip4/127.0.0.1/tcp/44635)



## `seda node peers list`

Lists all currently connected peers

**Usage:** `seda node peers list`



## `seda node peers remove`

Removes a connected peer

**Usage:** `seda node peers remove <PEER_ID>`

###### **Arguments:**

* `<PEER_ID>` — A libp2p peer id (ex. 12D3KooWRg13CAzihqGpVfifoeK4nmZ15D3vpZSPfmaDT53CBr9R)



## `seda sub-chain`

Debug commands to help interact with sub-chains

**Usage:** `seda sub-chain [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `call` — Calls the specified method on the specified chain with the given args and contract ID
* `view` — Views the specified method on the specified chain with the given args and contract ID

###### **Options:**

* `--chain-rpc-url <CHAIN_RPC_URL>` — An option to override the Near chain rpc url config value



## `seda sub-chain call`

Calls the specified method on the specified chain with the given args and contract ID

**Usage:** `seda sub-chain call [OPTIONS] <CHAIN> <CONTRACT_ID> <METHOD_NAME> <ARGS> <CALL_DEPOSIT>`

###### **Arguments:**

* `<CHAIN>` — The sub-chain to call

  Possible values: `another`, `near`

* `<CONTRACT_ID>` — The contract ID for the sub-chain
* `<METHOD_NAME>` — The method name to call
* `<ARGS>` — The args to pass to the call method
* `<CALL_DEPOSIT>` — The deposit for the call method

###### **Options:**

* `-d`, `--deposit <DEPOSIT>` — An option to override the node deposit config value
* `-g`, `--gas <GAS>` — An option to override the node gas config value
* `--secret-key <SECRET_KEY>` — An option to override the node secret key config value
* `--signer-account-id <SIGNER_ACCOUNT_ID>` — An option to override the node signer account ID config value
* `--contract-account-id <CONTRACT_ACCOUNT_ID>` — An option to override the node contract account ID config value
* `--public-key <PUBLIC_KEY>` — An option to override the node public key config value
* `--job-manager-interval-ms <JOB_MANAGER_INTERVAL_MS>` — An option to override the node job manager interval(ms) config value
* `--runtime-worker-threads <RUNTIME_WORKER_THREADS>` — An option to override the node runtime worker threads config value
* `--p2p-server-address <P2P_SERVER_ADDRESS>` — An option to override the node p2p server address config value
* `--p2p-known-peers <P2P_KNOWN_PEERS>` — An option to override the node p2p known peers config value



## `seda sub-chain view`

Views the specified method on the specified chain with the given args and contract ID

**Usage:** `seda sub-chain view <CHAIN> <CONTRACT_ID> <METHOD_NAME> <ARGS>`

###### **Arguments:**

* `<CHAIN>` — The sub-chain to call

  Possible values: `another`, `near`

* `<CONTRACT_ID>` — The contract ID for the sub-chain
* `<METHOD_NAME>` — The method name to view
* `<ARGS>` — The args to pass to the view method



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
