// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDefaultBranch`](crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl Into<String>)`](crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder::set_repository_name): <p>The name of the repository for which you want to set or change the default branch.</p>
    ///   - [`default_branch_name(impl Into<String>)`](crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder::default_branch_name) / [`set_default_branch_name(Option<String>)`](crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder::set_default_branch_name): <p>The name of the branch to set as the default branch.</p>
    /// - On success, responds with [`UpdateDefaultBranchOutput`](crate::operation::update_default_branch::UpdateDefaultBranchOutput)
    /// - On failure, responds with [`SdkError<UpdateDefaultBranchError>`](crate::operation::update_default_branch::UpdateDefaultBranchError)
    pub fn update_default_branch(&self) -> crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder {
        crate::operation::update_default_branch::builders::UpdateDefaultBranchFluentBuilder::new(self.handle.clone())
    }
}
