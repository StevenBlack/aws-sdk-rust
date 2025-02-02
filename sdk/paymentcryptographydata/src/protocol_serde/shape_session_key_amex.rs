// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_session_key_amex(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SessionKeyAmex,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.primary_account_number {
        object.key("PrimaryAccountNumber").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pan_sequence_number {
        object.key("PanSequenceNumber").string(var_2.as_str());
    }
    Ok(())
}
