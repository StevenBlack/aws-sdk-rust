// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_scheduled_audit::_delete_scheduled_audit_output::DeleteScheduledAuditOutputBuilder;

pub use crate::operation::delete_scheduled_audit::_delete_scheduled_audit_input::DeleteScheduledAuditInputBuilder;

impl DeleteScheduledAuditInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_scheduled_audit::DeleteScheduledAuditOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_scheduled_audit();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteScheduledAudit`.
///
/// <p>Deletes a scheduled audit.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeleteScheduledAudit</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteScheduledAuditFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_scheduled_audit::builders::DeleteScheduledAuditInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_scheduled_audit::DeleteScheduledAuditOutput,
        crate::operation::delete_scheduled_audit::DeleteScheduledAuditError,
    > for DeleteScheduledAuditFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditOutput,
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteScheduledAuditFluentBuilder {
    /// Creates a new `DeleteScheduledAudit`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteScheduledAudit as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_scheduled_audit::builders::DeleteScheduledAuditInputBuilder {
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
        crate::operation::delete_scheduled_audit::DeleteScheduledAuditOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_scheduled_audit::DeleteScheduledAudit::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_scheduled_audit::DeleteScheduledAudit::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditOutput,
            crate::operation::delete_scheduled_audit::DeleteScheduledAuditError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_scheduled_audit::DeleteScheduledAuditError>,
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
    /// <p>The name of the scheduled audit you want to delete.</p>
    pub fn scheduled_audit_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scheduled_audit_name(input.into());
        self
    }
    /// <p>The name of the scheduled audit you want to delete.</p>
    pub fn set_scheduled_audit_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_scheduled_audit_name(input);
        self
    }
    /// <p>The name of the scheduled audit you want to delete.</p>
    pub fn get_scheduled_audit_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_scheduled_audit_name()
    }
}
