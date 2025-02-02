// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_transit_gateway_multicast_domain::_associate_transit_gateway_multicast_domain_output::AssociateTransitGatewayMulticastDomainOutputBuilder;

pub use crate::operation::associate_transit_gateway_multicast_domain::_associate_transit_gateway_multicast_domain_input::AssociateTransitGatewayMulticastDomainInputBuilder;

impl AssociateTransitGatewayMulticastDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_transit_gateway_multicast_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateTransitGatewayMulticastDomain`.
///
/// <p>Associates the specified subnets and transit gateway attachments with the specified transit gateway multicast domain.</p>
/// <p>The transit gateway attachment must be in the available state before you can add a resource. Use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeTransitGatewayAttachments.html">DescribeTransitGatewayAttachments</a> to see the state of the attachment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateTransitGatewayMulticastDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput,
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
    > for AssociateTransitGatewayMulticastDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput,
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateTransitGatewayMulticastDomainFluentBuilder {
    /// Creates a new `AssociateTransitGatewayMulticastDomain`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateTransitGatewayMulticastDomain as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainInputBuilder {
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
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomain::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomain::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput,
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError,
        >,
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
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_multicast_domain_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_multicast_domain_id(input);
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_multicast_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_multicast_domain_id()
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_attachment_id()
    }
    /// Appends an item to `SubnetIds`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_ids(input.into());
        self
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_subnet_ids(input);
        self
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_subnet_ids()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
