use std::{collections::HashMap, path::PathBuf};

use holochain::{
    prelude::{DeleteCloneCellPayload, DisableCloneCellPayload, EnableCloneCellPayload},
    sweettest::SweetConductor,
};
use holochain_client::{
    AdminWebsocket, AppAgentWebsocket, AppWebsocket, AuthorizeSigningCredentialsPayload,
    ClientAgentSigner, ConductorApiError, InstallAppPayload,
};
use holochain_types::prelude::{
    AppBundleSource, CloneCellId, CloneId, CreateCloneCellPayload, DnaModifiersOpt, InstalledAppId,
};
use holochain_zome_types::{dependencies::holochain_integrity_types::ExternIO, prelude::RoleName};

#[tokio::test(flavor = "multi_thread")]
async fn clone_cell_management() {
    let conductor = SweetConductor::from_standard_config().await;
    let admin_port = conductor.get_arbitrary_admin_websocket_port().unwrap();
    let mut admin_ws = AdminWebsocket::connect(format!("127.0.0.1:{}", admin_port))
        .await
        .unwrap();
    let app_id: InstalledAppId = "test-app".into();
    let role_name: RoleName = "foo".into();
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
    let app_api_port = admin_ws.attach_app_interface(0).await.unwrap();
    let mut app_ws = AppWebsocket::connect(format!("127.0.0.1:{}", app_api_port))
        .await
        .unwrap();
    let clone_cell = {
        let clone_cell = app_ws
            .create_clone_cell(CreateCloneCellPayload {
                app_id: app_id.clone(),
                role_name: role_name.clone(),
                modifiers: DnaModifiersOpt::none().with_network_seed("seed".into()),
                membrane_proof: None,
                name: None,
            })
            .await
            .unwrap();
        assert_eq!(*clone_cell.cell_id.agent_pubkey(), agent_key);
        assert_eq!(clone_cell.clone_id, CloneId::new(&role_name, 0));
        clone_cell
    };
    let cell_id = clone_cell.cell_id.clone();

    let mut signer = ClientAgentSigner::default();
    let credentials = admin_ws
        .authorize_signing_credentials(AuthorizeSigningCredentialsPayload {
            cell_id: cell_id.clone(),
            functions: None,
        })
        .await
        .unwrap();
    signer.add_credentials(cell_id.clone(), credentials);

    let mut app_ws = AppAgentWebsocket::from_existing(app_ws, app_id.clone(), signer.into())
        .await
        .unwrap();

    const TEST_ZOME_NAME: &str = "foo";
    const TEST_FN_NAME: &str = "foo";

    // call clone cell should succeed
    let response = app_ws
        .call_zome(
            cell_id.clone().into(),
            TEST_ZOME_NAME.into(),
            TEST_FN_NAME.into(),
            ExternIO::encode(()).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.decode::<String>().unwrap(), "foo");

    // disable clone cell
    app_ws
        .disable_clone_cell(DisableCloneCellPayload {
            app_id: app_id.clone(),
            clone_cell_id: CloneCellId::CloneId(clone_cell.clone().clone_id),
        })
        .await
        .unwrap();

    // call disabled clone cell should fail
    let response = app_ws
        .call_zome(
            cell_id.clone().into(),
            TEST_ZOME_NAME.into(),
            TEST_FN_NAME.into(),
            ExternIO::encode(()).unwrap(),
        )
        .await;
    assert!(response.is_err());

    // enable clone cell
    let enabled_cell = app_ws
        .enable_clone_cell(EnableCloneCellPayload {
            app_id: app_id.clone(),
            clone_cell_id: CloneCellId::CloneId(clone_cell.clone().clone_id),
        })
        .await
        .unwrap();
    assert_eq!(enabled_cell, clone_cell);

    // call enabled clone cell should succeed
    let response = app_ws
        .call_zome(
            cell_id.clone().into(),
            TEST_ZOME_NAME.into(),
            TEST_FN_NAME.into(),
            ExternIO::encode(()).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.decode::<String>().unwrap(), "foo");

    // disable clone cell again
    app_ws
        .disable_clone_cell(DisableCloneCellPayload {
            app_id: app_id.clone(),
            clone_cell_id: CloneCellId::CloneId(clone_cell.clone().clone_id),
        })
        .await
        .unwrap();

    // delete disabled clone cell
    admin_ws
        .delete_clone_cell(DeleteCloneCellPayload {
            app_id: app_id.clone(),
            clone_cell_id: CloneCellId::CellId(clone_cell.clone().cell_id),
        })
        .await
        .unwrap();
    // restore deleted clone cells should fail
    let enable_clone_cell_response = app_ws
        .enable_clone_cell(EnableCloneCellPayload {
            app_id: app_id.clone(),
            clone_cell_id: CloneCellId::CloneId(clone_cell.clone_id),
        })
        .await;
    assert!(enable_clone_cell_response.is_err());
}

// Check that app info can be refreshed to allow zome calls to a clone cell identified by its clone cell id
#[tokio::test(flavor = "multi_thread")]
pub async fn app_info_refresh() {
    let conductor = SweetConductor::from_standard_config().await;
    let admin_port = conductor.get_arbitrary_admin_websocket_port().unwrap();
    let mut admin_ws = AdminWebsocket::connect(format!("127.0.0.1:{}", admin_port))
        .await
        .unwrap();
    let app_id: InstalledAppId = "test-app".into();
    let role_name: RoleName = "foo".into();

    // Create our agent key
    let agent_key = admin_ws.generate_agent_pub_key().await.unwrap();

    // Install and enable an app
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

    let mut signer = ClientAgentSigner::default();

    // Create an app interface and connect an app agent to it
    let app_api_port = admin_ws.attach_app_interface(0).await.unwrap();
    let mut app_agent_ws = AppAgentWebsocket::connect(
        format!("127.0.0.1:{}", app_api_port),
        app_id.clone(),
        signer.clone().into(),
    )
    .await
    .unwrap();

    // Create a clone cell, AFTER the app agent has been created
    let cloned_cell = app_agent_ws
        .create_clone_cell(CreateCloneCellPayload {
            app_id: app_id.clone(),
            role_name: role_name.clone(),
            modifiers: DnaModifiersOpt::none().with_network_seed("test seed".into()),
            membrane_proof: None,
            name: None,
        })
        .await
        .unwrap();

    // Authorise signing credentials for the cloned cell
    let credentials = admin_ws
        .authorize_signing_credentials(AuthorizeSigningCredentialsPayload {
            cell_id: cloned_cell.cell_id.clone(),
            functions: None,
        })
        .await
        .unwrap();
    signer.add_credentials(cloned_cell.cell_id.clone(), credentials);

    // Call the zome function on the clone cell, expecting a failure
    let err = app_agent_ws
        .call_zome(
            cloned_cell.clone_id.clone().into(),
            "foo".into(),
            "foo".into(),
            ExternIO::encode(()).unwrap(),
        )
        .await
        .expect_err("Should fail because the client doesn't know the clone cell exists");
    match err {
        ConductorApiError::CellNotFound => (),
        _ => panic!("Unexpected error: {:?}", err),
    }

    // Refresh the app info, which means the app agent will now know about the clone cell
    app_agent_ws.refresh_app_info().await.unwrap();

    // Call the zome function on the clone cell again, expecting success
    app_agent_ws
        .call_zome(
            cloned_cell.clone_id.clone().into(),
            "foo".into(),
            "foo".into(),
            ExternIO::encode(()).unwrap(),
        )
        .await
        .unwrap();
}
