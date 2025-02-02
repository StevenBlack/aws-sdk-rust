// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ibm3624_random_pin(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Ibm3624RandomPin,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.decimalization_table {
        object.key("DecimalizationTable").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pin_validation_data_pad_character {
        object.key("PinValidationDataPadCharacter").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pin_validation_data {
        object.key("PinValidationData").string(var_3.as_str());
    }
    Ok(())
}
