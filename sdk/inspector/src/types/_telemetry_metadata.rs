// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The metadata about the Amazon Inspector application data metrics collected by the agent. This data type is used as the response element in the <code>GetTelemetryMetadata</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TelemetryMetadata {
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    pub message_type: ::std::option::Option<::std::string::String>,
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    pub count: ::std::option::Option<i64>,
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    pub data_size: ::std::option::Option<i64>,
}
impl TelemetryMetadata {
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    pub fn message_type(&self) -> ::std::option::Option<&str> {
        self.message_type.as_deref()
    }
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn count(&self) -> ::std::option::Option<i64> {
        self.count
    }
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn data_size(&self) -> ::std::option::Option<i64> {
        self.data_size
    }
}
impl TelemetryMetadata {
    /// Creates a new builder-style object to manufacture [`TelemetryMetadata`](crate::types::TelemetryMetadata).
    pub fn builder() -> crate::types::builders::TelemetryMetadataBuilder {
        crate::types::builders::TelemetryMetadataBuilder::default()
    }
}

/// A builder for [`TelemetryMetadata`](crate::types::TelemetryMetadata).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TelemetryMetadataBuilder {
    pub(crate) message_type: ::std::option::Option<::std::string::String>,
    pub(crate) count: ::std::option::Option<i64>,
    pub(crate) data_size: ::std::option::Option<i64>,
}
impl TelemetryMetadataBuilder {
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    pub fn message_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    pub fn set_message_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_type = input;
        self
    }
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    pub fn get_message_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_type
    }
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn count(mut self, input: i64) -> Self {
        self.count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn set_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.count = input;
        self
    }
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn get_count(&self) -> &::std::option::Option<i64> {
        &self.count
    }
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn data_size(mut self, input: i64) -> Self {
        self.data_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn set_data_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.data_size = input;
        self
    }
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    pub fn get_data_size(&self) -> &::std::option::Option<i64> {
        &self.data_size
    }
    /// Consumes the builder and constructs a [`TelemetryMetadata`](crate::types::TelemetryMetadata).
    pub fn build(self) -> crate::types::TelemetryMetadata {
        crate::types::TelemetryMetadata {
            message_type: self.message_type,
            count: self.count,
            data_size: self.data_size,
        }
    }
}
