use std::fmt::Debug;

use libcnb::runtime::cnb_runtime;
use libcnb::LibCnbError;
use serde::Deserialize;

use crate::error::JvmFunctionInvokerBuildpackErrorHandler;
use crate::layers::bundle::BundleLayerError;
use crate::layers::runtime::RuntimeLayerError;
use libcnb::error::BuildpackErrorHandle;

mod build;
mod detect;
mod error;
mod heroku_shared_lib;
mod layers;

fn main() {
    cnb_runtime(
        detect::detect,
        build::build,
        JvmFunctionInvokerBuildpackErrorHandler {},
    );
}

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackMetadata {
    runtime: JvmFunctionInvokerBuildpackRuntimeMetadata,
}

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackRuntimeMetadata {
    url: String,
    sha256: String,
}
