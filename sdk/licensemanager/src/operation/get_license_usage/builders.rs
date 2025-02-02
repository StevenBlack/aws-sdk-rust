// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_license_usage::_get_license_usage_output::GetLicenseUsageOutputBuilder;

pub use crate::operation::get_license_usage::_get_license_usage_input::GetLicenseUsageInputBuilder;

impl GetLicenseUsageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_license_usage::GetLicenseUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_license_usage::GetLicenseUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_license_usage();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetLicenseUsage`.
///
/// <p>Gets detailed information about the usage of the specified license.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetLicenseUsageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_license_usage::builders::GetLicenseUsageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_license_usage::GetLicenseUsageOutput,
        crate::operation::get_license_usage::GetLicenseUsageError,
    > for GetLicenseUsageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_license_usage::GetLicenseUsageOutput,
            crate::operation::get_license_usage::GetLicenseUsageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetLicenseUsageFluentBuilder {
    /// Creates a new `GetLicenseUsage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetLicenseUsage as a reference.
    pub fn as_input(&self) -> &crate::operation::get_license_usage::builders::GetLicenseUsageInputBuilder {
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
        crate::operation::get_license_usage::GetLicenseUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_license_usage::GetLicenseUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_license_usage::GetLicenseUsage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_license_usage::GetLicenseUsage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_license_usage::GetLicenseUsageOutput,
            crate::operation::get_license_usage::GetLicenseUsageError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_license_usage::GetLicenseUsageError>,
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
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    pub fn license_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.license_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    pub fn set_license_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_license_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    pub fn get_license_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_license_arn()
    }
}
