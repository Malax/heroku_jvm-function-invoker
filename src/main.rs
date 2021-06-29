use std::fmt::Debug;

use libcnb::cnb_runtime;
use serde::Deserialize;

use crate::error::handle_buildpack_error;
use crate::heroku_shared_lib::error::HerokuBuildpackErrorHandler;
use crate::layers::bundle::BundleLayerError;
use crate::layers::runtime::RuntimeLayerError;

mod build;
mod detect;
mod error;
mod heroku_shared_lib;
mod layers;

fn main() {
    cnb_runtime(
        detect::detect,
        build::build,
        HerokuBuildpackErrorHandler::new(Box::new(handle_buildpack_error)),
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
