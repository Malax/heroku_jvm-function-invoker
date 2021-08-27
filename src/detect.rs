use std::path::PathBuf;

use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::{read_toml_file, DetectContext, DetectOutcome, Error, GenericPlatform, TomlFileError};
use toml::value::Table;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;

pub fn detect(
    context: DetectContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<DetectOutcome, Error<JvmFunctionInvokerBuildpackError>> {
    let function_toml_path = context.app_dir.join("function.toml");
    let project_toml_path = context.app_dir.join("project.toml");

    if function_toml_path.exists() || project_toml_declares_salesforce_function(project_toml_path)?
    {
        Ok(DetectOutcome::Pass(
            BuildPlanBuilder::new()
                .requires("jdk")
                .requires("jvm-application")
                .build(),
        ))
    } else {
        Ok(DetectOutcome::Fail)
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
