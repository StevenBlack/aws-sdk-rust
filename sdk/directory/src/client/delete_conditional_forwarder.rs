// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteConditionalForwarder`](crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder::set_directory_id): <p>The directory ID for which you are deleting the conditional forwarder.</p>
    ///   - [`remote_domain_name(impl Into<String>)`](crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder::remote_domain_name) / [`set_remote_domain_name(Option<String>)`](crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder::set_remote_domain_name): <p>The fully qualified domain name (FQDN) of the remote domain with which you are deleting the conditional forwarder.</p>
    /// - On success, responds with [`DeleteConditionalForwarderOutput`](crate::operation::delete_conditional_forwarder::DeleteConditionalForwarderOutput)
    /// - On failure, responds with [`SdkError<DeleteConditionalForwarderError>`](crate::operation::delete_conditional_forwarder::DeleteConditionalForwarderError)
    pub fn delete_conditional_forwarder(&self) -> crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder {
        crate::operation::delete_conditional_forwarder::builders::DeleteConditionalForwarderFluentBuilder::new(self.handle.clone())
    }
}
