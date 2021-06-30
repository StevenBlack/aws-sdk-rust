// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_create_health_check_create_health_check_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_hosted_zone_create_hosted_zone_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_key_signing_key_create_key_signing_key_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_query_logging_config_create_query_logging_config_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_reusable_delegation_set_create_reusable_delegation_set_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_create_traffic_policy_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_instance_create_traffic_policy_instance_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_version_create_traffic_policy_version_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}
