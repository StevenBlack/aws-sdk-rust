// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::post_to_connection::_post_to_connection_output::PostToConnectionOutputBuilder;

pub use crate::operation::post_to_connection::_post_to_connection_input::PostToConnectionInputBuilder;

impl PostToConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::post_to_connection::PostToConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::post_to_connection::PostToConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.post_to_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PostToConnection`.
///
/// <p>Sends the provided data to the specified connection.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PostToConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::post_to_connection::builders::PostToConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::post_to_connection::PostToConnectionOutput,
        crate::operation::post_to_connection::PostToConnectionError,
    > for PostToConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::post_to_connection::PostToConnectionOutput,
            crate::operation::post_to_connection::PostToConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PostToConnectionFluentBuilder {
    /// Creates a new `PostToConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PostToConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::post_to_connection::builders::PostToConnectionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::post_to_connection::PostToConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::post_to_connection::PostToConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::post_to_connection::PostToConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::post_to_connection::PostToConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::post_to_connection::PostToConnectionOutput,
            crate::operation::post_to_connection::PostToConnectionError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::post_to_connection::PostToConnectionError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation::new(self))
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The data to be sent to the client specified by its connection id.</p>
    pub fn data(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.data(input);
        self
    }
    /// <p>The data to be sent to the client specified by its connection id.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_data(input);
        self
    }
    /// <p>The data to be sent to the client specified by its connection id.</p>
    pub fn get_data(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_data()
    }
    /// <p>The identifier of the connection that a specific client is using.</p>
    pub fn connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_id(input.into());
        self
    }
    /// <p>The identifier of the connection that a specific client is using.</p>
    pub fn set_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_id(input);
        self
    }
    /// <p>The identifier of the connection that a specific client is using.</p>
    pub fn get_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_id()
    }
}
