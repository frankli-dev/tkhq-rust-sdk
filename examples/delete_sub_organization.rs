use std::{env, process};

use tkhq_rust_sdk::client::{self, CreateSubOrganization, DeleteSubOrganization};
use tkhq_rust_sdk::gen::external::activity::v1::{CreateSubOrganizationRequest, DeleteSubOrganizationRequest};
use tkhq_rust_sdk::r#gen::immutable::activity::v1::CreateSubOrganizationIntentV5;
use tkhq_rust_sdk::gen::immutable::activity::v1::{
    ActivityType, ApiKeyParams, DeleteSubOrganizationIntent, RootUserParams, RootUserParamsV2
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
    let sub_organization_id = args[1].clone();

    let organization_id = env::var("TURNKEY_ORGANIZATION_ID").unwrap();
    let root_user_public_key = env::var("TURNKEY_API_PUBLIC_KEY").unwrap();

    let tk = client::TurnkeyClient::new_from_env().unwrap();

    let timestamp_ms = tk.request_timestamp_ms();
    let req = DeleteSubOrganizationRequest {
        organization_id: sub_organization_id,
        r#type: ActivityType::DeleteSubOrganization
            .as_str_name()
            .to_owned(),
        timestamp_ms,
        parameters: Some(DeleteSubOrganizationIntent {
            delete_without_export: Some(true)
        }),
    };
    let resp = tk.request::<DeleteSubOrganization>(req).await.unwrap();
    println!("{:#?}", resp);
}
