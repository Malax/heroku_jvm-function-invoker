use std::fmt::Debug;

use libcnb::{cnb_runtime, GenericErrorHandler};
use serde::Deserialize;

mod build;
mod detect;
mod error;
mod layers;

fn main() {
    cnb_runtime(
        detect::detect,
        build::build,
        GenericErrorHandler
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
