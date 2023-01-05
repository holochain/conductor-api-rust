use holochain::{prelude::AppBundleSource, sweettest::SweetConductor};
use holochain_client::{AdminWebsocket, AppWebsocket, InstallAppPayload, InstalledAppId};
use holochain_conductor_api::CellInfo;
use holochain_zome_types::ExternIO;
use rand::Rng;
use std::{collections::HashMap, path::PathBuf};
use utilities::{authorize_signing_credentials, sign_zome_call};

#[tokio::test(flavor = "multi_thread")]
async fn app_interfaces() {
    let conductor = SweetConductor::from_standard_config().await;
    let admin_port = conductor.get_arbitrary_admin_websocket_port().unwrap();
    let mut admin_ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
        .await
        .unwrap();

    let app_interfaces = admin_ws.list_app_interfaces().await.unwrap();

    assert_eq!(app_interfaces.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn signed_zome_call() {
    let conductor = SweetConductor::from_standard_config().await;
    let admin_port = conductor.get_arbitrary_admin_websocket_port().unwrap();
    let mut admin_ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
        .await
        .unwrap();

    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen();
    let app_id: InstalledAppId = format!("test-app-{}", random_number).into();

    let agent_key = admin_ws.generate_agent_pub_key().await.unwrap();

    admin_ws
        .install_app(InstallAppPayload {
            agent_key: agent_key.clone(),
            installed_app_id: Some(app_id.clone()),
            membrane_proofs: HashMap::new(),
            network_seed: None,
            source: AppBundleSource::Path(PathBuf::from("./fixture/test.happ")),
        })
        .await
        .unwrap();

    admin_ws.enable_app(app_id.clone()).await.unwrap();

    let app_ws_port = admin_ws.attach_app_interface(30000).await.unwrap();
    let mut app_ws = AppWebsocket::connect(format!("ws://localhost:{}", app_ws_port))
        .await
        .unwrap();

    let installed_app = app_ws.app_info(app_id).await.unwrap().unwrap();

    let cells = installed_app.cell_info.into_values().next().unwrap();
    let cell_id = match cells[0].clone() {
        CellInfo::Provisioned(c) => c.cell_id,
        _ => panic!("Invalid cell type"),
    };

    // ******** SIGNED ZOME CALL  ********

    const TEST_ZOME_NAME: &str = "foo";
    const TEST_FN_NAME: &str = "foo";

    let signing_credentials = authorize_signing_credentials(&mut admin_ws, &cell_id).await;
    let signed_zome_call = sign_zome_call(
        &cell_id,
        &TEST_ZOME_NAME,
        &TEST_FN_NAME,
        &signing_credentials,
    )
    .await;

    let response = app_ws.call_zome(signed_zome_call).await.unwrap();
    assert_eq!(
        ExternIO::decode::<String>(&response).unwrap(),
        TEST_FN_NAME.to_string()
    );
}
