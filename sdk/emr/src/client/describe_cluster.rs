// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCluster`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl Into<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::set_cluster_id): <p>The identifier of the cluster to describe.</p>
    /// - On success, responds with [`DescribeClusterOutput`](crate::operation::describe_cluster::DescribeClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::describe_cluster::DescribeClusterOutput::cluster): <p>This output contains the details for the requested cluster.</p>
    /// - On failure, responds with [`SdkError<DescribeClusterError>`](crate::operation::describe_cluster::DescribeClusterError)
    pub fn describe_cluster(&self) -> crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder {
        crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::new(self.handle.clone())
    }
}
