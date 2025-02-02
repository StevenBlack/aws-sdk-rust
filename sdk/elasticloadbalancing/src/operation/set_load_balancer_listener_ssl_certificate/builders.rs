// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_load_balancer_listener_ssl_certificate::_set_load_balancer_listener_ssl_certificate_output::SetLoadBalancerListenerSslCertificateOutputBuilder;

pub use crate::operation::set_load_balancer_listener_ssl_certificate::_set_load_balancer_listener_ssl_certificate_input::SetLoadBalancerListenerSslCertificateInputBuilder;

impl SetLoadBalancerListenerSslCertificateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_load_balancer_listener_ssl_certificate();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetLoadBalancerListenerSSLCertificate`.
///
/// <p>Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port.</p>
/// <p>For more information about updating your SSL certificate, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-update-ssl-cert.html">Replace the SSL Certificate for Your Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetLoadBalancerListenerSSLCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_load_balancer_listener_ssl_certificate::builders::SetLoadBalancerListenerSslCertificateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateOutput,
        crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError,
    > for SetLoadBalancerListenerSSLCertificateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateOutput,
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetLoadBalancerListenerSSLCertificateFluentBuilder {
    /// Creates a new `SetLoadBalancerListenerSSLCertificate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetLoadBalancerListenerSSLCertificate as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::set_load_balancer_listener_ssl_certificate::builders::SetLoadBalancerListenerSslCertificateInputBuilder {
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
        crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificate::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificate::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateOutput,
            crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSSLCertificateError>,
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
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_load_balancer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn get_load_balancer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_load_balancer_name()
    }
    /// <p>The port that uses the specified SSL certificate.</p>
    pub fn load_balancer_port(mut self, input: i32) -> Self {
        self.inner = self.inner.load_balancer_port(input);
        self
    }
    /// <p>The port that uses the specified SSL certificate.</p>
    pub fn set_load_balancer_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_load_balancer_port(input);
        self
    }
    /// <p>The port that uses the specified SSL certificate.</p>
    pub fn get_load_balancer_port(&self) -> &::std::option::Option<i32> {
        self.inner.get_load_balancer_port()
    }
    /// <p>The Amazon Resource Name (ARN) of the SSL certificate.</p>
    pub fn ssl_certificate_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ssl_certificate_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SSL certificate.</p>
    pub fn set_ssl_certificate_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ssl_certificate_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SSL certificate.</p>
    pub fn get_ssl_certificate_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ssl_certificate_id()
    }
}
