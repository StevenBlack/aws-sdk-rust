// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVodSource`](crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_location_name(impl Into<String>)`](crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder::source_location_name) / [`set_source_location_name(Option<String>)`](crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder::set_source_location_name): <p>The name of the source location associated with this VOD Source.</p>
    ///   - [`vod_source_name(impl Into<String>)`](crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder::vod_source_name) / [`set_vod_source_name(Option<String>)`](crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder::set_vod_source_name): <p>The name of the VOD Source.</p>
    /// - On success, responds with [`DescribeVodSourceOutput`](crate::operation::describe_vod_source::DescribeVodSourceOutput) with field(s):
    ///   - [`ad_break_opportunities(Option<Vec<AdBreakOpportunity>>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::ad_break_opportunities): <p>The ad break opportunities within the VOD source.</p>
    ///   - [`arn(Option<String>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::arn): <p>The ARN of the VOD source.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::creation_time): <p>The timestamp that indicates when the VOD source was created.</p>
    ///   - [`http_package_configurations(Option<Vec<HttpPackageConfiguration>>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::http_package_configurations): <p>The HTTP package configurations.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::last_modified_time): <p>The last modified time of the VOD source.</p>
    ///   - [`source_location_name(Option<String>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::source_location_name): <p>The name of the source location associated with the VOD source.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::tags): <p>The tags assigned to the VOD source. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/tagging.html">Tagging AWS Elemental MediaTailor Resources</a>.</p>
    ///   - [`vod_source_name(Option<String>)`](crate::operation::describe_vod_source::DescribeVodSourceOutput::vod_source_name): <p>The name of the VOD source.</p>
    /// - On failure, responds with [`SdkError<DescribeVodSourceError>`](crate::operation::describe_vod_source::DescribeVodSourceError)
    pub fn describe_vod_source(&self) -> crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder {
        crate::operation::describe_vod_source::builders::DescribeVodSourceFluentBuilder::new(self.handle.clone())
    }
}
