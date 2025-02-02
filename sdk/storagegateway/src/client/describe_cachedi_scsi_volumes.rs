// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCachediSCSIVolumes`](crate::operation::describe_cachedi_scsi_volumes::builders::DescribeCachediSCSIVolumesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`volume_ar_ns(impl Into<String>)`](crate::operation::describe_cachedi_scsi_volumes::builders::DescribeCachediSCSIVolumesFluentBuilder::volume_ar_ns) / [`set_volume_ar_ns(Option<Vec<String>>)`](crate::operation::describe_cachedi_scsi_volumes::builders::DescribeCachediSCSIVolumesFluentBuilder::set_volume_ar_ns): <p>An array of strings where each string represents the Amazon Resource Name (ARN) of a cached volume. All of the specified cached volumes must be from the same gateway. Use <code>ListVolumes</code> to get volume ARNs for a gateway.</p>
    /// - On success, responds with [`DescribeCachediScsiVolumesOutput`](crate::operation::describe_cachedi_scsi_volumes::DescribeCachediScsiVolumesOutput) with field(s):
    ///   - [`cachedi_scsi_volumes(Option<Vec<CachediScsiVolume>>)`](crate::operation::describe_cachedi_scsi_volumes::DescribeCachediScsiVolumesOutput::cachedi_scsi_volumes): <p>An array of objects where each object contains metadata about one cached volume.</p>
    /// - On failure, responds with [`SdkError<DescribeCachediSCSIVolumesError>`](crate::operation::describe_cachedi_scsi_volumes::DescribeCachediSCSIVolumesError)
    pub fn describe_cachedi_scsi_volumes(
        &self,
    ) -> crate::operation::describe_cachedi_scsi_volumes::builders::DescribeCachediSCSIVolumesFluentBuilder {
        crate::operation::describe_cachedi_scsi_volumes::builders::DescribeCachediSCSIVolumesFluentBuilder::new(self.handle.clone())
    }
}
