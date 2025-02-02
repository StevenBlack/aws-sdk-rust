// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the launch specification for VM import.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ImportInstanceLaunchSpecification {
    /// <p>Reserved.</p>
    pub additional_info: ::std::option::Option<::std::string::String>,
    /// <p>The architecture of the instance.</p>
    pub architecture: ::std::option::Option<crate::types::ArchitectureValues>,
    /// <p>The security group IDs.</p>
    pub group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The security group names.</p>
    pub group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub instance_initiated_shutdown_behavior: ::std::option::Option<crate::types::ShutdownBehavior>,
    /// <p>The instance type. For more information about the instance types that you can import, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-instance-types">Instance Types</a> in the VM Import/Export User Guide.</p>
    pub instance_type: ::std::option::Option<crate::types::InstanceType>,
    /// <p>Indicates whether monitoring is enabled.</p>
    pub monitoring: ::std::option::Option<bool>,
    /// <p>The placement information for the instance.</p>
    pub placement: ::std::option::Option<crate::types::Placement>,
    /// <p>[EC2-VPC] An available IP address from the IP address range of the subnet.</p>
    pub private_ip_address: ::std::option::Option<::std::string::String>,
    /// <p>[EC2-VPC] The ID of the subnet in which to launch the instance.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The Base64-encoded user data to make available to the instance.</p>
    pub user_data: ::std::option::Option<crate::types::UserData>,
}
impl ImportInstanceLaunchSpecification {
    /// <p>Reserved.</p>
    pub fn additional_info(&self) -> ::std::option::Option<&str> {
        self.additional_info.as_deref()
    }
    /// <p>The architecture of the instance.</p>
    pub fn architecture(&self) -> ::std::option::Option<&crate::types::ArchitectureValues> {
        self.architecture.as_ref()
    }
    /// <p>The security group IDs.</p>
    pub fn group_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.group_ids.as_deref()
    }
    /// <p>The security group names.</p>
    pub fn group_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.group_names.as_deref()
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn instance_initiated_shutdown_behavior(&self) -> ::std::option::Option<&crate::types::ShutdownBehavior> {
        self.instance_initiated_shutdown_behavior.as_ref()
    }
    /// <p>The instance type. For more information about the instance types that you can import, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-instance-types">Instance Types</a> in the VM Import/Export User Guide.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>Indicates whether monitoring is enabled.</p>
    pub fn monitoring(&self) -> ::std::option::Option<bool> {
        self.monitoring
    }
    /// <p>The placement information for the instance.</p>
    pub fn placement(&self) -> ::std::option::Option<&crate::types::Placement> {
        self.placement.as_ref()
    }
    /// <p>[EC2-VPC] An available IP address from the IP address range of the subnet.</p>
    pub fn private_ip_address(&self) -> ::std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p>[EC2-VPC] The ID of the subnet in which to launch the instance.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The Base64-encoded user data to make available to the instance.</p>
    pub fn user_data(&self) -> ::std::option::Option<&crate::types::UserData> {
        self.user_data.as_ref()
    }
}
impl ::std::fmt::Debug for ImportInstanceLaunchSpecification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ImportInstanceLaunchSpecification");
        formatter.field("additional_info", &self.additional_info);
        formatter.field("architecture", &self.architecture);
        formatter.field("group_ids", &self.group_ids);
        formatter.field("group_names", &self.group_names);
        formatter.field("instance_initiated_shutdown_behavior", &self.instance_initiated_shutdown_behavior);
        formatter.field("instance_type", &self.instance_type);
        formatter.field("monitoring", &self.monitoring);
        formatter.field("placement", &self.placement);
        formatter.field("private_ip_address", &self.private_ip_address);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("user_data", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ImportInstanceLaunchSpecification {
    /// Creates a new builder-style object to manufacture [`ImportInstanceLaunchSpecification`](crate::types::ImportInstanceLaunchSpecification).
    pub fn builder() -> crate::types::builders::ImportInstanceLaunchSpecificationBuilder {
        crate::types::builders::ImportInstanceLaunchSpecificationBuilder::default()
    }
}

/// A builder for [`ImportInstanceLaunchSpecification`](crate::types::ImportInstanceLaunchSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ImportInstanceLaunchSpecificationBuilder {
    pub(crate) additional_info: ::std::option::Option<::std::string::String>,
    pub(crate) architecture: ::std::option::Option<crate::types::ArchitectureValues>,
    pub(crate) group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) instance_initiated_shutdown_behavior: ::std::option::Option<crate::types::ShutdownBehavior>,
    pub(crate) instance_type: ::std::option::Option<crate::types::InstanceType>,
    pub(crate) monitoring: ::std::option::Option<bool>,
    pub(crate) placement: ::std::option::Option<crate::types::Placement>,
    pub(crate) private_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) user_data: ::std::option::Option<crate::types::UserData>,
}
impl ImportInstanceLaunchSpecificationBuilder {
    /// <p>Reserved.</p>
    pub fn additional_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.additional_info = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Reserved.</p>
    pub fn set_additional_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.additional_info = input;
        self
    }
    /// <p>Reserved.</p>
    pub fn get_additional_info(&self) -> &::std::option::Option<::std::string::String> {
        &self.additional_info
    }
    /// <p>The architecture of the instance.</p>
    pub fn architecture(mut self, input: crate::types::ArchitectureValues) -> Self {
        self.architecture = ::std::option::Option::Some(input);
        self
    }
    /// <p>The architecture of the instance.</p>
    pub fn set_architecture(mut self, input: ::std::option::Option<crate::types::ArchitectureValues>) -> Self {
        self.architecture = input;
        self
    }
    /// <p>The architecture of the instance.</p>
    pub fn get_architecture(&self) -> &::std::option::Option<crate::types::ArchitectureValues> {
        &self.architecture
    }
    /// Appends an item to `group_ids`.
    ///
    /// To override the contents of this collection use [`set_group_ids`](Self::set_group_ids).
    ///
    /// <p>The security group IDs.</p>
    pub fn group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.group_ids.unwrap_or_default();
        v.push(input.into());
        self.group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security group IDs.</p>
    pub fn set_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.group_ids = input;
        self
    }
    /// <p>The security group IDs.</p>
    pub fn get_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.group_ids
    }
    /// Appends an item to `group_names`.
    ///
    /// To override the contents of this collection use [`set_group_names`](Self::set_group_names).
    ///
    /// <p>The security group names.</p>
    pub fn group_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.group_names.unwrap_or_default();
        v.push(input.into());
        self.group_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security group names.</p>
    pub fn set_group_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.group_names = input;
        self
    }
    /// <p>The security group names.</p>
    pub fn get_group_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.group_names
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn instance_initiated_shutdown_behavior(mut self, input: crate::types::ShutdownBehavior) -> Self {
        self.instance_initiated_shutdown_behavior = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn set_instance_initiated_shutdown_behavior(mut self, input: ::std::option::Option<crate::types::ShutdownBehavior>) -> Self {
        self.instance_initiated_shutdown_behavior = input;
        self
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn get_instance_initiated_shutdown_behavior(&self) -> &::std::option::Option<crate::types::ShutdownBehavior> {
        &self.instance_initiated_shutdown_behavior
    }
    /// <p>The instance type. For more information about the instance types that you can import, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-instance-types">Instance Types</a> in the VM Import/Export User Guide.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type. For more information about the instance types that you can import, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-instance-types">Instance Types</a> in the VM Import/Export User Guide.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::InstanceType>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The instance type. For more information about the instance types that you can import, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-instance-types">Instance Types</a> in the VM Import/Export User Guide.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::InstanceType> {
        &self.instance_type
    }
    /// <p>Indicates whether monitoring is enabled.</p>
    pub fn monitoring(mut self, input: bool) -> Self {
        self.monitoring = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether monitoring is enabled.</p>
    pub fn set_monitoring(mut self, input: ::std::option::Option<bool>) -> Self {
        self.monitoring = input;
        self
    }
    /// <p>Indicates whether monitoring is enabled.</p>
    pub fn get_monitoring(&self) -> &::std::option::Option<bool> {
        &self.monitoring
    }
    /// <p>The placement information for the instance.</p>
    pub fn placement(mut self, input: crate::types::Placement) -> Self {
        self.placement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The placement information for the instance.</p>
    pub fn set_placement(mut self, input: ::std::option::Option<crate::types::Placement>) -> Self {
        self.placement = input;
        self
    }
    /// <p>The placement information for the instance.</p>
    pub fn get_placement(&self) -> &::std::option::Option<crate::types::Placement> {
        &self.placement
    }
    /// <p>[EC2-VPC] An available IP address from the IP address range of the subnet.</p>
    pub fn private_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[EC2-VPC] An available IP address from the IP address range of the subnet.</p>
    pub fn set_private_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_ip_address = input;
        self
    }
    /// <p>[EC2-VPC] An available IP address from the IP address range of the subnet.</p>
    pub fn get_private_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.private_ip_address
    }
    /// <p>[EC2-VPC] The ID of the subnet in which to launch the instance.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The ID of the subnet in which to launch the instance.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>[EC2-VPC] The ID of the subnet in which to launch the instance.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// <p>The Base64-encoded user data to make available to the instance.</p>
    pub fn user_data(mut self, input: crate::types::UserData) -> Self {
        self.user_data = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Base64-encoded user data to make available to the instance.</p>
    pub fn set_user_data(mut self, input: ::std::option::Option<crate::types::UserData>) -> Self {
        self.user_data = input;
        self
    }
    /// <p>The Base64-encoded user data to make available to the instance.</p>
    pub fn get_user_data(&self) -> &::std::option::Option<crate::types::UserData> {
        &self.user_data
    }
    /// Consumes the builder and constructs a [`ImportInstanceLaunchSpecification`](crate::types::ImportInstanceLaunchSpecification).
    pub fn build(self) -> crate::types::ImportInstanceLaunchSpecification {
        crate::types::ImportInstanceLaunchSpecification {
            additional_info: self.additional_info,
            architecture: self.architecture,
            group_ids: self.group_ids,
            group_names: self.group_names,
            instance_initiated_shutdown_behavior: self.instance_initiated_shutdown_behavior,
            instance_type: self.instance_type,
            monitoring: self.monitoring,
            placement: self.placement,
            private_ip_address: self.private_ip_address,
            subnet_id: self.subnet_id,
            user_data: self.user_data,
        }
    }
}
impl ::std::fmt::Debug for ImportInstanceLaunchSpecificationBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ImportInstanceLaunchSpecificationBuilder");
        formatter.field("additional_info", &self.additional_info);
        formatter.field("architecture", &self.architecture);
        formatter.field("group_ids", &self.group_ids);
        formatter.field("group_names", &self.group_names);
        formatter.field("instance_initiated_shutdown_behavior", &self.instance_initiated_shutdown_behavior);
        formatter.field("instance_type", &self.instance_type);
        formatter.field("monitoring", &self.monitoring);
        formatter.field("placement", &self.placement);
        formatter.field("private_ip_address", &self.private_ip_address);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("user_data", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
