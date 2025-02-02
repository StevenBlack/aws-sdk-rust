// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_domain_nameservers::_update_domain_nameservers_output::UpdateDomainNameserversOutputBuilder;

pub use crate::operation::update_domain_nameservers::_update_domain_nameservers_input::UpdateDomainNameserversInputBuilder;

impl UpdateDomainNameserversInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_domain_nameservers::UpdateDomainNameserversOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_domain_nameservers::UpdateDomainNameserversError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_domain_nameservers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDomainNameservers`.
///
/// <p>This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p>
/// <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDomainNameserversFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_domain_nameservers::builders::UpdateDomainNameserversInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_domain_nameservers::UpdateDomainNameserversOutput,
        crate::operation::update_domain_nameservers::UpdateDomainNameserversError,
    > for UpdateDomainNameserversFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_domain_nameservers::UpdateDomainNameserversOutput,
            crate::operation::update_domain_nameservers::UpdateDomainNameserversError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDomainNameserversFluentBuilder {
    /// Creates a new `UpdateDomainNameservers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDomainNameservers as a reference.
    pub fn as_input(&self) -> &crate::operation::update_domain_nameservers::builders::UpdateDomainNameserversInputBuilder {
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
        crate::operation::update_domain_nameservers::UpdateDomainNameserversOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_domain_nameservers::UpdateDomainNameserversError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_domain_nameservers::UpdateDomainNameservers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_domain_nameservers::UpdateDomainNameservers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_domain_nameservers::UpdateDomainNameserversOutput,
            crate::operation::update_domain_nameservers::UpdateDomainNameserversError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_domain_nameservers::UpdateDomainNameserversError>,
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
    /// <p>The name of the domain that you want to change name servers for.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain that you want to change name servers for.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain that you want to change name servers for.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>The authorization key for .fi domains</p>
    #[deprecated]
    pub fn fi_auth_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fi_auth_key(input.into());
        self
    }
    /// <p>The authorization key for .fi domains</p>
    #[deprecated]
    pub fn set_fi_auth_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fi_auth_key(input);
        self
    }
    /// <p>The authorization key for .fi domains</p>
    #[deprecated]
    pub fn get_fi_auth_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fi_auth_key()
    }
    /// Appends an item to `Nameservers`.
    ///
    /// To override the contents of this collection use [`set_nameservers`](Self::set_nameservers).
    ///
    /// <p>A list of new name servers for the domain.</p>
    pub fn nameservers(mut self, input: crate::types::Nameserver) -> Self {
        self.inner = self.inner.nameservers(input);
        self
    }
    /// <p>A list of new name servers for the domain.</p>
    pub fn set_nameservers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Nameserver>>) -> Self {
        self.inner = self.inner.set_nameservers(input);
        self
    }
    /// <p>A list of new name servers for the domain.</p>
    pub fn get_nameservers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Nameserver>> {
        self.inner.get_nameservers()
    }
}
