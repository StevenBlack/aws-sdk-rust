// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_certificate::_associate_certificate_output::AssociateCertificateOutputBuilder;

pub use crate::operation::associate_certificate::_associate_certificate_input::AssociateCertificateInputBuilder;

impl AssociateCertificateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_certificate::AssociateCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_certificate::AssociateCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_certificate();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateCertificate`.
///
/// Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_certificate::builders::AssociateCertificateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_certificate::AssociateCertificateOutput,
        crate::operation::associate_certificate::AssociateCertificateError,
    > for AssociateCertificateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_certificate::AssociateCertificateOutput,
            crate::operation::associate_certificate::AssociateCertificateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateCertificateFluentBuilder {
    /// Creates a new `AssociateCertificate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateCertificate as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_certificate::builders::AssociateCertificateInputBuilder {
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
        crate::operation::associate_certificate::AssociateCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_certificate::AssociateCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_certificate::AssociateCertificate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_certificate::AssociateCertificate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::associate_certificate::AssociateCertificateOutput,
            crate::operation::associate_certificate::AssociateCertificateError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::associate_certificate::AssociateCertificateError>,
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
    /// The ARN of the ACM certificate that you want to associate with your MediaConvert resource.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// The ARN of the ACM certificate that you want to associate with your MediaConvert resource.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// The ARN of the ACM certificate that you want to associate with your MediaConvert resource.
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
}
