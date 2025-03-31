use std::{env, process};

use tkhq_rust_sdk::client::{self, CreateSubOrganization};
use tkhq_rust_sdk::gen::external::activity::v1::CreateSubOrganizationRequest;
use tkhq_rust_sdk::r#gen::immutable::activity::v1::{CreateSubOrganizationIntentV5, WalletAccountParams, WalletParams};
use tkhq_rust_sdk::gen::immutable::activity::v1::{
    ActivityType, ApiKeyParams, RootUserParamsV2,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: create_sub_organization NAME");
        process::exit(1);
    }
    let sub_organization_name = args[1].clone();

    let organization_id = env::var("TURNKEY_ORGANIZATION_ID").unwrap();
    let root_user_public_key = env::var("TURNKEY_API_PUBLIC_KEY").unwrap();

    let tk = client::TurnkeyClient::new_from_env().unwrap();

    let timestamp_ms = tk.request_timestamp_ms();
    let req = CreateSubOrganizationRequest {
        organization_id,
        r#type: ActivityType::CreateSubOrganizationV5
            .as_str_name()
            .to_owned(),
        timestamp_ms,
        parameters: Some(CreateSubOrganizationIntentV5 {
            sub_organization_name,
            root_users: vec![RootUserParamsV2 {
                user_name: "root".to_owned(),
                user_email: None,
                oauth_providers: vec![],
                api_keys: vec![ApiKeyParams {
                    api_key_name: "root_public_key".to_owned(),
                    public_key: root_user_public_key,
                    expiration_seconds: None,
                }],
                authenticators: vec![],
            }],
            root_quorum_threshold: 1,
            wallet: Some(WalletParams {
                wallet_name: "default".to_owned(),
                accounts: vec![WalletAccountParams {
                    // path_format: PathFormat::Bip32.into(),
                    path_format: "PATH_FORMAT_BIP32".to_owned(),
                    // curve: Curve::Ed25519.into(),
                    curve: "CURVE_ED25519".to_owned(),
                    path: "m/44'/501'/0'/0".to_owned(),
                    // address_format: AddressFormat::Solana.into(),
                    address_format: "ADDRESS_FORMAT_SOLANA".to_owned(),
                }],
                mnemonic_length: None,
            }),
            disable_email_recovery: None,
            disable_email_auth: None,
        }),
    };
    let resp = tk.request::<CreateSubOrganization>(req).await.unwrap();
    println!("{:#?}", resp);
}
