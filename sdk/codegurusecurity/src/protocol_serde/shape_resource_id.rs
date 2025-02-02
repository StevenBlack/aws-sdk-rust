// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_resource_id<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ResourceId>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.to_unescaped()?.as_ref() {
                        "codeArtifactId" => Some(crate::types::ResourceId::CodeArtifactId(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?
                                .unwrap_or_default(),
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ResourceId::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    Ok(variant)
}

pub fn ser_resource_id(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResourceId,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::types::ResourceId::CodeArtifactId(inner) => {
            object_4.key("codeArtifactId").string(inner.as_str());
        }
        crate::types::ResourceId::Unknown => return Err(::aws_smithy_http::operation::error::SerializationError::unknown_variant("ResourceId")),
    }
    Ok(())
}
