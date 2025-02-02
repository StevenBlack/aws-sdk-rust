// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_layer_version::_get_layer_version_output::GetLayerVersionOutputBuilder;

pub use crate::operation::get_layer_version::_get_layer_version_input::GetLayerVersionInputBuilder;

impl GetLayerVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_layer_version::GetLayerVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_layer_version::GetLayerVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_layer_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetLayerVersion`.
///
/// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetLayerVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_layer_version::GetLayerVersionOutput,
        crate::operation::get_layer_version::GetLayerVersionError,
    > for GetLayerVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_layer_version::GetLayerVersionOutput,
            crate::operation::get_layer_version::GetLayerVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetLayerVersionFluentBuilder {
    /// Creates a new `GetLayerVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetLayerVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder {
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
        crate::operation::get_layer_version::GetLayerVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_layer_version::GetLayerVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_layer_version::GetLayerVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_layer_version::GetLayerVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_layer_version::GetLayerVersionOutput,
            crate::operation::get_layer_version::GetLayerVersionError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_layer_version::GetLayerVersionError>,
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
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.layer_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_layer_name(input);
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn get_layer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_layer_name()
    }
    /// <p>The version number.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.inner = self.inner.version_number(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version_number(input);
        self
    }
    /// <p>The version number.</p>
    pub fn get_version_number(&self) -> &::std::option::Option<i64> {
        self.inner.get_version_number()
    }
}
