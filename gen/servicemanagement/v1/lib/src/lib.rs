#![doc = "# Resources and Methods\n* [operations](resources/operations/struct.OperationsActions.html)\n  * [*get*](resources/operations/struct.GetRequestBuilder.html), [*list*](resources/operations/struct.ListRequestBuilder.html)\n* [services](resources/services/struct.ServicesActions.html)\n  * [*create*](resources/services/struct.CreateRequestBuilder.html), [*delete*](resources/services/struct.DeleteRequestBuilder.html), [*generateConfigReport*](resources/services/struct.GenerateConfigReportRequestBuilder.html), [*get*](resources/services/struct.GetRequestBuilder.html), [*getConfig*](resources/services/struct.GetConfigRequestBuilder.html), [*getIamPolicy*](resources/services/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/services/struct.ListRequestBuilder.html), [*setIamPolicy*](resources/services/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/services/struct.TestIamPermissionsRequestBuilder.html), [*undelete*](resources/services/struct.UndeleteRequestBuilder.html)\n  * [configs](resources/services/configs/struct.ConfigsActions.html)\n    * [*create*](resources/services/configs/struct.CreateRequestBuilder.html), [*get*](resources/services/configs/struct.GetRequestBuilder.html), [*list*](resources/services/configs/struct.ListRequestBuilder.html), [*submit*](resources/services/configs/struct.SubmitRequestBuilder.html)\n  * [consumers](resources/services/consumers/struct.ConsumersActions.html)\n    * [*getIamPolicy*](resources/services/consumers/struct.GetIamPolicyRequestBuilder.html), [*setIamPolicy*](resources/services/consumers/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/services/consumers/struct.TestIamPermissionsRequestBuilder.html)\n  * [rollouts](resources/services/rollouts/struct.RolloutsActions.html)\n    * [*create*](resources/services/rollouts/struct.CreateRequestBuilder.html), [*get*](resources/services/rollouts/struct.GetRequestBuilder.html), [*list*](resources/services/rollouts/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud services and see the email address of your Google Account\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
    #[doc = "Manage your Google API service configuration\n\n`https://www.googleapis.com/auth/service.management`"]
    pub const SERVICE_MANAGEMENT: &str = "https://www.googleapis.com/auth/service.management";
    #[doc = "View your Google API service configuration\n\n`https://www.googleapis.com/auth/service.management.readonly`"]
    pub const SERVICE_MANAGEMENT_READONLY: &str =
        "https://www.googleapis.com/auth/service.management.readonly";
}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Advice {
        #[doc = "Useful description for why this advice was applied and what actions should be taken to mitigate any implied risks."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Advice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Advice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Api {
        #[doc = "The methods of this interface, in unspecified order."]
        #[serde(
            rename = "methods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub methods: ::std::option::Option<Vec<crate::schemas::Method>>,
        #[doc = "Included interfaces. See Mixin."]
        #[serde(
            rename = "mixins",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mixins: ::std::option::Option<Vec<crate::schemas::Mixin>>,
        #[doc = "The fully qualified name of this interface, including package name followed by the interface's simple name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the interface."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Source context for the protocol buffer service represented by this message."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax of the service."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::ApiSyntax>,
        #[doc = "A version string for this interface. If specified, must have the form `major-version.minor-version`, as in `1.10`. If the minor version is omitted, it defaults to zero. If the entire version field is empty, the major version is derived from the package name, as outlined below. If the field is not empty, the version in the package name will be verified to be consistent with what is provided here. The versioning schema uses [semantic versioning](http://semver.org) where the major version number indicates a breaking change and the minor version an additive, non-breaking change. Both version numbers are signals to users what to expect from different versions, and should be carefully chosen based on the product plan. The major version is also reflected in the package name of the interface, which must end in `v`, as in `google.feature.v1`. For major versions 0 and 1, the suffix can be omitted. Zero major versions must only be used for experimental, non-GA interfaces. "]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Api {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Api {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ApiSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                ApiSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => ApiSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => ApiSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => ApiSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => ApiSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs: ::std::option::Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataRead => "DATA_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfigLogType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuthProvider {
        #[doc = "The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, JWTs with audiences: - \"https://[service.name]/[google.protobuf.Api.name]\" - \"https://[service.name]/\" will be accepted. For example, if no audiences are in the setting, LibraryService API will accept JWTs with the following audiences: - https://library-example.googleapis.com/google.example.library.v1.LibraryService - https://library-example.googleapis.com/ Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
        #[serde(
            rename = "audiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audiences: ::std::option::Option<String>,
        #[doc = "Redirect URL if JWT token is required but not present or is expired. Implement authorizationUrl of securityDefinitions in OpenAPI spec."]
        #[serde(
            rename = "authorizationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorization_url: ::std::option::Option<String>,
        #[doc = "The unique identifier of the auth provider. It will be referred to by `AuthRequirement.provider_id`. Example: \"bookstore_auth\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies the principal that issued the JWT. See https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1 Usually a URL or an email address. Example: https://securetoken.google.com Example: 1234567-compute@developer.gserviceaccount.com"]
        #[serde(
            rename = "issuer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub issuer: ::std::option::Option<String>,
        #[doc = "URL of the provider's public key set to validate signature of the JWT. See [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata). Optional if the key set document: - can be retrieved from [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html) of the issuer. - can be inferred from the email domain of the issuer (e.g. a Google service account). Example: https://www.googleapis.com/oauth2/v1/certs"]
        #[serde(
            rename = "jwksUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwks_uri: ::std::option::Option<String>,
        #[doc = "Defines the locations to extract the JWT. For now it is only used by the Cloud Endpoints to store the OpenAPI extension [x-google-jwt-locations] (https://cloud.google.com/endpoints/docs/openapi/openapi-extensions#x-google-jwt-locations) JWT locations can be one of HTTP headers, URL query parameters or cookies. The rule is that the first match wins. If not specified, default to use following 3 locations: 1) Authorization: Bearer 2) x-goog-iap-jwt-assertion 3) access_token query parameter Default locations can be specified as followings: jwt_locations: - header: Authorization value_prefix: \"Bearer \" - header: x-goog-iap-jwt-assertion - query: access_token"]
        #[serde(
            rename = "jwtLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwt_locations: ::std::option::Option<Vec<crate::schemas::JwtLocation>>,
    }
    impl ::google_field_selector::FieldSelector for AuthProvider {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthProvider {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuthRequirement {
        #[doc = "NOTE: This will be deprecated soon, once AuthProvider.audiences is implemented and accepted in all the runtime components. The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, only JWTs with audience \"https://Service_name/API_name\" will be accepted. For example, if no audiences are in the setting, LibraryService API will only accept JWTs with the following audience \"https://library-example.googleapis.com/google.example.library.v1.LibraryService\". Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
        #[serde(
            rename = "audiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audiences: ::std::option::Option<String>,
        #[doc = "id from authentication provider. Example: provider_id: bookstore_auth"]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuthRequirement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthRequirement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Authentication {
        #[doc = "Defines a set of authentication providers that a service supports."]
        #[serde(
            rename = "providers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub providers: ::std::option::Option<Vec<crate::schemas::AuthProvider>>,
        #[doc = "A list of authentication rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::AuthenticationRule>>,
    }
    impl ::google_field_selector::FieldSelector for Authentication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Authentication {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuthenticationRule {
        #[doc = "If true, the service accepts API keys without any other credential. This flag only applies to HTTP and gRPC requests."]
        #[serde(
            rename = "allowWithoutCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_without_credential: ::std::option::Option<bool>,
        #[doc = "The requirements for OAuth credentials."]
        #[serde(
            rename = "oauth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth: ::std::option::Option<crate::schemas::OauthRequirements>,
        #[doc = "Requirements for additional authentication providers."]
        #[serde(
            rename = "requirements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requirements: ::std::option::Option<Vec<crate::schemas::AuthRequirement>>,
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuthenticationRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthenticationRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Backend {
        #[doc = "A list of API backend rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::BackendRule>>,
    }
    impl ::google_field_selector::FieldSelector for Backend {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Backend {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BackendRule {
        #[doc = "The address of the API backend. The scheme is used to determine the backend protocol and security. The following schemes are accepted: SCHEME PROTOCOL SECURITY http:// HTTP None https:// HTTP TLS grpc:// gRPC None grpcs:// gRPC TLS It is recommended to explicitly include a scheme. Leaving out the scheme may cause constrasting behaviors across platforms. If the port is unspecified, the default is: - 80 for schemes without TLS - 443 for schemes with TLS For HTTP backends, use protocol to specify the protocol version."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "The number of seconds to wait for a response from a request. The default varies based on the request protocol and deployment environment."]
        #[serde(
            rename = "deadline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deadline: ::std::option::Option<f64>,
        #[doc = "When disable_auth is true, a JWT ID token won't be generated and the original \"Authorization\" HTTP header will be preserved. If the header is used to carry the original token and is expected by the backend, this field must be set to true to preserve the header."]
        #[serde(
            rename = "disableAuth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_auth: ::std::option::Option<bool>,
        #[doc = "The JWT audience is used when generating a JWT ID token for the backend. This ID token will be added in the HTTP \"authorization\" header, and sent to the backend."]
        #[serde(
            rename = "jwtAudience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwt_audience: ::std::option::Option<String>,
        #[doc = "The number of seconds to wait for the completion of a long running operation. The default is no deadline."]
        #[serde(
            rename = "operationDeadline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_deadline: ::std::option::Option<f64>,
        #[serde(
            rename = "pathTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_translation: ::std::option::Option<crate::schemas::BackendRulePathTranslation>,
        #[doc = "The protocol used for sending a request to the backend. The supported values are \"http/1.1\" and \"h2\". The default value is inferred from the scheme in the address field: SCHEME PROTOCOL http:// http/1.1 https:// http/1.1 grpc:// h2 grpcs:// h2 For secure HTTP backends (https://) that support HTTP/2, set this field to \"h2\" for improved performance. Configuring this field to non-default values is only supported for secure HTTP backends. This field will be ignored for all other backends. See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for more details on the supported values."]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BackendRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackendRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BackendRulePathTranslation {
        #[doc = "The request path will be appended to the backend address. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.appspot.com Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST"]
        AppendPathToAddress,
        #[doc = "Use the backend address as-is, with no modification to the path. If the URL pattern contains variables, the variable names and values will be appended to the query string. If a query string parameter and a URL pattern variable have the same name, this may result in duplicate keys in the query string. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.cloudfunctions.net/getUser Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe"]
        ConstantAddress,
        PathTranslationUnspecified,
    }
    impl BackendRulePathTranslation {
        pub fn as_str(self) -> &'static str {
            match self {
                BackendRulePathTranslation::AppendPathToAddress => "APPEND_PATH_TO_ADDRESS",
                BackendRulePathTranslation::ConstantAddress => "CONSTANT_ADDRESS",
                BackendRulePathTranslation::PathTranslationUnspecified => {
                    "PATH_TRANSLATION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BackendRulePathTranslation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BackendRulePathTranslation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BackendRulePathTranslation, ()> {
            Ok(match s {
                "APPEND_PATH_TO_ADDRESS" => BackendRulePathTranslation::AppendPathToAddress,
                "CONSTANT_ADDRESS" => BackendRulePathTranslation::ConstantAddress,
                "PATH_TRANSLATION_UNSPECIFIED" => {
                    BackendRulePathTranslation::PathTranslationUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BackendRulePathTranslation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BackendRulePathTranslation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BackendRulePathTranslation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPEND_PATH_TO_ADDRESS" => BackendRulePathTranslation::AppendPathToAddress,
                "CONSTANT_ADDRESS" => BackendRulePathTranslation::ConstantAddress,
                "PATH_TRANSLATION_UNSPECIFIED" => {
                    BackendRulePathTranslation::PathTranslationUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BackendRulePathTranslation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackendRulePathTranslation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Billing {
        #[doc = "Billing configurations for sending metrics to the consumer project. There can be multiple consumer destinations per service, each one must have a different monitored resource type. A metric can be used in at most one consumer destination."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::BillingDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Billing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Billing {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BillingDestination {
        #[doc = "Names of the metrics to report to this billing destination. Each name must be defined in Service.metrics section."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BillingDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BillingDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Binding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ChangeReport {
        #[doc = "List of changes between two service configurations. The changes will be alphabetically sorted based on the identifier of each change. A ConfigChange identifier is a dot separated path to the configuration. Example: visibility.rules[selector='LibraryService.CreateBook'].restriction"]
        #[serde(
            rename = "configChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_changes: ::std::option::Option<Vec<crate::schemas::ConfigChange>>,
    }
    impl ::google_field_selector::FieldSelector for ChangeReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChangeReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigChange {
        #[doc = "Collection of advice provided for this change, useful for determining the possible impact of this change."]
        #[serde(
            rename = "advices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advices: ::std::option::Option<Vec<crate::schemas::Advice>>,
        #[doc = "The type for this change, either ADDED, REMOVED, or MODIFIED."]
        #[serde(
            rename = "changeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub change_type: ::std::option::Option<crate::schemas::ConfigChangeChangeType>,
        #[doc = "Object hierarchy path to the change, with levels separated by a '.' character. For repeated fields, an applicable unique identifier field is used for the index (usually selector, name, or id). For maps, the term 'key' is used. If the field has no unique identifier, the numeric index is used. Examples: - visibility.rules[selector==\"google.LibraryService.ListBooks\"].restriction - quota.metric_rules[selector==\"google\"].metric_costs[key==\"reads\"].value - logging.producer_destinations[0]"]
        #[serde(
            rename = "element",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element: ::std::option::Option<String>,
        #[doc = "Value of the changed object in the new Service configuration, in JSON format. This field will not be populated if ChangeType == REMOVED."]
        #[serde(
            rename = "newValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_value: ::std::option::Option<String>,
        #[doc = "Value of the changed object in the old Service configuration, in JSON format. This field will not be populated if ChangeType == ADDED."]
        #[serde(
            rename = "oldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigChange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigChangeChangeType {
        #[doc = "The changed object exists in the 'new' service configuration, but not in the 'old' service configuration."]
        Added,
        #[doc = "No value was provided."]
        ChangeTypeUnspecified,
        #[doc = "The changed object exists in both service configurations, but its value is different."]
        Modified,
        #[doc = "The changed object exists in the 'old' service configuration, but not in the 'new' service configuration."]
        Removed,
    }
    impl ConfigChangeChangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigChangeChangeType::Added => "ADDED",
                ConfigChangeChangeType::ChangeTypeUnspecified => "CHANGE_TYPE_UNSPECIFIED",
                ConfigChangeChangeType::Modified => "MODIFIED",
                ConfigChangeChangeType::Removed => "REMOVED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigChangeChangeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigChangeChangeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConfigChangeChangeType, ()> {
            Ok(match s {
                "ADDED" => ConfigChangeChangeType::Added,
                "CHANGE_TYPE_UNSPECIFIED" => ConfigChangeChangeType::ChangeTypeUnspecified,
                "MODIFIED" => ConfigChangeChangeType::Modified,
                "REMOVED" => ConfigChangeChangeType::Removed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigChangeChangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigChangeChangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigChangeChangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED" => ConfigChangeChangeType::Added,
                "CHANGE_TYPE_UNSPECIFIED" => ConfigChangeChangeType::ChangeTypeUnspecified,
                "MODIFIED" => ConfigChangeChangeType::Modified,
                "REMOVED" => ConfigChangeChangeType::Removed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigChangeChangeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigChangeChangeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigFile {
        #[doc = "The bytes that constitute the file."]
        #[serde(
            rename = "fileContents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_contents: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The file name of the configuration file (full or relative path)."]
        #[serde(
            rename = "filePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_path: ::std::option::Option<String>,
        #[doc = "The type of configuration file this represents."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<crate::schemas::ConfigFileFileType>,
    }
    impl ::google_field_selector::FieldSelector for ConfigFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigFileFileType {
        #[doc = "FileDescriptorSet, generated by protoc. To generate, use protoc with imports and source info included. For an example test.proto file, the following command would put the value in a new file named out.pb. $protoc --include_imports --include_source_info test.proto -o out.pb"]
        FileDescriptorSetProto,
        #[doc = "Unknown file type."]
        FileTypeUnspecified,
        #[doc = "OpenAPI specification, serialized in JSON."]
        OpenApiJson,
        #[doc = "OpenAPI specification, serialized in YAML."]
        OpenApiYaml,
        #[doc = "Uncompiled Proto file. Used for storage and display purposes only, currently server-side compilation is not supported. Should match the inputs to 'protoc' command used to generated FILE_DESCRIPTOR_SET_PROTO. A file of this type can only be included if at least one file of type FILE_DESCRIPTOR_SET_PROTO is included."]
        ProtoFile,
        #[doc = "YAML-specification of service."]
        ServiceConfigYaml,
    }
    impl ConfigFileFileType {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigFileFileType::FileDescriptorSetProto => "FILE_DESCRIPTOR_SET_PROTO",
                ConfigFileFileType::FileTypeUnspecified => "FILE_TYPE_UNSPECIFIED",
                ConfigFileFileType::OpenApiJson => "OPEN_API_JSON",
                ConfigFileFileType::OpenApiYaml => "OPEN_API_YAML",
                ConfigFileFileType::ProtoFile => "PROTO_FILE",
                ConfigFileFileType::ServiceConfigYaml => "SERVICE_CONFIG_YAML",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigFileFileType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigFileFileType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConfigFileFileType, ()> {
            Ok(match s {
                "FILE_DESCRIPTOR_SET_PROTO" => ConfigFileFileType::FileDescriptorSetProto,
                "FILE_TYPE_UNSPECIFIED" => ConfigFileFileType::FileTypeUnspecified,
                "OPEN_API_JSON" => ConfigFileFileType::OpenApiJson,
                "OPEN_API_YAML" => ConfigFileFileType::OpenApiYaml,
                "PROTO_FILE" => ConfigFileFileType::ProtoFile,
                "SERVICE_CONFIG_YAML" => ConfigFileFileType::ServiceConfigYaml,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigFileFileType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigFileFileType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigFileFileType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FILE_DESCRIPTOR_SET_PROTO" => ConfigFileFileType::FileDescriptorSetProto,
                "FILE_TYPE_UNSPECIFIED" => ConfigFileFileType::FileTypeUnspecified,
                "OPEN_API_JSON" => ConfigFileFileType::OpenApiJson,
                "OPEN_API_YAML" => ConfigFileFileType::OpenApiYaml,
                "PROTO_FILE" => ConfigFileFileType::ProtoFile,
                "SERVICE_CONFIG_YAML" => ConfigFileFileType::ServiceConfigYaml,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigFileFileType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigFileFileType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigRef {
        #[doc = "Resource name of a service config. It must have the following format: \"services/{service name}/configs/{config id}\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigSource {
        #[doc = "Set of source configuration files that are used to generate a service configuration (`google.api.Service`)."]
        #[serde(
            rename = "files",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files: ::std::option::Option<Vec<crate::schemas::ConfigFile>>,
        #[doc = "A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. If empty, the server may choose to generate one instead."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Context {
        #[doc = "A list of RPC context rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::ContextRule>>,
    }
    impl ::google_field_selector::FieldSelector for Context {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Context {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContextRule {
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from client to backend."]
        #[serde(
            rename = "allowedRequestExtensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_request_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from backend to client."]
        #[serde(
            rename = "allowedResponseExtensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_response_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of provided contexts."]
        #[serde(
            rename = "provided",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provided: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of requested contexts."]
        #[serde(
            rename = "requested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested: ::std::option::Option<Vec<String>>,
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContextRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContextRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Control {
        #[doc = "The service controller environment to use. If empty, no control plane feature (like quota and billing) will be enabled. The recommended value for most services is servicecontrol.googleapis.com"]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Control {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Control {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CustomError {
        #[doc = "The list of custom error rules that apply to individual API messages. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::CustomErrorRule>>,
        #[doc = "The list of custom error detail types, e.g. 'google.foo.v1.CustomError'."]
        #[serde(
            rename = "types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CustomError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CustomErrorRule {
        #[doc = "Mark this message as possible payload in error response. Otherwise, objects of this type will be filtered when they appear in error payload."]
        #[serde(
            rename = "isErrorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_error_type: ::std::option::Option<bool>,
        #[doc = "Selects messages to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomErrorRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomErrorRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CustomHttpPattern {
        #[doc = "The name of this custom HTTP verb."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The path matched by this custom verb."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomHttpPattern {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomHttpPattern {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteServiceStrategy {}
    impl ::google_field_selector::FieldSelector for DeleteServiceStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteServiceStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Diagnostic {
        #[doc = "The kind of diagnostic information provided."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::DiagnosticKind>,
        #[doc = "File name and line number of the error or warning."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Message describing the error or warning."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Diagnostic {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Diagnostic {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiagnosticKind {
        #[doc = "Only errors"]
        Error,
        #[doc = "Warnings and errors"]
        Warning,
    }
    impl DiagnosticKind {
        pub fn as_str(self) -> &'static str {
            match self {
                DiagnosticKind::Error => "ERROR",
                DiagnosticKind::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiagnosticKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiagnosticKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiagnosticKind, ()> {
            Ok(match s {
                "ERROR" => DiagnosticKind::Error,
                "WARNING" => DiagnosticKind::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiagnosticKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiagnosticKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiagnosticKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => DiagnosticKind::Error,
                "WARNING" => DiagnosticKind::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiagnosticKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiagnosticKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Documentation {
        #[doc = "The URL to the root of documentation."]
        #[serde(
            rename = "documentationRootUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_root_url: ::std::option::Option<String>,
        #[doc = "Declares a single overview page. For example: documentation: summary: ... overview: (== include overview.md ==) This is a shortcut for the following declaration (using pages style): documentation: summary: ... pages: - name: Overview content: (== include overview.md ==) Note: you cannot specify both `overview` field and `pages` field."]
        #[serde(
            rename = "overview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overview: ::std::option::Option<String>,
        #[doc = "The top level pages for the documentation set."]
        #[serde(
            rename = "pages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "A list of documentation rules that apply to individual API elements. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::DocumentationRule>>,
        #[doc = "Specifies the service root url if the default one (the service name from the yaml file) is not suitable. This can be seen in any fully specified service urls as well as sections that show a base that other urls are relative to."]
        #[serde(
            rename = "serviceRootUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_root_url: ::std::option::Option<String>,
        #[doc = "A short description of what the service does. The summary must be plain text. It becomes the overview of the service displayed in Google Cloud Console. NOTE: This field is equivalent to the standard field `description`."]
        #[serde(
            rename = "summary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Documentation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Documentation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DocumentationRule {
        #[doc = "Deprecation description of the selected element(s). It can be provided if an element is marked as `deprecated`."]
        #[serde(
            rename = "deprecationDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecation_description: ::std::option::Option<String>,
        #[doc = "Description of the selected proto element (e.g. a message, a method, a 'service' definition, or a field). Defaults to leading & trailing comments taken from the proto source definition of the proto element."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The selector is a comma-separated list of patterns for any element such as a method, a field, an enum value. Each pattern is a qualified name of the element which may end in \"*\", indicating a wildcard. Wildcards are only allowed at the end and for a whole component of the qualified name, i.e. \"foo.*\" is ok, but not \"foo.b*\" or \"foo.*.bar\". A wildcard will match one or more components. To specify a default for all applicable elements, the whole pattern \"*\" is used."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DocumentationRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentationRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Endpoint {
        #[doc = "Allowing [CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka cross-domain traffic, would allow the backends served from this endpoint to receive and respond to HTTP OPTIONS requests. The response will be used by the browser to determine whether the subsequent cross-origin request is allowed to proceed."]
        #[serde(
            rename = "allowCors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_cors: ::std::option::Option<bool>,
        #[doc = "The canonical name of this endpoint."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The specification of an Internet routable address of API frontend that will handle requests to this [API Endpoint](https://cloud.google.com/apis/design/glossary). It should be either a valid IPv4 address or a fully-qualified domain name. For example, \"8.8.8.8\" or \"myservice.appspot.com\"."]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Endpoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Endpoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Enum {
        #[doc = "Enum value definitions."]
        #[serde(
            rename = "enumvalue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enumvalue: ::std::option::Option<Vec<crate::schemas::EnumValue>>,
        #[doc = "Enum type name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::EnumSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Enum {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Enum {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnumSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl EnumSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                EnumSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnumSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnumSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnumSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => EnumSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => EnumSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnumSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => EnumSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => EnumSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnumSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct EnumValue {
        #[doc = "Enum value name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Enum value number."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "Protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
    }
    impl ::google_field_selector::FieldSelector for EnumValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Expr {
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Expr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Expr {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Field {
        #[doc = "The field cardinality."]
        #[serde(
            rename = "cardinality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cardinality: ::std::option::Option<crate::schemas::FieldCardinality>,
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_value: ::std::option::Option<String>,
        #[doc = "The field JSON name."]
        #[serde(
            rename = "jsonName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_name: ::std::option::Option<String>,
        #[doc = "The field type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::FieldKind>,
        #[doc = "The field name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The field number."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "The index of the field type in `Type.oneofs`, for message or enumeration types. The first type has index 1; zero means the type is not in the list."]
        #[serde(
            rename = "oneofIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneof_index: ::std::option::Option<i32>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Whether to use alternative packed wire representation."]
        #[serde(
            rename = "packed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packed: ::std::option::Option<bool>,
        #[doc = "The field type URL, without the scheme, for message or enumeration types. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`."]
        #[serde(
            rename = "typeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Field {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Field {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldCardinality {
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[doc = "For repeated fields."]
        CardinalityRepeated,
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
    }
    impl FieldCardinality {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldCardinality::CardinalityOptional => "CARDINALITY_OPTIONAL",
                FieldCardinality::CardinalityRepeated => "CARDINALITY_REPEATED",
                FieldCardinality::CardinalityRequired => "CARDINALITY_REQUIRED",
                FieldCardinality::CardinalityUnknown => "CARDINALITY_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldCardinality {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldCardinality {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldCardinality, ()> {
            Ok(match s {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldCardinality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldCardinality {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldCardinality {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldCardinality {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldCardinality {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldKind {
        #[doc = "Field type bool."]
        TypeBool,
        #[doc = "Field type bytes."]
        TypeBytes,
        #[doc = "Field type double."]
        TypeDouble,
        #[doc = "Field type enum."]
        TypeEnum,
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[doc = "Field type float."]
        TypeFloat,
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[doc = "Field type int32."]
        TypeInt32,
        #[doc = "Field type int64."]
        TypeInt64,
        #[doc = "Field type message."]
        TypeMessage,
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[doc = "Field type sint32."]
        TypeSint32,
        #[doc = "Field type sint64."]
        TypeSint64,
        #[doc = "Field type string."]
        TypeString,
        #[doc = "Field type uint32."]
        TypeUint32,
        #[doc = "Field type uint64."]
        TypeUint64,
        #[doc = "Field type unknown."]
        TypeUnknown,
    }
    impl FieldKind {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldKind::TypeBool => "TYPE_BOOL",
                FieldKind::TypeBytes => "TYPE_BYTES",
                FieldKind::TypeDouble => "TYPE_DOUBLE",
                FieldKind::TypeEnum => "TYPE_ENUM",
                FieldKind::TypeFixed32 => "TYPE_FIXED32",
                FieldKind::TypeFixed64 => "TYPE_FIXED64",
                FieldKind::TypeFloat => "TYPE_FLOAT",
                FieldKind::TypeGroup => "TYPE_GROUP",
                FieldKind::TypeInt32 => "TYPE_INT32",
                FieldKind::TypeInt64 => "TYPE_INT64",
                FieldKind::TypeMessage => "TYPE_MESSAGE",
                FieldKind::TypeSfixed32 => "TYPE_SFIXED32",
                FieldKind::TypeSfixed64 => "TYPE_SFIXED64",
                FieldKind::TypeSint32 => "TYPE_SINT32",
                FieldKind::TypeSint64 => "TYPE_SINT64",
                FieldKind::TypeString => "TYPE_STRING",
                FieldKind::TypeUint32 => "TYPE_UINT32",
                FieldKind::TypeUint64 => "TYPE_UINT64",
                FieldKind::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldKind, ()> {
            Ok(match s {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FlowErrorDetails {
        #[doc = "The type of exception (as a class name)."]
        #[serde(
            rename = "exceptionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exception_type: ::std::option::Option<String>,
        #[doc = "The step that failed."]
        #[serde(
            rename = "flowStepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flow_step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FlowErrorDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FlowErrorDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GenerateConfigReportRequest {
        #[doc = "Required. Service configuration for which we want to generate the report. For this version of API, the supported types are google.api.servicemanagement.v1.ConfigRef, google.api.servicemanagement.v1.ConfigSource, and google.api.Service"]
        #[serde(
            rename = "newConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. Service configuration against which the comparison will be done. For this version of API, the supported types are google.api.servicemanagement.v1.ConfigRef, google.api.servicemanagement.v1.ConfigSource, and google.api.Service"]
        #[serde(
            rename = "oldConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GenerateConfigReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateConfigReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GenerateConfigReportResponse {
        #[doc = "list of ChangeReport, each corresponding to comparison between two service configurations."]
        #[serde(
            rename = "changeReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub change_reports: ::std::option::Option<Vec<crate::schemas::ChangeReport>>,
        #[doc = "Errors / Linter warnings associated with the service definition this report belongs to."]
        #[serde(
            rename = "diagnostics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub diagnostics: ::std::option::Option<Vec<crate::schemas::Diagnostic>>,
        #[doc = "ID of the service configuration this report belongs to."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Name of the service this report belongs to."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GenerateConfigReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateConfigReportResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetIamPolicyRequest {
        #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::GetPolicyOptions>,
    }
    impl ::google_field_selector::FieldSelector for GetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIamPolicyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetPolicyOptions {
        #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "requestedPolicyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_policy_version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GetPolicyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetPolicyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Http {
        #[doc = "When set to true, URL path parameters will be fully URI-decoded except in cases of single segment matches in reserved expansion, where \"%2F\" will be left encoded. The default behavior is to not decode RFC 6570 reserved characters in multi segment matches."]
        #[serde(
            rename = "fullyDecodeReservedExpansion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_decode_reserved_expansion: ::std::option::Option<bool>,
        #[doc = "A list of HTTP configuration rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
    }
    impl ::google_field_selector::FieldSelector for Http {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Http {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct HttpRule {
        #[doc = "Additional HTTP bindings for the selector. Nested bindings must not contain an `additional_bindings` field themselves (that is, the nesting may only be one level deep)."]
        #[serde(
            rename = "additionalBindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_bindings: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
        #[doc = "The name of the request field whose value is mapped to the HTTP request body, or `*` for mapping all request fields not captured by the path pattern to the HTTP body, or omitted for not having any HTTP request body. NOTE: the referred field must be present at the top-level of the request message type."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "The custom pattern is used for specifying an HTTP method that is not included in the `pattern` field, such as HEAD, or \"*\" to leave the HTTP method unspecified for this rule. The wild-card rule is useful for services that provide content to Web (HTML) clients."]
        #[serde(
            rename = "custom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom: ::std::option::Option<crate::schemas::CustomHttpPattern>,
        #[doc = "Maps to HTTP DELETE. Used for deleting a resource."]
        #[serde(
            rename = "delete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete: ::std::option::Option<String>,
        #[doc = "Maps to HTTP GET. Used for listing and getting information about resources."]
        #[serde(
            rename = "get",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub get: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PATCH. Used for updating a resource."]
        #[serde(
            rename = "patch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch: ::std::option::Option<String>,
        #[doc = "Maps to HTTP POST. Used for creating a resource or performing an action."]
        #[serde(
            rename = "post",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PUT. Used for replacing a resource."]
        #[serde(
            rename = "put",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub put: ::std::option::Option<String>,
        #[doc = "Optional. The name of the response field whose value is mapped to the HTTP response body. When omitted, the entire response message will be used as the HTTP response body. NOTE: The referred field must be present at the top-level of the response message type."]
        #[serde(
            rename = "responseBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_body: ::std::option::Option<String>,
        #[doc = "Selects a method to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HttpRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct JwtLocation {
        #[doc = "Specifies cookie name to extract JWT token."]
        #[serde(
            rename = "cookie",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie: ::std::option::Option<String>,
        #[doc = "Specifies HTTP header name to extract JWT token."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "Specifies URL query parameter name to extract JWT token."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "The value prefix. The value format is \"value_prefix{token}\" Only applies to \"in\" header type. Must be empty for \"in\" query type. If not empty, the header value has to match (case sensitive) this prefix. If not matched, JWT will not be extracted. If matched, JWT will be extracted after the prefix is removed. For example, for \"Authorization: Bearer {JWT}\", value_prefix=\"Bearer \" with a space at the end."]
        #[serde(
            rename = "valuePrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JwtLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JwtLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LabelDescriptor {
        #[doc = "A human-readable description for the label."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The label key."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The type of data that can be assigned to the label."]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<crate::schemas::LabelDescriptorValueType>,
    }
    impl ::google_field_selector::FieldSelector for LabelDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LabelDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LabelDescriptorValueType {
        #[doc = "Boolean; true or false."]
        Bool,
        #[doc = "A 64-bit signed integer."]
        Int64,
        #[doc = "A variable-length string. This is the default."]
        String,
    }
    impl LabelDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                LabelDescriptorValueType::Bool => "BOOL",
                LabelDescriptorValueType::Int64 => "INT64",
                LabelDescriptorValueType::String => "STRING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LabelDescriptorValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LabelDescriptorValueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LabelDescriptorValueType, ()> {
            Ok(match s {
                "BOOL" => LabelDescriptorValueType::Bool,
                "INT64" => LabelDescriptorValueType::Int64,
                "STRING" => LabelDescriptorValueType::String,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LabelDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LabelDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LabelDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => LabelDescriptorValueType::Bool,
                "INT64" => LabelDescriptorValueType::Int64,
                "STRING" => LabelDescriptorValueType::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LabelDescriptorValueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LabelDescriptorValueType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListOperationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListServiceConfigsResponse {
        #[doc = "The token of the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of service configuration resources."]
        #[serde(
            rename = "serviceConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_configs: ::std::option::Option<Vec<crate::schemas::Service>>,
    }
    impl ::google_field_selector::FieldSelector for ListServiceConfigsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListServiceConfigsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListServiceConfigsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListServiceRolloutsResponse {
        #[doc = "The token of the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of rollout resources."]
        #[serde(
            rename = "rollouts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollouts: ::std::option::Option<Vec<crate::schemas::Rollout>>,
    }
    impl ::google_field_selector::FieldSelector for ListServiceRolloutsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListServiceRolloutsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListServiceRolloutsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListServicesResponse {
        #[doc = "Token that can be passed to `ListServices` to resume a paginated query."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The returned services will only have the name field set."]
        #[serde(
            rename = "services",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services: ::std::option::Option<Vec<crate::schemas::ManagedService>>,
    }
    impl ::google_field_selector::FieldSelector for ListServicesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListServicesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListServicesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LogDescriptor {
        #[doc = "A human-readable description of this log. This information appears in the documentation and can contain details."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name for this log. This information appears on the user interface and should be concise."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that are available to describe a specific log entry. Runtime requests that contain labels not specified here are considered invalid."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "The name of the log. It must be less than 512 characters long and can include the following characters: upper- and lower-case alphanumeric characters [A-Za-z0-9], and punctuation characters including slash, underscore, hyphen, period [/_-.]."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LogDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LogDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Logging {
        #[doc = "Logging configurations for sending logs to the consumer project. There can be multiple consumer destinations, each one must have a different monitored resource type. A log can be used in at most one consumer destination."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
        #[doc = "Logging configurations for sending logs to the producer project. There can be multiple producer destinations, each one must have a different monitored resource type. A log can be used in at most one producer destination."]
        #[serde(
            rename = "producerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Logging {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Logging {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LoggingDestination {
        #[doc = "Names of the logs to be sent to this destination. Each name must be defined in the Service.logs section. If the log name is not a domain scoped name, it will be automatically prefixed with the service name followed by \"/\"."]
        #[serde(
            rename = "logs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in the Service.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LoggingDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LoggingDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ManagedService {
        #[doc = "ID of the project that produces and owns this service."]
        #[serde(
            rename = "producerProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_project_id: ::std::option::Option<String>,
        #[doc = "The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedService {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedService {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Method {
        #[doc = "The simple name of this method."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the method."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "If true, the request is streamed."]
        #[serde(
            rename = "requestStreaming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_streaming: ::std::option::Option<bool>,
        #[doc = "A URL of the input message type."]
        #[serde(
            rename = "requestTypeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_type_url: ::std::option::Option<String>,
        #[doc = "If true, the response is streamed."]
        #[serde(
            rename = "responseStreaming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_streaming: ::std::option::Option<bool>,
        #[doc = "The URL of the output message type."]
        #[serde(
            rename = "responseTypeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_type_url: ::std::option::Option<String>,
        #[doc = "The source syntax of this method."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::MethodSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Method {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Method {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MethodSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl MethodSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                MethodSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                MethodSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MethodSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MethodSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MethodSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => MethodSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => MethodSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MethodSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MethodSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MethodSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => MethodSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => MethodSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MethodSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MethodSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricDescriptor {
        #[doc = "A detailed description of the metric, which can be used in documentation."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that can be used to describe a specific instance of this metric type. For example, the `appengine.googleapis.com/http/server/response_latencies` metric type has a label for the HTTP response code, `response_code`, so you can look at latencies for successful responses or just for responses that failed."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the metric definition."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage: ::std::option::Option<crate::schemas::MetricDescriptorLaunchStage>,
        #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::MetricDescriptorMetadata>,
        #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(
            rename = "metricKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_kind: ::std::option::Option<crate::schemas::MetricDescriptorMetricKind>,
        #[doc = "Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here."]
        #[serde(
            rename = "monitoredResourceTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource_types: ::std::option::Option<Vec<String>>,
        #[doc = "The resource name of the metric descriptor."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name `custom.googleapis.com` or `external.googleapis.com`. Metric types should use a natural hierarchical grouping. For example: \"custom.googleapis.com/invoice/paid/amount\" \"external.googleapis.com/prometheus/up\" \"appengine.googleapis.com/http/server/response_latencies\""]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The units in which the metric value is reported. It is only applicable if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit` defines the representation of the stored metric values. Different systems might scale the values to be more easily displayed (so a value of `0.02kBy` *might* be displayed as `20By`, and a value of `3523kBy` *might* be displayed as `3.5MBy`). However, if the `unit` is `kBy`, then the value of the metric is always in thousands of bytes, no matter how it might be displayed. If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an `INT64 CUMULATIVE` metric whose `unit` is `s{CPU}` (or equivalently `1s{CPU}` or just `s`). If the job uses 12,005 CPU-seconds, then the value is written as `12005`. Alternatively, if you want a custom metric to record data in a more granular way, you can create a `DOUBLE CUMULATIVE` metric whose `unit` is `ks{CPU}`, and then write the value `12.005` (which is `12005/1000`), or use `Kis{CPU}` and write `11.723` (which is `12005/1024`). The supported units are a subset of [The Unified Code for Units of Measure](https://unitsofmeasure.org/ucum.html) standard: **Basic units (UNIT)** * `bit` bit * `By` byte * `s` second * `min` minute * `h` hour * `d` day * `1` dimensionless **Prefixes (PREFIX)** * `k` kilo (10^3) * `M` mega (10^6) * `G` giga (10^9) * `T` tera (10^12) * `P` peta (10^15) * `E` exa (10^18) * `Z` zetta (10^21) * `Y` yotta (10^24) * `m` milli (10^-3) * `u` micro (10^-6) * `n` nano (10^-9) * `p` pico (10^-12) * `f` femto (10^-15) * `a` atto (10^-18) * `z` zepto (10^-21) * `y` yocto (10^-24) * `Ki` kibi (2^10) * `Mi` mebi (2^20) * `Gi` gibi (2^30) * `Ti` tebi (2^40) * `Pi` pebi (2^50) **Grammar** The grammar also includes these connectors: * `/` division or ratio (as an infix operator). For examples, `kBy/{email}` or `MiBy/10ms` (although you should almost never have `/s` in a metric `unit`; rates should always be computed at query time from the underlying cumulative or delta value). * `.` multiplication or composition (as an infix operator). For examples, `GBy.d` or `k{watt}.h`. The grammar for a unit is as follows: Expression = Component { \".\" Component } { \"/\" Component } ; Component = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ] | Annotation | \"1\" ; Annotation = \"{\" NAME \"}\" ; Notes: * `Annotation` is just a comment if it follows a `UNIT`. If the annotation is used alone, then the unit is equivalent to `1`. For examples, `{request}/s == 1/s`, `By{transmitted}/s == By/s`. * `NAME` is a sequence of non-blank printable ASCII characters not containing `{` or `}`. * `1` represents a unitary [dimensionless unit](https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in `1/s`. It is typically used when none of the basic units are appropriate. For example, \"new users per day\" can be represented as `1/d` or `{new-users}/d` (and a metric value `5` would mean \"5 new users). Alternatively, \"thousands of page views per day\" would be represented as `1000/d` or `k1/d` or `k{page_views}/d` (and a metric value of `5.3` would mean \"5300 page views per day\"). * `%` represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value `3` means \"3 percent\"). * `10^2.%` indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value `0.03` means \"3 percent\")."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<crate::schemas::MetricDescriptorValueType>,
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the \"Deprecation Policy\" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MetricDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorLaunchStage::Alpha => "ALPHA",
                MetricDescriptorLaunchStage::Beta => "BETA",
                MetricDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorLaunchStage::Ga => "GA",
                MetricDescriptorLaunchStage::LaunchStageUnspecified => "LAUNCH_STAGE_UNSPECIFIED",
                MetricDescriptorLaunchStage::Prelaunch => "PRELAUNCH",
                MetricDescriptorLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MetricDescriptorLaunchStage::Alpha,
                "BETA" => MetricDescriptorLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => MetricDescriptorLaunchStage::LaunchStageUnspecified,
                "PRELAUNCH" => MetricDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorLaunchStage::Alpha,
                "BETA" => MetricDescriptorLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => MetricDescriptorLaunchStage::LaunchStageUnspecified,
                "PRELAUNCH" => MetricDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorLaunchStage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetricKind {
        #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        Cumulative,
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
    }
    impl MetricDescriptorMetricKind {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetricKind::Cumulative => "CUMULATIVE",
                MetricDescriptorMetricKind::Delta => "DELTA",
                MetricDescriptorMetricKind::Gauge => "GAUGE",
                MetricDescriptorMetricKind::MetricKindUnspecified => "METRIC_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorMetricKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorMetricKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorMetricKind, ()> {
            Ok(match s {
                "CUMULATIVE" => MetricDescriptorMetricKind::Cumulative,
                "DELTA" => MetricDescriptorMetricKind::Delta,
                "GAUGE" => MetricDescriptorMetricKind::Gauge,
                "METRIC_KIND_UNSPECIFIED" => MetricDescriptorMetricKind::MetricKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetricKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetricKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetricKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUMULATIVE" => MetricDescriptorMetricKind::Cumulative,
                "DELTA" => MetricDescriptorMetricKind::Delta,
                "GAUGE" => MetricDescriptorMetricKind::Gauge,
                "METRIC_KIND_UNSPECIFIED" => MetricDescriptorMetricKind::MetricKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetricKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetricKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorValueType {
        #[doc = "The value is a boolean. This value type can be used only if the metric kind is `GAUGE`."]
        Bool,
        #[doc = "The value is a `Distribution`."]
        Distribution,
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[doc = "The value is money."]
        Money,
        #[doc = "The value is a text string. This value type can be used only if the metric kind is `GAUGE`."]
        String,
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
    }
    impl MetricDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorValueType::Bool => "BOOL",
                MetricDescriptorValueType::Distribution => "DISTRIBUTION",
                MetricDescriptorValueType::Double => "DOUBLE",
                MetricDescriptorValueType::Int64 => "INT64",
                MetricDescriptorValueType::Money => "MONEY",
                MetricDescriptorValueType::String => "STRING",
                MetricDescriptorValueType::ValueTypeUnspecified => "VALUE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorValueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorValueType, ()> {
            Ok(match s {
                "BOOL" => MetricDescriptorValueType::Bool,
                "DISTRIBUTION" => MetricDescriptorValueType::Distribution,
                "DOUBLE" => MetricDescriptorValueType::Double,
                "INT64" => MetricDescriptorValueType::Int64,
                "MONEY" => MetricDescriptorValueType::Money,
                "STRING" => MetricDescriptorValueType::String,
                "VALUE_TYPE_UNSPECIFIED" => MetricDescriptorValueType::ValueTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => MetricDescriptorValueType::Bool,
                "DISTRIBUTION" => MetricDescriptorValueType::Distribution,
                "DOUBLE" => MetricDescriptorValueType::Double,
                "INT64" => MetricDescriptorValueType::Int64,
                "MONEY" => MetricDescriptorValueType::Money,
                "STRING" => MetricDescriptorValueType::String,
                "VALUE_TYPE_UNSPECIFIED" => MetricDescriptorValueType::ValueTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorValueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorValueType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricDescriptorMetadata {
        #[doc = "The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors."]
        #[serde(
            rename = "ingestDelay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingest_delay: ::std::option::Option<String>,
        #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MetricDescriptorMetadataLaunchStage>,
        #[doc = "The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period."]
        #[serde(
            rename = "samplePeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_period: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetadataLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the \"Deprecation Policy\" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MetricDescriptorMetadataLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetadataLaunchStage::Alpha => "ALPHA",
                MetricDescriptorMetadataLaunchStage::Beta => "BETA",
                MetricDescriptorMetadataLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorMetadataLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorMetadataLaunchStage::Ga => "GA",
                MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
                MetricDescriptorMetadataLaunchStage::Prelaunch => "PRELAUNCH",
                MetricDescriptorMetadataLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorMetadataLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorMetadataLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorMetadataLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MetricDescriptorMetadataLaunchStage::Alpha,
                "BETA" => MetricDescriptorMetadataLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorMetadataLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorMetadataLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorMetadataLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MetricDescriptorMetadataLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorMetadataLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetadataLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetadataLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetadataLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorMetadataLaunchStage::Alpha,
                "BETA" => MetricDescriptorMetadataLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorMetadataLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorMetadataLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorMetadataLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MetricDescriptorMetadataLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorMetadataLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetadataLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetadataLaunchStage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricRule {
        #[doc = "Metrics to update when the selected methods are called, and the associated cost applied to each metric. The key of the map is the metric name, and the values are the amount increased for the metric against which the quota limits are defined. The value must not be negative."]
        #[serde(
            rename = "metricCosts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_costs: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Mixin {
        #[doc = "The fully qualified name of the interface which is included."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "If non-empty specifies a path under which inherited HTTP paths are rooted."]
        #[serde(
            rename = "root",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Mixin {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Mixin {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MonitoredResourceDescriptor {
        #[doc = "Optional. A detailed description of the monitored resource type that might be used in documentation."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, `\"Google Cloud SQL Database\"`."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels `\"database_id\"` and `\"zone\"`."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the monitored resource definition."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MonitoredResourceDescriptorLaunchStage>,
        #[doc = "Optional. The resource name of the monitored resource descriptor: `\"projects/{project_id}/monitoredResourceDescriptors/{type}\"` where {type} is the value of the `type` field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format `\"monitoredResourceDescriptors/{type}\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The monitored resource type. For example, the type `\"cloudsql_database\"` represents databases in Google Cloud SQL. For a list of types, see [Monitoring resource types](https://cloud.google.com/monitoring/api/resources) and [Logging resource types](https://cloud.google.com/logging/docs/api/v2/resource-list)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MonitoredResourceDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoredResourceDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MonitoredResourceDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the \"Deprecation Policy\" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MonitoredResourceDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MonitoredResourceDescriptorLaunchStage::Alpha => "ALPHA",
                MonitoredResourceDescriptorLaunchStage::Beta => "BETA",
                MonitoredResourceDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MonitoredResourceDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MonitoredResourceDescriptorLaunchStage::Ga => "GA",
                MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
                MonitoredResourceDescriptorLaunchStage::Prelaunch => "PRELAUNCH",
                MonitoredResourceDescriptorLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MonitoredResourceDescriptorLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MonitoredResourceDescriptorLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MonitoredResourceDescriptorLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MonitoredResourceDescriptorLaunchStage::Alpha,
                "BETA" => MonitoredResourceDescriptorLaunchStage::Beta,
                "DEPRECATED" => MonitoredResourceDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MonitoredResourceDescriptorLaunchStage::EarlyAccess,
                "GA" => MonitoredResourceDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MonitoredResourceDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MonitoredResourceDescriptorLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MonitoredResourceDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MonitoredResourceDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MonitoredResourceDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MonitoredResourceDescriptorLaunchStage::Alpha,
                "BETA" => MonitoredResourceDescriptorLaunchStage::Beta,
                "DEPRECATED" => MonitoredResourceDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MonitoredResourceDescriptorLaunchStage::EarlyAccess,
                "GA" => MonitoredResourceDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MonitoredResourceDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MonitoredResourceDescriptorLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MonitoredResourceDescriptorLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoredResourceDescriptorLaunchStage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Monitoring {
        #[doc = "Monitoring configurations for sending metrics to the consumer project. There can be multiple consumer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
        #[doc = "Monitoring configurations for sending metrics to the producer project. There can be multiple producer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
        #[serde(
            rename = "producerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Monitoring {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Monitoring {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MonitoringDestination {
        #[doc = "Types of the metrics to report to this monitoring destination. Each type must be defined in Service.metrics section."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MonitoringDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoringDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OauthRequirements {
        #[doc = "The list of publicly documented OAuth scopes that are allowed access. An OAuth token containing any of these scopes will be accepted. Example: canonical_scopes: https://www.googleapis.com/auth/calendar, https://www.googleapis.com/auth/calendar.read"]
        #[serde(
            rename = "canonicalScopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canonical_scopes: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OauthRequirements {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OauthRequirements {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OperationInfo {
        #[doc = "Required. The message name of the metadata type for this long-running operation. If the response is in a different package from the rpc, a fully-qualified message name must be used (e.g. `google.protobuf.Struct`). Note: Altering this value constitutes a breaking change."]
        #[serde(
            rename = "metadataType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_type: ::std::option::Option<String>,
        #[doc = "Required. The message name of the primary return type for this long-running operation. This type will be used to deserialize the LRO's response. If the response is in a different package from the rpc, a fully-qualified message name must be used (e.g. `google.protobuf.Struct`). Note: Altering this value constitutes a breaking change."]
        #[serde(
            rename = "responseType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OperationMetadata {
        #[doc = "Percentage of completion of this operation, ranging from 0 to 100."]
        #[serde(
            rename = "progressPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percentage: ::std::option::Option<i32>,
        #[doc = "The full name of the resources that this operation is directly associated with."]
        #[serde(
            rename = "resourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_names: ::std::option::Option<Vec<String>>,
        #[doc = "The start time of the operation."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Detailed status information for each step. The order is undetermined."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<Vec<crate::schemas::Step>>,
    }
    impl ::google_field_selector::FieldSelector for OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Option {
        #[doc = "The option's name. For protobuf built-in options (options defined in descriptor.proto), this is the short name. For example, `\"map_entry\"`. For custom options, it should be the fully-qualified name. For example, `\"google.api.http\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The option's value packed in an Any message. If the value is a primitive, the corresponding wrapper type defined in google/protobuf/wrappers.proto should be used. If the value is an enum, it should be stored as an int32 value using the google.protobuf.Int32Value type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Option {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Option {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Page {
        #[doc = "The Markdown content of the page. You can use (== include {path} ==) to include content from a Markdown file. The content can be used to produce the documentation page such as HTML format page."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The name of the page. It will be used as an identity of the page to generate URI of the page, text of the link to this page in navigation, etc. The full page name (start from the root page name to this page concatenated with `.`) can be used as reference to the page in your documentation. For example: pages: - name: Tutorial content: (== include tutorial.md ==) subpages: - name: Java content: (== include tutorial_java.md ==) You can reference `Java` page using Markdown reference link syntax: `Java`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Subpages of this page. The order of subpages specified here will be honored in the generated docset."]
        #[serde(
            rename = "subpages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subpages: ::std::option::Option<Vec<crate::schemas::Page>>,
    }
    impl ::google_field_selector::FieldSelector for Page {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Page {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Policy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Quota {
        #[doc = "List of `QuotaLimit` definitions for the service."]
        #[serde(
            rename = "limits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub limits: ::std::option::Option<Vec<crate::schemas::QuotaLimit>>,
        #[doc = "List of `MetricRule` definitions, each one mapping a selected method to one or more metrics."]
        #[serde(
            rename = "metricRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_rules: ::std::option::Option<Vec<crate::schemas::MetricRule>>,
    }
    impl ::google_field_selector::FieldSelector for Quota {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Quota {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct QuotaLimit {
        #[doc = "Default number of tokens that can be consumed during the specified duration. This is the number of tokens assigned when a client application developer activates the service for his/her project. Specifying a value of 0 will block all requests. This can be used if you are provisioning quota to selected consumers and blocking others. Similarly, a value of -1 will indicate an unlimited quota. No other negative values are allowed. Used by group-based quotas only."]
        #[serde(
            rename = "defaultLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub default_limit: ::std::option::Option<i64>,
        #[doc = "Optional. User-visible, extended description for this quota limit. Should be used only when more context is needed to understand this limit than provided by the limit's display name (see: `display_name`)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "User-visible display name for this limit. Optional. If not set, the UI will provide a default display name based on the quota configuration. This field can be used to override the default display name generated from the configuration."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Duration of this limit in textual notation. Must be \"100s\" or \"1d\". Used by group-based quotas only."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Free tier value displayed in the Developers Console for this limit. The free tier is the number of tokens that will be subtracted from the billed amount when billing is enabled. This field can only be set on a limit with duration \"1d\", in a billable group; it is invalid on any other limit. If this field is not set, it defaults to 0, indicating that there is no free tier for this service. Used by group-based quotas only."]
        #[serde(
            rename = "freeTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub free_tier: ::std::option::Option<i64>,
        #[doc = "Maximum number of tokens that can be consumed during the specified duration. Client application developers can override the default limit up to this maximum. If specified, this value cannot be set to a value less than the default limit. If not specified, it is set to the default limit. To allow clients to apply overrides with no upper bound, set this to -1, indicating unlimited maximum quota. Used by group-based quotas only."]
        #[serde(
            rename = "maxLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_limit: ::std::option::Option<i64>,
        #[doc = "The name of the metric this quota limit applies to. The quota limits with the same metric will be checked together during runtime. The metric must be defined within the service config."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "Name of the quota limit. The name must be provided, and it must be unique within the service. The name can only include alphanumeric characters as well as '-'. The maximum length of the limit name is 64 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Specify the unit of the quota limit. It uses the same syntax as Metric.unit. The supported unit kinds are determined by the quota backend system. Here are some examples: * \"1/min/{project}\" for quota per minute per project. Note: the order of unit components is insignificant. The \"1\" at the beginning is required to follow the metric unit syntax."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Tiered limit values. You must specify this as a key:value pair, with an integer value that is the maximum number of requests allowed for the specified unit. Currently only STANDARD is supported."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
    }
    impl ::google_field_selector::FieldSelector for QuotaLimit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuotaLimit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceReference {
        #[doc = "The resource type of a child collection that the annotated field references. This is useful for annotating the `parent` field that doesn't have a fixed resource type. Example: message ListLogEntriesRequest { string parent = 1 [(google.api.resource_reference) = { child_type: \"logging.googleapis.com/LogEntry\" }; }"]
        #[serde(
            rename = "childType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub child_type: ::std::option::Option<String>,
        #[doc = "The resource type that the annotated field references. Example: message Subscription { string topic = 2 [(google.api.resource_reference) = { type: \"pubsub.googleapis.com/Topic\" }]; } Occasionally, a field may reference an arbitrary resource. In this case, APIs use the special value * in their resource reference. Example: message GetIamPolicyRequest { string resource = 2 [(google.api.resource_reference) = { type: \"*\" }]; }"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResourceReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Rollout {
        #[doc = "Creation time of the rollout. Readonly."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The user who created the Rollout. Readonly."]
        #[serde(
            rename = "createdBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub created_by: ::std::option::Option<String>,
        #[doc = "The strategy associated with a rollout to delete a `ManagedService`. Readonly."]
        #[serde(
            rename = "deleteServiceStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_service_strategy: ::std::option::Option<crate::schemas::DeleteServiceStrategy>,
        #[doc = "Optional. Unique identifier of this Rollout. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If not specified by client, the server will generate one. The generated id will have the form of , where \"date\" is the create date in ISO 8601 format. \"revision number\" is a monotonically increasing positive number that is reset every day for each service. An example of the generated rollout_id is '2016-02-16r1'"]
        #[serde(
            rename = "rolloutId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_id: ::std::option::Option<String>,
        #[doc = "The name of the service associated with this Rollout."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
        #[doc = "The status of this rollout. Readonly. In case of a failed rollout, the system will automatically rollback to the current Rollout version. Readonly."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::RolloutStatus>,
        #[doc = "Google Service Control selects service configurations based on traffic percentage."]
        #[serde(
            rename = "trafficPercentStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub traffic_percent_strategy: ::std::option::Option<crate::schemas::TrafficPercentStrategy>,
    }
    impl ::google_field_selector::FieldSelector for Rollout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Rollout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RolloutStatus {
        #[doc = "The Rollout has been cancelled. This can happen if you have overlapping Rollout pushes, and the previous ones will be cancelled."]
        Cancelled,
        #[doc = "The Rollout has failed and the rollback attempt has failed too."]
        Failed,
        #[doc = "The Rollout has failed and rolled back to the previous successful Rollout."]
        FailedRolledBack,
        #[doc = "The Rollout is in progress."]
        InProgress,
        #[doc = "The Rollout has not started yet and is pending for execution."]
        Pending,
        #[doc = "No status specified."]
        RolloutStatusUnspecified,
        #[doc = "The Rollout has completed successfully."]
        Success,
    }
    impl RolloutStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                RolloutStatus::Cancelled => "CANCELLED",
                RolloutStatus::Failed => "FAILED",
                RolloutStatus::FailedRolledBack => "FAILED_ROLLED_BACK",
                RolloutStatus::InProgress => "IN_PROGRESS",
                RolloutStatus::Pending => "PENDING",
                RolloutStatus::RolloutStatusUnspecified => "ROLLOUT_STATUS_UNSPECIFIED",
                RolloutStatus::Success => "SUCCESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RolloutStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RolloutStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RolloutStatus, ()> {
            Ok(match s {
                "CANCELLED" => RolloutStatus::Cancelled,
                "FAILED" => RolloutStatus::Failed,
                "FAILED_ROLLED_BACK" => RolloutStatus::FailedRolledBack,
                "IN_PROGRESS" => RolloutStatus::InProgress,
                "PENDING" => RolloutStatus::Pending,
                "ROLLOUT_STATUS_UNSPECIFIED" => RolloutStatus::RolloutStatusUnspecified,
                "SUCCESS" => RolloutStatus::Success,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RolloutStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RolloutStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RolloutStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => RolloutStatus::Cancelled,
                "FAILED" => RolloutStatus::Failed,
                "FAILED_ROLLED_BACK" => RolloutStatus::FailedRolledBack,
                "IN_PROGRESS" => RolloutStatus::InProgress,
                "PENDING" => RolloutStatus::Pending,
                "ROLLOUT_STATUS_UNSPECIFIED" => RolloutStatus::RolloutStatusUnspecified,
                "SUCCESS" => RolloutStatus::Success,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RolloutStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RolloutStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Service {
        #[doc = "A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files."]
        #[serde(
            rename = "apis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration."]
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "API backend configuration."]
        #[serde(
            rename = "backend",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backend: ::std::option::Option<crate::schemas::Backend>,
        #[doc = "Billing configuration."]
        #[serde(
            rename = "billing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing: ::std::option::Option<crate::schemas::Billing>,
        #[doc = "Obsolete. Do not use. This field has no semantic meaning. The service config compiler always sets this field to `3`."]
        #[serde(
            rename = "configVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_version: ::std::option::Option<u32>,
        #[doc = "Context configuration."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[doc = "Configuration for the service control plane."]
        #[serde(
            rename = "control",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub control: ::std::option::Option<crate::schemas::Control>,
        #[doc = "Custom error configuration."]
        #[serde(
            rename = "customError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_error: ::std::option::Option<crate::schemas::CustomError>,
        #[doc = "Additional API documentation."]
        #[serde(
            rename = "documentation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs."]
        #[serde(
            rename = "endpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name by the configuration author. Example: enums: - name: google.someapi.v1.SomeEnum"]
        #[serde(
            rename = "enums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enums: ::std::option::Option<Vec<crate::schemas::Enum>>,
        #[doc = "HTTP configuration."]
        #[serde(
            rename = "http",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http: ::std::option::Option<crate::schemas::Http>,
        #[doc = "A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Logging configuration."]
        #[serde(
            rename = "logging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logging: ::std::option::Option<crate::schemas::Logging>,
        #[doc = "Defines the logs used by this service."]
        #[serde(
            rename = "logs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs: ::std::option::Option<Vec<crate::schemas::LogDescriptor>>,
        #[doc = "Defines the metrics used by this service."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MetricDescriptor>>,
        #[doc = "Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations."]
        #[serde(
            rename = "monitoredResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resources:
            ::std::option::Option<Vec<crate::schemas::MonitoredResourceDescriptor>>,
        #[doc = "Monitoring configuration."]
        #[serde(
            rename = "monitoring",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitoring: ::std::option::Option<crate::schemas::Monitoring>,
        #[doc = "The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google project that owns this service."]
        #[serde(
            rename = "producerProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_project_id: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(
            rename = "quota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "Output only. The source information for this configuration if available."]
        #[serde(
            rename = "sourceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_info: ::std::option::Option<crate::schemas::SourceInfo>,
        #[doc = "System parameter configuration."]
        #[serde(
            rename = "systemParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_parameters: ::std::option::Option<crate::schemas::SystemParameters>,
        #[doc = "A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF."]
        #[serde(
            rename = "systemTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "The product title for this service, it is the name displayed in Google Cloud Console."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name by the configuration author. Example: types: - name: google.protobuf.Int32"]
        #[serde(
            rename = "types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(
            rename = "usage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for Service {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Service {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetIamPolicyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SourceContext {
        #[doc = "The path-qualified name of the .proto file that contained the associated protobuf element. For example: `\"google/protobuf/source_context.proto\"`."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceInfo {
        #[doc = "All files used during config generation."]
        #[serde(
            rename = "sourceFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_files:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for SourceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Step {
        #[doc = "The short description of the step."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The status code."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::StepStatus>,
    }
    impl ::google_field_selector::FieldSelector for Step {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Step {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StepStatus {
        #[doc = "The operation or step has completed with cancellation."]
        Cancelled,
        #[doc = "The operation or step has completed without errors."]
        Done,
        #[doc = "The operation or step has completed with errors. If the operation is rollbackable, the rollback completed with errors too."]
        Failed,
        #[doc = "The operation or step is in progress."]
        InProgress,
        #[doc = "The operation or step has not started yet."]
        NotStarted,
        #[doc = "Unspecifed code."]
        StatusUnspecified,
    }
    impl StepStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                StepStatus::Cancelled => "CANCELLED",
                StepStatus::Done => "DONE",
                StepStatus::Failed => "FAILED",
                StepStatus::InProgress => "IN_PROGRESS",
                StepStatus::NotStarted => "NOT_STARTED",
                StepStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StepStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StepStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StepStatus, ()> {
            Ok(match s {
                "CANCELLED" => StepStatus::Cancelled,
                "DONE" => StepStatus::Done,
                "FAILED" => StepStatus::Failed,
                "IN_PROGRESS" => StepStatus::InProgress,
                "NOT_STARTED" => StepStatus::NotStarted,
                "STATUS_UNSPECIFIED" => StepStatus::StatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StepStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StepStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StepStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => StepStatus::Cancelled,
                "DONE" => StepStatus::Done,
                "FAILED" => StepStatus::Failed,
                "IN_PROGRESS" => StepStatus::InProgress,
                "NOT_STARTED" => StepStatus::NotStarted,
                "STATUS_UNSPECIFIED" => StepStatus::StatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StepStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StepStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SubmitConfigSourceRequest {
        #[doc = "Required. The source configuration for the service."]
        #[serde(
            rename = "configSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_source: ::std::option::Option<crate::schemas::ConfigSource>,
        #[doc = "Optional. If set, this will result in the generation of a `google.api.Service` configuration based on the `ConfigSource` provided, but the generated config and the sources will NOT be persisted."]
        #[serde(
            rename = "validateOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub validate_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SubmitConfigSourceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SubmitConfigSourceRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SubmitConfigSourceResponse {
        #[doc = "The generated service configuration."]
        #[serde(
            rename = "serviceConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_config: ::std::option::Option<crate::schemas::Service>,
    }
    impl ::google_field_selector::FieldSelector for SubmitConfigSourceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SubmitConfigSourceResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SystemParameter {
        #[doc = "Define the HTTP header name to use for the parameter. It is case insensitive."]
        #[serde(
            rename = "httpHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_header: ::std::option::Option<String>,
        #[doc = "Define the name of the parameter, such as \"api_key\" . It is case sensitive."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Define the URL query parameter name to use for the parameter. It is case sensitive."]
        #[serde(
            rename = "urlQueryParameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_query_parameter: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SystemParameterRule {
        #[doc = "Define parameters. Multiple names may be defined for a parameter. For a given method call, only one of them should be used. If multiple names are used the behavior is implementation-dependent. If none of the specified names are present the behavior is parameter-dependent."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::SystemParameter>>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameterRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameterRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SystemParameters {
        #[doc = "Define system parameters. The parameters defined here will override the default parameters implemented by the system. If this field is missing from the service config, default system parameters will be used. Default system parameters and names is implementation-dependent. Example: define api key for all methods system_parameters rules: - selector: \"*\" parameters: - name: api_key url_query_parameter: api_key Example: define 2 api key names for a specific method. system_parameters rules: - selector: \"/ListShelves\" parameters: - name: api_key http_header: Api-Key1 - name: api_key http_header: Api-Key2 **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::SystemParameterRule>>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrafficPercentStrategy {
        #[doc = "Maps service configuration IDs to their corresponding traffic percentage. Key is the service configuration ID, Value is the traffic percentage which must be greater than 0.0 and the sum must equal to 100.0."]
        #[serde(
            rename = "percentages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percentages: ::std::option::Option<::std::collections::BTreeMap<String, f64>>,
    }
    impl ::google_field_selector::FieldSelector for TrafficPercentStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrafficPercentStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Type {
        #[doc = "The list of fields."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::Field>>,
        #[doc = "The fully qualified message name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The list of types appearing in `oneof` definitions in this type."]
        #[serde(
            rename = "oneofs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneofs: ::std::option::Option<Vec<String>>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::TypeSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Type {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Type {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TypeSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl TypeSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                TypeSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                TypeSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TypeSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TypeSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TypeSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TypeSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TypeSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TypeSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TypeSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TypeSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UndeleteServiceResponse {
        #[doc = "Revived service resource."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<crate::schemas::ManagedService>,
    }
    impl ::google_field_selector::FieldSelector for UndeleteServiceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UndeleteServiceResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Usage {
        #[doc = "The full resource name of a channel used for sending notifications to the service producer. Google Service Management currently only supports [Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification channel. To use Google Cloud Pub/Sub as the channel, this must be the name of a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format documented in https://cloud.google.com/pubsub/docs/overview."]
        #[serde(
            rename = "producerNotificationChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_notification_channel: ::std::option::Option<String>,
        #[doc = "Requirements that must be satisfied before a consumer project can use the service. Each requirement is of the form /; for example 'serviceusage.googleapis.com/billing-enabled'. For Google APIs, a Terms of Service requirement must be included here. Google Cloud APIs must include \"serviceusage.googleapis.com/tos/cloud\". Other Google APIs should include \"serviceusage.googleapis.com/tos/universal\". Additional ToS can be included based on the business needs."]
        #[serde(
            rename = "requirements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requirements: ::std::option::Option<Vec<String>>,
        #[doc = "A list of usage rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::UsageRule>>,
    }
    impl ::google_field_selector::FieldSelector for Usage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Usage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UsageRule {
        #[doc = "If true, the selected method allows unregistered calls, e.g. calls that don't identify any user or application."]
        #[serde(
            rename = "allowUnregisteredCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_unregistered_calls: ::std::option::Option<bool>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
        #[doc = "If true, the selected method should skip service control and the control plane features, such as quota and billing, will not be available. This flag is used by Google Cloud Endpoints to bypass checks for internal methods, such as service health check methods."]
        #[serde(
            rename = "skipServiceControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_service_control: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for UsageRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the services resource"]
    pub fn services(&self) -> crate::resources::services::ServicesActions {
        crate::resources::services::ServicesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod operations {
        pub mod params {}
        pub struct OperationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OperationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Lists service operations that match the specified filter in the request."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    filter: None,
                    name: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            filter: ::std::option::Option<String>,
            name: ::std::option::Option<String>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "A string for filtering Operations. The following filter fields are supported: * serviceName: Required. Only `=` operator is allowed. * startTime: The time this job was started, in ISO 8601 format. Allowed operators are `>=`, `>`, `<=`, and `<`. * status: Can be `done`, `in_progress`, or `failed`. Allowed operators are `=`, and `!=`. Filter expression supports conjunction (AND) and disjunction (OR) logical operators. However, the serviceName restriction must be at the top-level and can only be combined with other restrictions via the AND logical operator. Examples: * `serviceName={some-service}.googleapis.com` * `serviceName={some-service}.googleapis.com AND startTime>=\"2017-02-01\"` * `serviceName={some-service}.googleapis.com AND status=done` * `serviceName={some-service}.googleapis.com AND (status=done OR startTime>=\"2017-02-01\")`"]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Not used."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = "The maximum number of operations to return. If unspecified, defaults to 50. The maximum value is 100."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The standard list page token."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_operations<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_operations_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_operations_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>> + 'a
            {
                self.stream_operations_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_operations_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>> + 'a
            {
                self.stream_operations_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_operations_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + 'a,
                F: AsRef<str>,
            {
                #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                struct Page<T> {
                    #[serde(rename = "nextPageToken")]
                    pub next_page_token: ::std::option::Option<String>,
                    #[serde(rename = "operations")]
                    pub items: Vec<T>,
                }
                impl<T> crate::GetNextPageToken for Page<T> {
                    fn next_page_token(&self) -> ::std::option::Option<String> {
                        self.next_page_token.to_owned()
                    }
                }
                impl<T> crate::stream::IntoPageItems for Page<T> {
                    type Items = Vec<T>;
                    fn into_page_items(self) -> Self::Items {
                        self.items
                    }
                }
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "operations").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::stream::page_item_stream::<_, Page<T>>(self)
            }
            #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
            #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
            #[doc = r" token is returned."]
            #[doc = r""]
            #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
            #[doc = r""]
            #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
            #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken
                    + ::serde::de::DeserializeOwned
                    + ::google_field_selector::FieldSelector
                    + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_with_fields(fields)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests the default set of fields from the server."]
            pub fn stream_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListOperationsResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]
            pub fn stream_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListOperationsResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(Some("*"))
            }
            #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
            #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
            #[doc = r" token is returned."]
            #[doc = r""]
            #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
            #[doc = r" empty, the `nextPageToken` field will be added to the list."]
            #[doc = r""]
            #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
            pub fn stream_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::stream::page_stream(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/operations");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("filter", &self.filter)]);
                req = req.query(&[("name", &self.name)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            async fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
            {
                self._execute().await
            }
        }
    }
    pub mod services {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetConfigView {
                #[doc = "Server response includes all fields except SourceInfo."]
                Basic,
                #[doc = "Server response includes all fields including SourceInfo. SourceFiles are of type 'google.api.servicemanagement.v1.ConfigFile' and are only available for configs created using the SubmitConfigSource method."]
                Full,
            }
            impl GetConfigView {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetConfigView::Basic => "BASIC",
                        GetConfigView::Full => "FULL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetConfigView {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetConfigView {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetConfigView, ()> {
                    Ok(match s {
                        "BASIC" => GetConfigView::Basic,
                        "FULL" => GetConfigView::Full,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetConfigView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetConfigView {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetConfigView {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "BASIC" => GetConfigView::Basic,
                        "FULL" => GetConfigView::Full,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetConfigView {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetConfigView {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ServicesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ServicesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a new managed service. A managed service is immutable, and is subject to mandatory 30-day data retention. You cannot move a service or recreate it within 30 days after deletion. One producer project can own no more than 500 services. For security and reliability purposes, a production service should be hosted in a dedicated producer project. Operation"]
            pub fn create(&self, request: crate::schemas::ManagedService) -> CreateRequestBuilder {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Deletes a managed service. This method will change the service to the `Soft-Delete` state for 30 days. Within this period, service producers may call UndeleteService to restore the service. After 30 days, the service will be permanently deleted. Operation"]
            pub fn delete(&self, service_name: impl Into<String>) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                }
            }
            #[doc = "Generates and returns a report (errors, warnings and changes from existing configurations) associated with GenerateConfigReportRequest.new_value If GenerateConfigReportRequest.old_value is specified, GenerateConfigReportRequest will contain a single ChangeReport based on the comparison between GenerateConfigReportRequest.new_value and GenerateConfigReportRequest.old_value. If GenerateConfigReportRequest.old_value is not specified, this method will compare GenerateConfigReportRequest.new_value with the last pushed service configuration."]
            pub fn generate_config_report(
                &self,
                request: crate::schemas::GenerateConfigReportRequest,
            ) -> GenerateConfigReportRequestBuilder {
                GenerateConfigReportRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Gets a managed service. Authentication is required unless the service is public."]
            pub fn get(&self, service_name: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                }
            }
            #[doc = "Gets a service configuration (version) for a managed service."]
            pub fn get_config(&self, service_name: impl Into<String>) -> GetConfigRequestBuilder {
                GetConfigRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                    config_id: None,
                    view: None,
                }
            }
            #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
            pub fn get_iam_policy(
                &self,
                request: crate::schemas::GetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> GetIamPolicyRequestBuilder {
                GetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Lists managed services. Returns all public services. For authenticated users, also returns all services the calling user has \"servicemanagement.services.get\" permission for."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    consumer_id: None,
                    page_size: None,
                    page_token: None,
                    producer_project_id: None,
                }
            }
            #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
            pub fn set_iam_policy(
                &self,
                request: crate::schemas::SetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> SetIamPolicyRequestBuilder {
                SetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."]
            pub fn test_iam_permissions(
                &self,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: impl Into<String>,
            ) -> TestIamPermissionsRequestBuilder {
                TestIamPermissionsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Revives a previously deleted managed service. The method restores the service using the configuration at the time the service was deleted. The target service must exist and must have been deleted within the last 30 days. Operation"]
            pub fn undelete(&self, service_name: impl Into<String>) -> UndeleteRequestBuilder {
                UndeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                }
            }
            #[doc = "Actions that can be performed on the configs resource"]
            pub fn configs(&self) -> crate::resources::services::configs::ConfigsActions {
                crate::resources::services::configs::ConfigsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the consumers resource"]
            pub fn consumers(&self) -> crate::resources::services::consumers::ConsumersActions {
                crate::resources::services::consumers::ConsumersActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the rollouts resource"]
            pub fn rollouts(&self) -> crate::resources::services::rollouts::RolloutsActions {
                crate::resources::services::rollouts::RolloutsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ServicesActions::create()](struct.ServicesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ManagedService,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::delete()](struct.ServicesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            service_name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::generate_config_report()](struct.ServicesActions.html#method.generate_config_report)"]
        #[derive(Debug, Clone)]
        pub struct GenerateConfigReportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GenerateConfigReportRequest,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GenerateConfigReportRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GenerateConfigReportResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GenerateConfigReportResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services:generateConfigReport");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::get()](struct.ServicesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            service_name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ManagedService, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ManagedService, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::get_config()](struct.ServicesActions.html#method.get_config)"]
        #[derive(Debug, Clone)]
        pub struct GetConfigRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            service_name: String,
            config_id: ::std::option::Option<String>,
            view: ::std::option::Option<crate::resources::services::params::GetConfigView>,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GetConfigRequestBuilder<'a> {
            #[doc = "Required. The id of the service configuration resource. This field must be specified for the server to return all fields, including `SourceInfo`."]
            pub fn config_id(mut self, value: impl Into<String>) -> Self {
                self.config_id = Some(value.into());
                self
            }
            #[doc = "Specifies which parts of the Service Config should be returned in the response."]
            pub fn view(
                mut self,
                value: crate::resources::services::params::GetConfigView,
            ) -> Self {
                self.view = Some(value);
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Service, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Service, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/config");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("configId", &self.config_id)]);
                req = req.query(&[("view", &self.view)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::get_iam_policy()](struct.ServicesActions.html#method.get_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct GetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GetIamPolicyRequest,
            resource: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GetIamPolicyRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getIamPolicy");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::list()](struct.ServicesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            consumer_id: ::std::option::Option<String>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            producer_project_id: ::std::option::Option<String>,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Include services consumed by the specified consumer. The Google Service Management implementation accepts the following forms: - project:"]
            pub fn consumer_id(mut self, value: impl Into<String>) -> Self {
                self.consumer_id = Some(value.into());
                self
            }
            #[doc = "The max number of items to include in the response list. Page size is 50 if not specified. Maximum value is 100."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Token identifying which result to start with; returned by a previous list call."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Include services produced by the specified project."]
            pub fn producer_project_id(mut self, value: impl Into<String>) -> Self {
                self.producer_project_id = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = "\nExecute the request and yield each item in the `services` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_services<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_services_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `services` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_services_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::ManagedService, crate::Error>> + 'a
            {
                self.stream_services_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `services` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_services_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::ManagedService, crate::Error>> + 'a
            {
                self.stream_services_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `services` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_services_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + 'a,
                F: AsRef<str>,
            {
                #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                struct Page<T> {
                    #[serde(rename = "nextPageToken")]
                    pub next_page_token: ::std::option::Option<String>,
                    #[serde(rename = "services")]
                    pub items: Vec<T>,
                }
                impl<T> crate::GetNextPageToken for Page<T> {
                    fn next_page_token(&self) -> ::std::option::Option<String> {
                        self.next_page_token.to_owned()
                    }
                }
                impl<T> crate::stream::IntoPageItems for Page<T> {
                    type Items = Vec<T>;
                    fn into_page_items(self) -> Self::Items {
                        self.items
                    }
                }
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "services").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::stream::page_item_stream::<_, Page<T>>(self)
            }
            #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
            #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
            #[doc = r" token is returned."]
            #[doc = r""]
            #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
            #[doc = r""]
            #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
            #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken
                    + ::serde::de::DeserializeOwned
                    + ::google_field_selector::FieldSelector
                    + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_with_fields(fields)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests the default set of fields from the server."]
            pub fn stream_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListServicesResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]
            pub fn stream_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListServicesResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(Some("*"))
            }
            #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
            #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
            #[doc = r" token is returned."]
            #[doc = r""]
            #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
            #[doc = r" empty, the `nextPageToken` field will be added to the list."]
            #[doc = r""]
            #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
            pub fn stream_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::stream::page_stream(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListServicesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListServicesResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("consumerId", &self.consumer_id)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("producerProjectId", &self.producer_project_id)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            async fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
            {
                self._execute().await
            }
        }
        #[doc = "Created via [ServicesActions::set_iam_policy()](struct.ServicesActions.html#method.set_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct SetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SetIamPolicyRequest,
            resource: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> SetIamPolicyRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":setIamPolicy");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::test_iam_permissions()](struct.ServicesActions.html#method.test_iam_permissions)"]
        #[derive(Debug, Clone)]
        pub struct TestIamPermissionsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::TestIamPermissionsRequest,
            resource: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> TestIamPermissionsRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":testIamPermissions");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::undelete()](struct.ServicesActions.html#method.undelete)"]
        #[derive(Debug, Clone)]
        pub struct UndeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            service_name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> UndeleteRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":undelete");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        pub mod configs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {
                    #[doc = "Server response includes all fields except SourceInfo."]
                    Basic,
                    #[doc = "Server response includes all fields including SourceInfo. SourceFiles are of type 'google.api.servicemanagement.v1.ConfigFile' and are only available for configs created using the SubmitConfigSource method."]
                    Full,
                }
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetView::Basic => "BASIC",
                            GetView::Full => "FULL",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                        Ok(match s {
                            "BASIC" => GetView::Basic,
                            "FULL" => GetView::Full,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "BASIC" => GetView::Basic,
                            "FULL" => GetView::Full,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ConfigsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConfigsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new service configuration (version) for a managed service. This method only stores the service configuration. To roll out the service configuration to backend systems please call CreateServiceRollout. Only the 100 most recent service configurations and ones referenced by existing rollouts are kept for each service. The rest will be deleted eventually."]
                pub fn create(
                    &self,
                    request: crate::schemas::Service,
                    service_name: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                    }
                }
                #[doc = "Gets a service configuration (version) for a managed service."]
                pub fn get(
                    &self,
                    service_name: impl Into<String>,
                    config_id: impl Into<String>,
                ) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                        config_id: config_id.into(),
                        view: None,
                    }
                }
                #[doc = "Lists the history of the service configuration for a managed service, from the newest to the oldest."]
                pub fn list(&self, service_name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Creates a new service configuration (version) for a managed service based on user-supplied configuration source files (for example: OpenAPI Specification). This method stores the source configurations as well as the generated service configuration. To rollout the service configuration to other services, please call CreateServiceRollout. Only the 100 most recent configuration sources and ones referenced by existing service configurtions are kept for each service. The rest will be deleted eventually. Operation"]
                pub fn submit(
                    &self,
                    request: crate::schemas::SubmitConfigSourceRequest,
                    service_name: impl Into<String>,
                ) -> SubmitRequestBuilder {
                    SubmitRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                    }
                }
            }
            #[doc = "Created via [ConfigsActions::create()](struct.ConfigsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Service,
                service_name: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Service, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Service, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/configs");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [ConfigsActions::get()](struct.ConfigsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                service_name: String,
                config_id: String,
                view: ::std::option::Option<crate::resources::services::configs::params::GetView>,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "Specifies which parts of the Service Config should be returned in the response."]
                pub fn view(
                    mut self,
                    value: crate::resources::services::configs::params::GetView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Service, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Service, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/configs/");
                    {
                        let var_as_str = &self.config_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("view", &self.view)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [ConfigsActions::list()](struct.ConfigsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                service_name: String,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The max number of items to include in the response list. Page size is 50 if not specified. Maximum value is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token of the page to retrieve."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = "\nExecute the request and yield each item in the `serviceConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_service_configs<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_service_configs_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `serviceConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_service_configs_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Service, crate::Error>> + 'a
                {
                    self.stream_service_configs_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `serviceConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_service_configs_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Service, crate::Error>> + 'a
                {
                    self.stream_service_configs_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `serviceConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_service_configs_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "serviceConfigs")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "serviceConfigs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListServiceConfigsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListServiceConfigsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::stream::page_stream(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListServiceConfigsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListServiceConfigsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/configs");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [ConfigsActions::submit()](struct.ConfigsActions.html#method.submit)"]
            #[derive(Debug, Clone)]
            pub struct SubmitRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SubmitConfigSourceRequest,
                service_name: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> SubmitRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/configs:submit");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
        }
        pub mod consumers {
            pub mod params {}
            pub struct ConsumersActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConsumersActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                pub fn get_iam_policy(
                    &self,
                    request: crate::schemas::GetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder {
                    GetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder {
                    SetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder {
                    TestIamPermissionsRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
            }
            #[doc = "Created via [ConsumersActions::get_iam_policy()](struct.ConsumersActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> GetIamPolicyRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [ConsumersActions::set_iam_policy()](struct.ConsumersActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> SetIamPolicyRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [ConsumersActions::test_iam_permissions()](struct.ConsumersActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> TestIamPermissionsRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
        }
        pub mod rollouts {
            pub mod params {}
            pub struct RolloutsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> RolloutsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new service configuration rollout. Based on rollout, the Google Service Management will roll out the service configurations to different backend services. For example, the logging configuration will be pushed to Google Cloud Logging. Please note that any previous pending and running Rollouts and associated Operations will be automatically cancelled so that the latest Rollout will not be blocked by previous Rollouts. Only the 100 most recent (in any state) and the last 10 successful (if not already part of the set of 100 most recent) rollouts are kept for each service. The rest will be deleted eventually. Operation"]
                pub fn create(
                    &self,
                    request: crate::schemas::Rollout,
                    service_name: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                    }
                }
                #[doc = "Gets a service configuration rollout."]
                pub fn get(
                    &self,
                    service_name: impl Into<String>,
                    rollout_id: impl Into<String>,
                ) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                        rollout_id: rollout_id.into(),
                    }
                }
                #[doc = "Lists the history of the service configuration rollouts for a managed service, from the newest to the oldest."]
                pub fn list(&self, service_name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        service_name: service_name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [RolloutsActions::create()](struct.RolloutsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Rollout,
                service_name: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/rollouts");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [RolloutsActions::get()](struct.RolloutsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                service_name: String,
                rollout_id: String,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Rollout, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Rollout, crate::Error> {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/rollouts/");
                    {
                        let var_as_str = &self.rollout_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [RolloutsActions::list()](struct.RolloutsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                service_name: String,
                filter: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
                access_token: ::std::option::Option<String>,
                alt: ::std::option::Option<crate::params::Alt>,
                callback: ::std::option::Option<String>,
                fields: ::std::option::Option<String>,
                key: ::std::option::Option<String>,
                oauth_token: ::std::option::Option<String>,
                pretty_print: ::std::option::Option<bool>,
                quota_user: ::std::option::Option<String>,
                upload_protocol: ::std::option::Option<String>,
                upload_type: ::std::option::Option<String>,
                xgafv: ::std::option::Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Required. Use `filter` to return subset of rollouts. The following filters are supported: -- By status. For example, `filter='status=SUCCESS'` -- By strategy. For example, `filter='strategy=TrafficPercentStrategy'`"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The max number of items to include in the response list. Page size is 50 if not specified. Maximum value is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token of the page to retrieve."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = "\nExecute the request and yield each item in the `rollouts` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_rollouts<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_rollouts_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `rollouts` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_rollouts_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Rollout, crate::Error>> + 'a
                {
                    self.stream_rollouts_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `rollouts` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_rollouts_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Rollout, crate::Error>> + 'a
                {
                    self.stream_rollouts_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `rollouts` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_rollouts_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "rollouts")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "rollouts").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListServiceRolloutsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListServiceRolloutsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::stream::page_stream(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListServiceRolloutsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListServiceRolloutsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://servicemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/services/");
                    {
                        let var_as_str = &self.service_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/rollouts");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
            use RelatedMultiPartReaderState::*;

            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }

            futures::task::Poll::Ready(Ok(bytes_written))
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
/// Represent the ability to extract the `nextPageToken` from a response.
pub trait GetNextPageToken {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<String>;
}

impl GetNextPageToken for ::serde_json::Map<String, ::serde_json::Value> {
    fn next_page_token(&self) -> ::std::option::Option<String> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned())
    }
}
/// Traits and functions to improve streamable (multiple page) API method handling.
pub mod stream {
    use super::GetNextPageToken;

    /// Extract the items embedded in a page like response.
    pub trait IntoPageItems {
        /// Type of the items list in the page.
        type Items: IntoIterator;

        /// Consume the response and return the embedded items.
        fn into_page_items(self) -> Self::Items;
    }

    /// Represent a API method which can be invoked multiple times to retrieve
    /// multiple pages of items.
    #[async_trait::async_trait]
    pub trait StreamableMethod {
        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: String);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken + ::serde::de::DeserializeOwned,
    {
        ::futures::stream::unfold((method, false), |(mut method, mut finished)| async move {
            if finished {
                return None;
            }
            let response = match method.execute::<T>().await {
                Ok(r) => r,
                Err(err) => return Some((Err(err), (method, false))),
            };
            if let Some(next_page_token) = response.next_page_token() {
                method.set_page_token(next_page_token);
            } else {
                finished = true;
            }

            Some((Ok(response), (method, finished)))
        })
    }

    /// Return a [`Stream`](::futures::Stream) over the items in all pages of
    /// the given API method.
    pub fn page_item_stream<M, T>(
        method: M,
    ) -> impl ::futures::Stream<Item = Result<<T::Items as IntoIterator>::Item, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
