// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the GetLoggingOptions operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLoggingOptionsInput {}
impl GetLoggingOptionsInput {
    /// Creates a new builder-style object to manufacture [`GetLoggingOptionsInput`](crate::operation::get_logging_options::GetLoggingOptionsInput).
    pub fn builder() -> crate::operation::get_logging_options::builders::GetLoggingOptionsInputBuilder {
        crate::operation::get_logging_options::builders::GetLoggingOptionsInputBuilder::default()
    }
}

/// A builder for [`GetLoggingOptionsInput`](crate::operation::get_logging_options::GetLoggingOptionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetLoggingOptionsInputBuilder {}
impl GetLoggingOptionsInputBuilder {
    /// Consumes the builder and constructs a [`GetLoggingOptionsInput`](crate::operation::get_logging_options::GetLoggingOptionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_logging_options::GetLoggingOptionsInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_logging_options::GetLoggingOptionsInput {})
    }
}
