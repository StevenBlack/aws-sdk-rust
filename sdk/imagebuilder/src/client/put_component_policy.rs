// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutComponentPolicy`](crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`component_arn(impl Into<String>)`](crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder::component_arn) / [`set_component_arn(Option<String>)`](crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder::set_component_arn): <p>The Amazon Resource Name (ARN) of the component that this policy should be applied to.</p>
    ///   - [`policy(impl Into<String>)`](crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder::set_policy): <p>The policy to apply.</p>
    /// - On success, responds with [`PutComponentPolicyOutput`](crate::operation::put_component_policy::PutComponentPolicyOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::put_component_policy::PutComponentPolicyOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`component_arn(Option<String>)`](crate::operation::put_component_policy::PutComponentPolicyOutput::component_arn): <p>The Amazon Resource Name (ARN) of the component that this policy was applied to.</p>
    /// - On failure, responds with [`SdkError<PutComponentPolicyError>`](crate::operation::put_component_policy::PutComponentPolicyError)
    pub fn put_component_policy(&self) -> crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder {
        crate::operation::put_component_policy::builders::PutComponentPolicyFluentBuilder::new(self.handle.clone())
    }
}
