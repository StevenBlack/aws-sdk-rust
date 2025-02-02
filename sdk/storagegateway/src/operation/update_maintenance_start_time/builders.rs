// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_maintenance_start_time::_update_maintenance_start_time_output::UpdateMaintenanceStartTimeOutputBuilder;

pub use crate::operation::update_maintenance_start_time::_update_maintenance_start_time_input::UpdateMaintenanceStartTimeInputBuilder;

impl UpdateMaintenanceStartTimeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_maintenance_start_time();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateMaintenanceStartTime`.
///
/// <p>Updates a gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateMaintenanceStartTimeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_maintenance_start_time::builders::UpdateMaintenanceStartTimeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeOutput,
        crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError,
    > for UpdateMaintenanceStartTimeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeOutput,
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateMaintenanceStartTimeFluentBuilder {
    /// Creates a new `UpdateMaintenanceStartTime`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateMaintenanceStartTime as a reference.
    pub fn as_input(&self) -> &crate::operation::update_maintenance_start_time::builders::UpdateMaintenanceStartTimeInputBuilder {
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
        crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTime::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTime::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeOutput,
            crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_maintenance_start_time::UpdateMaintenanceStartTimeError>,
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
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn get_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_arn()
    }
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    pub fn hour_of_day(mut self, input: i32) -> Self {
        self.inner = self.inner.hour_of_day(input);
        self
    }
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    pub fn set_hour_of_day(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_hour_of_day(input);
        self
    }
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    pub fn get_hour_of_day(&self) -> &::std::option::Option<i32> {
        self.inner.get_hour_of_day()
    }
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    pub fn minute_of_hour(mut self, input: i32) -> Self {
        self.inner = self.inner.minute_of_hour(input);
        self
    }
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    pub fn set_minute_of_hour(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_minute_of_hour(input);
        self
    }
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    pub fn get_minute_of_hour(&self) -> &::std::option::Option<i32> {
        self.inner.get_minute_of_hour()
    }
    /// <p>The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    pub fn day_of_week(mut self, input: i32) -> Self {
        self.inner = self.inner.day_of_week(input);
        self
    }
    /// <p>The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    pub fn set_day_of_week(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_day_of_week(input);
        self
    }
    /// <p>The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    pub fn get_day_of_week(&self) -> &::std::option::Option<i32> {
        self.inner.get_day_of_week()
    }
    /// <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    pub fn day_of_month(mut self, input: i32) -> Self {
        self.inner = self.inner.day_of_month(input);
        self
    }
    /// <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    pub fn set_day_of_month(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_day_of_month(input);
        self
    }
    /// <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    pub fn get_day_of_month(&self) -> &::std::option::Option<i32> {
        self.inner.get_day_of_month()
    }
}
