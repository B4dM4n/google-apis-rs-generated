#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [organizations](resources/organizations/struct.OrganizationsActions.html)\n  * [protected_resources](resources/organizations/protected_resources/struct.ProtectedResourcesActions.html)\n    * [*search*](resources/organizations/protected_resources/struct.SearchRequestBuilder.html)\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [crypto_keys](resources/projects/crypto_keys/struct.CryptoKeysActions.html)\n    * [*list*](resources/projects/crypto_keys/struct.ListRequestBuilder.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [key_rings](resources/projects/locations/key_rings/struct.KeyRingsActions.html)\n      * [crypto_keys](resources/projects/locations/key_rings/crypto_keys/struct.CryptoKeysActions.html)\n        * [*getProtectedResourcesSummary*](resources/projects/locations/key_rings/crypto_keys/struct.GetProtectedResourcesSummaryRequestBuilder.html)\n"]
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
    pub struct GoogleCloudKmsInventoryV1ListCryptoKeysResponse {
        #[doc = "The list of CryptoKeys."]
        #[serde(
            rename = "cryptoKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_keys: ::std::option::Option<Vec<crate::schemas::GoogleCloudKmsV1CryptoKey>>,
        #[doc = "The page token returned from the previous response if the next page is desired."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsInventoryV1ListCryptoKeysResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsInventoryV1ListCryptoKeysResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleCloudKmsInventoryV1ListCryptoKeysResponse {
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
    pub struct GoogleCloudKmsInventoryV1ProtectedResource {
        #[doc = "The Cloud product that owns the resource. Example: `compute`"]
        #[serde(
            rename = "cloudProduct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_product: ::std::option::Option<String>,
        #[doc = "Output only. The time at which this resource was created. The granularity is in seconds. Timestamp.nanos will always be 0."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The name of the Cloud KMS [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en) used to protect this resource via CMEK. This field is empty if the Google Cloud product owning the resource does not provide key version data to Asset Inventory. If there are multiple key versions protecting the resource, then this is same value as the first element of crypto_key_versions."]
        #[serde(
            rename = "cryptoKeyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_version: ::std::option::Option<String>,
        #[doc = "The names of the Cloud KMS [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en) used to protect this resource via CMEK. This field is empty if the Google Cloud product owning the resource does not provide key versions data to Asset Inventory. The first element of this field is stored in crypto_key_version."]
        #[serde(
            rename = "cryptoKeyVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_versions: ::std::option::Option<Vec<String>>,
        #[doc = "A key-value pair of the resource’s labels (v1) to their values."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The full resource name of the resource. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Format: `projects/{PROJECT_NUMBER}`."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The ID of the project that owns the resource."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Example: `compute.googleapis.com/Disk`"]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsInventoryV1ProtectedResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsInventoryV1ProtectedResource {
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
    pub struct GoogleCloudKmsInventoryV1ProtectedResourcesSummary {
        #[doc = "The number of resources protected by the key grouped by Cloud product."]
        #[serde(
            rename = "cloudProducts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_products: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "The number of resources protected by the key grouped by region."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "The full name of the ProtectedResourcesSummary resource. Example: projects/test-project/locations/us/keyRings/test-keyring/cryptoKeys/test-key/protectedResourcesSummary"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The number of distinct Cloud projects in the same Cloud organization as the key that have resources protected by the key."]
        #[serde(
            rename = "projectCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_count: ::std::option::Option<i32>,
        #[doc = "The total number of protected resources in the same Cloud organization as the key."]
        #[serde(
            rename = "resourceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub resource_count: ::std::option::Option<i64>,
        #[doc = "The number of resources protected by the key grouped by resource type."]
        #[serde(
            rename = "resourceTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_types: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsInventoryV1ProtectedResourcesSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsInventoryV1ProtectedResourcesSummary {
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
    pub struct GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse {
        #[doc = "A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Protected resources for this page."]
        #[serde(
            rename = "protectedResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_resources:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudKmsInventoryV1ProtectedResource>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse {
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
    pub struct GoogleCloudKmsV1CryptoKey {
        #[doc = "Output only. The time at which this CryptoKey was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Immutable. The resource name of the backend environment where the key material for all CryptoKeyVersions associated with this CryptoKey reside and where all related cryptographic operations are performed. Only applicable if CryptoKeyVersions have a ProtectionLevel of EXTERNAL_VPC, with the resource name in the format `projects/*/locations/*/ekmConnections/*`. Note, this list is non-exhaustive and may apply to additional ProtectionLevels in the future."]
        #[serde(
            rename = "cryptoKeyBackend",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_backend: ::std::option::Option<String>,
        #[doc = "Immutable. The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED. If not specified at creation time, the default duration is 24 hours."]
        #[serde(
            rename = "destroyScheduledDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destroy_scheduled_duration: ::std::option::Option<String>,
        #[doc = "Immutable. Whether this key may contain imported versions only."]
        #[serde(
            rename = "importOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_only: ::std::option::Option<bool>,
        #[doc = "Labels with user-defined metadata. For more information, see [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys)."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "At next_rotation_time, the Key Management Service will automatically: 1. Create a new version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted."]
        #[serde(
            rename = "nextRotationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_rotation_time: ::std::option::Option<String>,
        #[doc = "Output only. A copy of the “primary” CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey’s primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted."]
        #[serde(
            rename = "primary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary: ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyVersion>,
        #[doc = "Immutable. The immutable purpose of this CryptoKey."]
        #[serde(
            rename = "purpose",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub purpose: ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyPurpose>,
        #[doc = "next_rotation_time will be advanced by this period when the service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted."]
        #[serde(
            rename = "rotationPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotation_period: ::std::option::Option<String>,
        #[doc = "A template describing settings for new CryptoKeyVersion instances. The properties of new CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are controlled by this template."]
        #[serde(
            rename = "versionTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_template:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyVersionTemplate>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKey {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyPurpose {
        #[doc = "CryptoKeys with this purpose may be used with AsymmetricDecrypt and GetPublicKey."]
        AsymmetricDecrypt,
        #[doc = "CryptoKeys with this purpose may be used with AsymmetricSign and GetPublicKey."]
        AsymmetricSign,
        #[doc = "Not specified."]
        CryptoKeyPurposeUnspecified,
        #[doc = "CryptoKeys with this purpose may be used with Encrypt and Decrypt."]
        EncryptDecrypt,
        #[doc = "CryptoKeys with this purpose may be used with MacSign."]
        Mac,
    }
    impl GoogleCloudKmsV1CryptoKeyPurpose {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricDecrypt => "ASYMMETRIC_DECRYPT",
                GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricSign => "ASYMMETRIC_SIGN",
                GoogleCloudKmsV1CryptoKeyPurpose::CryptoKeyPurposeUnspecified => {
                    "CRYPTO_KEY_PURPOSE_UNSPECIFIED"
                }
                GoogleCloudKmsV1CryptoKeyPurpose::EncryptDecrypt => "ENCRYPT_DECRYPT",
                GoogleCloudKmsV1CryptoKeyPurpose::Mac => "MAC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyPurpose {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyPurpose {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyPurpose, ()> {
            Ok(match s {
                "ASYMMETRIC_DECRYPT" => GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricDecrypt,
                "ASYMMETRIC_SIGN" => GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricSign,
                "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyPurpose::CryptoKeyPurposeUnspecified
                }
                "ENCRYPT_DECRYPT" => GoogleCloudKmsV1CryptoKeyPurpose::EncryptDecrypt,
                "MAC" => GoogleCloudKmsV1CryptoKeyPurpose::Mac,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyPurpose {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyPurpose {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyPurpose {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASYMMETRIC_DECRYPT" => GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricDecrypt,
                "ASYMMETRIC_SIGN" => GoogleCloudKmsV1CryptoKeyPurpose::AsymmetricSign,
                "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyPurpose::CryptoKeyPurposeUnspecified
                }
                "ENCRYPT_DECRYPT" => GoogleCloudKmsV1CryptoKeyPurpose::EncryptDecrypt,
                "MAC" => GoogleCloudKmsV1CryptoKeyPurpose::Mac,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyPurpose {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyPurpose {
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
    pub struct GoogleCloudKmsV1CryptoKeyVersion {
        #[doc = "Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyVersionAlgorithm>,
        #[doc = "Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1KeyOperationAttestation>,
        #[doc = "Output only. The time at which this CryptoKeyVersion was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this CryptoKeyVersion’s key material was destroyed. Only present if state is DESTROYED."]
        #[serde(
            rename = "destroyEventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destroy_event_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this CryptoKeyVersion’s key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED."]
        #[serde(
            rename = "destroyTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destroy_time: ::std::option::Option<String>,
        #[doc = "ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels."]
        #[serde(
            rename = "externalProtectionLevelOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_protection_level_options:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1ExternalProtectionLevelOptions>,
        #[doc = "Output only. The time this CryptoKeyVersion’s key material was generated."]
        #[serde(
            rename = "generateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub generate_time: ::std::option::Option<String>,
        #[doc = "Output only. The root cause of the most recent import failure. Only present if state is IMPORT_FAILED."]
        #[serde(
            rename = "importFailureReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_failure_reason: ::std::option::Option<String>,
        #[doc = "Output only. The name of the ImportJob used in the most recent import of this CryptoKeyVersion. Only present if the underlying key material was imported."]
        #[serde(
            rename = "importJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_job: ::std::option::Option<String>,
        #[doc = "Output only. The time at which this CryptoKeyVersion’s key material was most recently imported."]
        #[serde(
            rename = "importTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_time: ::std::option::Option<String>,
        #[doc = "Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyVersionProtectionLevel>,
        #[doc = "Output only. Whether or not this key version is eligible for reimport, by being specified as a target in ImportCryptoKeyVersionRequest.crypto_key_version."]
        #[serde(
            rename = "reimportEligible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reimport_eligible: ::std::option::Option<bool>,
        #[doc = "The current state of the CryptoKeyVersion."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudKmsV1CryptoKeyVersionState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[doc = "ECDSA on the non-NIST secp256k1 curve. This curve is only supported for HSM protection level."]
        EcSignSecp256K1Sha256,
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[doc = "HMAC-SHA1 signing with a 160 bit key."]
        HmacSha1,
        #[doc = "HMAC-SHA224 signing with a 224 bit key."]
        HmacSha224,
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
        #[doc = "HMAC-SHA384 signing with a 384 bit key."]
        HmacSha384,
        #[doc = "HMAC-SHA512 signing with a 512 bit key."]
        HmacSha512,
        #[doc = "RSAES-OAEP 2048 bit key with a SHA1 digest."]
        RsaDecryptOaep2048Sha1,
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[doc = "RSAES-OAEP 3072 bit key with a SHA1 digest."]
        RsaDecryptOaep3072Sha1,
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA1 digest."]
        RsaDecryptOaep4096Sha1,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key."]
        RsaSignRawPkcs12048,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key."]
        RsaSignRawPkcs13072,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key."]
        RsaSignRawPkcs14096,
    }
    impl GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP256Sha256 => {
                    "EC_SIGN_P256_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP384Sha384 => {
                    "EC_SIGN_P384_SHA384"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256 => {
                    "EC_SIGN_SECP256K1_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption => {
                    "EXTERNAL_SYMMETRIC_ENCRYPTION"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption => {
                    "GOOGLE_SYMMETRIC_ENCRYPTION"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha1 => "HMAC_SHA1",
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha224 => "HMAC_SHA224",
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha256 => "HMAC_SHA256",
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha384 => "HMAC_SHA384",
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha512 => "HMAC_SHA512",
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1 => {
                    "RSA_DECRYPT_OAEP_2048_SHA1"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256 => {
                    "RSA_DECRYPT_OAEP_2048_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1 => {
                    "RSA_DECRYPT_OAEP_3072_SHA1"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256 => {
                    "RSA_DECRYPT_OAEP_3072_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1 => {
                    "RSA_DECRYPT_OAEP_4096_SHA1"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256 => {
                    "RSA_DECRYPT_OAEP_4096_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512 => {
                    "RSA_DECRYPT_OAEP_4096_SHA512"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256 => {
                    "RSA_SIGN_PKCS1_2048_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256 => {
                    "RSA_SIGN_PKCS1_3072_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256 => {
                    "RSA_SIGN_PKCS1_4096_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512 => {
                    "RSA_SIGN_PKCS1_4096_SHA512"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256 => {
                    "RSA_SIGN_PSS_2048_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256 => {
                    "RSA_SIGN_PSS_3072_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256 => {
                    "RSA_SIGN_PSS_4096_SHA256"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512 => {
                    "RSA_SIGN_PSS_4096_SHA512"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048 => {
                    "RSA_SIGN_RAW_PKCS1_2048"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072 => {
                    "RSA_SIGN_RAW_PKCS1_3072"
                }
                GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096 => {
                    "RSA_SIGN_RAW_PKCS1_4096"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyVersionAlgorithm, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP256Sha256
                }
                "EC_SIGN_P384_SHA384" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP384Sha384
                }
                "EC_SIGN_SECP256K1_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA1" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha1,
                "HMAC_SHA224" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha224,
                "HMAC_SHA256" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha256,
                "HMAC_SHA384" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha384,
                "HMAC_SHA512" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha512,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048
                }
                "RSA_SIGN_RAW_PKCS1_3072" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072
                }
                "RSA_SIGN_RAW_PKCS1_4096" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP256Sha256
                }
                "EC_SIGN_P384_SHA384" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignP384Sha384
                }
                "EC_SIGN_SECP256K1_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA1" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha1,
                "HMAC_SHA224" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha224,
                "HMAC_SHA256" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha256,
                "HMAC_SHA384" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha384,
                "HMAC_SHA512" => GoogleCloudKmsV1CryptoKeyVersionAlgorithm::HmacSha512,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048
                }
                "RSA_SIGN_RAW_PKCS1_3072" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072
                }
                "RSA_SIGN_RAW_PKCS1_4096" => {
                    GoogleCloudKmsV1CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096
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
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersionAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
        #[doc = "Crypto operations are performed in an EKM-over-VPC backend."]
        ExternalVpc,
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[doc = "Crypto operations are performed in software."]
        Software,
    }
    impl GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::External => "EXTERNAL",
                GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Hsm => "HSM",
                GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyVersionProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::External,
                "EXTERNAL_VPC" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ExternalVpc,
                "HSM" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::External,
                "EXTERNAL_VPC" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ExternalVpc,
                "HSM" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => GoogleCloudKmsV1CryptoKeyVersionProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersionProtectionLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyVersionState {
        #[doc = "Not specified."]
        CryptoKeyVersionStateUnspecified,
        #[doc = "This version is scheduled for destruction, and will be destroyed soon. Call RestoreCryptoKeyVersion to put it back into the DISABLED state."]
        DestroyScheduled,
        #[doc = "This version is destroyed, and the key material is no longer stored. This version may only become ENABLED again if this version is reimport_eligible and the original key material is reimported with a call to KeyManagementService.ImportCryptoKeyVersion."]
        Destroyed,
        #[doc = "This version may not be used, but the key material is still available, and the version can be placed back into the ENABLED state."]
        Disabled,
        #[doc = "This version may be used for cryptographic operations."]
        Enabled,
        #[doc = "This version was not imported successfully. It may not be used, enabled, disabled, or destroyed. The submitted key material has been discarded. Additional details can be found in CryptoKeyVersion.import_failure_reason."]
        ImportFailed,
        #[doc = "This version is still being generated. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready."]
        PendingGeneration,
        #[doc = "This version is still being imported. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready."]
        PendingImport,
    }
    impl GoogleCloudKmsV1CryptoKeyVersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudKmsV1CryptoKeyVersionState::CryptoKeyVersionStateUnspecified => {
                    "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED"
                }
                GoogleCloudKmsV1CryptoKeyVersionState::DestroyScheduled => "DESTROY_SCHEDULED",
                GoogleCloudKmsV1CryptoKeyVersionState::Destroyed => "DESTROYED",
                GoogleCloudKmsV1CryptoKeyVersionState::Disabled => "DISABLED",
                GoogleCloudKmsV1CryptoKeyVersionState::Enabled => "ENABLED",
                GoogleCloudKmsV1CryptoKeyVersionState::ImportFailed => "IMPORT_FAILED",
                GoogleCloudKmsV1CryptoKeyVersionState::PendingGeneration => "PENDING_GENERATION",
                GoogleCloudKmsV1CryptoKeyVersionState::PendingImport => "PENDING_IMPORT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyVersionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyVersionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyVersionState, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionState::CryptoKeyVersionStateUnspecified
                }
                "DESTROY_SCHEDULED" => GoogleCloudKmsV1CryptoKeyVersionState::DestroyScheduled,
                "DESTROYED" => GoogleCloudKmsV1CryptoKeyVersionState::Destroyed,
                "DISABLED" => GoogleCloudKmsV1CryptoKeyVersionState::Disabled,
                "ENABLED" => GoogleCloudKmsV1CryptoKeyVersionState::Enabled,
                "IMPORT_FAILED" => GoogleCloudKmsV1CryptoKeyVersionState::ImportFailed,
                "PENDING_GENERATION" => GoogleCloudKmsV1CryptoKeyVersionState::PendingGeneration,
                "PENDING_IMPORT" => GoogleCloudKmsV1CryptoKeyVersionState::PendingImport,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyVersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyVersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyVersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => {
                    GoogleCloudKmsV1CryptoKeyVersionState::CryptoKeyVersionStateUnspecified
                }
                "DESTROY_SCHEDULED" => GoogleCloudKmsV1CryptoKeyVersionState::DestroyScheduled,
                "DESTROYED" => GoogleCloudKmsV1CryptoKeyVersionState::Destroyed,
                "DISABLED" => GoogleCloudKmsV1CryptoKeyVersionState::Disabled,
                "ENABLED" => GoogleCloudKmsV1CryptoKeyVersionState::Enabled,
                "IMPORT_FAILED" => GoogleCloudKmsV1CryptoKeyVersionState::ImportFailed,
                "PENDING_GENERATION" => GoogleCloudKmsV1CryptoKeyVersionState::PendingGeneration,
                "PENDING_IMPORT" => GoogleCloudKmsV1CryptoKeyVersionState::PendingImport,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersionState {
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
    pub struct GoogleCloudKmsV1CryptoKeyVersionTemplate {
        #[doc = "Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<
            crate::schemas::GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm,
        >,
        #[doc = "ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<
            crate::schemas::GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersionTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersionTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[doc = "ECDSA on the non-NIST secp256k1 curve. This curve is only supported for HSM protection level."]
        EcSignSecp256K1Sha256,
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[doc = "HMAC-SHA1 signing with a 160 bit key."]
        HmacSha1,
        #[doc = "HMAC-SHA224 signing with a 224 bit key."]
        HmacSha224,
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
        #[doc = "HMAC-SHA384 signing with a 384 bit key."]
        HmacSha384,
        #[doc = "HMAC-SHA512 signing with a 512 bit key."]
        HmacSha512,
        #[doc = "RSAES-OAEP 2048 bit key with a SHA1 digest."]
        RsaDecryptOaep2048Sha1,
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[doc = "RSAES-OAEP 3072 bit key with a SHA1 digest."]
        RsaDecryptOaep3072Sha1,
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA1 digest."]
        RsaDecryptOaep4096Sha1,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key."]
        RsaSignRawPkcs12048,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key."]
        RsaSignRawPkcs13072,
        #[doc = "RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key."]
        RsaSignRawPkcs14096,
    }
    impl GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: CryptoKeyVersionAlgorithmUnspecified => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP256Sha256 => "EC_SIGN_P256_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP384Sha384 => "EC_SIGN_P384_SHA384" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignSecp256K1Sha256 => "EC_SIGN_SECP256K1_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: ExternalSymmetricEncryption => "EXTERNAL_SYMMETRIC_ENCRYPTION" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: GoogleSymmetricEncryption => "GOOGLE_SYMMETRIC_ENCRYPTION" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha1 => "HMAC_SHA1" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha224 => "HMAC_SHA224" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha256 => "HMAC_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha384 => "HMAC_SHA384" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha512 => "HMAC_SHA512" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha1 => "RSA_DECRYPT_OAEP_2048_SHA1" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha256 => "RSA_DECRYPT_OAEP_2048_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha1 => "RSA_DECRYPT_OAEP_3072_SHA1" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha256 => "RSA_DECRYPT_OAEP_3072_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha1 => "RSA_DECRYPT_OAEP_4096_SHA1" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha256 => "RSA_DECRYPT_OAEP_4096_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha512 => "RSA_DECRYPT_OAEP_4096_SHA512" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs12048Sha256 => "RSA_SIGN_PKCS1_2048_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs13072Sha256 => "RSA_SIGN_PKCS1_3072_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha256 => "RSA_SIGN_PKCS1_4096_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha512 => "RSA_SIGN_PKCS1_4096_SHA512" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss2048Sha256 => "RSA_SIGN_PSS_2048_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss3072Sha256 => "RSA_SIGN_PSS_3072_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha256 => "RSA_SIGN_PSS_4096_SHA256" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha512 => "RSA_SIGN_PSS_4096_SHA512" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs12048 => "RSA_SIGN_RAW_PKCS1_2048" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs13072 => "RSA_SIGN_RAW_PKCS1_3072" , GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs14096 => "RSA_SIGN_RAW_PKCS1_4096" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm, ()> {
            Ok (match s { "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: CryptoKeyVersionAlgorithmUnspecified , "EC_SIGN_P256_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP256Sha256 , "EC_SIGN_P384_SHA384" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP384Sha384 , "EC_SIGN_SECP256K1_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignSecp256K1Sha256 , "EXTERNAL_SYMMETRIC_ENCRYPTION" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: ExternalSymmetricEncryption , "GOOGLE_SYMMETRIC_ENCRYPTION" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: GoogleSymmetricEncryption , "HMAC_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha1 , "HMAC_SHA224" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha224 , "HMAC_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha256 , "HMAC_SHA384" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha384 , "HMAC_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha512 , "RSA_DECRYPT_OAEP_2048_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha1 , "RSA_DECRYPT_OAEP_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha256 , "RSA_DECRYPT_OAEP_3072_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha1 , "RSA_DECRYPT_OAEP_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha256 , "RSA_DECRYPT_OAEP_4096_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha1 , "RSA_DECRYPT_OAEP_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha256 , "RSA_DECRYPT_OAEP_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha512 , "RSA_SIGN_PKCS1_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs12048Sha256 , "RSA_SIGN_PKCS1_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs13072Sha256 , "RSA_SIGN_PKCS1_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha256 , "RSA_SIGN_PKCS1_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha512 , "RSA_SIGN_PSS_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss2048Sha256 , "RSA_SIGN_PSS_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss3072Sha256 , "RSA_SIGN_PSS_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha256 , "RSA_SIGN_PSS_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha512 , "RSA_SIGN_RAW_PKCS1_2048" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs12048 , "RSA_SIGN_RAW_PKCS1_3072" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs13072 , "RSA_SIGN_RAW_PKCS1_4096" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs14096 , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: CryptoKeyVersionAlgorithmUnspecified , "EC_SIGN_P256_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP256Sha256 , "EC_SIGN_P384_SHA384" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignP384Sha384 , "EC_SIGN_SECP256K1_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: EcSignSecp256K1Sha256 , "EXTERNAL_SYMMETRIC_ENCRYPTION" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: ExternalSymmetricEncryption , "GOOGLE_SYMMETRIC_ENCRYPTION" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: GoogleSymmetricEncryption , "HMAC_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha1 , "HMAC_SHA224" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha224 , "HMAC_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha256 , "HMAC_SHA384" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha384 , "HMAC_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: HmacSha512 , "RSA_DECRYPT_OAEP_2048_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha1 , "RSA_DECRYPT_OAEP_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep2048Sha256 , "RSA_DECRYPT_OAEP_3072_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha1 , "RSA_DECRYPT_OAEP_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep3072Sha256 , "RSA_DECRYPT_OAEP_4096_SHA1" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha1 , "RSA_DECRYPT_OAEP_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha256 , "RSA_DECRYPT_OAEP_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaDecryptOaep4096Sha512 , "RSA_SIGN_PKCS1_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs12048Sha256 , "RSA_SIGN_PKCS1_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs13072Sha256 , "RSA_SIGN_PKCS1_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha256 , "RSA_SIGN_PKCS1_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPkcs14096Sha512 , "RSA_SIGN_PSS_2048_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss2048Sha256 , "RSA_SIGN_PSS_3072_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss3072Sha256 , "RSA_SIGN_PSS_4096_SHA256" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha256 , "RSA_SIGN_PSS_4096_SHA512" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignPss4096Sha512 , "RSA_SIGN_RAW_PKCS1_2048" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs12048 , "RSA_SIGN_RAW_PKCS1_3072" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs13072 , "RSA_SIGN_RAW_PKCS1_4096" => GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm :: RsaSignRawPkcs14096 , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1CryptoKeyVersionTemplateAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
        #[doc = "Crypto operations are performed in an EKM-over-VPC backend."]
        ExternalVpc,
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[doc = "Crypto operations are performed in software."]
        Software,
    }
    impl GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: External => "EXTERNAL" , GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ExternalVpc => "EXTERNAL_VPC" , GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Hsm => "HSM" , GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ProtectionLevelUnspecified => "PROTECTION_LEVEL_UNSPECIFIED" , GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Software => "SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel, ()>
        {
            Ok (match s { "EXTERNAL" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: External , "EXTERNAL_VPC" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ExternalVpc , "HSM" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Hsm , "PROTECTION_LEVEL_UNSPECIFIED" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ProtectionLevelUnspecified , "SOFTWARE" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Software , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXTERNAL" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: External , "EXTERNAL_VPC" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ExternalVpc , "HSM" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Hsm , "PROTECTION_LEVEL_UNSPECIFIED" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: ProtectionLevelUnspecified , "SOFTWARE" => GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel :: Software , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudKmsV1CryptoKeyVersionTemplateProtectionLevel
    {
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
    pub struct GoogleCloudKmsV1ExternalProtectionLevelOptions {
        #[doc = "The path to the external key material on the EKM when using EkmConnection e.g., “v0/my/key”. Set this field instead of external_key_uri when using an EkmConnection."]
        #[serde(
            rename = "ekmConnectionKeyPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ekm_connection_key_path: ::std::option::Option<String>,
        #[doc = "The URI for an external resource that this CryptoKeyVersion represents."]
        #[serde(
            rename = "externalKeyUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_key_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1ExternalProtectionLevelOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1ExternalProtectionLevelOptions {
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
    pub struct GoogleCloudKmsV1KeyOperationAttestation {
        #[doc = "Output only. The certificate chains needed to validate the attestation"]
        #[serde(
            rename = "certChains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_chains: ::std::option::Option<
            crate::schemas::GoogleCloudKmsV1KeyOperationAttestationCertificateChains,
        >,
        #[doc = "Output only. The attestation data provided by the HSM when the key operation was performed."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Output only. The format of the attestation data."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format:
            ::std::option::Option<crate::schemas::GoogleCloudKmsV1KeyOperationAttestationFormat>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1KeyOperationAttestation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1KeyOperationAttestation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudKmsV1KeyOperationAttestationFormat {
        #[doc = "Not specified."]
        AttestationFormatUnspecified,
        #[doc = "Cavium HSM attestation compressed with gzip. Note that this format is defined by Cavium and subject to change at any time. See https://www.marvell.com/products/security-solutions/nitrox-hs-adapters/software-key-attestation.html."]
        CaviumV1Compressed,
        #[doc = "Cavium HSM attestation V2 compressed with gzip. This is a new format introduced in Cavium’s version 3.2-08."]
        CaviumV2Compressed,
    }
    impl GoogleCloudKmsV1KeyOperationAttestationFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudKmsV1KeyOperationAttestationFormat::AttestationFormatUnspecified => {
                    "ATTESTATION_FORMAT_UNSPECIFIED"
                }
                GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV1Compressed => {
                    "CAVIUM_V1_COMPRESSED"
                }
                GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV2Compressed => {
                    "CAVIUM_V2_COMPRESSED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudKmsV1KeyOperationAttestationFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudKmsV1KeyOperationAttestationFormat {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudKmsV1KeyOperationAttestationFormat, ()> {
            Ok(match s {
                "ATTESTATION_FORMAT_UNSPECIFIED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::AttestationFormatUnspecified
                }
                "CAVIUM_V1_COMPRESSED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV1Compressed
                }
                "CAVIUM_V2_COMPRESSED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV2Compressed
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudKmsV1KeyOperationAttestationFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudKmsV1KeyOperationAttestationFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudKmsV1KeyOperationAttestationFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION_FORMAT_UNSPECIFIED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::AttestationFormatUnspecified
                }
                "CAVIUM_V1_COMPRESSED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV1Compressed
                }
                "CAVIUM_V2_COMPRESSED" => {
                    GoogleCloudKmsV1KeyOperationAttestationFormat::CaviumV2Compressed
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
    impl ::google_field_selector::FieldSelector for GoogleCloudKmsV1KeyOperationAttestationFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudKmsV1KeyOperationAttestationFormat {
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
    pub struct GoogleCloudKmsV1KeyOperationAttestationCertificateChains {
        #[doc = "Cavium certificate chain corresponding to the attestation."]
        #[serde(
            rename = "caviumCerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cavium_certs: ::std::option::Option<Vec<String>>,
        #[doc = "Google card certificate chain corresponding to the attestation."]
        #[serde(
            rename = "googleCardCerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_card_certs: ::std::option::Option<Vec<String>>,
        #[doc = "Google partition certificate chain corresponding to the attestation."]
        #[serde(
            rename = "googlePartitionCerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_partition_certs: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudKmsV1KeyOperationAttestationCertificateChains
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudKmsV1KeyOperationAttestationCertificateChains
    {
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
    #[doc = "Actions that can be performed on the organizations resource"]
    pub fn organizations(&self) -> crate::resources::organizations::OrganizationsActions {
        crate::resources::organizations::OrganizationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
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
    pub mod organizations {
        pub mod params {}
        pub struct OrganizationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OrganizationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the protected_resources resource"]
            pub fn protected_resources(
                &self,
            ) -> crate::resources::organizations::protected_resources::ProtectedResourcesActions
            {
                crate::resources::organizations::protected_resources::ProtectedResourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod protected_resources {
            pub mod params {}
            pub struct ProtectedResourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ProtectedResourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns metadata about the resources protected by the given Cloud KMS CryptoKey in the given Cloud organization."]
                pub fn search(&self, scope: impl Into<String>) -> SearchRequestBuilder {
                    SearchRequestBuilder {
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
                        scope: scope.into(),
                        crypto_key: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [ProtectedResourcesActions::search()](struct.ProtectedResourcesActions.html#method.search)"]
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                scope: String,
                crypto_key: ::std::option::Option<String>,
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
            impl<'a> SearchRequestBuilder<'a> {
                #[doc = "Required. The resource name of the CryptoKey."]
                pub fn crypto_key(mut self, value: impl Into<String>) -> Self {
                    self.crypto_key = Some(value.into());
                    self
                }
                #[doc = "The maximum number of resources to return. The service may return fewer than this value. If unspecified, at most 500 resources will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A page token, received from a previous KeyTrackingService.SearchProtectedResources call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to KeyTrackingService.SearchProtectedResources must match the call that provided the page token."]
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
                #[doc = "\nExecute the request and yield each item in the `protectedResources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_protected_resources<T>(
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
                    self.stream_protected_resources_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `protectedResources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_protected_resources_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudKmsInventoryV1ProtectedResource,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_protected_resources_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `protectedResources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_protected_resources_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudKmsInventoryV1ProtectedResource,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_protected_resources_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `protectedResources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_protected_resources_with_fields<T, F>(
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
                        #[serde(rename = "protectedResources")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
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
                            concat!("nextPageToken,", "protectedResources").to_owned();
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
                    T: crate::GetNextPageToken<String>
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
                        crate::schemas::GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse,
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
                        crate::schemas::GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse,
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
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                ) -> Result<
                    crate::schemas::GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudKmsInventoryV1SearchProtectedResourcesResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://kmsinventory.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.scope;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/protectedResources:search");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("cryptoKey", &self.crypto_key)]);
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
            impl<'a> crate::stream::StreamableMethod for SearchRequestBuilder<'a> {
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
        }
    }
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
            #[doc = "Actions that can be performed on the crypto_keys resource"]
            pub fn crypto_keys(
                &self,
            ) -> crate::resources::projects::crypto_keys::CryptoKeysActions {
                crate::resources::projects::crypto_keys::CryptoKeysActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod crypto_keys {
            pub mod params {}
            pub struct CryptoKeysActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> CryptoKeysActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns cryptographic keys managed by Cloud KMS in a given Cloud project. Note that this data is sourced from snapshots, meaning it may not completely reflect the actual state of key metadata at call time."]
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
            #[doc = "Created via [CryptoKeysActions::list()](struct.CryptoKeysActions.html#method.list)"]
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
                #[doc = "Optional. The maximum number of keys to return. The service may return fewer than this value. If unspecified, at most 1000 keys will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. Pass this into a subsequent request in order to receive the next page of results."]
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
                #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_crypto_keys<T>(
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
                    self.stream_crypto_keys_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_crypto_keys_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::GoogleCloudKmsV1CryptoKey, crate::Error>,
                > + 'a {
                    self.stream_crypto_keys_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_crypto_keys_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::GoogleCloudKmsV1CryptoKey, crate::Error>,
                > + 'a {
                    self.stream_crypto_keys_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_crypto_keys_with_fields<T, F>(
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
                        #[serde(rename = "cryptoKeys")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
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
                        let mut selector = concat!("nextPageToken,", "cryptoKeys").to_owned();
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
                    T: crate::GetNextPageToken<String>
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
                        crate::schemas::GoogleCloudKmsInventoryV1ListCryptoKeysResponse,
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
                        crate::schemas::GoogleCloudKmsInventoryV1ListCryptoKeysResponse,
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
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                ) -> Result<
                    crate::schemas::GoogleCloudKmsInventoryV1ListCryptoKeysResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudKmsInventoryV1ListCryptoKeysResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://kmsinventory.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/cryptoKeys");
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
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
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
                #[doc = "Actions that can be performed on the key_rings resource"]
                pub fn key_rings(
                    &self,
                ) -> crate::resources::projects::locations::key_rings::KeyRingsActions
                {
                    crate::resources::projects::locations::key_rings::KeyRingsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod key_rings {
                pub mod params {}
                pub struct KeyRingsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> KeyRingsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Actions that can be performed on the crypto_keys resource"]                    pub fn crypto_keys (& self) -> crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: CryptoKeysActions{
                        crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: CryptoKeysActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                pub mod crypto_keys {
                    pub mod params {}
                    pub struct CryptoKeysActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> CryptoKeysActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Returns aggregate information about the resources protected by the given Cloud KMS CryptoKey. Only resources within the same Cloud organization as the key will be returned. The project that holds the key must be part of an organization in order for this call to succeed."]
                        pub fn get_protected_resources_summary(
                            &self,
                            name: impl Into<String>,
                        ) -> GetProtectedResourcesSummaryRequestBuilder {
                            GetProtectedResourcesSummaryRequestBuilder {
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
                    }
                    #[doc = "Created via [CryptoKeysActions::get_protected_resources_summary()](struct.CryptoKeysActions.html#method.get_protected_resources_summary)"]
                    #[derive(Debug, Clone)]
                    pub struct GetProtectedResourcesSummaryRequestBuilder<'a> {
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
                    impl<'a> GetProtectedResourcesSummaryRequestBuilder<'a> {
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
                        ) -> Result<
                            crate::schemas::GoogleCloudKmsInventoryV1ProtectedResourcesSummary,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudKmsInventoryV1ProtectedResourcesSummary,
                            crate::Error,
                        > {
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
                            let mut output = "https://kmsinventory.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/protectedResourcesSummary");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
pub trait GetNextPageToken<T> {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<T>;
}

impl<T: ::std::convert::From<::std::string::String>> GetNextPageToken<T>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn next_page_token(&self) -> ::std::option::Option<T> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned().into())
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
        /// Type of the `pageToken` and `nextPageToken` fields.
        type PageToken;

        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: Self::PageToken);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken<Self::PageToken> + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned,
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
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
