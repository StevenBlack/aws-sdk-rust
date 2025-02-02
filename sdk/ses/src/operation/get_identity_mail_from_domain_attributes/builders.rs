// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_identity_mail_from_domain_attributes::_get_identity_mail_from_domain_attributes_output::GetIdentityMailFromDomainAttributesOutputBuilder;

pub use crate::operation::get_identity_mail_from_domain_attributes::_get_identity_mail_from_domain_attributes_input::GetIdentityMailFromDomainAttributesInputBuilder;

impl GetIdentityMailFromDomainAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_identity_mail_from_domain_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetIdentityMailFromDomainAttributes`.
///
/// <p>Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains).</p>
/// <p>This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identities at a time.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetIdentityMailFromDomainAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_identity_mail_from_domain_attributes::builders::GetIdentityMailFromDomainAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
        crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
    > for GetIdentityMailFromDomainAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetIdentityMailFromDomainAttributesFluentBuilder {
    /// Creates a new `GetIdentityMailFromDomainAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetIdentityMailFromDomainAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::get_identity_mail_from_domain_attributes::builders::GetIdentityMailFromDomainAttributesInputBuilder {
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
        crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributes::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
            crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError>,
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
    /// Appends an item to `Identities`.
    ///
    /// To override the contents of this collection use [`set_identities`](Self::set_identities).
    ///
    /// <p>A list of one or more identities.</p>
    pub fn identities(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identities(input.into());
        self
    }
    /// <p>A list of one or more identities.</p>
    pub fn set_identities(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_identities(input);
        self
    }
    /// <p>A list of one or more identities.</p>
    pub fn get_identities(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_identities()
    }
}
