use std::{env, process};

use tkhq_rust_sdk::gen::services::coordinator::public::v1::GetSubOrgIdsRequest;
use tkhq_rust_sdk::client::{self, GetSubOrgIds};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        eprintln!("usage: list_sub_organizations");
        process::exit(1);
    }

    let organization_id = env::var("TURNKEY_ORGANIZATION_ID").unwrap();

    let tk = client::TurnkeyClient::new_from_env().unwrap();

    let req = GetSubOrgIdsRequest {
        organization_id,
        filter_type: "OIDC_TOKEN"
            .to_owned(),
        filter_value: "eyJhbGciOiJSUzI1NiIsImtpZCI6IjgyMWYzYmM2NmYwNzUxZjc4NDA2MDY3OTliMWFkZjllOWZiNjBkZmIiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIxMDYzNjAxODUzMTItYW9qZnBkb2Q3bTloc2Y2OWhtNWVmMDJjZWc5NzdkYW4uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIxMDYzNjAxODUzMTItYW9qZnBkb2Q3bTloc2Y2OWhtNWVmMDJjZWc5NzdkYW4uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTQ2MTI2ODg0Mzg2OTI5ODAwNTkiLCJoZCI6Imxvb3Rlci5haSIsImVtYWlsIjoiZnJhbmtAbG9vdGVyLmFpIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiZGE5ODJhZjBiNDVjODllMDIzODQ1NDM3NDgzMGEzNTU0M2NhZTNmYjFhMmMzYTMwNzU3YWY3MThjZjNiZjlhYSIsIm5iZiI6MTc0MzM4NzkwMCwibmFtZSI6IkZyYW5rIChMb290ZXIpIiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0tOeFd4cUNLZzZpOW9CRVNDQmVjT2RVMy1vNWVZZDJhTHpDV1F6YTF3ZWNZaVRjQT1zOTYtYyIsImdpdmVuX25hbWUiOiJGcmFuayIsImZhbWlseV9uYW1lIjoiKExvb3RlcikiLCJpYXQiOjE3NDMzODgyMDAsImV4cCI6MTc0MzM5MTgwMCwianRpIjoiMDM2YTk2NTMzMzg0MTk3YWJlMGE3ZTkwZGQxMTM2NDkwNWI1MjYxMCJ9.XT264MvUhQ7foQPWTcFK2JEMb0fyFafRLghA7LgfdBsHaHzQjzrhz9fTirQe1l5ZNA8AM4q1eIlrdht4YAeSpqOaLlAOH1JksnYkQM0yvUyHkbjOFqDlM-dJ9xrHsjh84sbOSNBNCpdBPzluhfp-hcGZRh21ODR-sdiu-YZsgBM34ixWzsSpbMuJuDpRg2fVez1AZ1im7BBAPZzaIr_JwV03GeKqPOm73nOvMXOmcNN82nwnBg1M0N2_yqbX_KYRW9_-YyuTj8HhBjUrQs_u5V4TJfYpS1w7jmrMZMJoTLozQjllf5Z_eYsno6lIbb2LsHPqKeU6tdHp80jqRqsh9w".to_owned(),
        pagination_options: None
    };
    let resp = tk.request::<GetSubOrgIds>(req).await.unwrap();
    println!("{:#?}", resp);
}
