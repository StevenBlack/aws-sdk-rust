// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateConfiguration`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_id(impl Into<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::configuration_id) / [`set_configuration_id(Option<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::set_configuration_id): <p>The unique ID that Amazon MQ generates for the configuration.</p>
    ///   - [`data(impl Into<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::data) / [`set_data(Option<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::set_data): <p>Amazon MQ for Active MQ: The base64-encoded XML configuration. Amazon MQ for RabbitMQ: the base64-encoded Cuttlefish configuration.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::set_description): <p>The description of the configuration.</p>
    /// - On success, responds with [`UpdateConfigurationOutput`](crate::operation::update_configuration::UpdateConfigurationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_configuration::UpdateConfigurationOutput::arn): <p>The Amazon Resource Name (ARN) of the configuration.</p>
    ///   - [`created(Option<DateTime>)`](crate::operation::update_configuration::UpdateConfigurationOutput::created): <p>Required. The date and time of the configuration.</p>
    ///   - [`id(Option<String>)`](crate::operation::update_configuration::UpdateConfigurationOutput::id): <p>The unique ID that Amazon MQ generates for the configuration.</p>
    ///   - [`latest_revision(Option<ConfigurationRevision>)`](crate::operation::update_configuration::UpdateConfigurationOutput::latest_revision): <p>The latest revision of the configuration.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_configuration::UpdateConfigurationOutput::name): <p>The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    ///   - [`warnings(Option<Vec<SanitizationWarning>>)`](crate::operation::update_configuration::UpdateConfigurationOutput::warnings): <p>The list of the first 20 warnings about the configuration elements or attributes that were sanitized.</p>
    /// - On failure, responds with [`SdkError<UpdateConfigurationError>`](crate::operation::update_configuration::UpdateConfigurationError)
    pub fn update_configuration(&self) -> crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder {
        crate::operation::update_configuration::builders::UpdateConfigurationFluentBuilder::new(self.handle.clone())
    }
}
