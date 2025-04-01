use tkhq_rust_sdk::gen::immutable::activity::v1::ExportWalletAccountIntent;
use tkhq_rust_sdk::gen::external::activity::v1::ExportWalletAccountRequest;
use tkhq_rust_sdk::client::{self, ExportWalletAccount, GetWalletAccounts, GetWallets};
use tkhq_rust_sdk::r#gen::immutable::activity::v1::ActivityType;
use tkhq_rust_sdk::gen::services::coordinator::public::v1 as api;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    let bundle_public_key = std::env::var("TURNKEY_BUNDLE_PUBLIC_KEY").unwrap();
    let tk = client::TurnkeyClient::new_from_env().unwrap();
    let req = api::GetWalletAccountsRequest { organization_id: "8ef2e67f-8569-43bd-a0f8-e1c6d5324435".to_owned(), wallet_id: "9e2399f2-122c-57b8-964a-9307990f6b83".to_owned(), pagination_options: None };
    let resp = tk.request::<GetWalletAccounts>(req).await.unwrap();
    println!("{:#?}", resp);

    let timestamp_ms = tk.request_timestamp_ms();
    let resp = tk.request::<ExportWalletAccount>(ExportWalletAccountRequest {
        organization_id: "8ef2e67f-8569-43bd-a0f8-e1c6d5324435".to_owned(),
        timestamp_ms,
        r#type: ActivityType::ExportWalletAccount.as_str_name().to_owned(),
        parameters: Some(ExportWalletAccountIntent {
            address: "GcbTpLPPKNjfxH7rRjigKWEKy88CGMVdTMUWMV7ZL68v".to_owned(),
            target_public_key: bundle_public_key.clone(),
        }),
    }).await.unwrap();
    println!("{:#?}", resp);
}
