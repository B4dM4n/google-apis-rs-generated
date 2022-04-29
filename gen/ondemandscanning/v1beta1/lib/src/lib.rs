#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [operations](resources/projects/locations/operations/struct.OperationsActions.html)\n      * [*cancel*](resources/projects/locations/operations/struct.CancelRequestBuilder.html), [*delete*](resources/projects/locations/operations/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/operations/struct.ListRequestBuilder.html), [*wait*](resources/projects/locations/operations/struct.WaitRequestBuilder.html)\n    * [scans](resources/projects/locations/scans/struct.ScansActions.html)\n      * [*analyzePackages*](resources/projects/locations/scans/struct.AnalyzePackagesRequestBuilder.html)\n      * [vulnerabilities](resources/projects/locations/scans/vulnerabilities/struct.VulnerabilitiesActions.html)\n        * [*list*](resources/projects/locations/scans/vulnerabilities/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct AliasContext {
        #[doc = "The alias kind."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::AliasContextKind>,
        #[doc = "The alias name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AliasContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AliasContextKind {
        #[doc = "Git tag."]
        Fixed,
        #[doc = "Unknown."]
        KindUnspecified,
        #[doc = "Git branch."]
        Movable,
        #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named “refs/foo/bar”."]
        Other,
    }
    impl AliasContextKind {
        pub fn as_str(self) -> &'static str {
            match self {
                AliasContextKind::Fixed => "FIXED",
                AliasContextKind::KindUnspecified => "KIND_UNSPECIFIED",
                AliasContextKind::Movable => "MOVABLE",
                AliasContextKind::Other => "OTHER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AliasContextKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AliasContextKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AliasContextKind, ()> {
            Ok(match s {
                "FIXED" => AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => AliasContextKind::KindUnspecified,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AliasContextKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AliasContextKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AliasContextKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FIXED" => AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => AliasContextKind::KindUnspecified,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AliasContextKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContextKind {
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
    pub struct AnalyzePackagesMetadata {
        #[doc = "When the scan was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The resource URI of the container image being scanned."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzePackagesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzePackagesMetadata {
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
    pub struct AnalyzePackagesMetadataV1 {
        #[doc = "When the scan was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The resource URI of the container image being scanned."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzePackagesMetadataV1 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzePackagesMetadataV1 {
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
    pub struct AnalyzePackagesRequest {
        #[doc = "The packages to analyze."]
        #[serde(
            rename = "packages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packages: ::std::option::Option<Vec<crate::schemas::PackageData>>,
        #[doc = "Required. The resource URI of the container image being scanned."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzePackagesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzePackagesRequest {
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
    pub struct AnalyzePackagesResponse {
        #[doc = "The name of the scan resource created by this successful scan."]
        #[serde(
            rename = "scan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzePackagesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzePackagesResponse {
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
    pub struct AnalyzePackagesResponseV1 {
        #[doc = "The name of the scan resource created by this successful scan."]
        #[serde(
            rename = "scan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzePackagesResponseV1 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzePackagesResponseV1 {
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
    pub struct Artifact {
        #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<String>,
        #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Artifact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Artifact {
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
    pub struct AttestationOccurrence {
        #[doc = "One or more JWTs encoding a self-contained attestation. Each JWT encodes the payload that it verifies within the JWT itself. Verifier implementation SHOULD ignore the `serialized_payload` field when verifying these JWTs. If only JWTs are present on this AttestationOccurrence, then the `serialized_payload` SHOULD be left empty. Each JWT SHOULD encode a claim specific to the `resource_uri` of this Occurrence, but this is not validated by Grafeas metadata API implementations. The JWT itself is opaque to Grafeas."]
        #[serde(
            rename = "jwts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwts: ::std::option::Option<Vec<crate::schemas::Jwt>>,
        #[doc = "Required. The serialized payload that is verified by one or more `signatures`."]
        #[serde(
            rename = "serializedPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serialized_payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification."]
        #[serde(
            rename = "signatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signatures: ::std::option::Option<Vec<crate::schemas::Signature>>,
    }
    impl ::google_field_selector::FieldSelector for AttestationOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttestationOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BuildOccurrence {
        #[doc = "Deprecated. See InTotoStatement for the replacement. In-toto Provenance representation as defined in spec."]
        #[serde(
            rename = "intotoProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "In-toto Statement representation as defined in spec. The intoto_statement can contain any type of provenance. The serialized payload of the statement can be stored and signed in the Occurrence’s envelope."]
        #[serde(
            rename = "intotoStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_statement: ::std::option::Option<crate::schemas::InTotoStatement>,
        #[doc = "The actual provenance for the build."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::BuildProvenance>,
        #[doc = "Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
        #[serde(
            rename = "provenanceBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance_bytes: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildOccurrence {
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
    pub struct BuildProvenance {
        #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
        #[serde(
            rename = "buildOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_options: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Version string of the builder at the time this build was executed."]
        #[serde(
            rename = "builderVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_version: ::std::option::Option<String>,
        #[doc = "Output of the build."]
        #[serde(
            rename = "builtArtifacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub built_artifacts: ::std::option::Option<Vec<crate::schemas::Artifact>>,
        #[doc = "Commands requested by the build."]
        #[serde(
            rename = "commands",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commands: ::std::option::Option<Vec<crate::schemas::Command>>,
        #[doc = "Time at which the build was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "E-mail address of the user who initiated this build. Note that this was the user’s e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<String>,
        #[doc = "Time at which execution of the build was finished."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Required. Unique identifier of the build."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "URI where any logs for this provenance were written."]
        #[serde(
            rename = "logsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs_uri: ::std::option::Option<String>,
        #[doc = "ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Details of the Source input to the build."]
        #[serde(
            rename = "sourceProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_provenance: ::std::option::Option<crate::schemas::Source>,
        #[doc = "Time at which execution of the build was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
        #[serde(
            rename = "triggerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildProvenance {
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
    pub struct BuilderConfig {
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuilderConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuilderConfig {
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
    pub struct Category {
        #[doc = "The identifier of the category."]
        #[serde(
            rename = "categoryId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_id: ::std::option::Option<String>,
        #[doc = "The localized name of the category."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Category {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Category {
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
    pub struct CloudRepoSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The ID of the repo."]
        #[serde(
            rename = "repoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_id: ::std::option::Option<crate::schemas::RepoId>,
        #[doc = "A revision ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloudRepoSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudRepoSourceContext {
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
    pub struct Command {
        #[doc = "Command-line arguments used when executing this command."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Working directory (relative to project source root) used when running this command."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Environment variables set before running this command."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<Vec<String>>,
        #[doc = "Optional unique identifier for this command, used in wait_for to reference this command as a dependency."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID(s) of the command(s) that this command depends on."]
        #[serde(
            rename = "waitFor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_for: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Command {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Command {
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
    pub struct Completeness {
        #[doc = "If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that recipe.environment is claimed to be complete."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called “hermetic”."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Completeness {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Completeness {
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
    pub struct ComplianceOccurrence {
        #[serde(
            rename = "nonComplianceReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_reason: ::std::option::Option<String>,
        #[serde(
            rename = "nonCompliantFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliant_files: ::std::option::Option<Vec<crate::schemas::NonCompliantFile>>,
    }
    impl ::google_field_selector::FieldSelector for ComplianceOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComplianceOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cvss {
        #[serde(
            rename = "attackComplexity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_complexity: ::std::option::Option<crate::schemas::CvssAttackComplexity>,
        #[doc = "Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments."]
        #[serde(
            rename = "attackVector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_vector: ::std::option::Option<crate::schemas::CvssAttackVector>,
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::CvssAuthentication>,
        #[serde(
            rename = "availabilityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availability_impact: ::std::option::Option<crate::schemas::CvssAvailabilityImpact>,
        #[doc = "The base score is a function of the base metric scores."]
        #[serde(
            rename = "baseScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_score: ::std::option::Option<f32>,
        #[serde(
            rename = "confidentialityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidentiality_impact:
            ::std::option::Option<crate::schemas::CvssConfidentialityImpact>,
        #[serde(
            rename = "exploitabilityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exploitability_score: ::std::option::Option<f32>,
        #[serde(
            rename = "impactScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impact_score: ::std::option::Option<f32>,
        #[serde(
            rename = "integrityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrity_impact: ::std::option::Option<crate::schemas::CvssIntegrityImpact>,
        #[serde(
            rename = "privilegesRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privileges_required: ::std::option::Option<crate::schemas::CvssPrivilegesRequired>,
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::CvssScope>,
        #[serde(
            rename = "userInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_interaction: ::std::option::Option<crate::schemas::CvssUserInteraction>,
    }
    impl ::google_field_selector::FieldSelector for Cvss {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvss {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAttackComplexity {
        AttackComplexityHigh,
        AttackComplexityLow,
        AttackComplexityUnspecified,
    }
    impl CvssAttackComplexity {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAttackComplexity::AttackComplexityHigh => "ATTACK_COMPLEXITY_HIGH",
                CvssAttackComplexity::AttackComplexityLow => "ATTACK_COMPLEXITY_LOW",
                CvssAttackComplexity::AttackComplexityUnspecified => {
                    "ATTACK_COMPLEXITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAttackComplexity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAttackComplexity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAttackComplexity, ()> {
            Ok(match s {
                "ATTACK_COMPLEXITY_HIGH" => CvssAttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => CvssAttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    CvssAttackComplexity::AttackComplexityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAttackComplexity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAttackComplexity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAttackComplexity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_COMPLEXITY_HIGH" => CvssAttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => CvssAttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    CvssAttackComplexity::AttackComplexityUnspecified
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
    impl ::google_field_selector::FieldSelector for CvssAttackComplexity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAttackComplexity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAttackVector {
        AttackVectorAdjacent,
        AttackVectorLocal,
        AttackVectorNetwork,
        AttackVectorPhysical,
        AttackVectorUnspecified,
    }
    impl CvssAttackVector {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAttackVector::AttackVectorAdjacent => "ATTACK_VECTOR_ADJACENT",
                CvssAttackVector::AttackVectorLocal => "ATTACK_VECTOR_LOCAL",
                CvssAttackVector::AttackVectorNetwork => "ATTACK_VECTOR_NETWORK",
                CvssAttackVector::AttackVectorPhysical => "ATTACK_VECTOR_PHYSICAL",
                CvssAttackVector::AttackVectorUnspecified => "ATTACK_VECTOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAttackVector {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAttackVector {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAttackVector, ()> {
            Ok(match s {
                "ATTACK_VECTOR_ADJACENT" => CvssAttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => CvssAttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => CvssAttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => CvssAttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => CvssAttackVector::AttackVectorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAttackVector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAttackVector {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAttackVector {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_VECTOR_ADJACENT" => CvssAttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => CvssAttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => CvssAttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => CvssAttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => CvssAttackVector::AttackVectorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAttackVector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAttackVector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAuthentication {
        AuthenticationMultiple,
        AuthenticationNone,
        AuthenticationSingle,
        AuthenticationUnspecified,
    }
    impl CvssAuthentication {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAuthentication::AuthenticationMultiple => "AUTHENTICATION_MULTIPLE",
                CvssAuthentication::AuthenticationNone => "AUTHENTICATION_NONE",
                CvssAuthentication::AuthenticationSingle => "AUTHENTICATION_SINGLE",
                CvssAuthentication::AuthenticationUnspecified => "AUTHENTICATION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAuthentication {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAuthentication {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAuthentication, ()> {
            Ok(match s {
                "AUTHENTICATION_MULTIPLE" => CvssAuthentication::AuthenticationMultiple,
                "AUTHENTICATION_NONE" => CvssAuthentication::AuthenticationNone,
                "AUTHENTICATION_SINGLE" => CvssAuthentication::AuthenticationSingle,
                "AUTHENTICATION_UNSPECIFIED" => CvssAuthentication::AuthenticationUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAuthentication {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAuthentication {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAuthentication {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_MULTIPLE" => CvssAuthentication::AuthenticationMultiple,
                "AUTHENTICATION_NONE" => CvssAuthentication::AuthenticationNone,
                "AUTHENTICATION_SINGLE" => CvssAuthentication::AuthenticationSingle,
                "AUTHENTICATION_UNSPECIFIED" => CvssAuthentication::AuthenticationUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAuthentication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAuthentication {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAvailabilityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssAvailabilityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAvailabilityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssAvailabilityImpact::ImpactLow => "IMPACT_LOW",
                CvssAvailabilityImpact::ImpactNone => "IMPACT_NONE",
                CvssAvailabilityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAvailabilityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAvailabilityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAvailabilityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssAvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssAvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => CvssAvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssAvailabilityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAvailabilityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAvailabilityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAvailabilityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssAvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssAvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => CvssAvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssAvailabilityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAvailabilityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAvailabilityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssConfidentialityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssConfidentialityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssConfidentialityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssConfidentialityImpact::ImpactLow => "IMPACT_LOW",
                CvssConfidentialityImpact::ImpactNone => "IMPACT_NONE",
                CvssConfidentialityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssConfidentialityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssConfidentialityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssConfidentialityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => CvssConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssConfidentialityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssConfidentialityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssConfidentialityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssConfidentialityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => CvssConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssConfidentialityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssConfidentialityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssConfidentialityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssIntegrityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssIntegrityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssIntegrityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssIntegrityImpact::ImpactLow => "IMPACT_LOW",
                CvssIntegrityImpact::ImpactNone => "IMPACT_NONE",
                CvssIntegrityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssIntegrityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssIntegrityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssIntegrityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssIntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssIntegrityImpact::ImpactLow,
                "IMPACT_NONE" => CvssIntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssIntegrityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssIntegrityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssIntegrityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssIntegrityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssIntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssIntegrityImpact::ImpactLow,
                "IMPACT_NONE" => CvssIntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssIntegrityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssIntegrityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssIntegrityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssPrivilegesRequired {
        PrivilegesRequiredHigh,
        PrivilegesRequiredLow,
        PrivilegesRequiredNone,
        PrivilegesRequiredUnspecified,
    }
    impl CvssPrivilegesRequired {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssPrivilegesRequired::PrivilegesRequiredHigh => "PRIVILEGES_REQUIRED_HIGH",
                CvssPrivilegesRequired::PrivilegesRequiredLow => "PRIVILEGES_REQUIRED_LOW",
                CvssPrivilegesRequired::PrivilegesRequiredNone => "PRIVILEGES_REQUIRED_NONE",
                CvssPrivilegesRequired::PrivilegesRequiredUnspecified => {
                    "PRIVILEGES_REQUIRED_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssPrivilegesRequired {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssPrivilegesRequired {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssPrivilegesRequired, ()> {
            Ok(match s {
                "PRIVILEGES_REQUIRED_HIGH" => CvssPrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => CvssPrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => CvssPrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    CvssPrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssPrivilegesRequired {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssPrivilegesRequired {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssPrivilegesRequired {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVILEGES_REQUIRED_HIGH" => CvssPrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => CvssPrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => CvssPrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    CvssPrivilegesRequired::PrivilegesRequiredUnspecified
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
    impl ::google_field_selector::FieldSelector for CvssPrivilegesRequired {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssPrivilegesRequired {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssScope {
        ScopeChanged,
        ScopeUnchanged,
        ScopeUnspecified,
    }
    impl CvssScope {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssScope::ScopeChanged => "SCOPE_CHANGED",
                CvssScope::ScopeUnchanged => "SCOPE_UNCHANGED",
                CvssScope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssScope, ()> {
            Ok(match s {
                "SCOPE_CHANGED" => CvssScope::ScopeChanged,
                "SCOPE_UNCHANGED" => CvssScope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => CvssScope::ScopeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_CHANGED" => CvssScope::ScopeChanged,
                "SCOPE_UNCHANGED" => CvssScope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => CvssScope::ScopeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssUserInteraction {
        UserInteractionNone,
        UserInteractionRequired,
        UserInteractionUnspecified,
    }
    impl CvssUserInteraction {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssUserInteraction::UserInteractionNone => "USER_INTERACTION_NONE",
                CvssUserInteraction::UserInteractionRequired => "USER_INTERACTION_REQUIRED",
                CvssUserInteraction::UserInteractionUnspecified => "USER_INTERACTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssUserInteraction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssUserInteraction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssUserInteraction, ()> {
            Ok(match s {
                "USER_INTERACTION_NONE" => CvssUserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => CvssUserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => CvssUserInteraction::UserInteractionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssUserInteraction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssUserInteraction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssUserInteraction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_INTERACTION_NONE" => CvssUserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => CvssUserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => CvssUserInteraction::UserInteractionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssUserInteraction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssUserInteraction {
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
    pub struct DeploymentOccurrence {
        #[doc = "Address of the runtime element hosting this deployment."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "Configuration used to create this deployment."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<String>,
        #[doc = "Required. Beginning of the lifetime of this deployment."]
        #[serde(
            rename = "deployTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deploy_time: ::std::option::Option<String>,
        #[doc = "Platform hosting this deployment."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::DeploymentOccurrencePlatform>,
        #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<Vec<String>>,
        #[doc = "End of the lifetime of this deployment."]
        #[serde(
            rename = "undeployTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub undeploy_time: ::std::option::Option<String>,
        #[doc = "Identity of the user that triggered this deployment."]
        #[serde(
            rename = "userEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeploymentOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeploymentOccurrencePlatform {
        #[doc = "Custom user-defined platform."]
        Custom,
        #[doc = "Google App Engine: Flexible Environment."]
        Flex,
        #[doc = "Google Container Engine."]
        Gke,
        #[doc = "Unknown."]
        PlatformUnspecified,
    }
    impl DeploymentOccurrencePlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DeploymentOccurrencePlatform::Custom => "CUSTOM",
                DeploymentOccurrencePlatform::Flex => "FLEX",
                DeploymentOccurrencePlatform::Gke => "GKE",
                DeploymentOccurrencePlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeploymentOccurrencePlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeploymentOccurrencePlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeploymentOccurrencePlatform, ()> {
            Ok(match s {
                "CUSTOM" => DeploymentOccurrencePlatform::Custom,
                "FLEX" => DeploymentOccurrencePlatform::Flex,
                "GKE" => DeploymentOccurrencePlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentOccurrencePlatform::PlatformUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeploymentOccurrencePlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeploymentOccurrencePlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeploymentOccurrencePlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => DeploymentOccurrencePlatform::Custom,
                "FLEX" => DeploymentOccurrencePlatform::Flex,
                "GKE" => DeploymentOccurrencePlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentOccurrencePlatform::PlatformUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeploymentOccurrencePlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentOccurrencePlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DiscoveryOccurrence {
        #[doc = "The status of discovery for the resource."]
        #[serde(
            rename = "analysisStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status:
            ::std::option::Option<crate::schemas::DiscoveryOccurrenceAnalysisStatus>,
        #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API."]
        #[serde(
            rename = "analysisStatusError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status_error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Output only. The time occurrences related to this discovery occurrence were archived."]
        #[serde(
            rename = "archiveTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_time: ::std::option::Option<String>,
        #[doc = "Whether the resource is continuously analyzed."]
        #[serde(
            rename = "continuousAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub continuous_analysis:
            ::std::option::Option<crate::schemas::DiscoveryOccurrenceContinuousAnalysis>,
        #[doc = "The CPE of the resource being scanned."]
        #[serde(
            rename = "cpe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe: ::std::option::Option<String>,
        #[doc = "The last time this resource was scanned."]
        #[serde(
            rename = "lastScanTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_scan_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryOccurrenceAnalysisStatus {
        #[doc = "Unknown."]
        AnalysisStatusUnspecified,
        #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
        FinishedFailed,
        #[doc = "Analysis has finished successfully."]
        FinishedSuccess,
        #[doc = "The resource is known not to be supported"]
        FinishedUnsupported,
        #[doc = "Resource is known but no action has been taken yet."]
        Pending,
        #[doc = "Resource is being analyzed."]
        Scanning,
    }
    impl DiscoveryOccurrenceAnalysisStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified => {
                    "ANALYSIS_STATUS_UNSPECIFIED"
                }
                DiscoveryOccurrenceAnalysisStatus::FinishedFailed => "FINISHED_FAILED",
                DiscoveryOccurrenceAnalysisStatus::FinishedSuccess => "FINISHED_SUCCESS",
                DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported => "FINISHED_UNSUPPORTED",
                DiscoveryOccurrenceAnalysisStatus::Pending => "PENDING",
                DiscoveryOccurrenceAnalysisStatus::Scanning => "SCANNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryOccurrenceAnalysisStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryOccurrenceAnalysisStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryOccurrenceAnalysisStatus, ()> {
            Ok(match s {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveryOccurrenceAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveryOccurrenceAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveryOccurrenceAnalysisStatus::Pending,
                "SCANNING" => DiscoveryOccurrenceAnalysisStatus::Scanning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryOccurrenceAnalysisStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryOccurrenceAnalysisStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryOccurrenceAnalysisStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveryOccurrenceAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveryOccurrenceAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveryOccurrenceAnalysisStatus::Pending,
                "SCANNING" => DiscoveryOccurrenceAnalysisStatus::Scanning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrenceAnalysisStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrenceAnalysisStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryOccurrenceContinuousAnalysis {
        #[doc = "The resource is continuously analyzed."]
        Active,
        #[doc = "Unknown."]
        ContinuousAnalysisUnspecified,
        #[doc = "The resource is ignored for continuous analysis."]
        Inactive,
    }
    impl DiscoveryOccurrenceContinuousAnalysis {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryOccurrenceContinuousAnalysis::Active => "ACTIVE",
                DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified => {
                    "CONTINUOUS_ANALYSIS_UNSPECIFIED"
                }
                DiscoveryOccurrenceContinuousAnalysis::Inactive => "INACTIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryOccurrenceContinuousAnalysis {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryOccurrenceContinuousAnalysis {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryOccurrenceContinuousAnalysis, ()> {
            Ok(match s {
                "ACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Inactive,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryOccurrenceContinuousAnalysis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryOccurrenceContinuousAnalysis {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryOccurrenceContinuousAnalysis {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Inactive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrenceContinuousAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrenceContinuousAnalysis {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DsseattestationOccurrence {
        #[doc = "If doing something security critical, make sure to verify the signatures in this metadata."]
        #[serde(
            rename = "envelope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub envelope: ::std::option::Option<crate::schemas::Envelope>,
        #[serde(
            rename = "statement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statement: ::std::option::Option<crate::schemas::InTotoStatement>,
    }
    impl ::google_field_selector::FieldSelector for DsseattestationOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DsseattestationOccurrence {
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
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct Envelope {
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[serde(
            rename = "payloadType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload_type: ::std::option::Option<String>,
        #[serde(
            rename = "signatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signatures: ::std::option::Option<Vec<crate::schemas::EnvelopeSignature>>,
    }
    impl ::google_field_selector::FieldSelector for Envelope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Envelope {
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
    pub struct EnvelopeSignature {
        #[serde(
            rename = "keyid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyid: ::std::option::Option<String>,
        #[serde(
            rename = "sig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sig: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for EnvelopeSignature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvelopeSignature {
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
    pub struct FileHashes {
        #[doc = "Required. Collection of file hashes."]
        #[serde(
            rename = "fileHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hash: ::std::option::Option<Vec<crate::schemas::Hash>>,
    }
    impl ::google_field_selector::FieldSelector for FileHashes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileHashes {
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
    pub struct Fingerprint {
        #[doc = "Required. The layer ID of the final layer in the Docker image’s v1 representation."]
        #[serde(
            rename = "v1Name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_1_name: ::std::option::Option<String>,
        #[doc = "Required. The ordered list of v2 blobs that represent a given image."]
        #[serde(
            rename = "v2Blob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_2_blob: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The name of the image’s v2 blobs computed via: \\[bottom\\] := v2_blobbottom := sha256(v2_blob\\[N\\] + “ “ + v2_name\\[N+1\\]) Only the name of the final blob is kept."]
        #[serde(
            rename = "v2Name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_2_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Fingerprint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Fingerprint {
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
    pub struct GerritSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The full project name within the host. Projects may be nested, so “project/subproject” is a valid project name. The “repo name” is the hostURI/project."]
        #[serde(
            rename = "gerritProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit_project: ::std::option::Option<String>,
        #[doc = "The URI of a running Gerrit instance."]
        #[serde(
            rename = "hostUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_uri: ::std::option::Option<String>,
        #[doc = "A revision (commit) ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GerritSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GerritSourceContext {
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
    pub struct GitSourceContext {
        #[doc = "Git commit hash."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Git repository URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GitSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GitSourceContext {
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
    pub struct Hash {
        #[doc = "Required. The type of hash that was performed, e.g. “SHA-256”."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Required. The hash value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Hash {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Hash {
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
    pub struct Identity {
        #[doc = "The revision number of the update."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
        #[doc = "The revision independent identifier of the update."]
        #[serde(
            rename = "updateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Identity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Identity {
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
    pub struct ImageOccurrence {
        #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
        #[serde(
            rename = "baseResourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_resource_url: ::std::option::Option<String>,
        #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
        #[serde(
            rename = "distance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distance: ::std::option::Option<i32>,
        #[doc = "Required. The fingerprint of the derived image."]
        #[serde(
            rename = "fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fingerprint: ::std::option::Option<crate::schemas::Fingerprint>,
        #[doc = "This contains layer-specific metadata, if populated it has length “distance” and is ordered with \\[distance\\] being the layer immediately following the base image and \\[1\\] being the final layer."]
        #[serde(
            rename = "layerInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layer_info: ::std::option::Option<Vec<crate::schemas::Layer>>,
    }
    impl ::google_field_selector::FieldSelector for ImageOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InTotoProvenance {
        #[doc = "required"]
        #[serde(
            rename = "builderConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_config: ::std::option::Option<crate::schemas::BuilderConfig>,
        #[doc = "The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::Metadata>,
        #[doc = "Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required"]
        #[serde(
            rename = "recipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipe: ::std::option::Option<crate::schemas::Recipe>,
    }
    impl ::google_field_selector::FieldSelector for InTotoProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InTotoProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InTotoStatement {
        #[doc = "`https://slsa.dev/provenance/v0.1` for SlsaProvenance."]
        #[serde(
            rename = "predicateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predicate_type: ::std::option::Option<String>,
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "Always `https://in-toto.io/Statement/v0.1`."]
        #[serde(
            rename = "_type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "slsaProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slsa_provenance: ::std::option::Option<crate::schemas::SlsaProvenance>,
        #[serde(
            rename = "subject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject: ::std::option::Option<Vec<crate::schemas::Subject>>,
    }
    impl ::google_field_selector::FieldSelector for InTotoStatement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InTotoStatement {
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
    pub struct Jwt {
        #[doc = "The compact encoding of a JWS, which is always three base64 encoded strings joined by periods. For details, see: https://tools.ietf.org/html/rfc7515.html#section-3.1"]
        #[serde(
            rename = "compactJwt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compact_jwt: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Jwt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Jwt {
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
    pub struct Layer {
        #[doc = "The recovered arguments to the Dockerfile directive."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<String>,
        #[doc = "Required. The recovered Dockerfile directive used to construct this layer. See https://docs.docker.com/engine/reference/builder/ for more information."]
        #[serde(
            rename = "directive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub directive: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Layer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Layer {
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
    pub struct ListVulnerabilitiesResponse {
        #[doc = "A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of Vulnerability Occurrences resulting from a scan."]
        #[serde(
            rename = "occurrences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences: ::std::option::Option<Vec<crate::schemas::Occurrence>>,
    }
    impl ::google_field_selector::FieldSelector for ListVulnerabilitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListVulnerabilitiesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListVulnerabilitiesResponse {
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
    pub struct Location {
        #[doc = "Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The path from which we gathered that this package/version is installed."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "The version installed at this location."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::Version>,
    }
    impl ::google_field_selector::FieldSelector for Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Location {
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
    pub struct Material {
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Material {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Material {
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
    pub struct Metadata {
        #[doc = "The timestamp of when the build completed."]
        #[serde(
            rename = "buildFinishedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_finished_on: ::std::option::Option<String>,
        #[doc = "Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec."]
        #[serde(
            rename = "buildInvocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_invocation_id: ::std::option::Option<String>,
        #[doc = "The timestamp of when the build started."]
        #[serde(
            rename = "buildStartedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_started_on: ::std::option::Option<String>,
        #[doc = "Indicates that the builder claims certain fields in this message to be complete."]
        #[serde(
            rename = "completeness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completeness: ::std::option::Option<crate::schemas::Completeness>,
        #[doc = "If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output."]
        #[serde(
            rename = "reproducible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reproducible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Metadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metadata {
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
    pub struct NonCompliantFile {
        #[doc = "Command to display the non-compliant files."]
        #[serde(
            rename = "displayCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_command: ::std::option::Option<String>,
        #[doc = "Empty if `display_command` is set."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "Explains why a file is non compliant for a CIS check."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NonCompliantFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonCompliantFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Occurrence {
        #[doc = "Describes an attestation of an artifact."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation: ::std::option::Option<crate::schemas::AttestationOccurrence>,
        #[doc = "Describes a verifiable build."]
        #[serde(
            rename = "build",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build: ::std::option::Option<crate::schemas::BuildOccurrence>,
        #[doc = "Describes a compliance violation on a linked resource."]
        #[serde(
            rename = "compliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance: ::std::option::Option<crate::schemas::ComplianceOccurrence>,
        #[doc = "Output only. The time this occurrence was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Describes the deployment of an artifact on a runtime."]
        #[serde(
            rename = "deployment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment: ::std::option::Option<crate::schemas::DeploymentOccurrence>,
        #[doc = "Describes when a resource was discovered."]
        #[serde(
            rename = "discovery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery: ::std::option::Option<crate::schemas::DiscoveryOccurrence>,
        #[doc = "Describes an attestation of an artifact using dsse."]
        #[serde(
            rename = "dsseAttestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dsse_attestation: ::std::option::Option<crate::schemas::DsseattestationOccurrence>,
        #[doc = "https://github.com/secure-systems-lab/dsse"]
        #[serde(
            rename = "envelope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub envelope: ::std::option::Option<crate::schemas::Envelope>,
        #[doc = "Describes how this resource derives from the basis in the associated note."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ImageOccurrence>,
        #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::OccurrenceKind>,
        #[doc = "Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests."]
        #[serde(
            rename = "noteName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note_name: ::std::option::Option<String>,
        #[doc = "Describes the installation of a package on the linked resource."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<crate::schemas::PackageOccurrence>,
        #[doc = "A description of actions that can be taken to remedy the note."]
        #[serde(
            rename = "remediation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation: ::std::option::Option<String>,
        #[doc = "Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
        #[doc = "Output only. The time this occurrence was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Describes an available package upgrade on the linked resource."]
        #[serde(
            rename = "upgrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade: ::std::option::Option<crate::schemas::UpgradeOccurrence>,
        #[doc = "Describes a security vulnerability."]
        #[serde(
            rename = "vulnerability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability: ::std::option::Option<crate::schemas::VulnerabilityOccurrence>,
    }
    impl ::google_field_selector::FieldSelector for Occurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Occurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OccurrenceKind {
        #[doc = "This represents a logical “role” that can attest to artifacts."]
        Attestation,
        #[doc = "The note and occurrence assert build provenance."]
        Build,
        #[doc = "This represents a Compliance Note"]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployment,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        Image,
        #[doc = "Default value. This value is unused."]
        NoteKindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        Package,
        #[doc = "This represents an available package upgrade."]
        Upgrade,
        #[doc = "The note and occurrence represent a package vulnerability."]
        Vulnerability,
    }
    impl OccurrenceKind {
        pub fn as_str(self) -> &'static str {
            match self {
                OccurrenceKind::Attestation => "ATTESTATION",
                OccurrenceKind::Build => "BUILD",
                OccurrenceKind::Compliance => "COMPLIANCE",
                OccurrenceKind::Deployment => "DEPLOYMENT",
                OccurrenceKind::Discovery => "DISCOVERY",
                OccurrenceKind::DsseAttestation => "DSSE_ATTESTATION",
                OccurrenceKind::Image => "IMAGE",
                OccurrenceKind::NoteKindUnspecified => "NOTE_KIND_UNSPECIFIED",
                OccurrenceKind::Package => "PACKAGE",
                OccurrenceKind::Upgrade => "UPGRADE",
                OccurrenceKind::Vulnerability => "VULNERABILITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OccurrenceKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OccurrenceKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OccurrenceKind, ()> {
            Ok(match s {
                "ATTESTATION" => OccurrenceKind::Attestation,
                "BUILD" => OccurrenceKind::Build,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYMENT" => OccurrenceKind::Deployment,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE" => OccurrenceKind::Image,
                "NOTE_KIND_UNSPECIFIED" => OccurrenceKind::NoteKindUnspecified,
                "PACKAGE" => OccurrenceKind::Package,
                "UPGRADE" => OccurrenceKind::Upgrade,
                "VULNERABILITY" => OccurrenceKind::Vulnerability,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OccurrenceKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OccurrenceKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OccurrenceKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION" => OccurrenceKind::Attestation,
                "BUILD" => OccurrenceKind::Build,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYMENT" => OccurrenceKind::Deployment,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE" => OccurrenceKind::Image,
                "NOTE_KIND_UNSPECIFIED" => OccurrenceKind::NoteKindUnspecified,
                "PACKAGE" => OccurrenceKind::Package,
                "UPGRADE" => OccurrenceKind::Upgrade,
                "VULNERABILITY" => OccurrenceKind::Vulnerability,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OccurrenceKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OccurrenceKind {
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
    pub struct PackageData {
        #[doc = "The cpe_uri in \\[cpe format\\] (https://cpe.mitre.org/specification/) in which the vulnerability may manifest. Examples include distro or storage location for vulnerable jar."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The OS affected by a vulnerability This field is deprecated and the information is in cpe_uri"]
        #[serde(
            rename = "os",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os: ::std::option::Option<String>,
        #[doc = "The version of the OS This field is deprecated and the information is in cpe_uri"]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
        #[doc = "The package being analysed for vulnerabilities"]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "The type of package: os, maven, go, etc."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<crate::schemas::PackageDataPackageType>,
        #[doc = "The path to the jar file / go binary file. The same jar file can be in multiple locations - all of them will be listed."]
        #[serde(
            rename = "pathToFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_to_file: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "unused",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unused: ::std::option::Option<String>,
        #[doc = "The version of the package being analysed"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PackageDataPackageType {
        #[doc = "Go third-party packages."]
        Go,
        #[doc = "Go toolchain + standard library packages."]
        GoStdlib,
        #[doc = "Java packages from Maven."]
        Maven,
        #[doc = "Operating System"]
        Os,
        PackageTypeUnspecified,
    }
    impl PackageDataPackageType {
        pub fn as_str(self) -> &'static str {
            match self {
                PackageDataPackageType::Go => "GO",
                PackageDataPackageType::GoStdlib => "GO_STDLIB",
                PackageDataPackageType::Maven => "MAVEN",
                PackageDataPackageType::Os => "OS",
                PackageDataPackageType::PackageTypeUnspecified => "PACKAGE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PackageDataPackageType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PackageDataPackageType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PackageDataPackageType, ()> {
            Ok(match s {
                "GO" => PackageDataPackageType::Go,
                "GO_STDLIB" => PackageDataPackageType::GoStdlib,
                "MAVEN" => PackageDataPackageType::Maven,
                "OS" => PackageDataPackageType::Os,
                "PACKAGE_TYPE_UNSPECIFIED" => PackageDataPackageType::PackageTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PackageDataPackageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PackageDataPackageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PackageDataPackageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GO" => PackageDataPackageType::Go,
                "GO_STDLIB" => PackageDataPackageType::GoStdlib,
                "MAVEN" => PackageDataPackageType::Maven,
                "OS" => PackageDataPackageType::Os,
                "PACKAGE_TYPE_UNSPECIFIED" => PackageDataPackageType::PackageTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PackageDataPackageType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageDataPackageType {
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
    pub struct PackageIssue {
        #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was found in."]
        #[serde(
            rename = "affectedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_cpe_uri: ::std::option::Option<String>,
        #[doc = "Required. The package this vulnerability was found in."]
        #[serde(
            rename = "affectedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_package: ::std::option::Option<String>,
        #[doc = "Required. The version of the package that is installed on the resource affected by this vulnerability."]
        #[serde(
            rename = "affectedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when it is not available."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::PackageIssueEffectiveSeverity>,
        #[doc = "Output only. Whether a fix is available for this package."]
        #[serde(
            rename = "fixAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fix_available: ::std::option::Option<bool>,
        #[doc = "The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was fixed in. It is possible for this to be different from the affected_cpe_uri."]
        #[serde(
            rename = "fixedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_cpe_uri: ::std::option::Option<String>,
        #[doc = "The package this vulnerability was fixed in. It is possible for this to be different from the affected_package."]
        #[serde(
            rename = "fixedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_package: ::std::option::Option<String>,
        #[doc = "Required. The version of the package this vulnerability was fixed in. Setting this to VersionKind.MAXIMUM means no fix is yet available."]
        #[serde(
            rename = "fixedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "The type of package (e.g. OS, MAVEN, GO)."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageIssue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PackageIssueEffectiveSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl PackageIssueEffectiveSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                PackageIssueEffectiveSeverity::Critical => "CRITICAL",
                PackageIssueEffectiveSeverity::High => "HIGH",
                PackageIssueEffectiveSeverity::Low => "LOW",
                PackageIssueEffectiveSeverity::Medium => "MEDIUM",
                PackageIssueEffectiveSeverity::Minimal => "MINIMAL",
                PackageIssueEffectiveSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PackageIssueEffectiveSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PackageIssueEffectiveSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PackageIssueEffectiveSeverity, ()> {
            Ok(match s {
                "CRITICAL" => PackageIssueEffectiveSeverity::Critical,
                "HIGH" => PackageIssueEffectiveSeverity::High,
                "LOW" => PackageIssueEffectiveSeverity::Low,
                "MEDIUM" => PackageIssueEffectiveSeverity::Medium,
                "MINIMAL" => PackageIssueEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => PackageIssueEffectiveSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PackageIssueEffectiveSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PackageIssueEffectiveSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PackageIssueEffectiveSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => PackageIssueEffectiveSeverity::Critical,
                "HIGH" => PackageIssueEffectiveSeverity::High,
                "LOW" => PackageIssueEffectiveSeverity::Low,
                "MEDIUM" => PackageIssueEffectiveSeverity::Medium,
                "MINIMAL" => PackageIssueEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => PackageIssueEffectiveSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PackageIssueEffectiveSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageIssueEffectiveSeverity {
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
    pub struct PackageOccurrence {
        #[doc = "Required. All of the places within the filesystem versions of this package have been found."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "Output only. The name of the installed package."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageOccurrence {
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
    pub struct ProjectRepoId {
        #[doc = "The ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The name of the repo. Leave empty for the default repo."]
        #[serde(
            rename = "repoName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProjectRepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectRepoId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Recipe {
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were “make”, then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Since the arguments field can greatly vary in structure, depending on the builder and recipe type, this is of form “Any”."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were “make”, then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn’t come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were “make”, then this would reference the directory in which to run make as well as which target to use."]
        #[serde(
            rename = "entryPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point: ::std::option::Option<String>,
        #[doc = "Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Since the environment field can greatly vary in structure, depending on the builder and recipe type, this is of form “Any”."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Recipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Recipe {
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
    pub struct RelatedUrl {
        #[doc = "Label to describe usage of the URL."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "Specific URL associated with the resource."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelatedUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedUrl {
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
    pub struct RepoId {
        #[doc = "A combination of a project ID and a repo name."]
        #[serde(
            rename = "projectRepoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_repo_id: ::std::option::Option<crate::schemas::ProjectRepoId>,
        #[doc = "A server-assigned, globally unique identifier."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepoId {
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
    pub struct Signature {
        #[doc = "The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * “openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA” See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * “ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU” * “nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5”"]
        #[serde(
            rename = "publicKeyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key_id: ::std::option::Option<String>,
        #[doc = "The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Signature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Signature {
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
    pub struct SlsaBuilder {
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SlsaBuilder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaBuilder {
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
    pub struct SlsaCompleteness {
        #[doc = "If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that recipe.environment is claimed to be complete."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called “hermetic”."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlsaCompleteness {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaCompleteness {
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
    pub struct SlsaMetadata {
        #[doc = "The timestamp of when the build completed."]
        #[serde(
            rename = "buildFinishedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_finished_on: ::std::option::Option<String>,
        #[doc = "Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec."]
        #[serde(
            rename = "buildInvocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_invocation_id: ::std::option::Option<String>,
        #[doc = "The timestamp of when the build started."]
        #[serde(
            rename = "buildStartedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_started_on: ::std::option::Option<String>,
        #[doc = "Indicates that the builder claims certain fields in this message to be complete."]
        #[serde(
            rename = "completeness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completeness: ::std::option::Option<crate::schemas::SlsaCompleteness>,
        #[doc = "If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output."]
        #[serde(
            rename = "reproducible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reproducible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlsaMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SlsaProvenance {
        #[doc = "required"]
        #[serde(
            rename = "builder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder: ::std::option::Option<crate::schemas::SlsaBuilder>,
        #[doc = "The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<Vec<crate::schemas::Material>>,
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::SlsaMetadata>,
        #[doc = "Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required"]
        #[serde(
            rename = "recipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipe: ::std::option::Option<crate::schemas::SlsaRecipe>,
    }
    impl ::google_field_selector::FieldSelector for SlsaProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SlsaRecipe {
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were “make”, then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Depending on the recipe Type, the structure may be different."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were “make”, then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn’t come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were “make”, then this would reference the directory in which to run make as well as which target to use."]
        #[serde(
            rename = "entryPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point: ::std::option::Option<String>,
        #[doc = "Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Depending on the recipe Type, the structure may be different."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SlsaRecipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaRecipe {
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
    pub struct Source {
        #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
        #[serde(
            rename = "additionalContexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_contexts: ::std::option::Option<Vec<crate::schemas::SourceContext>>,
        #[doc = "If provided, the input binary artifacts for the build came from this location."]
        #[serde(
            rename = "artifactStorageSourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_storage_source_uri: ::std::option::Option<String>,
        #[doc = "If provided, the source code used for the build came from this location."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
        #[serde(
            rename = "fileHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hashes:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::FileHashes>>,
    }
    impl ::google_field_selector::FieldSelector for Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Source {
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
        #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
        #[serde(
            rename = "cloudRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_repo: ::std::option::Option<crate::schemas::CloudRepoSourceContext>,
        #[doc = "A SourceContext referring to a Gerrit project."]
        #[serde(
            rename = "gerrit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit: ::std::option::Option<crate::schemas::GerritSourceContext>,
        #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
        #[serde(
            rename = "git",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git: ::std::option::Option<crate::schemas::GitSourceContext>,
        #[doc = "Labels with user defined metadata."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
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
    pub struct Subject {
        #[doc = "`\"\": \"\"` Algorithms can be e.g. sha256, sha512 See https://github.com/in-toto/attestation/blob/main/spec/field_types.md#DigestSet"]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Subject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Subject {
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
    pub struct UpgradeDistribution {
        #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed. For Windows the classification is one of the category_ids listed at https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)"]
        #[serde(
            rename = "classification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub classification: ::std::option::Option<String>,
        #[doc = "Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The cve tied to this Upgrade."]
        #[serde(
            rename = "cve",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cve: ::std::option::Option<Vec<String>>,
        #[doc = "The severity as specified by the upstream operating system."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeDistribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeDistribution {
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
    pub struct UpgradeOccurrence {
        #[doc = "Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<crate::schemas::UpgradeDistribution>,
        #[doc = "Required for non-Windows OS. The package this Upgrade is for."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "Required for non-Windows OS. The version of the package in a machine + human readable form."]
        #[serde(
            rename = "parsedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parsed_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Required for Windows OS. Represents the metadata about the Windows update."]
        #[serde(
            rename = "windowsUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_update: ::std::option::Option<crate::schemas::WindowsUpdate>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeOccurrence {
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
    pub struct Version {
        #[doc = "Used to correct mistakes in the version numbering scheme."]
        #[serde(
            rename = "epoch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub epoch: ::std::option::Option<i32>,
        #[doc = "Human readable version string. This string is of the form :- and is only set when kind is NORMAL."]
        #[serde(
            rename = "fullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_name: ::std::option::Option<String>,
        #[doc = "Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range."]
        #[serde(
            rename = "inclusive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inclusive: ::std::option::Option<bool>,
        #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::VersionKind>,
        #[doc = "Required only when version kind is NORMAL. The main part of the version name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The iteration of the package build from the above version."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Version {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Version {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VersionKind {
        #[doc = "A special version representing positive infinity."]
        Maximum,
        #[doc = "A special version representing negative infinity."]
        Minimum,
        #[doc = "A standard package version."]
        Normal,
        #[doc = "Unknown."]
        VersionKindUnspecified,
    }
    impl VersionKind {
        pub fn as_str(self) -> &'static str {
            match self {
                VersionKind::Maximum => "MAXIMUM",
                VersionKind::Minimum => "MINIMUM",
                VersionKind::Normal => "NORMAL",
                VersionKind::VersionKindUnspecified => "VERSION_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VersionKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VersionKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VersionKind, ()> {
            Ok(match s {
                "MAXIMUM" => VersionKind::Maximum,
                "MINIMUM" => VersionKind::Minimum,
                "NORMAL" => VersionKind::Normal,
                "VERSION_KIND_UNSPECIFIED" => VersionKind::VersionKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VersionKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VersionKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VersionKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MAXIMUM" => VersionKind::Maximum,
                "MINIMUM" => VersionKind::Minimum,
                "NORMAL" => VersionKind::Normal,
                "VERSION_KIND_UNSPECIFIED" => VersionKind::VersionKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VersionKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VersionKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityOccurrence {
        #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
        #[serde(
            rename = "cvssScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_score: ::std::option::Option<f32>,
        #[doc = "The cvss v3 score for the vulnerability."]
        #[serde(
            rename = "cvssv3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvssv_3: ::std::option::Option<crate::schemas::Cvss>,
        #[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::VulnerabilityOccurrenceEffectiveSeverity>,
        #[doc = "Output only. Whether at least one of the affected packages has a fix available."]
        #[serde(
            rename = "fixAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fix_available: ::std::option::Option<bool>,
        #[doc = "Output only. A detailed description of this vulnerability."]
        #[serde(
            rename = "longDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_description: ::std::option::Option<String>,
        #[doc = "Required. The set of affected locations and their fixes (if available) within the associated resource."]
        #[serde(
            rename = "packageIssue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_issue: ::std::option::Option<Vec<crate::schemas::PackageIssue>>,
        #[doc = "The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. URLs related to this vulnerability."]
        #[serde(
            rename = "relatedUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_urls: ::std::option::Option<Vec<crate::schemas::RelatedUrl>>,
        #[doc = "Output only. The note provider assigned severity of this vulnerability."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::VulnerabilityOccurrenceSeverity>,
        #[doc = "Output only. A one sentence description of this vulnerability."]
        #[serde(
            rename = "shortDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_description: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityOccurrenceEffectiveSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl VulnerabilityOccurrenceEffectiveSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityOccurrenceEffectiveSeverity::Critical => "CRITICAL",
                VulnerabilityOccurrenceEffectiveSeverity::High => "HIGH",
                VulnerabilityOccurrenceEffectiveSeverity::Low => "LOW",
                VulnerabilityOccurrenceEffectiveSeverity::Medium => "MEDIUM",
                VulnerabilityOccurrenceEffectiveSeverity::Minimal => "MINIMAL",
                VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified => {
                    "SEVERITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityOccurrenceEffectiveSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityOccurrenceEffectiveSeverity {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<VulnerabilityOccurrenceEffectiveSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityOccurrenceEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceEffectiveSeverity::High,
                "LOW" => VulnerabilityOccurrenceEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityOccurrenceEffectiveSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityOccurrenceEffectiveSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityOccurrenceEffectiveSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityOccurrenceEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceEffectiveSeverity::High,
                "LOW" => VulnerabilityOccurrenceEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified
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
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrenceEffectiveSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrenceEffectiveSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityOccurrenceSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl VulnerabilityOccurrenceSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityOccurrenceSeverity::Critical => "CRITICAL",
                VulnerabilityOccurrenceSeverity::High => "HIGH",
                VulnerabilityOccurrenceSeverity::Low => "LOW",
                VulnerabilityOccurrenceSeverity::Medium => "MEDIUM",
                VulnerabilityOccurrenceSeverity::Minimal => "MINIMAL",
                VulnerabilityOccurrenceSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityOccurrenceSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityOccurrenceSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityOccurrenceSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityOccurrenceSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceSeverity::High,
                "LOW" => VulnerabilityOccurrenceSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityOccurrenceSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityOccurrenceSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityOccurrenceSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityOccurrenceSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityOccurrenceSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceSeverity::High,
                "LOW" => VulnerabilityOccurrenceSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityOccurrenceSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrenceSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrenceSeverity {
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
    pub struct WindowsUpdate {
        #[doc = "The list of categories to which the update belongs."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<crate::schemas::Category>>,
        #[doc = "The localized description of the update."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required - The unique identifier for the update."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity: ::std::option::Option<crate::schemas::Identity>,
        #[doc = "The Microsoft Knowledge Base article IDs that are associated with the update."]
        #[serde(
            rename = "kbArticleIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kb_article_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The last published timestamp of the update."]
        #[serde(
            rename = "lastPublishedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_published_timestamp: ::std::option::Option<String>,
        #[doc = "The hyperlink to the support information for the update."]
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<String>,
        #[doc = "The localized title of the update."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdate {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::projects::locations::operations::OperationsActions
                {
                    crate::resources::projects::locations::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the scans resource"]
                pub fn scans(&self) -> crate::resources::projects::locations::scans::ScansActions {
                    crate::resources::projects::locations::scans::ScansActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
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
                    #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn’t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
                    pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                        CancelRequestBuilder {
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
                    #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn’t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
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
                            name: name.into(),
                        }
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
                    #[doc = "Lists operations that match the specified filter in the request. If the server doesn’t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
                    pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
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
                            name: name.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done."]
                    pub fn wait(&self, name: impl Into<String>) -> WaitRequestBuilder {
                        WaitRequestBuilder {
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
                            timeout: None,
                        }
                    }
                }
                #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                #[derive(Debug, Clone)]
                pub struct CancelRequestBuilder<'a> {
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
                impl<'a> CancelRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":cancel");
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
                #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                    name: String,
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
                    #[doc = "The standard list filter."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The standard list page size."]
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                        T: ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
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
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>>
                           + 'a {
                        self.stream_operations_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_operations_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>>
                           + 'a {
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
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/operations");
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
                #[doc = "Created via [OperationsActions::wait()](struct.OperationsActions.html#method.wait)"]
                #[derive(Debug, Clone)]
                pub struct WaitRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    timeout: ::std::option::Option<String>,
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
                impl<'a> WaitRequestBuilder<'a> {
                    #[doc = "The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used."]
                    pub fn timeout(mut self, value: impl Into<String>) -> Self {
                        self.timeout = Some(value.into());
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":wait");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("timeout", &self.timeout)]);
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
            pub mod scans {
                pub mod params {}
                pub struct ScansActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ScansActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Initiates an analysis of the provided packages."]
                    pub fn analyze_packages(
                        &self,
                        request: crate::schemas::AnalyzePackagesRequest,
                        parent: impl Into<String>,
                    ) -> AnalyzePackagesRequestBuilder {
                        AnalyzePackagesRequestBuilder {
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the vulnerabilities resource"]                    pub fn vulnerabilities (& self) -> crate :: resources :: projects :: locations :: scans :: vulnerabilities :: VulnerabilitiesActions{
                        crate :: resources :: projects :: locations :: scans :: vulnerabilities :: VulnerabilitiesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [ScansActions::analyze_packages()](struct.ScansActions.html#method.analyze_packages)"]
                #[derive(Debug, Clone)]
                pub struct AnalyzePackagesRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::AnalyzePackagesRequest,
                    parent: String,
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
                impl<'a> AnalyzePackagesRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                        let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/scans:analyzePackages");
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
                pub mod vulnerabilities {
                    pub mod params {}
                    pub struct VulnerabilitiesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> VulnerabilitiesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Lists vulnerabilities resulting from a successfully completed scan."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                                parent: parent.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [VulnerabilitiesActions::list()](struct.VulnerabilitiesActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
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
                        #[doc = "The number of vulnerabilities to retrieve."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "The page token, resulting from a previous call to ListVulnerabilities."]
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
                        #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_occurrences<T>(
                            self,
                        ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector
                                + 'a,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: ::std::option::Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.stream_occurrences_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_occurrences_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Occurrence, crate::Error>,
                        > + 'a {
                            self.stream_occurrences_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_occurrences_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Occurrence, crate::Error>,
                        > + 'a {
                            self.stream_occurrences_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_occurrences_with_fields<T, F>(
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
                                #[serde(rename = "occurrences")]
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
                                let mut selector =
                                    concat!("nextPageToken,", "occurrences").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
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
                            Item = Result<
                                crate::schemas::ListVulnerabilitiesResponse,
                                crate::Error,
                            >,
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
                            Item = Result<
                                crate::schemas::ListVulnerabilitiesResponse,
                                crate::Error,
                            >,
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
                            let mut fields =
                                fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<crate::schemas::ListVulnerabilitiesResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListVulnerabilitiesResponse, crate::Error>
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
                            let mut output = "https://ondemandscanning.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/vulnerabilities");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
