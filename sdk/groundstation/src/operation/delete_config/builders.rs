// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_config::_delete_config_output::DeleteConfigOutputBuilder;

pub use crate::operation::delete_config::_delete_config_input::DeleteConfigInputBuilder;

impl DeleteConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_config::DeleteConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_config::DeleteConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteConfig`.
///
/// <p>Deletes a <code>Config</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_config::builders::DeleteConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_config::DeleteConfigOutput,
        crate::operation::delete_config::DeleteConfigError,
    > for DeleteConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_config::DeleteConfigOutput,
            crate::operation::delete_config::DeleteConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteConfigFluentBuilder {
    /// Creates a new `DeleteConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_config::builders::DeleteConfigInputBuilder {
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
        crate::operation::delete_config::DeleteConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_config::DeleteConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_config::DeleteConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_config::DeleteConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_config::DeleteConfigOutput,
            crate::operation::delete_config::DeleteConfigError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_config::DeleteConfigError>,
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
    /// <p>UUID of a <code>Config</code>.</p>
    pub fn config_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.config_id(input.into());
        self
    }
    /// <p>UUID of a <code>Config</code>.</p>
    pub fn set_config_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_config_id(input);
        self
    }
    /// <p>UUID of a <code>Config</code>.</p>
    pub fn get_config_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_config_id()
    }
    /// <p>Type of a <code>Config</code>.</p>
    pub fn config_type(mut self, input: crate::types::ConfigCapabilityType) -> Self {
        self.inner = self.inner.config_type(input);
        self
    }
    /// <p>Type of a <code>Config</code>.</p>
    pub fn set_config_type(mut self, input: ::std::option::Option<crate::types::ConfigCapabilityType>) -> Self {
        self.inner = self.inner.set_config_type(input);
        self
    }
    /// <p>Type of a <code>Config</code>.</p>
    pub fn get_config_type(&self) -> &::std::option::Option<crate::types::ConfigCapabilityType> {
        self.inner.get_config_type()
    }
}
