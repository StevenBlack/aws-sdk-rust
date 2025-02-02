// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataSet`](crate::operation::delete_data_set::builders::DeleteDataSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`data_set_id(impl Into<String>)`](crate::operation::delete_data_set::builders::DeleteDataSetFluentBuilder::data_set_id) / [`set_data_set_id(Option<String>)`](crate::operation::delete_data_set::builders::DeleteDataSetFluentBuilder::set_data_set_id): <p>The unique identifier for a data set.</p>
    /// - On success, responds with [`DeleteDataSetOutput`](crate::operation::delete_data_set::DeleteDataSetOutput)
    /// - On failure, responds with [`SdkError<DeleteDataSetError>`](crate::operation::delete_data_set::DeleteDataSetError)
    pub fn delete_data_set(&self) -> crate::operation::delete_data_set::builders::DeleteDataSetFluentBuilder {
        crate::operation::delete_data_set::builders::DeleteDataSetFluentBuilder::new(self.handle.clone())
    }
}
