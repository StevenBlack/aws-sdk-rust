// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPartition`](crate::operation::get_partition::builders::GetPartitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::set_catalog_id): <p>The ID of the Data Catalog where the partition in question resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl Into<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::set_database_name): <p>The name of the catalog database where the partition resides.</p>
    ///   - [`table_name(impl Into<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::set_table_name): <p>The name of the partition's table.</p>
    ///   - [`partition_values(impl Into<String>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::partition_values) / [`set_partition_values(Option<Vec<String>>)`](crate::operation::get_partition::builders::GetPartitionFluentBuilder::set_partition_values): <p>The values that define the partition.</p>
    /// - On success, responds with [`GetPartitionOutput`](crate::operation::get_partition::GetPartitionOutput) with field(s):
    ///   - [`partition(Option<Partition>)`](crate::operation::get_partition::GetPartitionOutput::partition): <p>The requested information, in the form of a <code>Partition</code> object.</p>
    /// - On failure, responds with [`SdkError<GetPartitionError>`](crate::operation::get_partition::GetPartitionError)
    pub fn get_partition(&self) -> crate::operation::get_partition::builders::GetPartitionFluentBuilder {
        crate::operation::get_partition::builders::GetPartitionFluentBuilder::new(self.handle.clone())
    }
}
