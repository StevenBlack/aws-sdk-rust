// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPullRequestOverrideState`](crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pull_request_id(impl Into<String>)`](crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder::pull_request_id) / [`set_pull_request_id(Option<String>)`](crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder::set_pull_request_id): <p>The ID of the pull request for which you want to get information about whether approval rules have been set aside (overridden).</p>
    ///   - [`revision_id(impl Into<String>)`](crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder::set_revision_id): <p>The system-generated ID of the revision for the pull request. To retrieve the most recent revision ID, use <code>GetPullRequest</code>.</p>
    /// - On success, responds with [`GetPullRequestOverrideStateOutput`](crate::operation::get_pull_request_override_state::GetPullRequestOverrideStateOutput) with field(s):
    ///   - [`overridden(bool)`](crate::operation::get_pull_request_override_state::GetPullRequestOverrideStateOutput::overridden): <p>A Boolean value that indicates whether a pull request has had its rules set aside (TRUE) or whether all approval rules still apply (FALSE).</p>
    ///   - [`overrider(Option<String>)`](crate::operation::get_pull_request_override_state::GetPullRequestOverrideStateOutput::overrider): <p>The Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p>
    /// - On failure, responds with [`SdkError<GetPullRequestOverrideStateError>`](crate::operation::get_pull_request_override_state::GetPullRequestOverrideStateError)
    pub fn get_pull_request_override_state(
        &self,
    ) -> crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder {
        crate::operation::get_pull_request_override_state::builders::GetPullRequestOverrideStateFluentBuilder::new(self.handle.clone())
    }
}
