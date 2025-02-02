// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_model_transform_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CustomModelTransformParameters,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_s3_directory_path {
        object.key("sourceS3DirectoryPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.transform_entry_point_script {
        object.key("transformEntryPointScript").string(var_2.as_str());
    }
    Ok(())
}
