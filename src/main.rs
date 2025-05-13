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

struct StarknetContext {
    provider: JsonRpcClient<HttpTransport>,
    signer: LocalWallet,
    address: Felt,
}

#[tokio::main]
async fn main() {
    let context = starknet_call_context();
    let account = starknet_account(
        context.provider,
        context.signer,
        context.address,
        chain_id::SEPOLIA,
    );
    let calls = vec![];

    let result = starknet_call(account, "mint_lords", calls).await;

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}

async fn starknet_call(
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    selector: &str,
    calls: Vec<Felt>,
) -> InvokeTransactionResult {
    let result = account
        .execute_v3(vec![Call {
            to: Felt::from_hex(
                &std::env::var("STARKNET_CONTRACT_ADDRESS")
                    .expect("cannot find STARKNET_CONTRACT_ADDRESS env"),
            )
            .unwrap(),
            selector: get_selector_from_name(selector).unwrap(),
            calldata: calls,
        }])
        .send()
        .await
        .unwrap();

    result
}

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
