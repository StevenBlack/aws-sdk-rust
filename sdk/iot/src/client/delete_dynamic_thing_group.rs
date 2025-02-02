// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDynamicThingGroup`](crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_group_name(impl Into<String>)`](crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder::thing_group_name) / [`set_thing_group_name(Option<String>)`](crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder::set_thing_group_name): <p>The name of the dynamic thing group to delete.</p>
    ///   - [`expected_version(i64)`](crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder::expected_version) / [`set_expected_version(Option<i64>)`](crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder::set_expected_version): <p>The expected version of the dynamic thing group to delete.</p>
    /// - On success, responds with [`DeleteDynamicThingGroupOutput`](crate::operation::delete_dynamic_thing_group::DeleteDynamicThingGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteDynamicThingGroupError>`](crate::operation::delete_dynamic_thing_group::DeleteDynamicThingGroupError)
    pub fn delete_dynamic_thing_group(&self) -> crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder {
        crate::operation::delete_dynamic_thing_group::builders::DeleteDynamicThingGroupFluentBuilder::new(self.handle.clone())
    }
}
