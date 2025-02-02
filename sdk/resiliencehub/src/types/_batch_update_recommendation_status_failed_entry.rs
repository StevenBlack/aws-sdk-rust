// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List of operational recommendations that did not get included or excluded.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchUpdateRecommendationStatusFailedEntry {
    /// <p>An identifier of an entry in this batch that is used to communicate the result.</p> <note>
    /// <p>The <code>entryId</code>s of a batch request need to be unique within a request.</p>
    /// </note>
    pub entry_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the error that occurred while excluding an operational recommendation.</p>
    pub error_message: ::std::option::Option<::std::string::String>,
}
impl BatchUpdateRecommendationStatusFailedEntry {
    /// <p>An identifier of an entry in this batch that is used to communicate the result.</p> <note>
    /// <p>The <code>entryId</code>s of a batch request need to be unique within a request.</p>
    /// </note>
    pub fn entry_id(&self) -> ::std::option::Option<&str> {
        self.entry_id.as_deref()
    }
    /// <p>Indicates the error that occurred while excluding an operational recommendation.</p>
    pub fn error_message(&self) -> ::std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
impl BatchUpdateRecommendationStatusFailedEntry {
    /// Creates a new builder-style object to manufacture [`BatchUpdateRecommendationStatusFailedEntry`](crate::types::BatchUpdateRecommendationStatusFailedEntry).
    pub fn builder() -> crate::types::builders::BatchUpdateRecommendationStatusFailedEntryBuilder {
        crate::types::builders::BatchUpdateRecommendationStatusFailedEntryBuilder::default()
    }
}

/// A builder for [`BatchUpdateRecommendationStatusFailedEntry`](crate::types::BatchUpdateRecommendationStatusFailedEntry).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BatchUpdateRecommendationStatusFailedEntryBuilder {
    pub(crate) entry_id: ::std::option::Option<::std::string::String>,
    pub(crate) error_message: ::std::option::Option<::std::string::String>,
}
impl BatchUpdateRecommendationStatusFailedEntryBuilder {
    /// <p>An identifier of an entry in this batch that is used to communicate the result.</p> <note>
    /// <p>The <code>entryId</code>s of a batch request need to be unique within a request.</p>
    /// </note>
    pub fn entry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entry_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier of an entry in this batch that is used to communicate the result.</p> <note>
    /// <p>The <code>entryId</code>s of a batch request need to be unique within a request.</p>
    /// </note>
    pub fn set_entry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entry_id = input;
        self
    }
    /// <p>An identifier of an entry in this batch that is used to communicate the result.</p> <note>
    /// <p>The <code>entryId</code>s of a batch request need to be unique within a request.</p>
    /// </note>
    pub fn get_entry_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.entry_id
    }
    /// <p>Indicates the error that occurred while excluding an operational recommendation.</p>
    pub fn error_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates the error that occurred while excluding an operational recommendation.</p>
    pub fn set_error_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_message = input;
        self
    }
    /// <p>Indicates the error that occurred while excluding an operational recommendation.</p>
    pub fn get_error_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_message
    }
    /// Consumes the builder and constructs a [`BatchUpdateRecommendationStatusFailedEntry`](crate::types::BatchUpdateRecommendationStatusFailedEntry).
    pub fn build(self) -> crate::types::BatchUpdateRecommendationStatusFailedEntry {
        crate::types::BatchUpdateRecommendationStatusFailedEntry {
            entry_id: self.entry_id,
            error_message: self.error_message,
        }
    }
}
