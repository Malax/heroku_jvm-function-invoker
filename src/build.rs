use libcnb::build::BuildContext;
use libcnb::data::launch::{Launch, Process};
use libcnb::generic::GenericPlatform;
use libcnb::layer_lifecycle::execute_layer_lifecycle;
use libcnb::LibCnbError;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::layers::bundle::BundleLayerLifecycle;
use crate::layers::opt::OptLayerLifecycle;
use crate::layers::runtime::RuntimeLayerLifecycle;
use crate::JvmFunctionInvokerBuildpackMetadata;
use std::path::PathBuf;

pub fn build(
    context: BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<(), LibCnbError<JvmFunctionInvokerBuildpackError>> {
    let run_sh_path: PathBuf = execute_layer_lifecycle("opt", OptLayerLifecycle {}, &context)?;

    let runtime_jar_path: PathBuf =
        execute_layer_lifecycle("runtime", RuntimeLayerLifecycle {}, &context)?;

    let bundle_path = execute_layer_lifecycle(
        "bundle",
        BundleLayerLifecycle {
            invoker_jar_path: runtime_jar_path.clone(),
        },
        &context,
    )?;

    let launch = Launch::default().process(Process::new(
        "web",
        run_sh_path.to_string_lossy(),
        vec![
            runtime_jar_path.to_string_lossy(),
            bundle_path.to_string_lossy(),
        ],
        false,
    )?);

    context.write_launch(launch);

    Ok(())
}
