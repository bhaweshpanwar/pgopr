/*
 * Eclipse Public License - v 2.0
 *
 *   THE ACCOMPANYING PROGRAM IS PROVIDED UNDER THE TERMS OF THIS ECLIPSE
 *   PUBLIC LICENSE ("AGREEMENT"). ANY USE, REPRODUCTION OR DISTRIBUTION
 *   OF THE PROGRAM CONSTITUTES RECIPIENT'S ACCEPTANCE OF THIS AGREEMENT.
 */

 use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, EnvVar, PersistentVolumeClaimVolumeSource, PodSpec,
            PodTemplateSpec, Volume, VolumeMount,
        },
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
 };

 use kube::{
    Api, Client, Error,
    api::{DeleteParams, ObjectMeta, PostParams},
};
use log::{info};
use std::collections::BTreeMap;
use std::fs;


/// Creates a pgmoneta deployment
/// 
/// # Arguments
/// - `client` - A Kubernetes client to create the deployment with
/// - `name` - Name of the deployment to be created
/// - `namespace` - Namespace to create the Kubernetes Deployment in
/// 
pub async fn pgmoneta_deploy(   
    client: Client, 
    name: &str, 
    namespace: &str
) -> Result<Deployment, Error> {
    let deployment: Deployment = pgmoneta_create(name, namespace);

    let api: Api<Deployment> = Api::namespaced(client,namespace);
    match api
        .create(&PostParams::default(), &deployment)
        .await
    {
        Ok(o) => {
            info!("Created pgmoneta");
            Ok(o)
        }
        Err(e) => Err(e),
    }
}

/// Deletes an existing pgmoneta deployment
/// 
/// # Arguments
/// - `client` - A Kubernetes client to delete the Deployment with
/// - `name` - Name of the deployment to delete
/// - `namespace` - Namespace the existing deployment resides in
///     
pub async fn pgmoneta_undeploy(
    client: Client,
    name: &str,
    namespace: &str
) -> Result<(), Error> {
    let api: Api<Deployment> = Api::namespaced(client,namespace);

    match api.delete(name, &DeleteParams::default()).await {
        Ok(_) => {
            info!("Deleted pgmoneta");
        }
        Err(e ) => return Err(e),
    }
    Ok(())
}


/// Primary: Generate
pub fn pgmoneta_generate() {
    let data = serde_yaml::to_string(&pgmoneta_create("pgmoneta", "default"))
        .expect("Can't serialize pgopr-pgmoneta.yaml");
    fs::write("pgopr-pgmoneta.yaml", data).expect("Unable to write file: pgopr-pgmoneta.yaml");
}

fn pgmoneta_create(name: &str, namespace: &str) -> Deployment {
    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    labels.insert("app".to_owned(), name.to_owned());

    // Definition of the deployment
    let deployment: Deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..ObjectMeta::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(1i32),
            selector: LabelSelector {
                match_expressions: None,
                match_labels: Some(labels.clone()),
            },
            template: PodTemplateSpec {
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.to_owned(),
                        image: Some("pgmoneta-rocky10:latest".to_string()),
                        args: Some(vec![
                            "ppmoneta".to_string(),
                            "-c".to_string(),
                            "/etc/pgmoneta/pgmoneta.conf".to_string(),
                            "-c".to_string(),
                            "/etc/pgmoneta/pgmoeta.conf".to_string(),
                        ]),
                        image_pull_policy: Some("IfNotPresent".to_string()),
                        ports: Some(vec![
                            ContainerPort {
                                name: Some("proxy".to_string()),
                                container_port: 8432,
                                ..ContainerPort::default()
                            },
                            ContainerPort {
                                name: Some("metrics".to_string()),
                                container_port: 5001,
                                ..ContainerPort::default()
                            }
                        ]),
                        env: Some(vec![
                            EnvVar {
                                name: "PG_PRIMARY_NAME".to_string(),
                                value: Some("postgresql".to_string()),
                                ..EnvVar::default()
                            },
                            EnvVar {
                                name: "PG_PRIMARY_PORT".to_string(),
                                value: Some("5432".to_string()),
                                ..EnvVar::default()
                            },
                            EnvVar {
                                name: "PG_BACKUP_SLOT".to_string(),
                                value: Some("pgmoneta_slot".to_string()),
                                ..EnvVar::default()
                            },
                        ]),
                        volume_mounts: Some(vec![
                            VolumeMount {
                                name: "pgmoneta-storage".to_string(),
                                mount_path: "/home/pgmoneta".to_string(),
                                ..VolumeMount::default()
                        }]),
                        ..Container::default()
                    }],
                    volumes: Some(vec![
                        Volume {
                            name: "pgmoneta-storage".to_string(),
                            persistent_volume_claim: Some(PersistentVolumeClaimVolumeSource {
                                claim_name: "pgmoneta-storage".to_string(),
                                ..PersistentVolumeClaimVolumeSource::default()
                            }),
                        ..Volume::default()
                    }]),
                    ..PodSpec::default()
                }),
                metadata: Some(ObjectMeta {
                    labels: Some(labels.clone()),
                    ..ObjectMeta::default()
                }),
            },
            ..DeploymentSpec::default()
        }),
        ..Deployment::default()
    };

    deployment
}