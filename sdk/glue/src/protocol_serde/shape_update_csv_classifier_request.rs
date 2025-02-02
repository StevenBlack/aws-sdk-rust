// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_csv_classifier_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateCsvClassifierRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.delimiter {
        object.key("Delimiter").string(var_2.as_str());
    }
    if let Some(var_3) = &input.quote_symbol {
        object.key("QuoteSymbol").string(var_3.as_str());
    }
    if let Some(var_4) = &input.contains_header {
        object.key("ContainsHeader").string(var_4.as_str());
    }
    if let Some(var_5) = &input.header {
        let mut array_6 = object.key("Header").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.disable_value_trimming {
        object.key("DisableValueTrimming").boolean(*var_8);
    }
    if let Some(var_9) = &input.allow_single_column {
        object.key("AllowSingleColumn").boolean(*var_9);
    }
    if let Some(var_10) = &input.custom_datatype_configured {
        object.key("CustomDatatypeConfigured").boolean(*var_10);
    }
    if let Some(var_11) = &input.custom_datatypes {
        let mut array_12 = object.key("CustomDatatypes").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.serde {
        object.key("Serde").string(var_14.as_str());
    }
    Ok(())
}
