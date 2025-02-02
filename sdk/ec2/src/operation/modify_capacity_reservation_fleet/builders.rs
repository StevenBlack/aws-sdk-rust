// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_capacity_reservation_fleet::_modify_capacity_reservation_fleet_output::ModifyCapacityReservationFleetOutputBuilder;

pub use crate::operation::modify_capacity_reservation_fleet::_modify_capacity_reservation_fleet_input::ModifyCapacityReservationFleetInputBuilder;

impl ModifyCapacityReservationFleetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_capacity_reservation_fleet();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyCapacityReservationFleet`.
///
/// <p>Modifies a Capacity Reservation Fleet.</p>
/// <p>When you modify the total target capacity of a Capacity Reservation Fleet, the Fleet automatically creates new Capacity Reservations, or modifies or cancels existing Capacity Reservations in the Fleet to meet the new total target capacity. When you modify the end date for the Fleet, the end dates for all of the individual Capacity Reservations in the Fleet are updated accordingly.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyCapacityReservationFleetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput,
        crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError,
    > for ModifyCapacityReservationFleetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput,
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyCapacityReservationFleetFluentBuilder {
    /// Creates a new `ModifyCapacityReservationFleet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyCapacityReservationFleet as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetInputBuilder {
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
        crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput,
            crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError>,
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
    /// <p>The ID of the Capacity Reservation Fleet to modify.</p>
    pub fn capacity_reservation_fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.capacity_reservation_fleet_id(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation Fleet to modify.</p>
    pub fn set_capacity_reservation_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_capacity_reservation_fleet_id(input);
        self
    }
    /// <p>The ID of the Capacity Reservation Fleet to modify.</p>
    pub fn get_capacity_reservation_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_capacity_reservation_fleet_id()
    }
    /// <p>The total number of capacity units to be reserved by the Capacity Reservation Fleet. This value, together with the instance type weights that you assign to each instance type used by the Fleet determine the number of instances for which the Fleet reserves capacity. Both values are based on units that make sense for your workload. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target capacity</a> in the Amazon EC2 User Guide.</p>
    pub fn total_target_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.total_target_capacity(input);
        self
    }
    /// <p>The total number of capacity units to be reserved by the Capacity Reservation Fleet. This value, together with the instance type weights that you assign to each instance type used by the Fleet determine the number of instances for which the Fleet reserves capacity. Both values are based on units that make sense for your workload. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target capacity</a> in the Amazon EC2 User Guide.</p>
    pub fn set_total_target_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_total_target_capacity(input);
        self
    }
    /// <p>The total number of capacity units to be reserved by the Capacity Reservation Fleet. This value, together with the instance type weights that you assign to each instance type used by the Fleet determine the number of instances for which the Fleet reserves capacity. Both values are based on units that make sense for your workload. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target capacity</a> in the Amazon EC2 User Guide.</p>
    pub fn get_total_target_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_total_target_capacity()
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires. When the Capacity Reservation Fleet expires, its state changes to <code>expired</code> and all of the Capacity Reservations in the Fleet expire.</p>
    /// <p>The Capacity Reservation Fleet expires within an hour after the specified time. For example, if you specify <code>5/31/2019</code>, <code>13:30:55</code>, the Capacity Reservation Fleet is guaranteed to expire between <code>13:30:55</code> and <code>14:30:55</code> on <code>5/31/2019</code>.</p>
    /// <p>You can't specify <b>EndDate</b> and <b> RemoveEndDate</b> in the same request.</p>
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires. When the Capacity Reservation Fleet expires, its state changes to <code>expired</code> and all of the Capacity Reservations in the Fleet expire.</p>
    /// <p>The Capacity Reservation Fleet expires within an hour after the specified time. For example, if you specify <code>5/31/2019</code>, <code>13:30:55</code>, the Capacity Reservation Fleet is guaranteed to expire between <code>13:30:55</code> and <code>14:30:55</code> on <code>5/31/2019</code>.</p>
    /// <p>You can't specify <b>EndDate</b> and <b> RemoveEndDate</b> in the same request.</p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires. When the Capacity Reservation Fleet expires, its state changes to <code>expired</code> and all of the Capacity Reservations in the Fleet expire.</p>
    /// <p>The Capacity Reservation Fleet expires within an hour after the specified time. For example, if you specify <code>5/31/2019</code>, <code>13:30:55</code>, the Capacity Reservation Fleet is guaranteed to expire between <code>13:30:55</code> and <code>14:30:55</code> on <code>5/31/2019</code>.</p>
    /// <p>You can't specify <b>EndDate</b> and <b> RemoveEndDate</b> in the same request.</p>
    pub fn get_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_date()
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
    /// <p>Indicates whether to remove the end date from the Capacity Reservation Fleet. If you remove the end date, the Capacity Reservation Fleet does not expire and it remains active until you explicitly cancel it using the <b>CancelCapacityReservationFleet</b> action.</p>
    /// <p>You can't specify <b>RemoveEndDate</b> and <b> EndDate</b> in the same request.</p>
    pub fn remove_end_date(mut self, input: bool) -> Self {
        self.inner = self.inner.remove_end_date(input);
        self
    }
    /// <p>Indicates whether to remove the end date from the Capacity Reservation Fleet. If you remove the end date, the Capacity Reservation Fleet does not expire and it remains active until you explicitly cancel it using the <b>CancelCapacityReservationFleet</b> action.</p>
    /// <p>You can't specify <b>RemoveEndDate</b> and <b> EndDate</b> in the same request.</p>
    pub fn set_remove_end_date(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_remove_end_date(input);
        self
    }
    /// <p>Indicates whether to remove the end date from the Capacity Reservation Fleet. If you remove the end date, the Capacity Reservation Fleet does not expire and it remains active until you explicitly cancel it using the <b>CancelCapacityReservationFleet</b> action.</p>
    /// <p>You can't specify <b>RemoveEndDate</b> and <b> EndDate</b> in the same request.</p>
    pub fn get_remove_end_date(&self) -> &::std::option::Option<bool> {
        self.inner.get_remove_end_date()
    }
}
