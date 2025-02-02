// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluation_form_section(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EvaluationFormSection,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.title {
        object.key("Title").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ref_id {
        object.key("RefId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instructions {
        object.key("Instructions").string(var_3.as_str());
    }
    if let Some(var_4) = &input.items {
        let mut array_5 = object.key("Items").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_evaluation_form_item::ser_evaluation_form_item(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if input.weight != 0.0 {
        object.key("Weight").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.weight).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_evaluation_form_section<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::EvaluationFormSection>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EvaluationFormSectionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Title" => {
                            builder = builder.set_title(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RefId" => {
                            builder = builder.set_ref_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Instructions" => {
                            builder = builder.set_instructions(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Items" => {
                            builder = builder.set_items(crate::protocol_serde::shape_evaluation_form_items_list::de_evaluation_form_items_list(
                                tokens,
                            )?);
                        }
                        "Weight" => {
                            builder = builder
                                .set_weight(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
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
