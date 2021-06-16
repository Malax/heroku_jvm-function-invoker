use std::path::PathBuf;

use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::detect::{DetectContext, DetectResult};
use libcnb::generic::GenericPlatform;
use libcnb::LibCnbError;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;
use libcnb::shared::{read_toml_file, TomlFileError};
use toml::value::Table;

pub fn detect(
    context: DetectContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<DetectResult, LibCnbError<JvmFunctionInvokerBuildpackError>> {
    let function_toml_path = context.app_dir.join("function.toml");
    let project_toml_path = context.app_dir.join("project.toml");

    if function_toml_path.exists() || project_toml_declares_salesforce_function(project_toml_path)?
    {
        Ok(DetectResult::Pass(
            BuildPlanBuilder::new()
                .requires("jdk")
                .requires("jvm-application")
                .build(),
        ))
    } else {
        Ok(DetectResult::Fail)
    }
}

fn project_toml_declares_salesforce_function(
    project_toml_path: PathBuf,
) -> Result<bool, JvmFunctionInvokerBuildpackError> {
    let result: Result<Table, TomlFileError> = read_toml_file(&project_toml_path);

    match result {
        Ok(table) => Ok(table
            .get("com.salesforce.type")
            .and_then(|value| value.as_str())
            .map(|value| value == "function")
            .unwrap_or(false)),
        Err(_) => Ok(false),
    }
}
