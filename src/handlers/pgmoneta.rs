/*
 * Eclipse Public License - v 2.0
 *
 *   THE ACCOMPANYING PROGRAM IS PROVIDED UNDER THE TERMS OF THIS ECLIPSE
 *   PUBLIC LICENSE ("AGREEMENT"). ANY USE, REPRODUCTION OR DISTRIBUTION
 *   OF THE PROGRAM CONSTITUTES RECIPIENT'S ACCEPTANCE OF THIS AGREEMENT.
 */

use crate::{k8s, persistent, services, pgmoneta::pgmoneta}; 
use kube::Client;

/// Provisions pgmoneta components
pub async fn handle_provision_pgmoneta() {
    super::print_header();
    let client: Client = k8s::k8s_client().await;
    let namespace = "default".to_owned();

    let _pvc = persistent::persistent_volume_claim_deploy(
        client.clone(),
        "pgmoneta-storage",
        &namespace,
        10u32,
    ).await;

    let _d = pgmoneta::pgmoneta_deploy(client.clone(), "pgmoneta", &namespace).await;
    let _s = services::service_deploy(client.clone(), "pgmoneta", &namespace).await;
}

/// Removes pgmoneta components
pub async fn handle_retire_pgmoneta() {
    super::print_header();
    let client: Client = k8s::k8s_client().await;
    let namespace = "default".to_owned();

    let _s = services::service_undeploy(client.clone(), "pgmoneta", &namespace).await;
    let _d = pgmoneta::pgmoneta_undeploy(client.clone(), "pgmoneta", &namespace).await;
    let _pvc = persistent::persistent_volume_claim_undeploy(
        client.clone(),
        "pgmoneta-storage",
        &namespace,
    ).await;
}