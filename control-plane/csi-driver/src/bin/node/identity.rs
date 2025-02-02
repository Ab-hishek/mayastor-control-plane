//! Implementation of gRPC methods from CSI Identity gRPC service.

use csi_driver::{csi::*, CSI_PLUGIN_NAME};
use std::{boxed::Box, collections::HashMap};
use tonic::{Request, Response, Status};
use tracing::debug;

// TODO: can we generate version with commit SHA dynamically?
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub(crate) struct Identity {}

impl Identity {}
#[tonic::async_trait]
impl identity_server::Identity for Identity {
    async fn get_plugin_info(
        &self,
        _request: Request<GetPluginInfoRequest>,
    ) -> Result<Response<GetPluginInfoResponse>, Status> {
        debug!(
            "GetPluginInfo request ({}:{})",
            CSI_PLUGIN_NAME, PLUGIN_VERSION
        );

        Ok(Response::new(GetPluginInfoResponse {
            name: CSI_PLUGIN_NAME.to_owned(),
            vendor_version: PLUGIN_VERSION.to_owned(),
            manifest: HashMap::new(),
        }))
    }

    async fn get_plugin_capabilities(
        &self,
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
        let caps = vec![
            plugin_capability::service::Type::ControllerService,
            plugin_capability::service::Type::VolumeAccessibilityConstraints,
        ];
        debug!("GetPluginCapabilities request: {:?}", caps);

        Ok(Response::new(GetPluginCapabilitiesResponse {
            capabilities: caps
                .into_iter()
                .map(|c| PluginCapability {
                    r#type: Some(plugin_capability::Type::Service(
                        plugin_capability::Service { r#type: c as i32 },
                    )),
                })
                .collect(),
        }))
    }

    async fn probe(
        &self,
        _request: Request<ProbeRequest>,
    ) -> Result<Response<ProbeResponse>, Status> {
        // CSI plugin is independent of the io-engine so it's always ready
        Ok(Response::new(ProbeResponse { ready: Some(true) }))
    }
}
