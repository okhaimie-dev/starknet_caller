use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{Call, Felt, InvokeTransactionResult},
        utils::get_selector_from_name,
    },
    providers::{
        Url,
        jsonrpc::{HttpTransport, JsonRpcClient},
    },
    signers::{LocalWallet, SigningKey},
};

/// A structure to hold Starknet connection context information.
/// This groups the necessary components for interacting with Starknet.
struct StarknetContext {
    /// JSON-RPC client for communicating with a Starknet node
    provider: JsonRpcClient<HttpTransport>,
    /// Local wallet used for signing transactions
    signer: LocalWallet,
    /// Starknet account address
    address: Felt,
}

/// Main function that demonstrates interacting with a Starknet smart contract.
///
/// This function:
/// 1. Initializes the Starknet context from environment variables
/// 2. Creates a Starknet account
/// 3. Retrieves the contract address from environment
/// 4. Executes a transaction to call the mint_lords function
/// 5. Prints the transaction hash
///
/// # Environment Variables Required
///
/// * All variables required by `starknet_call_context()`
/// * `STARKNET_CONTRACT_ADDRESS` - Address of the target contract
#[tokio::main]
async fn main() {
    let context: StarknetContext = starknet_call_context();
    let account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> = starknet_account(
        context.provider,
        context.signer,
        context.address,
        chain_id::SEPOLIA,
    );
    let contract_address = Felt::from_hex(
        &std::env::var("STARKNET_CONTRACT_ADDRESS")
            .expect("cannot find STARKNET_CONTRACT_ADDRESS env"),
    )
    .unwrap();
    let selector_name = "mint_lords";

    let call = vec![Call {
        to: contract_address,
        selector: get_selector_from_name(selector_name).unwrap(),
        calldata: vec![],
    }];

    let result = starknet_call(account, call).await;

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}

/// Executes a Starknet transaction with the specified calls.
///
/// This function takes a Starknet account and a vector of Call objects, executes them
/// as a single transaction, and returns the result of that transaction.
///
/// # Arguments
///
/// * `account` - The Starknet account used to execute the transaction
/// * `call` - A vector of Call objects representing the function calls to execute
///
/// # Returns
///
/// * `InvokeTransactionResult` - The result of the transaction execution, including
///   transaction hash and other relevant information
///
/// # Panics
///
/// * Panics if the transaction execution fails for any reason (e.g., insufficient
///   balance, invalid function call, contract error)
///
/// # Example
///
/// ```
/// let result = starknet_call(account, vec![Call {
///     to: contract_address,
///     selector: get_selector_from_name("mint_lords").unwrap(),
///     calldata: vec![],
/// }]).await;
/// ```
async fn starknet_call(
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    call: Vec<Call>,
) -> InvokeTransactionResult {
    let result = account.execute_v3(call).send().await.unwrap();

    result
}

/// Creates a StarknetContext from environment variables.
///
/// # Returns
///
/// * `StarknetContext` - Structure containing provider, signer, and address
///
/// # Environment Variables
///
/// * `STARKNET_RPC_URL` - URL of the Starknet JSON-RPC endpoint
/// * `STARKNET_PRIVATE_KEY` - Private key for the Starknet account
/// * `STARKNET_ACCOUNT_ADDRESS` - Address of the Starknet account
///
/// # Panics
///
/// * If any of the required environment variables are not set
/// * If parsing the URL or hex values fails
fn starknet_call_context() -> StarknetContext {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&std::env::var("STARKNET_RPC_URL").expect("cannot find STARKNET_RPC_URL env"))
            .unwrap(),
    ));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(
            &std::env::var("STARKNET_PRIVATE_KEY").expect("cannot find STARKNET_PRIVATE_KEY env"),
        )
        .unwrap(),
    ));
    let address = Felt::from_hex(
        &std::env::var("STARKNET_ACCOUNT_ADDRESS")
            .expect("cannot find STARKNET_ACCOUNT_ADDRESS env"),
    )
    .unwrap();

    StarknetContext {
        provider,
        signer,
        address,
    }
}

/// Creates a SingleOwnerAccount from the provided components.
///
/// # Arguments
///
/// * `provider` - JSON-RPC client for Starknet node communication
/// * `signer` - Wallet for transaction signing
/// * `address` - Account address
/// * `chain_id` - Starknet chain ID
///
/// # Returns
///
/// * `SingleOwnerAccount` - The initialized Starknet account
fn starknet_account(
    provider: JsonRpcClient<HttpTransport>,
    signer: LocalWallet,
    address: Felt,
    chain_id: Felt,
) -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);
    account
}
