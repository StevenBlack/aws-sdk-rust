// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_asset_bundle_import_job_data_source_override_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssetBundleImportJobDataSourceOverrideParameters,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data_source_id {
        object.key("DataSourceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.data_source_parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DataSourceParameters").start_object();
        crate::protocol_serde::shape_data_source_parameters::ser_data_source_parameters(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.vpc_connection_properties {
        #[allow(unused_mut)]
        let mut object_6 = object.key("VpcConnectionProperties").start_object();
        crate::protocol_serde::shape_vpc_connection_properties::ser_vpc_connection_properties(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.ssl_properties {
        #[allow(unused_mut)]
        let mut object_8 = object.key("SslProperties").start_object();
        crate::protocol_serde::shape_ssl_properties::ser_ssl_properties(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.credentials {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Credentials").start_object();
        crate::protocol_serde::shape_asset_bundle_import_job_data_source_credentials::ser_asset_bundle_import_job_data_source_credentials(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    Ok(())
}

pub(crate) fn de_asset_bundle_import_job_data_source_override_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::AssetBundleImportJobDataSourceOverrideParameters>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssetBundleImportJobDataSourceOverrideParametersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DataSourceId" => {
                            builder = builder.set_data_source_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DataSourceParameters" => {
                            builder = builder
                                .set_data_source_parameters(crate::protocol_serde::shape_data_source_parameters::de_data_source_parameters(tokens)?);
                        }
                        "VpcConnectionProperties" => {
                            builder = builder.set_vpc_connection_properties(
                                crate::protocol_serde::shape_vpc_connection_properties::de_vpc_connection_properties(tokens)?,
                            );
                        }
                        "SslProperties" => {
                            builder = builder.set_ssl_properties(crate::protocol_serde::shape_ssl_properties::de_ssl_properties(tokens)?);
                        }
                        "Credentials" => {
                            builder = builder.set_credentials(
                                    crate::protocol_serde::shape_asset_bundle_import_job_data_source_credentials::de_asset_bundle_import_job_data_source_credentials(tokens)?
                                );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
