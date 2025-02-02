// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAlert`](crate::operation::describe_alert::builders::DescribeAlertFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alert_arn(impl Into<String>)`](crate::operation::describe_alert::builders::DescribeAlertFluentBuilder::alert_arn) / [`set_alert_arn(Option<String>)`](crate::operation::describe_alert::builders::DescribeAlertFluentBuilder::set_alert_arn): <p>The ARN of the alert to describe.</p>
    /// - On success, responds with [`DescribeAlertOutput`](crate::operation::describe_alert::DescribeAlertOutput) with field(s):
    ///   - [`alert(Option<Alert>)`](crate::operation::describe_alert::DescribeAlertOutput::alert): <p>Contains information about an alert.</p>
    /// - On failure, responds with [`SdkError<DescribeAlertError>`](crate::operation::describe_alert::DescribeAlertError)
    pub fn describe_alert(&self) -> crate::operation::describe_alert::builders::DescribeAlertFluentBuilder {
        crate::operation::describe_alert::builders::DescribeAlertFluentBuilder::new(self.handle.clone())
    }
}
