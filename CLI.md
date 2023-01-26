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

* `bridge` — 
* `get` — Get a node from a given node ID if it exists
* `get-nodes` — Get a list of nodes limited by the given size from an offset
* `register` — Register a node from the given deposit and socket address
* `update` — Update a node by either accepting ownership, setting the pending owner, or changing the socket address
* `unregister` — Unregister a node from the given node ID

###### **Options:**

* `--chain-rpc-url <CHAIN_RPC_URL>` — An option to override the Near chain rpc url config value



## `seda node bridge`

**Usage:** `seda node bridge --chain <CHAIN> --sub-chain-contract-id <SUB_CHAIN_CONTRACT_ID> --sub-chain-method-name <SUB_CHAIN_METHOD_NAME> --args <ARGS>`

###### **Options:**

* `-c`, `--chain <CHAIN>`

  Possible values: `another`, `near`

* `--sub-chain-contract-id <SUB_CHAIN_CONTRACT_ID>`
* `--sub-chain-method-name <SUB_CHAIN_METHOD_NAME>`
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
 d e   t h e   n o d e   d e p o s i t   c o n f i g   v a l u e 
 *   ` - g ` ,   ` - - g a s   < G A S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   g a s   c o n f i g   v a l u e 
 *   ` - - s e c r e t - k e y   < S E C R E T _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s e c r e t   k e y   c o n f i g   v a l u e 
 *   ` - - s i g n e r - a c c o u n t - i d   < S I G N E R _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s i g n e r   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - c o n t r a c t - a c c o u n t - i d   < C O N T R A C T _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   c o n t r a c t   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - p u b l i c - k e y   < P U B L I C _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p u b l i c   k e y   c o n f i g   v a l u e 
 *   ` - - j o b - m a n a g e r - i n t e r v a l - m s   < J O B _ M A N A G E R _ I N T E R V A L _ M S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   j o b   m a n a g e r   i n t e r v a l ( m s )   c o n f i g   v a l u e 
 *   ` - - r u n t i m e - w o r k e r - t h r e a d s   < R U N T I M E _ W O R K E R _ T H R E A D S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   r u n t i m e   w o r k e r   t h r e a d s   c o n f i g   v a l u e 
 *   ` - - p 2 p - s e r v e r - a d d r e s s   < P 2 P _ S E R V E R _ A D D R E S S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   s e r v e r   a d d r e s s   c o n f i g   v a l u e 
 *   ` - - p 2 p - k n o w n - p e e r s   < P 2 P _ K N O W N _ P E E R S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   k n o w n   p e e r s   c o n f i g   v a l u e 
 
 
 
 # #   ` s e d a   n o d e   u p d a t e   a c c e p t - o w n e r s h i p ` 
 
 * * U s a g e : * *   ` s e d a   n o d e   u p d a t e   a c c e p t - o w n e r s h i p ` 
 
 
 
 # #   ` s e d a   n o d e   u p d a t e   s e t - p e n d i n g - o w n e r ` 
 
 * * U s a g e : * *   ` s e d a   n o d e   u p d a t e   s e t - p e n d i n g - o w n e r   < O W N E R > ` 
 
 # # # # # #   * * A r g u m e n t s : * * 
 
 *   ` < O W N E R > ` 
 
 
 
 # #   ` s e d a   n o d e   u p d a t e   s e t - s o c k e t - a d d r e s s ` 
 
 * * U s a g e : * *   ` s e d a   n o d e   u p d a t e   s e t - s o c k e t - a d d r e s s   < A D D R E S S > ` 
 
 # # # # # #   * * A r g u m e n t s : * * 
 
 *   ` < A D D R E S S > ` 
 
 
 
 # #   ` s e d a   n o d e   u n r e g i s t e r ` 
 
 U n r e g i s t e r   a   n o d e   f r o m   t h e   g i v e n   n o d e   I D 
 
 * * U s a g e : * *   ` s e d a   n o d e   u n r e g i s t e r   [ O P T I O N S ]   - - n o d e - i d   < N O D E _ I D > ` 
 
 # # # # # #   * * O p t i o n s : * * 
 
 *   ` - n ` ,   ` - - n o d e - i d   < N O D E _ I D > ` 
 *   ` - d ` ,   ` - - d e p o s i t   < D E P O S I T > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   d e p o s i t   c o n f i g   v a l u e 
 *   ` - g ` ,   ` - - g a s   < G A S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   g a s   c o n f i g   v a l u e 
 *   ` - - s e c r e t - k e y   < S E C R E T _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s e c r e t   k e y   c o n f i g   v a l u e 
 *   ` - - s i g n e r - a c c o u n t - i d   < S I G N E R _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s i g n e r   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - c o n t r a c t - a c c o u n t - i d   < C O N T R A C T _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   c o n t r a c t   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - p u b l i c - k e y   < P U B L I C _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p u b l i c   k e y   c o n f i g   v a l u e 
 *   ` - - j o b - m a n a g e r - i n t e r v a l - m s   < J O B _ M A N A G E R _ I N T E R V A L _ M S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   j o b   m a n a g e r   i n t e r v a l ( m s )   c o n f i g   v a l u e 
 *   ` - - r u n t i m e - w o r k e r - t h r e a d s   < R U N T I M E _ W O R K E R _ T H R E A D S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   r u n t i m e   w o r k e r   t h r e a d s   c o n f i g   v a l u e 
 *   ` - - p 2 p - s e r v e r - a d d r e s s   < P 2 P _ S E R V E R _ A D D R E S S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   s e r v e r   a d d r e s s   c o n f i g   v a l u e 
 *   ` - - p 2 p - k n o w n - p e e r s   < P 2 P _ K N O W N _ P E E R S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   k n o w n   p e e r s   c o n f i g   v a l u e 
 
 
 
 # #   ` s e d a   s u b - c h a i n ` 
 
 D e b u g   c o m m a n d s   t o   h e l p   i n t e r a c t   w i t h   s u b - c h a i n s 
 
 * * U s a g e : * *   ` s e d a   s u b - c h a i n   [ O P T I O N S ]   < C O M M A N D > ` 
 
 # # # # # #   * * S u b c o m m a n d s : * * 
 
 *   ` c a l l `      C a l l s   t h e   s p e c i f i e d   m e t h o d   o n   t h e   s p e c i f i e d   c h a i n   w i t h   t h e   g i v e n   a r g s   a n d   c o n t r a c t   I D 
 *   ` v i e w `      V i e w s   t h e   s p e c i f i e d   m e t h o d   o n   t h e   s p e c i f i e d   c h a i n   w i t h   t h e   g i v e n   a r g s   a n d   c o n t r a c t   I D 
 
 # # # # # #   * * O p t i o n s : * * 
 
 *   ` - - c h a i n - r p c - u r l   < C H A I N _ R P C _ U R L > `      A n   o p t i o n   t o   o v e r r i d e   t h e   N e a r   c h a i n   r p c   u r l   c o n f i g   v a l u e 
 
 
 
 # #   ` s e d a   s u b - c h a i n   c a l l ` 
 
 C a l l s   t h e   s p e c i f i e d   m e t h o d   o n   t h e   s p e c i f i e d   c h a i n   w i t h   t h e   g i v e n   a r g s   a n d   c o n t r a c t   I D 
 
 * * U s a g e : * *   ` s e d a   s u b - c h a i n   c a l l   [ O P T I O N S ]   < C H A I N >   < C O N T R A C T _ I D >   < M E T H O D _ N A M E >   < A R G S >   < C A L L _ D E P O S I T > ` 
 
 # # # # # #   * * A r g u m e n t s : * * 
 
 *   ` < C H A I N > `      T h e   s u b - c h a i n   t o   c a l l 
 
     P o s s i b l e   v a l u e s :   ` a n o t h e r ` ,   ` n e a r ` 
 
 *   ` < C O N T R A C T _ I D > `      T h e   c o n t r a c t   I D   f o r   t h e   s u b - c h a i n 
 *   ` < M E T H O D _ N A M E > `      T h e   m e t h o d   n a m e   t o   c a l l 
 *   ` < A R G S > `      T h e   a r g s   t o   p a s s   t o   t h e   c a l l   m e t h o d 
 *   ` < C A L L _ D E P O S I T > `      T h e   d e p o s i t   f o r   t h e   c a l l   m e t h o d 
 
 # # # # # #   * * O p t i o n s : * * 
 
 *   ` - d ` ,   ` - - d e p o s i t   < D E P O S I T > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   d e p o s i t   c o n f i g   v a l u e 
 *   ` - g ` ,   ` - - g a s   < G A S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   g a s   c o n f i g   v a l u e 
 *   ` - - s e c r e t - k e y   < S E C R E T _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s e c r e t   k e y   c o n f i g   v a l u e 
 *   ` - - s i g n e r - a c c o u n t - i d   < S I G N E R _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   s i g n e r   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - c o n t r a c t - a c c o u n t - i d   < C O N T R A C T _ A C C O U N T _ I D > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   c o n t r a c t   a c c o u n t   I D   c o n f i g   v a l u e 
 *   ` - - p u b l i c - k e y   < P U B L I C _ K E Y > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p u b l i c   k e y   c o n f i g   v a l u e 
 *   ` - - j o b - m a n a g e r - i n t e r v a l - m s   < J O B _ M A N A G E R _ I N T E R V A L _ M S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   j o b   m a n a g e r   i n t e r v a l ( m s )   c o n f i g   v a l u e 
 *   ` - - r u n t i m e - w o r k e r - t h r e a d s   < R U N T I M E _ W O R K E R _ T H R E A D S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   r u n t i m e   w o r k e r   t h r e a d s   c o n f i g   v a l u e 
 *   ` - - p 2 p - s e r v e r - a d d r e s s   < P 2 P _ S E R V E R _ A D D R E S S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   s e r v e r   a d d r e s s   c o n f i g   v a l u e 
 *   ` - - p 2 p - k n o w n - p e e r s   < P 2 P _ K N O W N _ P E E R S > `      A n   o p t i o n   t o   o v e r r i d e   t h e   n o d e   p 2 p   k n o w n   p e e r s   c o n f i g   v a l u e 
 
 
 
 # #   ` s e d a   s u b - c h a i n   v i e w ` 
 
 V i e w s   t h e   s p e c i f i e d   m e t h o d   o n   t h e   s p e c i f i e d   c h a i n   w i t h   t h e   g i v e n   a r g s   a n d   c o n t r a c t   I D 
 
 * * U s a g e : * *   ` s e d a   s u b - c h a i n   v i e w   < C H A I N >   < C O N T R A C T _ I D >   < M E T H O D _ N A M E >   < A R G S > ` 
 
 # # # # # #   * * A r g u m e n t s : * * 
 
 *   ` < C H A I N > `      T h e   s u b - c h a i n   t o   c a l l 
 
     P o s s i b l e   v a l u e s :   ` a n o t h e r ` ,   ` n e a r ` 
 
 *   ` < C O N T R A C T _ I D > `      T h e   c o n t r a c t   I D   f o r   t h e   s u b - c h a i n 
 *   ` < M E T H O D _ N A M E > `      T h e   m e t h o d   n a m e   t o   v i e w 
 *   ` < A R G S > `      T h e   a r g s   t o   p a s s   t o   t h e   v i e w   m e t h o d 
 
 
 
 < h r / > 
 
 < s m a l l > < i > 
         T h i s   d o c u m e n t   w a s   g e n e r a t e d   a u t o m a t i c a l l y   b y 
         < a   h r e f = " h t t p s : / / c r a t e s . i o / c r a t e s / c l a p - m a r k d o w n " > < c o d e > c l a p - m a r k d o w n < / c o d e > < / a > . 
 < / i > < / s m a l l > 
 
 