#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [*generateRandomBytes*](resources/projects/locations/struct.GenerateRandomBytesRequestBuilder.html), [*get*](resources/projects/locations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/struct.ListRequestBuilder.html)\n    * [ekm_connections](resources/projects/locations/ekm_connections/struct.EkmConnectionsActions.html)\n      * [*create*](resources/projects/locations/ekm_connections/struct.CreateRequestBuilder.html), [*get*](resources/projects/locations/ekm_connections/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/locations/ekm_connections/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/locations/ekm_connections/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/ekm_connections/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/ekm_connections/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/ekm_connections/struct.TestIamPermissionsRequestBuilder.html)\n    * [key_rings](resources/projects/locations/key_rings/struct.KeyRingsActions.html)\n      * [*create*](resources/projects/locations/key_rings/struct.CreateRequestBuilder.html), [*get*](resources/projects/locations/key_rings/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/locations/key_rings/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/locations/key_rings/struct.ListRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/key_rings/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/key_rings/struct.TestIamPermissionsRequestBuilder.html)\n      * [crypto_keys](resources/projects/locations/key_rings/crypto_keys/struct.CryptoKeysActions.html)\n        * [*create*](resources/projects/locations/key_rings/crypto_keys/struct.CreateRequestBuilder.html), [*decrypt*](resources/projects/locations/key_rings/crypto_keys/struct.DecryptRequestBuilder.html), [*encrypt*](resources/projects/locations/key_rings/crypto_keys/struct.EncryptRequestBuilder.html), [*get*](resources/projects/locations/key_rings/crypto_keys/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/locations/key_rings/crypto_keys/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/locations/key_rings/crypto_keys/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/key_rings/crypto_keys/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/key_rings/crypto_keys/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/key_rings/crypto_keys/struct.TestIamPermissionsRequestBuilder.html), [*updatePrimaryVersion*](resources/projects/locations/key_rings/crypto_keys/struct.UpdatePrimaryVersionRequestBuilder.html)\n        * [crypto_key_versions](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.CryptoKeyVersionsActions.html)\n          * [*asymmetricDecrypt*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.AsymmetricDecryptRequestBuilder.html), [*asymmetricSign*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.AsymmetricSignRequestBuilder.html), [*create*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.CreateRequestBuilder.html), [*destroy*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.DestroyRequestBuilder.html), [*get*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.GetRequestBuilder.html), [*getPublicKey*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.GetPublicKeyRequestBuilder.html), [*import*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.ImportRequestBuilder.html), [*list*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.ListRequestBuilder.html), [*macSign*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.MacSignRequestBuilder.html), [*macVerify*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.MacVerifyRequestBuilder.html), [*patch*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.PatchRequestBuilder.html), [*restore*](resources/projects/locations/key_rings/crypto_keys/crypto_key_versions/struct.RestoreRequestBuilder.html)\n      * [import_jobs](resources/projects/locations/key_rings/import_jobs/struct.ImportJobsActions.html)\n        * [*create*](resources/projects/locations/key_rings/import_jobs/struct.CreateRequestBuilder.html), [*get*](resources/projects/locations/key_rings/import_jobs/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/locations/key_rings/import_jobs/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/locations/key_rings/import_jobs/struct.ListRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/key_rings/import_jobs/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/key_rings/import_jobs/struct.TestIamPermissionsRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View and manage your keys and secrets stored in Cloud Key Management Service\n\n`https://www.googleapis.com/auth/cloudkms`"]
    pub const CLOUDKMS: &str = "https://www.googleapis.com/auth/cloudkms";
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
    pub struct AsymmetricDecryptRequest {
        #[doc = "Required. The data encrypted with the named CryptoKeyVersion's public key using OAEP."]
        #[serde(
            rename = "ciphertext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ciphertext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the AsymmetricDecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received AsymmetricDecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricDecryptRequest.ciphertext) is equal to AsymmetricDecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "ciphertextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ciphertext_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AsymmetricDecryptRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricDecryptRequest {
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
    pub struct AsymmetricDecryptResponse {
        #[doc = "The decrypted data originally encrypted with the matching public key."]
        #[serde(
            rename = "plaintext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plaintext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned AsymmetricDecryptResponse.plaintext. An integrity check of AsymmetricDecryptResponse.plaintext can be performed by computing the CRC32C checksum of AsymmetricDecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "plaintextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub plaintext_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::AsymmetricDecryptResponseProtectionLevel>,
        #[doc = "Integrity verification field. A flag indicating whether AsymmetricDecryptRequest.ciphertext_crc32c was received by KeyManagementService and used for the integrity verification of the ciphertext. A false value of this field indicates either that AsymmetricDecryptRequest.ciphertext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricDecryptRequest.ciphertext_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedCiphertextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_ciphertext_crc_3_2c: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AsymmetricDecryptResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricDecryptResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AsymmetricDecryptResponseProtectionLevel {
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
    impl AsymmetricDecryptResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                AsymmetricDecryptResponseProtectionLevel::External => "EXTERNAL",
                AsymmetricDecryptResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                AsymmetricDecryptResponseProtectionLevel::Hsm => "HSM",
                AsymmetricDecryptResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                AsymmetricDecryptResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AsymmetricDecryptResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AsymmetricDecryptResponseProtectionLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AsymmetricDecryptResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => AsymmetricDecryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => AsymmetricDecryptResponseProtectionLevel::ExternalVpc,
                "HSM" => AsymmetricDecryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    AsymmetricDecryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => AsymmetricDecryptResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AsymmetricDecryptResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AsymmetricDecryptResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AsymmetricDecryptResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => AsymmetricDecryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => AsymmetricDecryptResponseProtectionLevel::ExternalVpc,
                "HSM" => AsymmetricDecryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    AsymmetricDecryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => AsymmetricDecryptResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AsymmetricDecryptResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricDecryptResponseProtectionLevel {
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
    pub struct AsymmetricSignRequest {
        #[doc = "Optional. The data to sign. It can't be supplied if AsymmetricSignRequest.digest is supplied."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the AsymmetricSignRequest.data. If specified, KeyManagementService will verify the integrity of the received AsymmetricSignRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricSignRequest.data) is equal to AsymmetricSignRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "dataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub data_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "Optional. The digest of the data to sign. The digest must be produced with the same digest algorithm as specified by the key version's algorithm. This field may not be supplied if AsymmetricSignRequest.data is supplied."]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<crate::schemas::Digest>,
        #[doc = "Optional. An optional CRC32C checksum of the AsymmetricSignRequest.digest. If specified, KeyManagementService will verify the integrity of the received AsymmetricSignRequest.digest using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricSignRequest.digest) is equal to AsymmetricSignRequest.digest_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "digestCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub digest_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AsymmetricSignRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricSignRequest {
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
    pub struct AsymmetricSignResponse {
        #[doc = "The resource name of the CryptoKeyVersion used for signing. Check this field to verify that the intended resource was used for signing."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used for signing."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::AsymmetricSignResponseProtectionLevel>,
        #[doc = "The created signature."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned AsymmetricSignResponse.signature. An integrity check of AsymmetricSignResponse.signature can be performed by computing the CRC32C checksum of AsymmetricSignResponse.signature and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "signatureCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub signature_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "Integrity verification field. A flag indicating whether AsymmetricSignRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that AsymmetricSignRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricSignRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_data_crc_3_2c: ::std::option::Option<bool>,
        #[doc = "Integrity verification field. A flag indicating whether AsymmetricSignRequest.digest_crc32c was received by KeyManagementService and used for the integrity verification of the digest. A false value of this field indicates either that AsymmetricSignRequest.digest_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricSignRequest.digest_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedDigestCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_digest_crc_3_2c: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AsymmetricSignResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricSignResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AsymmetricSignResponseProtectionLevel {
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
    impl AsymmetricSignResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                AsymmetricSignResponseProtectionLevel::External => "EXTERNAL",
                AsymmetricSignResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                AsymmetricSignResponseProtectionLevel::Hsm => "HSM",
                AsymmetricSignResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                AsymmetricSignResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AsymmetricSignResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AsymmetricSignResponseProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AsymmetricSignResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => AsymmetricSignResponseProtectionLevel::External,
                "EXTERNAL_VPC" => AsymmetricSignResponseProtectionLevel::ExternalVpc,
                "HSM" => AsymmetricSignResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    AsymmetricSignResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => AsymmetricSignResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AsymmetricSignResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AsymmetricSignResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AsymmetricSignResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => AsymmetricSignResponseProtectionLevel::External,
                "EXTERNAL_VPC" => AsymmetricSignResponseProtectionLevel::ExternalVpc,
                "HSM" => AsymmetricSignResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    AsymmetricSignResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => AsymmetricSignResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AsymmetricSignResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AsymmetricSignResponseProtectionLevel {
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
    pub struct Certificate {
        #[doc = "Output only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true."]
        #[serde(
            rename = "issuer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub issuer: ::std::option::Option<String>,
        #[doc = "Output only. The certificate is not valid after this time. Only present if parsed is true."]
        #[serde(
            rename = "notAfterTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_after_time: ::std::option::Option<String>,
        #[doc = "Output only. The certificate is not valid before this time. Only present if parsed is true."]
        #[serde(
            rename = "notBeforeTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_before_time: ::std::option::Option<String>,
        #[doc = "Output only. True if the certificate was parsed successfully."]
        #[serde(
            rename = "parsed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parsed: ::std::option::Option<bool>,
        #[doc = "Required. The raw certificate bytes in DER format."]
        #[serde(
            rename = "rawDer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_der: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Output only. The certificate serial number as a hex string. Only present if parsed is true."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true."]
        #[serde(
            rename = "sha256Fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256_fingerprint: ::std::option::Option<String>,
        #[doc = "Output only. The subject distinguished name in RFC 2253 format. Only present if parsed is true."]
        #[serde(
            rename = "subject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject: ::std::option::Option<String>,
        #[doc = "Output only. The subject Alternative DNS names. Only present if parsed is true."]
        #[serde(
            rename = "subjectAlternativeDnsNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_alternative_dns_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Certificate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Certificate {
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
    pub struct CertificateChains {
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
    impl ::google_field_selector::FieldSelector for CertificateChains {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CertificateChains {
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
    pub struct CryptoKey {
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
        #[doc = "Output only. A copy of the \"primary\" CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted."]
        #[serde(
            rename = "primary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary: ::std::option::Option<crate::schemas::CryptoKeyVersion>,
        #[doc = "Immutable. The immutable purpose of this CryptoKey."]
        #[serde(
            rename = "purpose",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub purpose: ::std::option::Option<crate::schemas::CryptoKeyPurpose>,
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
        pub version_template: ::std::option::Option<crate::schemas::CryptoKeyVersionTemplate>,
    }
    impl ::google_field_selector::FieldSelector for CryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKey {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyPurpose {
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
    impl CryptoKeyPurpose {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyPurpose::AsymmetricDecrypt => "ASYMMETRIC_DECRYPT",
                CryptoKeyPurpose::AsymmetricSign => "ASYMMETRIC_SIGN",
                CryptoKeyPurpose::CryptoKeyPurposeUnspecified => "CRYPTO_KEY_PURPOSE_UNSPECIFIED",
                CryptoKeyPurpose::EncryptDecrypt => "ENCRYPT_DECRYPT",
                CryptoKeyPurpose::Mac => "MAC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyPurpose {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyPurpose {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyPurpose, ()> {
            Ok(match s {
                "ASYMMETRIC_DECRYPT" => CryptoKeyPurpose::AsymmetricDecrypt,
                "ASYMMETRIC_SIGN" => CryptoKeyPurpose::AsymmetricSign,
                "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => CryptoKeyPurpose::CryptoKeyPurposeUnspecified,
                "ENCRYPT_DECRYPT" => CryptoKeyPurpose::EncryptDecrypt,
                "MAC" => CryptoKeyPurpose::Mac,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyPurpose {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyPurpose {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyPurpose {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASYMMETRIC_DECRYPT" => CryptoKeyPurpose::AsymmetricDecrypt,
                "ASYMMETRIC_SIGN" => CryptoKeyPurpose::AsymmetricSign,
                "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => CryptoKeyPurpose::CryptoKeyPurposeUnspecified,
                "ENCRYPT_DECRYPT" => CryptoKeyPurpose::EncryptDecrypt,
                "MAC" => CryptoKeyPurpose::Mac,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyPurpose {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyPurpose {
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
    pub struct CryptoKeyVersion {
        #[doc = "Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::CryptoKeyVersionAlgorithm>,
        #[doc = "Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation: ::std::option::Option<crate::schemas::KeyOperationAttestation>,
        #[doc = "Output only. The time at which this CryptoKeyVersion was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this CryptoKeyVersion's key material was destroyed. Only present if state is DESTROYED."]
        #[serde(
            rename = "destroyEventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destroy_event_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this CryptoKeyVersion's key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED."]
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
            ::std::option::Option<crate::schemas::ExternalProtectionLevelOptions>,
        #[doc = "Output only. The time this CryptoKeyVersion's key material was generated."]
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
        #[doc = "Output only. The time at which this CryptoKeyVersion's key material was most recently imported."]
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
            ::std::option::Option<crate::schemas::CryptoKeyVersionProtectionLevel>,
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
        pub state: ::std::option::Option<crate::schemas::CryptoKeyVersionState>,
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyVersionAlgorithm {
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
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
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
    impl CryptoKeyVersionAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                CryptoKeyVersionAlgorithm::EcSignP256Sha256 => "EC_SIGN_P256_SHA256",
                CryptoKeyVersionAlgorithm::EcSignP384Sha384 => "EC_SIGN_P384_SHA384",
                CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256 => "EC_SIGN_SECP256K1_SHA256",
                CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption => {
                    "EXTERNAL_SYMMETRIC_ENCRYPTION"
                }
                CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption => {
                    "GOOGLE_SYMMETRIC_ENCRYPTION"
                }
                CryptoKeyVersionAlgorithm::HmacSha256 => "HMAC_SHA256",
                CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1 => "RSA_DECRYPT_OAEP_2048_SHA1",
                CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256 => {
                    "RSA_DECRYPT_OAEP_2048_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1 => "RSA_DECRYPT_OAEP_3072_SHA1",
                CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256 => {
                    "RSA_DECRYPT_OAEP_3072_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1 => "RSA_DECRYPT_OAEP_4096_SHA1",
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256 => {
                    "RSA_DECRYPT_OAEP_4096_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512 => {
                    "RSA_DECRYPT_OAEP_4096_SHA512"
                }
                CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256 => "RSA_SIGN_PKCS1_2048_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256 => "RSA_SIGN_PKCS1_3072_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256 => "RSA_SIGN_PKCS1_4096_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512 => "RSA_SIGN_PKCS1_4096_SHA512",
                CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256 => "RSA_SIGN_PSS_2048_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256 => "RSA_SIGN_PSS_3072_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256 => "RSA_SIGN_PSS_4096_SHA256",
                CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512 => "RSA_SIGN_PSS_4096_SHA512",
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048 => "RSA_SIGN_RAW_PKCS1_2048",
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072 => "RSA_SIGN_RAW_PKCS1_3072",
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096 => "RSA_SIGN_RAW_PKCS1_4096",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyVersionAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyVersionAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyVersionAlgorithm, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => CryptoKeyVersionAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => CryptoKeyVersionAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256,
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => CryptoKeyVersionAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1,
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1,
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1,
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256,
                "RSA_SIGN_PKCS1_3072_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256,
                "RSA_SIGN_PKCS1_4096_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256,
                "RSA_SIGN_PKCS1_4096_SHA512" => CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512,
                "RSA_SIGN_PSS_2048_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256,
                "RSA_SIGN_PSS_3072_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256,
                "RSA_SIGN_PSS_4096_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256,
                "RSA_SIGN_PSS_4096_SHA512" => CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512,
                "RSA_SIGN_RAW_PKCS1_2048" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyVersionAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyVersionAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyVersionAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    CryptoKeyVersionAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => CryptoKeyVersionAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => CryptoKeyVersionAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => CryptoKeyVersionAlgorithm::EcSignSecp256K1Sha256,
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => CryptoKeyVersionAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1,
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1,
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1,
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256,
                "RSA_SIGN_PKCS1_3072_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256,
                "RSA_SIGN_PKCS1_4096_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256,
                "RSA_SIGN_PKCS1_4096_SHA512" => CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512,
                "RSA_SIGN_PSS_2048_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256,
                "RSA_SIGN_PSS_3072_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256,
                "RSA_SIGN_PSS_4096_SHA256" => CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256,
                "RSA_SIGN_PSS_4096_SHA512" => CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512,
                "RSA_SIGN_RAW_PKCS1_2048" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyVersionProtectionLevel {
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
    impl CryptoKeyVersionProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyVersionProtectionLevel::External => "EXTERNAL",
                CryptoKeyVersionProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                CryptoKeyVersionProtectionLevel::Hsm => "HSM",
                CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                CryptoKeyVersionProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyVersionProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyVersionProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyVersionProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => CryptoKeyVersionProtectionLevel::External,
                "EXTERNAL_VPC" => CryptoKeyVersionProtectionLevel::ExternalVpc,
                "HSM" => CryptoKeyVersionProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => CryptoKeyVersionProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyVersionProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyVersionProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyVersionProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => CryptoKeyVersionProtectionLevel::External,
                "EXTERNAL_VPC" => CryptoKeyVersionProtectionLevel::ExternalVpc,
                "HSM" => CryptoKeyVersionProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    CryptoKeyVersionProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => CryptoKeyVersionProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionProtectionLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyVersionState {
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
    impl CryptoKeyVersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyVersionState::CryptoKeyVersionStateUnspecified => {
                    "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED"
                }
                CryptoKeyVersionState::DestroyScheduled => "DESTROY_SCHEDULED",
                CryptoKeyVersionState::Destroyed => "DESTROYED",
                CryptoKeyVersionState::Disabled => "DISABLED",
                CryptoKeyVersionState::Enabled => "ENABLED",
                CryptoKeyVersionState::ImportFailed => "IMPORT_FAILED",
                CryptoKeyVersionState::PendingGeneration => "PENDING_GENERATION",
                CryptoKeyVersionState::PendingImport => "PENDING_IMPORT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyVersionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyVersionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyVersionState, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => {
                    CryptoKeyVersionState::CryptoKeyVersionStateUnspecified
                }
                "DESTROY_SCHEDULED" => CryptoKeyVersionState::DestroyScheduled,
                "DESTROYED" => CryptoKeyVersionState::Destroyed,
                "DISABLED" => CryptoKeyVersionState::Disabled,
                "ENABLED" => CryptoKeyVersionState::Enabled,
                "IMPORT_FAILED" => CryptoKeyVersionState::ImportFailed,
                "PENDING_GENERATION" => CryptoKeyVersionState::PendingGeneration,
                "PENDING_IMPORT" => CryptoKeyVersionState::PendingImport,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyVersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyVersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyVersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => {
                    CryptoKeyVersionState::CryptoKeyVersionStateUnspecified
                }
                "DESTROY_SCHEDULED" => CryptoKeyVersionState::DestroyScheduled,
                "DESTROYED" => CryptoKeyVersionState::Destroyed,
                "DISABLED" => CryptoKeyVersionState::Disabled,
                "ENABLED" => CryptoKeyVersionState::Enabled,
                "IMPORT_FAILED" => CryptoKeyVersionState::ImportFailed,
                "PENDING_GENERATION" => CryptoKeyVersionState::PendingGeneration,
                "PENDING_IMPORT" => CryptoKeyVersionState::PendingImport,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionState {
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
    pub struct CryptoKeyVersionTemplate {
        #[doc = "Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::CryptoKeyVersionTemplateAlgorithm>,
        #[doc = "ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::CryptoKeyVersionTemplateProtectionLevel>,
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyVersionTemplateAlgorithm {
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
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
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
    impl CryptoKeyVersionTemplateAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyVersionTemplateAlgorithm::CryptoKeyVersionAlgorithmUnspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                CryptoKeyVersionTemplateAlgorithm::EcSignP256Sha256 => "EC_SIGN_P256_SHA256",
                CryptoKeyVersionTemplateAlgorithm::EcSignP384Sha384 => "EC_SIGN_P384_SHA384",
                CryptoKeyVersionTemplateAlgorithm::EcSignSecp256K1Sha256 => {
                    "EC_SIGN_SECP256K1_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::ExternalSymmetricEncryption => {
                    "EXTERNAL_SYMMETRIC_ENCRYPTION"
                }
                CryptoKeyVersionTemplateAlgorithm::GoogleSymmetricEncryption => {
                    "GOOGLE_SYMMETRIC_ENCRYPTION"
                }
                CryptoKeyVersionTemplateAlgorithm::HmacSha256 => "HMAC_SHA256",
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha1 => {
                    "RSA_DECRYPT_OAEP_2048_SHA1"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha256 => {
                    "RSA_DECRYPT_OAEP_2048_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha1 => {
                    "RSA_DECRYPT_OAEP_3072_SHA1"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha256 => {
                    "RSA_DECRYPT_OAEP_3072_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha1 => {
                    "RSA_DECRYPT_OAEP_4096_SHA1"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha256 => {
                    "RSA_DECRYPT_OAEP_4096_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha512 => {
                    "RSA_DECRYPT_OAEP_4096_SHA512"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs12048Sha256 => {
                    "RSA_SIGN_PKCS1_2048_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs13072Sha256 => {
                    "RSA_SIGN_PKCS1_3072_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha256 => {
                    "RSA_SIGN_PKCS1_4096_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha512 => {
                    "RSA_SIGN_PKCS1_4096_SHA512"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPss2048Sha256 => {
                    "RSA_SIGN_PSS_2048_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPss3072Sha256 => {
                    "RSA_SIGN_PSS_3072_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha256 => {
                    "RSA_SIGN_PSS_4096_SHA256"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha512 => {
                    "RSA_SIGN_PSS_4096_SHA512"
                }
                CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs12048 => "RSA_SIGN_RAW_PKCS1_2048",
                CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs13072 => "RSA_SIGN_RAW_PKCS1_3072",
                CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs14096 => "RSA_SIGN_RAW_PKCS1_4096",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyVersionTemplateAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyVersionTemplateAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyVersionTemplateAlgorithm, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    CryptoKeyVersionTemplateAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => CryptoKeyVersionTemplateAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => CryptoKeyVersionTemplateAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionTemplateAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionTemplateAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => CryptoKeyVersionTemplateAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs14096,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyVersionTemplateAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyVersionTemplateAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyVersionTemplateAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    CryptoKeyVersionTemplateAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => CryptoKeyVersionTemplateAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => CryptoKeyVersionTemplateAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionTemplateAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    CryptoKeyVersionTemplateAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => CryptoKeyVersionTemplateAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    CryptoKeyVersionTemplateAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => CryptoKeyVersionTemplateAlgorithm::RsaSignRawPkcs14096,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionTemplateAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionTemplateAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CryptoKeyVersionTemplateProtectionLevel {
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
    impl CryptoKeyVersionTemplateProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                CryptoKeyVersionTemplateProtectionLevel::External => "EXTERNAL",
                CryptoKeyVersionTemplateProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                CryptoKeyVersionTemplateProtectionLevel::Hsm => "HSM",
                CryptoKeyVersionTemplateProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                CryptoKeyVersionTemplateProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CryptoKeyVersionTemplateProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CryptoKeyVersionTemplateProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CryptoKeyVersionTemplateProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => CryptoKeyVersionTemplateProtectionLevel::External,
                "EXTERNAL_VPC" => CryptoKeyVersionTemplateProtectionLevel::ExternalVpc,
                "HSM" => CryptoKeyVersionTemplateProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    CryptoKeyVersionTemplateProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => CryptoKeyVersionTemplateProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CryptoKeyVersionTemplateProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CryptoKeyVersionTemplateProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CryptoKeyVersionTemplateProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => CryptoKeyVersionTemplateProtectionLevel::External,
                "EXTERNAL_VPC" => CryptoKeyVersionTemplateProtectionLevel::ExternalVpc,
                "HSM" => CryptoKeyVersionTemplateProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    CryptoKeyVersionTemplateProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => CryptoKeyVersionTemplateProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CryptoKeyVersionTemplateProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CryptoKeyVersionTemplateProtectionLevel {
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
    pub struct DecryptRequest {
        #[doc = "Optional. Optional data that must match the data originally supplied in EncryptRequest.additional_authenticated_data."]
        #[serde(
            rename = "additionalAuthenticatedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_authenticated_data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the DecryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.additional_authenticated_data) is equal to DecryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "additionalAuthenticatedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub additional_authenticated_data_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "Required. The encrypted data originally returned in EncryptResponse.ciphertext."]
        #[serde(
            rename = "ciphertext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ciphertext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the DecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.ciphertext) is equal to DecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "ciphertextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ciphertext_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DecryptRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DecryptRequest {
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
    pub struct DecryptResponse {
        #[doc = "The decrypted data originally supplied in EncryptRequest.plaintext."]
        #[serde(
            rename = "plaintext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plaintext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned DecryptResponse.plaintext. An integrity check of DecryptResponse.plaintext can be performed by computing the CRC32C checksum of DecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: receiving this response message indicates that KeyManagementService is able to successfully decrypt the ciphertext. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "plaintextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub plaintext_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<crate::schemas::DecryptResponseProtectionLevel>,
        #[doc = "Whether the Decryption was performed using the primary key version."]
        #[serde(
            rename = "usedPrimary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub used_primary: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DecryptResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DecryptResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DecryptResponseProtectionLevel {
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
    impl DecryptResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                DecryptResponseProtectionLevel::External => "EXTERNAL",
                DecryptResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                DecryptResponseProtectionLevel::Hsm => "HSM",
                DecryptResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                DecryptResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DecryptResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DecryptResponseProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DecryptResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => DecryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => DecryptResponseProtectionLevel::ExternalVpc,
                "HSM" => DecryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    DecryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => DecryptResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DecryptResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DecryptResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DecryptResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => DecryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => DecryptResponseProtectionLevel::ExternalVpc,
                "HSM" => DecryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    DecryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => DecryptResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DecryptResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DecryptResponseProtectionLevel {
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
    pub struct DestroyCryptoKeyVersionRequest {}
    impl ::google_field_selector::FieldSelector for DestroyCryptoKeyVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestroyCryptoKeyVersionRequest {
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
    pub struct Digest {
        #[doc = "A message digest produced with the SHA-256 algorithm."]
        #[serde(
            rename = "sha256",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A message digest produced with the SHA-384 algorithm."]
        #[serde(
            rename = "sha384",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_384: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A message digest produced with the SHA-512 algorithm."]
        #[serde(
            rename = "sha512",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_512: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Digest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Digest {
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
    pub struct EkmConnection {
        #[doc = "Output only. The time at which the EkmConnection was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "This checksum is computed by the server based on the value of other fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. The resource name for the EkmConnection in the format `projects/*/locations/*/ekmConnections/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported."]
        #[serde(
            rename = "serviceResolvers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_resolvers: ::std::option::Option<Vec<crate::schemas::ServiceResolver>>,
    }
    impl ::google_field_selector::FieldSelector for EkmConnection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EkmConnection {
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
    pub struct EncryptRequest {
        #[doc = "Optional. Optional data that, if specified, must also be provided during decryption through DecryptRequest.additional_authenticated_data. The maximum size depends on the key version's protection_level. For SOFTWARE, EXTERNAL, and EXTERNAL_VPC keys the AAD must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB."]
        #[serde(
            rename = "additionalAuthenticatedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_authenticated_data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the EncryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.additional_authenticated_data) is equal to EncryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "additionalAuthenticatedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub additional_authenticated_data_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "Required. The data to encrypt. Must be no larger than 64KiB. The maximum size depends on the key version's protection_level. For SOFTWARE, EXTERNAL, and EXTERNAL_VPC keys, the plaintext must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB."]
        #[serde(
            rename = "plaintext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plaintext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the EncryptRequest.plaintext. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.plaintext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.plaintext) is equal to EncryptRequest.plaintext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "plaintextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub plaintext_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for EncryptRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncryptRequest {
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
    pub struct EncryptResponse {
        #[doc = "The encrypted data."]
        #[serde(
            rename = "ciphertext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ciphertext: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned EncryptResponse.ciphertext. An integrity check of EncryptResponse.ciphertext can be performed by computing the CRC32C checksum of EncryptResponse.ciphertext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "ciphertextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ciphertext_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "The resource name of the CryptoKeyVersion used in encryption. Check this field to verify that the intended resource was used for encryption."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in encryption."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<crate::schemas::EncryptResponseProtectionLevel>,
        #[doc = "Integrity verification field. A flag indicating whether EncryptRequest.additional_authenticated_data_crc32c was received by KeyManagementService and used for the integrity verification of the AAD. A false value of this field indicates either that EncryptRequest.additional_authenticated_data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.additional_authenticated_data_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedAdditionalAuthenticatedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_additional_authenticated_data_crc_3_2c: ::std::option::Option<bool>,
        #[doc = "Integrity verification field. A flag indicating whether EncryptRequest.plaintext_crc32c was received by KeyManagementService and used for the integrity verification of the plaintext. A false value of this field indicates either that EncryptRequest.plaintext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.plaintext_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedPlaintextCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_plaintext_crc_3_2c: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for EncryptResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncryptResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EncryptResponseProtectionLevel {
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
    impl EncryptResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                EncryptResponseProtectionLevel::External => "EXTERNAL",
                EncryptResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                EncryptResponseProtectionLevel::Hsm => "HSM",
                EncryptResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                EncryptResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EncryptResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EncryptResponseProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EncryptResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => EncryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => EncryptResponseProtectionLevel::ExternalVpc,
                "HSM" => EncryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    EncryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => EncryptResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EncryptResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EncryptResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EncryptResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => EncryptResponseProtectionLevel::External,
                "EXTERNAL_VPC" => EncryptResponseProtectionLevel::ExternalVpc,
                "HSM" => EncryptResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    EncryptResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => EncryptResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EncryptResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncryptResponseProtectionLevel {
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
    pub struct ExternalProtectionLevelOptions {
        #[doc = "The path to the external key material on the EKM when using EkmConnection e.g., \"v0/my/key\". Set this field instead of external_key_uri when using an EkmConnection."]
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
    impl ::google_field_selector::FieldSelector for ExternalProtectionLevelOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalProtectionLevelOptions {
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
    pub struct GenerateRandomBytesRequest {
        #[doc = "The length in bytes of the amount of randomness to retrieve. Minimum 8 bytes, maximum 1024 bytes."]
        #[serde(
            rename = "lengthBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub length_bytes: ::std::option::Option<i32>,
        #[doc = "The ProtectionLevel to use when generating the random data. Currently, only HSM protection level is supported."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::GenerateRandomBytesRequestProtectionLevel>,
    }
    impl ::google_field_selector::FieldSelector for GenerateRandomBytesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateRandomBytesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GenerateRandomBytesRequestProtectionLevel {
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
    impl GenerateRandomBytesRequestProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                GenerateRandomBytesRequestProtectionLevel::External => "EXTERNAL",
                GenerateRandomBytesRequestProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                GenerateRandomBytesRequestProtectionLevel::Hsm => "HSM",
                GenerateRandomBytesRequestProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                GenerateRandomBytesRequestProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GenerateRandomBytesRequestProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GenerateRandomBytesRequestProtectionLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GenerateRandomBytesRequestProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => GenerateRandomBytesRequestProtectionLevel::External,
                "EXTERNAL_VPC" => GenerateRandomBytesRequestProtectionLevel::ExternalVpc,
                "HSM" => GenerateRandomBytesRequestProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    GenerateRandomBytesRequestProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => GenerateRandomBytesRequestProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GenerateRandomBytesRequestProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GenerateRandomBytesRequestProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GenerateRandomBytesRequestProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => GenerateRandomBytesRequestProtectionLevel::External,
                "EXTERNAL_VPC" => GenerateRandomBytesRequestProtectionLevel::ExternalVpc,
                "HSM" => GenerateRandomBytesRequestProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    GenerateRandomBytesRequestProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => GenerateRandomBytesRequestProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GenerateRandomBytesRequestProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateRandomBytesRequestProtectionLevel {
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
    pub struct GenerateRandomBytesResponse {
        #[doc = "The generated data."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned GenerateRandomBytesResponse.data. An integrity check of GenerateRandomBytesResponse.data can be performed by computing the CRC32C checksum of GenerateRandomBytesResponse.data and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "dataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub data_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GenerateRandomBytesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateRandomBytesResponse {
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
    pub struct ImportCryptoKeyVersionRequest {
        #[doc = "Required. The algorithm of the key being imported. This does not need to match the version_template of the CryptoKey this version imports into."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm:
            ::std::option::Option<crate::schemas::ImportCryptoKeyVersionRequestAlgorithm>,
        #[doc = "Optional. The optional name of an existing CryptoKeyVersion to target for an import operation. If this field is not present, a new CryptoKeyVersion containing the supplied key material is created. If this field is present, the supplied key material is imported into the existing CryptoKeyVersion. To import into an existing CryptoKeyVersion, the CryptoKeyVersion must be a child of ImportCryptoKeyVersionRequest.parent, have been previously created via ImportCryptoKeyVersion, and be in DESTROYED or IMPORT_FAILED state. The key material and algorithm must match the previous CryptoKeyVersion exactly if the CryptoKeyVersion has ever contained key material."]
        #[serde(
            rename = "cryptoKeyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_version: ::std::option::Option<String>,
        #[doc = "Required. The name of the ImportJob that was used to wrap this key material."]
        #[serde(
            rename = "importJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_job: ::std::option::Option<String>,
        #[doc = "Wrapped key material produced with RSA_OAEP_3072_SHA1_AES_256 or RSA_OAEP_4096_SHA1_AES_256. This field contains the concatenation of two wrapped keys: 1. An ephemeral AES-256 wrapping key wrapped with the public_key using RSAES-OAEP with SHA-1/SHA-256, MGF1 with SHA-1/SHA-256, and an empty label. 2. The key to be imported, wrapped with the ephemeral AES-256 key using AES-KWP (RFC 5649). If importing symmetric key material, it is expected that the unwrapped key contains plain bytes. If importing asymmetric key material, it is expected that the unwrapped key is in PKCS#8-encoded DER format (the PrivateKeyInfo structure from RFC 5208). This format is the same as the format produced by PKCS#11 mechanism CKM_RSA_AES_KEY_WRAP."]
        #[serde(
            rename = "rsaAesWrappedKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rsa_aes_wrapped_key: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ImportCryptoKeyVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportCryptoKeyVersionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportCryptoKeyVersionRequestAlgorithm {
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
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
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
    impl ImportCryptoKeyVersionRequestAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportCryptoKeyVersionRequestAlgorithm::CryptoKeyVersionAlgorithmUnspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                ImportCryptoKeyVersionRequestAlgorithm::EcSignP256Sha256 => "EC_SIGN_P256_SHA256",
                ImportCryptoKeyVersionRequestAlgorithm::EcSignP384Sha384 => "EC_SIGN_P384_SHA384",
                ImportCryptoKeyVersionRequestAlgorithm::EcSignSecp256K1Sha256 => {
                    "EC_SIGN_SECP256K1_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::ExternalSymmetricEncryption => {
                    "EXTERNAL_SYMMETRIC_ENCRYPTION"
                }
                ImportCryptoKeyVersionRequestAlgorithm::GoogleSymmetricEncryption => {
                    "GOOGLE_SYMMETRIC_ENCRYPTION"
                }
                ImportCryptoKeyVersionRequestAlgorithm::HmacSha256 => "HMAC_SHA256",
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha1 => {
                    "RSA_DECRYPT_OAEP_2048_SHA1"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha256 => {
                    "RSA_DECRYPT_OAEP_2048_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha1 => {
                    "RSA_DECRYPT_OAEP_3072_SHA1"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha256 => {
                    "RSA_DECRYPT_OAEP_3072_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha1 => {
                    "RSA_DECRYPT_OAEP_4096_SHA1"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha256 => {
                    "RSA_DECRYPT_OAEP_4096_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha512 => {
                    "RSA_DECRYPT_OAEP_4096_SHA512"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs12048Sha256 => {
                    "RSA_SIGN_PKCS1_2048_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs13072Sha256 => {
                    "RSA_SIGN_PKCS1_3072_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha256 => {
                    "RSA_SIGN_PKCS1_4096_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha512 => {
                    "RSA_SIGN_PKCS1_4096_SHA512"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss2048Sha256 => {
                    "RSA_SIGN_PSS_2048_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss3072Sha256 => {
                    "RSA_SIGN_PSS_3072_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha256 => {
                    "RSA_SIGN_PSS_4096_SHA256"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha512 => {
                    "RSA_SIGN_PSS_4096_SHA512"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs12048 => {
                    "RSA_SIGN_RAW_PKCS1_2048"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs13072 => {
                    "RSA_SIGN_RAW_PKCS1_3072"
                }
                ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs14096 => {
                    "RSA_SIGN_RAW_PKCS1_4096"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImportCryptoKeyVersionRequestAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImportCryptoKeyVersionRequestAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImportCryptoKeyVersionRequestAlgorithm, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    ImportCryptoKeyVersionRequestAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => ImportCryptoKeyVersionRequestAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => ImportCryptoKeyVersionRequestAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    ImportCryptoKeyVersionRequestAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    ImportCryptoKeyVersionRequestAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => ImportCryptoKeyVersionRequestAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs12048
                }
                "RSA_SIGN_RAW_PKCS1_3072" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs13072
                }
                "RSA_SIGN_RAW_PKCS1_4096" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs14096
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImportCryptoKeyVersionRequestAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportCryptoKeyVersionRequestAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportCryptoKeyVersionRequestAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    ImportCryptoKeyVersionRequestAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => ImportCryptoKeyVersionRequestAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => ImportCryptoKeyVersionRequestAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::EcSignSecp256K1Sha256
                }
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    ImportCryptoKeyVersionRequestAlgorithm::ExternalSymmetricEncryption
                }
                "GOOGLE_SYMMETRIC_ENCRYPTION" => {
                    ImportCryptoKeyVersionRequestAlgorithm::GoogleSymmetricEncryption
                }
                "HMAC_SHA256" => ImportCryptoKeyVersionRequestAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha1
                }
                "RSA_DECRYPT_OAEP_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep2048Sha256
                }
                "RSA_DECRYPT_OAEP_3072_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha1
                }
                "RSA_DECRYPT_OAEP_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep3072Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA1" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha1
                }
                "RSA_DECRYPT_OAEP_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha256
                }
                "RSA_DECRYPT_OAEP_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaDecryptOaep4096Sha512
                }
                "RSA_SIGN_PKCS1_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs12048Sha256
                }
                "RSA_SIGN_PKCS1_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs13072Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha256
                }
                "RSA_SIGN_PKCS1_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPkcs14096Sha512
                }
                "RSA_SIGN_PSS_2048_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss2048Sha256
                }
                "RSA_SIGN_PSS_3072_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss3072Sha256
                }
                "RSA_SIGN_PSS_4096_SHA256" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha256
                }
                "RSA_SIGN_PSS_4096_SHA512" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignPss4096Sha512
                }
                "RSA_SIGN_RAW_PKCS1_2048" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs12048
                }
                "RSA_SIGN_RAW_PKCS1_3072" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs13072
                }
                "RSA_SIGN_RAW_PKCS1_4096" => {
                    ImportCryptoKeyVersionRequestAlgorithm::RsaSignRawPkcs14096
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
    impl ::google_field_selector::FieldSelector for ImportCryptoKeyVersionRequestAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportCryptoKeyVersionRequestAlgorithm {
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
    pub struct ImportJob {
        #[doc = "Output only. Statement that was generated and signed by the key creator (for example, an HSM) at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only present if the chosen ImportMethod is one with a protection level of HSM."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation: ::std::option::Option<crate::schemas::KeyOperationAttestation>,
        #[doc = "Output only. The time at which this ImportJob was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this ImportJob expired. Only present if state is EXPIRED."]
        #[serde(
            rename = "expireEventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expire_event_time: ::std::option::Option<String>,
        #[doc = "Output only. The time at which this ImportJob is scheduled for expiration and can no longer be used to import key material."]
        #[serde(
            rename = "expireTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expire_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this ImportJob's key material was generated."]
        #[serde(
            rename = "generateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub generate_time: ::std::option::Option<String>,
        #[doc = "Required. Immutable. The wrapping method to be used for incoming key material."]
        #[serde(
            rename = "importMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_method: ::std::option::Option<crate::schemas::ImportJobImportMethod>,
        #[doc = "Output only. The resource name for this ImportJob in the format `projects/*/locations/*/keyRings/*/importJobs/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<crate::schemas::ImportJobProtectionLevel>,
        #[doc = "Output only. The public key with which to wrap key material prior to import. Only returned if state is ACTIVE."]
        #[serde(
            rename = "publicKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key: ::std::option::Option<crate::schemas::WrappingPublicKey>,
        #[doc = "Output only. The current state of the ImportJob, indicating if it can be used."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ImportJobState>,
    }
    impl ::google_field_selector::FieldSelector for ImportJob {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportJob {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportJobImportMethod {
        #[doc = "Not specified."]
        ImportMethodUnspecified,
        #[doc = "This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 3072 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908)."]
        RsaOaep3072Sha1Aes256,
        #[doc = "This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 4096 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908)."]
        RsaOaep4096Sha1Aes256,
    }
    impl ImportJobImportMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportJobImportMethod::ImportMethodUnspecified => "IMPORT_METHOD_UNSPECIFIED",
                ImportJobImportMethod::RsaOaep3072Sha1Aes256 => "RSA_OAEP_3072_SHA1_AES_256",
                ImportJobImportMethod::RsaOaep4096Sha1Aes256 => "RSA_OAEP_4096_SHA1_AES_256",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImportJobImportMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImportJobImportMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImportJobImportMethod, ()> {
            Ok(match s {
                "IMPORT_METHOD_UNSPECIFIED" => ImportJobImportMethod::ImportMethodUnspecified,
                "RSA_OAEP_3072_SHA1_AES_256" => ImportJobImportMethod::RsaOaep3072Sha1Aes256,
                "RSA_OAEP_4096_SHA1_AES_256" => ImportJobImportMethod::RsaOaep4096Sha1Aes256,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImportJobImportMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportJobImportMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportJobImportMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPORT_METHOD_UNSPECIFIED" => ImportJobImportMethod::ImportMethodUnspecified,
                "RSA_OAEP_3072_SHA1_AES_256" => ImportJobImportMethod::RsaOaep3072Sha1Aes256,
                "RSA_OAEP_4096_SHA1_AES_256" => ImportJobImportMethod::RsaOaep4096Sha1Aes256,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImportJobImportMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportJobImportMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportJobProtectionLevel {
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
    impl ImportJobProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportJobProtectionLevel::External => "EXTERNAL",
                ImportJobProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                ImportJobProtectionLevel::Hsm => "HSM",
                ImportJobProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                ImportJobProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImportJobProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImportJobProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImportJobProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => ImportJobProtectionLevel::External,
                "EXTERNAL_VPC" => ImportJobProtectionLevel::ExternalVpc,
                "HSM" => ImportJobProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    ImportJobProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => ImportJobProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImportJobProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportJobProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportJobProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => ImportJobProtectionLevel::External,
                "EXTERNAL_VPC" => ImportJobProtectionLevel::ExternalVpc,
                "HSM" => ImportJobProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    ImportJobProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => ImportJobProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImportJobProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportJobProtectionLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportJobState {
        #[doc = "This job may be used in CreateCryptoKey and CreateCryptoKeyVersion requests."]
        Active,
        #[doc = "This job can no longer be used and may not leave this state once entered."]
        Expired,
        #[doc = "Not specified."]
        ImportJobStateUnspecified,
        #[doc = "The wrapping key for this job is still being generated. It may not be used. Cloud KMS will automatically mark this job as ACTIVE as soon as the wrapping key is generated."]
        PendingGeneration,
    }
    impl ImportJobState {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportJobState::Active => "ACTIVE",
                ImportJobState::Expired => "EXPIRED",
                ImportJobState::ImportJobStateUnspecified => "IMPORT_JOB_STATE_UNSPECIFIED",
                ImportJobState::PendingGeneration => "PENDING_GENERATION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImportJobState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImportJobState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImportJobState, ()> {
            Ok(match s {
                "ACTIVE" => ImportJobState::Active,
                "EXPIRED" => ImportJobState::Expired,
                "IMPORT_JOB_STATE_UNSPECIFIED" => ImportJobState::ImportJobStateUnspecified,
                "PENDING_GENERATION" => ImportJobState::PendingGeneration,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImportJobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportJobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportJobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => ImportJobState::Active,
                "EXPIRED" => ImportJobState::Expired,
                "IMPORT_JOB_STATE_UNSPECIFIED" => ImportJobState::ImportJobStateUnspecified,
                "PENDING_GENERATION" => ImportJobState::PendingGeneration,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImportJobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportJobState {
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
    pub struct KeyOperationAttestation {
        #[doc = "Output only. The certificate chains needed to validate the attestation"]
        #[serde(
            rename = "certChains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_chains: ::std::option::Option<crate::schemas::CertificateChains>,
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
        pub format: ::std::option::Option<crate::schemas::KeyOperationAttestationFormat>,
    }
    impl ::google_field_selector::FieldSelector for KeyOperationAttestation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyOperationAttestation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KeyOperationAttestationFormat {
        #[doc = "Not specified."]
        AttestationFormatUnspecified,
        #[doc = "Cavium HSM attestation compressed with gzip. Note that this format is defined by Cavium and subject to change at any time. See https://www.marvell.com/products/security-solutions/nitrox-hs-adapters/software-key-attestation.html."]
        CaviumV1Compressed,
        #[doc = "Cavium HSM attestation V2 compressed with gzip. This is a new format introduced in Cavium's version 3.2-08."]
        CaviumV2Compressed,
    }
    impl KeyOperationAttestationFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                KeyOperationAttestationFormat::AttestationFormatUnspecified => {
                    "ATTESTATION_FORMAT_UNSPECIFIED"
                }
                KeyOperationAttestationFormat::CaviumV1Compressed => "CAVIUM_V1_COMPRESSED",
                KeyOperationAttestationFormat::CaviumV2Compressed => "CAVIUM_V2_COMPRESSED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KeyOperationAttestationFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KeyOperationAttestationFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KeyOperationAttestationFormat, ()> {
            Ok(match s {
                "ATTESTATION_FORMAT_UNSPECIFIED" => {
                    KeyOperationAttestationFormat::AttestationFormatUnspecified
                }
                "CAVIUM_V1_COMPRESSED" => KeyOperationAttestationFormat::CaviumV1Compressed,
                "CAVIUM_V2_COMPRESSED" => KeyOperationAttestationFormat::CaviumV2Compressed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KeyOperationAttestationFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KeyOperationAttestationFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KeyOperationAttestationFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION_FORMAT_UNSPECIFIED" => {
                    KeyOperationAttestationFormat::AttestationFormatUnspecified
                }
                "CAVIUM_V1_COMPRESSED" => KeyOperationAttestationFormat::CaviumV1Compressed,
                "CAVIUM_V2_COMPRESSED" => KeyOperationAttestationFormat::CaviumV2Compressed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KeyOperationAttestationFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyOperationAttestationFormat {
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
    pub struct KeyRing {
        #[doc = "Output only. The time at which this KeyRing was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The resource name for the KeyRing in the format `projects/*/locations/*/keyRings/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KeyRing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyRing {
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
    pub struct ListCryptoKeyVersionsResponse {
        #[doc = "The list of CryptoKeyVersions."]
        #[serde(
            rename = "cryptoKeyVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_versions: ::std::option::Option<Vec<crate::schemas::CryptoKeyVersion>>,
        #[doc = "A token to retrieve next page of results. Pass this value in ListCryptoKeyVersionsRequest.page_token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of CryptoKeyVersions that matched the query."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListCryptoKeyVersionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCryptoKeyVersionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListCryptoKeyVersionsResponse {
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
    pub struct ListCryptoKeysResponse {
        #[doc = "The list of CryptoKeys."]
        #[serde(
            rename = "cryptoKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_keys: ::std::option::Option<Vec<crate::schemas::CryptoKey>>,
        #[doc = "A token to retrieve next page of results. Pass this value in ListCryptoKeysRequest.page_token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of CryptoKeys that matched the query."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListCryptoKeysResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCryptoKeysResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListCryptoKeysResponse {
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
    pub struct ListEkmConnectionsResponse {
        #[doc = "The list of EkmConnections."]
        #[serde(
            rename = "ekmConnections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ekm_connections: ::std::option::Option<Vec<crate::schemas::EkmConnection>>,
        #[doc = "A token to retrieve next page of results. Pass this value in ListEkmConnectionsRequest.page_token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of EkmConnections that matched the query."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListEkmConnectionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEkmConnectionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListEkmConnectionsResponse {
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
    pub struct ListImportJobsResponse {
        #[doc = "The list of ImportJobs."]
        #[serde(
            rename = "importJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_jobs: ::std::option::Option<Vec<crate::schemas::ImportJob>>,
        #[doc = "A token to retrieve next page of results. Pass this value in ListImportJobsRequest.page_token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of ImportJobs that matched the query."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListImportJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListImportJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListImportJobsResponse {
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
    pub struct ListKeyRingsResponse {
        #[doc = "The list of KeyRings."]
        #[serde(
            rename = "keyRings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_rings: ::std::option::Option<Vec<crate::schemas::KeyRing>>,
        #[doc = "A token to retrieve next page of results. Pass this value in ListKeyRingsRequest.page_token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of KeyRings that matched the query."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListKeyRingsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListKeyRingsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListKeyRingsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListLocationsResponse {
        #[doc = "A list of locations that matches the specified filter in the request."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListLocationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListLocationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListLocationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Location {
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        #[serde(
            rename = "locationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_id: ::std::option::Option<String>,
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
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
    pub struct LocationMetadata {
        #[doc = "Indicates whether CryptoKeys with protection_level EXTERNAL can be created in this location."]
        #[serde(
            rename = "ekmAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ekm_available: ::std::option::Option<bool>,
        #[doc = "Indicates whether CryptoKeys with protection_level HSM can be created in this location."]
        #[serde(
            rename = "hsmAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hsm_available: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for LocationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationMetadata {
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
    pub struct MacSignRequest {
        #[doc = "Required. The data to sign. The MAC tag is computed over this data field based on the specific algorithm."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the MacSignRequest.data. If specified, KeyManagementService will verify the integrity of the received MacSignRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacSignRequest.data) is equal to MacSignRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "dataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub data_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MacSignRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacSignRequest {
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
    pub struct MacSignResponse {
        #[doc = "The created signature."]
        #[serde(
            rename = "mac",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mac: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned MacSignResponse.mac. An integrity check of MacSignResponse.mac can be performed by computing the CRC32C checksum of MacSignResponse.mac and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "macCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub mac_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "The resource name of the CryptoKeyVersion used for signing. Check this field to verify that the intended resource was used for signing."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used for signing."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<crate::schemas::MacSignResponseProtectionLevel>,
        #[doc = "Integrity verification field. A flag indicating whether MacSignRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacSignRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacSignRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_data_crc_3_2c: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for MacSignResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacSignResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MacSignResponseProtectionLevel {
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
    impl MacSignResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                MacSignResponseProtectionLevel::External => "EXTERNAL",
                MacSignResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                MacSignResponseProtectionLevel::Hsm => "HSM",
                MacSignResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                MacSignResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MacSignResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MacSignResponseProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MacSignResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => MacSignResponseProtectionLevel::External,
                "EXTERNAL_VPC" => MacSignResponseProtectionLevel::ExternalVpc,
                "HSM" => MacSignResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    MacSignResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => MacSignResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MacSignResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MacSignResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MacSignResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => MacSignResponseProtectionLevel::External,
                "EXTERNAL_VPC" => MacSignResponseProtectionLevel::ExternalVpc,
                "HSM" => MacSignResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    MacSignResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => MacSignResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MacSignResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacSignResponseProtectionLevel {
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
    pub struct MacVerifyRequest {
        #[doc = "Required. The data used previously as a MacSignRequest.data to generate the MAC tag."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the MacVerifyRequest.data. If specified, KeyManagementService will verify the integrity of the received MacVerifyRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacVerifyRequest.data) is equal to MacVerifyRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "dataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub data_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "Required. The signature to verify."]
        #[serde(
            rename = "mac",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mac: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. An optional CRC32C checksum of the MacVerifyRequest.mac. If specified, KeyManagementService will verify the integrity of the received MacVerifyRequest.mac using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacVerifyRequest.tag) is equal to MacVerifyRequest.mac_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type."]
        #[serde(
            rename = "macCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub mac_crc_3_2c: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MacVerifyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacVerifyRequest {
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
    pub struct MacVerifyResponse {
        #[doc = "The resource name of the CryptoKeyVersion used for verification. Check this field to verify that the intended resource was used for verification."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used for verification."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level:
            ::std::option::Option<crate::schemas::MacVerifyResponseProtectionLevel>,
        #[doc = "This field indicates whether or not the verification operation for MacVerifyRequest.mac over MacVerifyRequest.data was successful."]
        #[serde(
            rename = "success",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub success: ::std::option::Option<bool>,
        #[doc = "Integrity verification field. A flag indicating whether MacVerifyRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacVerifyRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacVerifyRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedDataCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_data_crc_3_2c: ::std::option::Option<bool>,
        #[doc = "Integrity verification field. A flag indicating whether MacVerifyRequest.mac_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacVerifyRequest.mac_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacVerifyRequest.mac_crc32c but this field is still false, discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedMacCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_mac_crc_3_2c: ::std::option::Option<bool>,
        #[doc = "Integrity verification field. This value is used for the integrity verification of [MacVerifyResponse.success]. If the value of this field contradicts the value of [MacVerifyResponse.success], discard the response and perform a limited number of retries."]
        #[serde(
            rename = "verifiedSuccessIntegrity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_success_integrity: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for MacVerifyResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacVerifyResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MacVerifyResponseProtectionLevel {
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
    impl MacVerifyResponseProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                MacVerifyResponseProtectionLevel::External => "EXTERNAL",
                MacVerifyResponseProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                MacVerifyResponseProtectionLevel::Hsm => "HSM",
                MacVerifyResponseProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                MacVerifyResponseProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MacVerifyResponseProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MacVerifyResponseProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MacVerifyResponseProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => MacVerifyResponseProtectionLevel::External,
                "EXTERNAL_VPC" => MacVerifyResponseProtectionLevel::ExternalVpc,
                "HSM" => MacVerifyResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    MacVerifyResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => MacVerifyResponseProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MacVerifyResponseProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MacVerifyResponseProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MacVerifyResponseProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => MacVerifyResponseProtectionLevel::External,
                "EXTERNAL_VPC" => MacVerifyResponseProtectionLevel::ExternalVpc,
                "HSM" => MacVerifyResponseProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    MacVerifyResponseProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => MacVerifyResponseProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MacVerifyResponseProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MacVerifyResponseProtectionLevel {
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
    pub struct PublicKey {
        #[doc = "The Algorithm associated with this key."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::PublicKeyAlgorithm>,
        #[doc = "The name of the CryptoKeyVersion public key. Provided here for verification. NOTE: This field is in Beta."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13)."]
        #[serde(
            rename = "pem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pem: ::std::option::Option<String>,
        #[doc = "Integrity verification field. A CRC32C checksum of the returned PublicKey.pem. An integrity check of PublicKey.pem can be performed by computing the CRC32C checksum of PublicKey.pem and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        #[serde(
            rename = "pemCrc32c",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pem_crc_3_2c: ::std::option::Option<i64>,
        #[doc = "The ProtectionLevel of the CryptoKeyVersion public key."]
        #[serde(
            rename = "protectionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protection_level: ::std::option::Option<crate::schemas::PublicKeyProtectionLevel>,
    }
    impl ::google_field_selector::FieldSelector for PublicKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublicKey {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PublicKeyAlgorithm {
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
        #[doc = "HMAC-SHA256 signing with a 256 bit key."]
        HmacSha256,
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
    impl PublicKeyAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                PublicKeyAlgorithm::CryptoKeyVersionAlgorithmUnspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                PublicKeyAlgorithm::EcSignP256Sha256 => "EC_SIGN_P256_SHA256",
                PublicKeyAlgorithm::EcSignP384Sha384 => "EC_SIGN_P384_SHA384",
                PublicKeyAlgorithm::EcSignSecp256K1Sha256 => "EC_SIGN_SECP256K1_SHA256",
                PublicKeyAlgorithm::ExternalSymmetricEncryption => "EXTERNAL_SYMMETRIC_ENCRYPTION",
                PublicKeyAlgorithm::GoogleSymmetricEncryption => "GOOGLE_SYMMETRIC_ENCRYPTION",
                PublicKeyAlgorithm::HmacSha256 => "HMAC_SHA256",
                PublicKeyAlgorithm::RsaDecryptOaep2048Sha1 => "RSA_DECRYPT_OAEP_2048_SHA1",
                PublicKeyAlgorithm::RsaDecryptOaep2048Sha256 => "RSA_DECRYPT_OAEP_2048_SHA256",
                PublicKeyAlgorithm::RsaDecryptOaep3072Sha1 => "RSA_DECRYPT_OAEP_3072_SHA1",
                PublicKeyAlgorithm::RsaDecryptOaep3072Sha256 => "RSA_DECRYPT_OAEP_3072_SHA256",
                PublicKeyAlgorithm::RsaDecryptOaep4096Sha1 => "RSA_DECRYPT_OAEP_4096_SHA1",
                PublicKeyAlgorithm::RsaDecryptOaep4096Sha256 => "RSA_DECRYPT_OAEP_4096_SHA256",
                PublicKeyAlgorithm::RsaDecryptOaep4096Sha512 => "RSA_DECRYPT_OAEP_4096_SHA512",
                PublicKeyAlgorithm::RsaSignPkcs12048Sha256 => "RSA_SIGN_PKCS1_2048_SHA256",
                PublicKeyAlgorithm::RsaSignPkcs13072Sha256 => "RSA_SIGN_PKCS1_3072_SHA256",
                PublicKeyAlgorithm::RsaSignPkcs14096Sha256 => "RSA_SIGN_PKCS1_4096_SHA256",
                PublicKeyAlgorithm::RsaSignPkcs14096Sha512 => "RSA_SIGN_PKCS1_4096_SHA512",
                PublicKeyAlgorithm::RsaSignPss2048Sha256 => "RSA_SIGN_PSS_2048_SHA256",
                PublicKeyAlgorithm::RsaSignPss3072Sha256 => "RSA_SIGN_PSS_3072_SHA256",
                PublicKeyAlgorithm::RsaSignPss4096Sha256 => "RSA_SIGN_PSS_4096_SHA256",
                PublicKeyAlgorithm::RsaSignPss4096Sha512 => "RSA_SIGN_PSS_4096_SHA512",
                PublicKeyAlgorithm::RsaSignRawPkcs12048 => "RSA_SIGN_RAW_PKCS1_2048",
                PublicKeyAlgorithm::RsaSignRawPkcs13072 => "RSA_SIGN_RAW_PKCS1_3072",
                PublicKeyAlgorithm::RsaSignRawPkcs14096 => "RSA_SIGN_RAW_PKCS1_4096",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PublicKeyAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PublicKeyAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PublicKeyAlgorithm, ()> {
            Ok(match s {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    PublicKeyAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => PublicKeyAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => PublicKeyAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => PublicKeyAlgorithm::EcSignSecp256K1Sha256,
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => PublicKeyAlgorithm::ExternalSymmetricEncryption,
                "GOOGLE_SYMMETRIC_ENCRYPTION" => PublicKeyAlgorithm::GoogleSymmetricEncryption,
                "HMAC_SHA256" => PublicKeyAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep2048Sha1,
                "RSA_DECRYPT_OAEP_2048_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep2048Sha256,
                "RSA_DECRYPT_OAEP_3072_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep3072Sha1,
                "RSA_DECRYPT_OAEP_3072_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep3072Sha256,
                "RSA_DECRYPT_OAEP_4096_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha1,
                "RSA_DECRYPT_OAEP_4096_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha256,
                "RSA_DECRYPT_OAEP_4096_SHA512" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha512,
                "RSA_SIGN_PKCS1_2048_SHA256" => PublicKeyAlgorithm::RsaSignPkcs12048Sha256,
                "RSA_SIGN_PKCS1_3072_SHA256" => PublicKeyAlgorithm::RsaSignPkcs13072Sha256,
                "RSA_SIGN_PKCS1_4096_SHA256" => PublicKeyAlgorithm::RsaSignPkcs14096Sha256,
                "RSA_SIGN_PKCS1_4096_SHA512" => PublicKeyAlgorithm::RsaSignPkcs14096Sha512,
                "RSA_SIGN_PSS_2048_SHA256" => PublicKeyAlgorithm::RsaSignPss2048Sha256,
                "RSA_SIGN_PSS_3072_SHA256" => PublicKeyAlgorithm::RsaSignPss3072Sha256,
                "RSA_SIGN_PSS_4096_SHA256" => PublicKeyAlgorithm::RsaSignPss4096Sha256,
                "RSA_SIGN_PSS_4096_SHA512" => PublicKeyAlgorithm::RsaSignPss4096Sha512,
                "RSA_SIGN_RAW_PKCS1_2048" => PublicKeyAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => PublicKeyAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => PublicKeyAlgorithm::RsaSignRawPkcs14096,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PublicKeyAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PublicKeyAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PublicKeyAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => {
                    PublicKeyAlgorithm::CryptoKeyVersionAlgorithmUnspecified
                }
                "EC_SIGN_P256_SHA256" => PublicKeyAlgorithm::EcSignP256Sha256,
                "EC_SIGN_P384_SHA384" => PublicKeyAlgorithm::EcSignP384Sha384,
                "EC_SIGN_SECP256K1_SHA256" => PublicKeyAlgorithm::EcSignSecp256K1Sha256,
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => PublicKeyAlgorithm::ExternalSymmetricEncryption,
                "GOOGLE_SYMMETRIC_ENCRYPTION" => PublicKeyAlgorithm::GoogleSymmetricEncryption,
                "HMAC_SHA256" => PublicKeyAlgorithm::HmacSha256,
                "RSA_DECRYPT_OAEP_2048_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep2048Sha1,
                "RSA_DECRYPT_OAEP_2048_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep2048Sha256,
                "RSA_DECRYPT_OAEP_3072_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep3072Sha1,
                "RSA_DECRYPT_OAEP_3072_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep3072Sha256,
                "RSA_DECRYPT_OAEP_4096_SHA1" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha1,
                "RSA_DECRYPT_OAEP_4096_SHA256" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha256,
                "RSA_DECRYPT_OAEP_4096_SHA512" => PublicKeyAlgorithm::RsaDecryptOaep4096Sha512,
                "RSA_SIGN_PKCS1_2048_SHA256" => PublicKeyAlgorithm::RsaSignPkcs12048Sha256,
                "RSA_SIGN_PKCS1_3072_SHA256" => PublicKeyAlgorithm::RsaSignPkcs13072Sha256,
                "RSA_SIGN_PKCS1_4096_SHA256" => PublicKeyAlgorithm::RsaSignPkcs14096Sha256,
                "RSA_SIGN_PKCS1_4096_SHA512" => PublicKeyAlgorithm::RsaSignPkcs14096Sha512,
                "RSA_SIGN_PSS_2048_SHA256" => PublicKeyAlgorithm::RsaSignPss2048Sha256,
                "RSA_SIGN_PSS_3072_SHA256" => PublicKeyAlgorithm::RsaSignPss3072Sha256,
                "RSA_SIGN_PSS_4096_SHA256" => PublicKeyAlgorithm::RsaSignPss4096Sha256,
                "RSA_SIGN_PSS_4096_SHA512" => PublicKeyAlgorithm::RsaSignPss4096Sha512,
                "RSA_SIGN_RAW_PKCS1_2048" => PublicKeyAlgorithm::RsaSignRawPkcs12048,
                "RSA_SIGN_RAW_PKCS1_3072" => PublicKeyAlgorithm::RsaSignRawPkcs13072,
                "RSA_SIGN_RAW_PKCS1_4096" => PublicKeyAlgorithm::RsaSignRawPkcs14096,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PublicKeyAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublicKeyAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PublicKeyProtectionLevel {
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
    impl PublicKeyProtectionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                PublicKeyProtectionLevel::External => "EXTERNAL",
                PublicKeyProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
                PublicKeyProtectionLevel::Hsm => "HSM",
                PublicKeyProtectionLevel::ProtectionLevelUnspecified => {
                    "PROTECTION_LEVEL_UNSPECIFIED"
                }
                PublicKeyProtectionLevel::Software => "SOFTWARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PublicKeyProtectionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PublicKeyProtectionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PublicKeyProtectionLevel, ()> {
            Ok(match s {
                "EXTERNAL" => PublicKeyProtectionLevel::External,
                "EXTERNAL_VPC" => PublicKeyProtectionLevel::ExternalVpc,
                "HSM" => PublicKeyProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    PublicKeyProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => PublicKeyProtectionLevel::Software,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PublicKeyProtectionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PublicKeyProtectionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PublicKeyProtectionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => PublicKeyProtectionLevel::External,
                "EXTERNAL_VPC" => PublicKeyProtectionLevel::ExternalVpc,
                "HSM" => PublicKeyProtectionLevel::Hsm,
                "PROTECTION_LEVEL_UNSPECIFIED" => {
                    PublicKeyProtectionLevel::ProtectionLevelUnspecified
                }
                "SOFTWARE" => PublicKeyProtectionLevel::Software,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PublicKeyProtectionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublicKeyProtectionLevel {
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
    pub struct RestoreCryptoKeyVersionRequest {}
    impl ::google_field_selector::FieldSelector for RestoreCryptoKeyVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestoreCryptoKeyVersionRequest {
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
    pub struct ServiceResolver {
        #[doc = "Optional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest."]
        #[serde(
            rename = "endpointFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoint_filter: ::std::option::Option<String>,
        #[doc = "Required. The hostname of the EKM replica used at TLS and HTTP layers."]
        #[serde(
            rename = "hostname",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hostname: ::std::option::Option<String>,
        #[doc = "Required. A list of leaf server certificates used to authenticate HTTPS connections to the EKM replica. Currently, a maximum of 10 Certificate is supported."]
        #[serde(
            rename = "serverCertificates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_certificates: ::std::option::Option<Vec<crate::schemas::Certificate>>,
        #[doc = "Required. The resource name of the Service Directory service pointing to an EKM replica, in the format `projects/*/locations/*/namespaces/*/services/*`."]
        #[serde(
            rename = "serviceDirectoryService",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_directory_service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ServiceResolver {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceResolver {
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
    pub struct UpdateCryptoKeyPrimaryVersionRequest {
        #[doc = "Required. The id of the child CryptoKeyVersion to use as primary."]
        #[serde(
            rename = "cryptoKeyVersionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_version_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateCryptoKeyPrimaryVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateCryptoKeyPrimaryVersionRequest {
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
    pub struct WrappingPublicKey {
        #[doc = "The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13)."]
        #[serde(
            rename = "pem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pem: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WrappingPublicKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WrappingPublicKey {
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
                #[doc = "Generate random bytes using the Cloud KMS randomness source in the provided location."]
                pub fn generate_random_bytes(
                    &self,
                    request: crate::schemas::GenerateRandomBytesRequest,
                    location: impl Into<String>,
                ) -> GenerateRandomBytesRequestBuilder {
                    GenerateRandomBytesRequestBuilder {
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
                        location: location.into(),
                    }
                }
                #[doc = "Gets information about a location."]
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
                #[doc = "Lists information about the supported locations for this service."]
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
                #[doc = "Actions that can be performed on the ekm_connections resource"]
                pub fn ekm_connections(
                    &self,
                ) -> crate::resources::projects::locations::ekm_connections::EkmConnectionsActions
                {
                    crate::resources::projects::locations::ekm_connections::EkmConnectionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
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
            #[doc = "Created via [LocationsActions::generate_random_bytes()](struct.LocationsActions.html#method.generate_random_bytes)"]
            #[derive(Debug, Clone)]
            pub struct GenerateRandomBytesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GenerateRandomBytesRequest,
                location: String,
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
            impl<'a> GenerateRandomBytesRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GenerateRandomBytesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GenerateRandomBytesResponse, crate::Error>
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
                    let mut output = "https://cloudkms.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.location;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":generateRandomBytes");
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
            #[doc = "Created via [LocationsActions::get()](struct.LocationsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Location, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Location, crate::Error> {
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
                    let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
            #[doc = "Created via [LocationsActions::list()](struct.LocationsActions.html#method.list)"]
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
                #[doc = "A filter to narrow down results to a preferred subset. The filtering language accepts strings like `\"displayName=tokyo\"`, and is documented in more detail in [AIP-160](https://google.aip.dev/160)."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The maximum number of results to return. If not set, the service selects a default."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A page token received from the `next_page_token` field in the response. Send that page token to receive the subsequent page."]
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
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_locations<T>(
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
                    self.stream_locations_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_locations_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Location, crate::Error>> + 'a
                {
                    self.stream_locations_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_locations_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Location, crate::Error>> + 'a
                {
                    self.stream_locations_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_locations_with_fields<T, F>(
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
                        #[serde(rename = "locations")]
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
                        let mut selector = concat!("nextPageToken,", "locations").to_owned();
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
                    Item = Result<crate::schemas::ListLocationsResponse, crate::Error>,
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
                    Item = Result<crate::schemas::ListLocationsResponse, crate::Error>,
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
                ) -> Result<crate::schemas::ListLocationsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListLocationsResponse, crate::Error> {
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
                    let mut output = "https://cloudkms.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/locations");
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
            pub mod ekm_connections {
                pub mod params {}
                pub struct EkmConnectionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> EkmConnectionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a new EkmConnection in a given Project and Location."]
                    pub fn create(
                        &self,
                        request: crate::schemas::EkmConnection,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                            ekm_connection_id: None,
                        }
                    }
                    #[doc = "Returns metadata for a given EkmConnection."]
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
                    #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder {
                        GetIamPolicyRequestBuilder {
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
                            resource: resource.into(),
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Lists EkmConnections."]
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
                            filter: None,
                            order_by: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates an EkmConnection's metadata."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::EkmConnection,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
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
                            name: name.into(),
                            update_mask: None,
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
                #[doc = "Created via [EkmConnectionsActions::create()](struct.EkmConnectionsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::EkmConnection,
                    parent: String,
                    ekm_connection_id: ::std::option::Option<String>,
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
                    #[doc = "Required. It must be unique within a location and match the regular expression `[a-zA-Z0-9_-]{1,63}`."]
                    pub fn ekm_connection_id(mut self, value: impl Into<String>) -> Self {
                        self.ekm_connection_id = Some(value.into());
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
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/ekmConnections");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("ekmConnectionId", &self.ekm_connection_id)]);
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
                #[doc = "Created via [EkmConnectionsActions::get()](struct.EkmConnectionsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                #[doc = "Created via [EkmConnectionsActions::get_iam_policy()](struct.EkmConnectionsActions.html#method.get_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    resource: String,
                    options_requested_policy_version: ::std::option::Option<i32>,
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
                    #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                    pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                        self.options_requested_policy_version = Some(value);
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
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
                #[doc = "Created via [EkmConnectionsActions::list()](struct.EkmConnectionsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    order_by: ::std::option::Option<String>,
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
                    #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                    pub fn order_by(mut self, value: impl Into<String>) -> Self {
                        self.order_by = Some(value.into());
                        self
                    }
                    #[doc = "Optional. Optional limit on the number of EkmConnections to include in the response. Further EkmConnections can subsequently be obtained by including the ListEkmConnectionsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. Optional pagination token, returned earlier via ListEkmConnectionsResponse.next_page_token."]
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
                    #[doc = "\nExecute the request and yield each item in the `ekmConnections` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_ekm_connections<T>(
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
                        self.stream_ekm_connections_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `ekmConnections` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_ekm_connections_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::EkmConnection, crate::Error>,
                    > + 'a {
                        self.stream_ekm_connections_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `ekmConnections` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_ekm_connections_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::EkmConnection, crate::Error>,
                    > + 'a {
                        self.stream_ekm_connections_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `ekmConnections` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_ekm_connections_with_fields<T, F>(
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
                            #[serde(rename = "ekmConnections")]
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
                                concat!("nextPageToken,", "ekmConnections").to_owned();
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
                        Item = Result<crate::schemas::ListEkmConnectionsResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListEkmConnectionsResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListEkmConnectionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListEkmConnectionsResponse, crate::Error>
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/ekmConnections");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("orderBy", &self.order_by)]);
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
                #[doc = "Created via [EkmConnectionsActions::patch()](struct.EkmConnectionsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::EkmConnection,
                    name: String,
                    update_mask: ::std::option::Option<String>,
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
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Required. List of fields to be updated in this request."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
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
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::EkmConnection, crate::Error> {
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        req = req.query(&[("updateMask", &self.update_mask)]);
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
                #[doc = "Created via [EkmConnectionsActions::set_iam_policy()](struct.EkmConnectionsActions.html#method.set_iam_policy)"]
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                #[doc = "Created via [EkmConnectionsActions::test_iam_permissions()](struct.EkmConnectionsActions.html#method.test_iam_permissions)"]
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                    #[doc = "Create a new KeyRing in a given Project and Location."]
                    pub fn create(
                        &self,
                        request: crate::schemas::KeyRing,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                            key_ring_id: None,
                        }
                    }
                    #[doc = "Returns metadata for a given KeyRing."]
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
                    #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder {
                        GetIamPolicyRequestBuilder {
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
                            resource: resource.into(),
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Lists KeyRings."]
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
                            filter: None,
                            order_by: None,
                            page_size: None,
                            page_token: None,
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
                    #[doc = "Actions that can be performed on the crypto_keys resource"]                    pub fn crypto_keys (& self) -> crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: CryptoKeysActions{
                        crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: CryptoKeysActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the import_jobs resource"]                    pub fn import_jobs (& self) -> crate :: resources :: projects :: locations :: key_rings :: import_jobs :: ImportJobsActions{
                        crate :: resources :: projects :: locations :: key_rings :: import_jobs :: ImportJobsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [KeyRingsActions::create()](struct.KeyRingsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::KeyRing,
                    parent: String,
                    key_ring_id: ::std::option::Option<String>,
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
                    #[doc = "Required. It must be unique within a location and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                    pub fn key_ring_id(mut self, value: impl Into<String>) -> Self {
                        self.key_ring_id = Some(value.into());
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
                    ) -> Result<crate::schemas::KeyRing, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::KeyRing, crate::Error> {
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/keyRings");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("keyRingId", &self.key_ring_id)]);
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
                #[doc = "Created via [KeyRingsActions::get()](struct.KeyRingsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::KeyRing, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::KeyRing, crate::Error> {
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                #[doc = "Created via [KeyRingsActions::get_iam_policy()](struct.KeyRingsActions.html#method.get_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    resource: String,
                    options_requested_policy_version: ::std::option::Option<i32>,
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
                    #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                    pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                        self.options_requested_policy_version = Some(value);
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
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
                #[doc = "Created via [KeyRingsActions::list()](struct.KeyRingsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    order_by: ::std::option::Option<String>,
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
                    #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                    pub fn order_by(mut self, value: impl Into<String>) -> Self {
                        self.order_by = Some(value.into());
                        self
                    }
                    #[doc = "Optional. Optional limit on the number of KeyRings to include in the response. Further KeyRings can subsequently be obtained by including the ListKeyRingsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. Optional pagination token, returned earlier via ListKeyRingsResponse.next_page_token."]
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
                    #[doc = "\nExecute the request and yield each item in the `keyRings` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_key_rings<T>(
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
                        self.stream_key_rings_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `keyRings` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_key_rings_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::KeyRing, crate::Error>> + 'a
                    {
                        self.stream_key_rings_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `keyRings` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_key_rings_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::KeyRing, crate::Error>> + 'a
                    {
                        self.stream_key_rings_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `keyRings` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_key_rings_with_fields<T, F>(
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
                            #[serde(rename = "keyRings")]
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
                            let mut selector = concat!("nextPageToken,", "keyRings").to_owned();
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
                        Item = Result<crate::schemas::ListKeyRingsResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListKeyRingsResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListKeyRingsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListKeyRingsResponse, crate::Error>
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/keyRings");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("orderBy", &self.order_by)]);
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
                #[doc = "Created via [KeyRingsActions::set_iam_policy()](struct.KeyRingsActions.html#method.set_iam_policy)"]
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                #[doc = "Created via [KeyRingsActions::test_iam_permissions()](struct.KeyRingsActions.html#method.test_iam_permissions)"]
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
                        let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                pub mod crypto_keys {
                    pub mod params {
                        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                        pub enum ListVersionView {
                            #[doc = "Default view for each CryptoKeyVersion. Does not include the attestation field."]
                            CryptoKeyVersionViewUnspecified,
                            #[doc = "Provides all fields in each CryptoKeyVersion, including the attestation."]
                            Full,
                        }
                        impl ListVersionView {
                            pub fn as_str(self) -> &'static str {
                                match self {
                                    ListVersionView::CryptoKeyVersionViewUnspecified => {
                                        "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
                                    }
                                    ListVersionView::Full => "FULL",
                                }
                            }
                        }
                        impl ::std::convert::AsRef<str> for ListVersionView {
                            fn as_ref(&self) -> &str {
                                self.as_str()
                            }
                        }
                        impl ::std::str::FromStr for ListVersionView {
                            type Err = ();
                            fn from_str(s: &str) -> ::std::result::Result<ListVersionView, ()> {
                                Ok(match s {
                                    "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => {
                                        ListVersionView::CryptoKeyVersionViewUnspecified
                                    }
                                    "FULL" => ListVersionView::Full,
                                    _ => return Err(()),
                                })
                            }
                        }
                        impl ::std::fmt::Display for ListVersionView {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                                f.write_str(self.as_str())
                            }
                        }
                        impl ::serde::Serialize for ListVersionView {
                            fn serialize<S>(
                                &self,
                                serializer: S,
                            ) -> ::std::result::Result<S::Ok, S::Error>
                            where
                                S: ::serde::ser::Serializer,
                            {
                                serializer.serialize_str(self.as_str())
                            }
                        }
                        impl<'de> ::serde::Deserialize<'de> for ListVersionView {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> ::std::result::Result<Self, D::Error>
                            where
                                D: ::serde::de::Deserializer<'de>,
                            {
                                let value: &'de str = <&str>::deserialize(deserializer)?;
                                Ok(match value {
                                    "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => {
                                        ListVersionView::CryptoKeyVersionViewUnspecified
                                    }
                                    "FULL" => ListVersionView::Full,
                                    _ => {
                                        return Err(::serde::de::Error::custom(format!(
                                            "invalid enum for #name: {}",
                                            value
                                        )))
                                    }
                                })
                            }
                        }
                        impl ::google_field_selector::FieldSelector for ListVersionView {
                            fn fields() -> Vec<::google_field_selector::Field> {
                                Vec::new()
                            }
                        }
                        impl ::google_field_selector::ToFieldType for ListVersionView {
                            fn field_type() -> ::google_field_selector::FieldType {
                                ::google_field_selector::FieldType::Leaf
                            }
                        }
                    }
                    pub struct CryptoKeysActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> CryptoKeysActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Create a new CryptoKey within a KeyRing. CryptoKey.purpose and CryptoKey.version_template.algorithm are required."]
                        pub fn create(
                            &self,
                            request: crate::schemas::CryptoKey,
                            parent: impl Into<String>,
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
                                parent: parent.into(),
                                crypto_key_id: None,
                                skip_initial_version_creation: None,
                            }
                        }
                        #[doc = "Decrypts data that was protected by Encrypt. The CryptoKey.purpose must be ENCRYPT_DECRYPT."]
                        pub fn decrypt(
                            &self,
                            request: crate::schemas::DecryptRequest,
                            name: impl Into<String>,
                        ) -> DecryptRequestBuilder {
                            DecryptRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Encrypts data, so that it can only be recovered by a call to Decrypt. The CryptoKey.purpose must be ENCRYPT_DECRYPT."]
                        pub fn encrypt(
                            &self,
                            request: crate::schemas::EncryptRequest,
                            name: impl Into<String>,
                        ) -> EncryptRequestBuilder {
                            EncryptRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Returns metadata for a given CryptoKey, as well as its primary CryptoKeyVersion."]
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
                        #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                        pub fn get_iam_policy(
                            &self,
                            resource: impl Into<String>,
                        ) -> GetIamPolicyRequestBuilder {
                            GetIamPolicyRequestBuilder {
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
                                resource: resource.into(),
                                options_requested_policy_version: None,
                            }
                        }
                        #[doc = "Lists CryptoKeys."]
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
                                filter: None,
                                order_by: None,
                                page_size: None,
                                page_token: None,
                                version_view: None,
                            }
                        }
                        #[doc = "Update a CryptoKey."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::CryptoKey,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder {
                            PatchRequestBuilder {
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
                                name: name.into(),
                                update_mask: None,
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
                        #[doc = "Update the version of a CryptoKey that will be used in Encrypt. Returns an error if called on a key whose purpose is not ENCRYPT_DECRYPT."]
                        pub fn update_primary_version(
                            &self,
                            request: crate::schemas::UpdateCryptoKeyPrimaryVersionRequest,
                            name: impl Into<String>,
                        ) -> UpdatePrimaryVersionRequestBuilder {
                            UpdatePrimaryVersionRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Actions that can be performed on the crypto_key_versions resource"]                        pub fn crypto_key_versions (& self) -> crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: crypto_key_versions :: CryptoKeyVersionsActions{
                            crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: crypto_key_versions :: CryptoKeyVersionsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                    }
                    #[doc = "Created via [CryptoKeysActions::create()](struct.CryptoKeysActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::CryptoKey,
                        parent: String,
                        crypto_key_id: ::std::option::Option<String>,
                        skip_initial_version_creation: ::std::option::Option<bool>,
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
                        #[doc = "Required. It must be unique within a KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                        pub fn crypto_key_id(mut self, value: impl Into<String>) -> Self {
                            self.crypto_key_id = Some(value.into());
                            self
                        }
                        #[doc = "If set to true, the request will create a CryptoKey without any CryptoKeyVersions. You must manually call CreateCryptoKeyVersion or ImportCryptoKeyVersion before you can use this CryptoKey."]
                        pub fn skip_initial_version_creation(mut self, value: bool) -> Self {
                            self.skip_initial_version_creation = Some(value);
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
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                            req = req.query(&[("cryptoKeyId", &self.crypto_key_id)]);
                            req = req.query(&[(
                                "skipInitialVersionCreation",
                                &self.skip_initial_version_creation,
                            )]);
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
                    #[doc = "Created via [CryptoKeysActions::decrypt()](struct.CryptoKeysActions.html#method.decrypt)"]
                    #[derive(Debug, Clone)]
                    pub struct DecryptRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::DecryptRequest,
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
                    impl<'a> DecryptRequestBuilder<'a> {
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
                        ) -> Result<crate::schemas::DecryptResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::DecryptResponse, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":decrypt");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [CryptoKeysActions::encrypt()](struct.CryptoKeysActions.html#method.encrypt)"]
                    #[derive(Debug, Clone)]
                    pub struct EncryptRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::EncryptRequest,
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
                    impl<'a> EncryptRequestBuilder<'a> {
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
                        ) -> Result<crate::schemas::EncryptResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::EncryptResponse, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":encrypt");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [CryptoKeysActions::get()](struct.CryptoKeysActions.html#method.get)"]
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
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                    #[doc = "Created via [CryptoKeysActions::get_iam_policy()](struct.CryptoKeysActions.html#method.get_iam_policy)"]
                    #[derive(Debug, Clone)]
                    pub struct GetIamPolicyRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        resource: String,
                        options_requested_policy_version: ::std::option::Option<i32>,
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
                        #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                        pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                            self.options_requested_policy_version = Some(value);
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[(
                                "options.requestedPolicyVersion",
                                &self.options_requested_policy_version,
                            )]);
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
                    #[doc = "Created via [CryptoKeysActions::list()](struct.CryptoKeysActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , parent : String , filter : :: std :: option :: Option < String > , order_by : :: std :: option :: Option < String > , page_size : :: std :: option :: Option < i32 > , page_token : :: std :: option :: Option < String > , version_view : :: std :: option :: Option < crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: params :: ListVersionView > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                        pub fn order_by(mut self, value: impl Into<String>) -> Self {
                            self.order_by = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Optional limit on the number of CryptoKeys to include in the response. Further CryptoKeys can subsequently be obtained by including the ListCryptoKeysResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. Optional pagination token, returned earlier via ListCryptoKeysResponse.next_page_token."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "The fields of the primary version to include in the response."]
                        pub fn version_view(
                            mut self,
                            value : crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: params :: ListVersionView,
                        ) -> Self {
                            self.version_view = Some(value);
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
                        #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_crypto_keys<T>(
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
                            self.stream_crypto_keys_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_crypto_keys_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::CryptoKey, crate::Error>,
                        > + 'a {
                            self.stream_crypto_keys_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `cryptoKeys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_crypto_keys_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::CryptoKey, crate::Error>,
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
                                    concat!("nextPageToken,", "cryptoKeys").to_owned();
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
                            Item = Result<crate::schemas::ListCryptoKeysResponse, crate::Error>,
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
                            Item = Result<crate::schemas::ListCryptoKeysResponse, crate::Error>,
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
                        ) -> Result<crate::schemas::ListCryptoKeysResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListCryptoKeysResponse, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("orderBy", &self.order_by)]);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
                            req = req.query(&[("versionView", &self.version_view)]);
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
                    #[doc = "Created via [CryptoKeysActions::patch()](struct.CryptoKeysActions.html#method.patch)"]
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::CryptoKey,
                        name: String,
                        update_mask: ::std::option::Option<String>,
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
                    impl<'a> PatchRequestBuilder<'a> {
                        #[doc = "Required. List of fields to be updated in this request."]
                        pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                            self.update_mask = Some(value.into());
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
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            req = req.query(&[("updateMask", &self.update_mask)]);
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
                    #[doc = "Created via [CryptoKeysActions::set_iam_policy()](struct.CryptoKeysActions.html#method.set_iam_policy)"]
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [CryptoKeysActions::test_iam_permissions()](struct.CryptoKeysActions.html#method.test_iam_permissions)"]
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [CryptoKeysActions::update_primary_version()](struct.CryptoKeysActions.html#method.update_primary_version)"]
                    #[derive(Debug, Clone)]
                    pub struct UpdatePrimaryVersionRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::UpdateCryptoKeyPrimaryVersionRequest,
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
                    impl<'a> UpdatePrimaryVersionRequestBuilder<'a> {
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
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::CryptoKey, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":updatePrimaryVersion");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    pub mod crypto_key_versions {
                        pub mod params {
                            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                            pub enum ListView {
                                #[doc = "Default view for each CryptoKeyVersion. Does not include the attestation field."]
                                CryptoKeyVersionViewUnspecified,
                                #[doc = "Provides all fields in each CryptoKeyVersion, including the attestation."]
                                Full,
                            }
                            impl ListView {
                                pub fn as_str(self) -> &'static str {
                                    match self {
                                        ListView::CryptoKeyVersionViewUnspecified => {
                                            "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
                                        }
                                        ListView::Full => "FULL",
                                    }
                                }
                            }
                            impl ::std::convert::AsRef<str> for ListView {
                                fn as_ref(&self) -> &str {
                                    self.as_str()
                                }
                            }
                            impl ::std::str::FromStr for ListView {
                                type Err = ();
                                fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                                    Ok(match s {
                                        "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => {
                                            ListView::CryptoKeyVersionViewUnspecified
                                        }
                                        "FULL" => ListView::Full,
                                        _ => return Err(()),
                                    })
                                }
                            }
                            impl ::std::fmt::Display for ListView {
                                fn fmt(
                                    &self,
                                    f: &mut std::fmt::Formatter<'_>,
                                ) -> ::std::fmt::Result {
                                    f.write_str(self.as_str())
                                }
                            }
                            impl ::serde::Serialize for ListView {
                                fn serialize<S>(
                                    &self,
                                    serializer: S,
                                ) -> ::std::result::Result<S::Ok, S::Error>
                                where
                                    S: ::serde::ser::Serializer,
                                {
                                    serializer.serialize_str(self.as_str())
                                }
                            }
                            impl<'de> ::serde::Deserialize<'de> for ListView {
                                fn deserialize<D>(
                                    deserializer: D,
                                ) -> ::std::result::Result<Self, D::Error>
                                where
                                    D: ::serde::de::Deserializer<'de>,
                                {
                                    let value: &'de str = <&str>::deserialize(deserializer)?;
                                    Ok(match value {
                                        "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => {
                                            ListView::CryptoKeyVersionViewUnspecified
                                        }
                                        "FULL" => ListView::Full,
                                        _ => {
                                            return Err(::serde::de::Error::custom(format!(
                                                "invalid enum for #name: {}",
                                                value
                                            )))
                                        }
                                    })
                                }
                            }
                            impl ::google_field_selector::FieldSelector for ListView {
                                fn fields() -> Vec<::google_field_selector::Field> {
                                    Vec::new()
                                }
                            }
                            impl ::google_field_selector::ToFieldType for ListView {
                                fn field_type() -> ::google_field_selector::FieldType {
                                    ::google_field_selector::FieldType::Leaf
                                }
                            }
                        }
                        pub struct CryptoKeyVersionsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> CryptoKeyVersionsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Decrypts data that was encrypted with a public key retrieved from GetPublicKey corresponding to a CryptoKeyVersion with CryptoKey.purpose ASYMMETRIC_DECRYPT."]
                            pub fn asymmetric_decrypt(
                                &self,
                                request: crate::schemas::AsymmetricDecryptRequest,
                                name: impl Into<String>,
                            ) -> AsymmetricDecryptRequestBuilder {
                                AsymmetricDecryptRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                            #[doc = "Signs data using a CryptoKeyVersion with CryptoKey.purpose ASYMMETRIC_SIGN, producing a signature that can be verified with the public key retrieved from GetPublicKey."]
                            pub fn asymmetric_sign(
                                &self,
                                request: crate::schemas::AsymmetricSignRequest,
                                name: impl Into<String>,
                            ) -> AsymmetricSignRequestBuilder {
                                AsymmetricSignRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                            #[doc = "Create a new CryptoKeyVersion in a CryptoKey. The server will assign the next sequential id. If unset, state will be set to ENABLED."]
                            pub fn create(
                                &self,
                                request: crate::schemas::CryptoKeyVersion,
                                parent: impl Into<String>,
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
                                    parent: parent.into(),
                                }
                            }
                            #[doc = "Schedule a CryptoKeyVersion for destruction. Upon calling this method, CryptoKeyVersion.state will be set to DESTROY_SCHEDULED, and destroy_time will be set to the time destroy_scheduled_duration in the future. At that time, the state will automatically change to DESTROYED, and the key material will be irrevocably destroyed. Before the destroy_time is reached, RestoreCryptoKeyVersion may be called to reverse the process."]
                            pub fn destroy(
                                &self,
                                request: crate::schemas::DestroyCryptoKeyVersionRequest,
                                name: impl Into<String>,
                            ) -> DestroyRequestBuilder {
                                DestroyRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                            #[doc = "Returns metadata for a given CryptoKeyVersion."]
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
                            #[doc = "Returns the public key for the given CryptoKeyVersion. The CryptoKey.purpose must be ASYMMETRIC_SIGN or ASYMMETRIC_DECRYPT."]
                            pub fn get_public_key(
                                &self,
                                name: impl Into<String>,
                            ) -> GetPublicKeyRequestBuilder {
                                GetPublicKeyRequestBuilder {
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
                            #[doc = "Import wrapped key material into a CryptoKeyVersion. All requests must specify a CryptoKey. If a CryptoKeyVersion is additionally specified in the request, key material will be reimported into that version. Otherwise, a new version will be created, and will be assigned the next sequential id within the CryptoKey."]
                            pub fn import(
                                &self,
                                request: crate::schemas::ImportCryptoKeyVersionRequest,
                                parent: impl Into<String>,
                            ) -> ImportRequestBuilder {
                                ImportRequestBuilder {
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
                            #[doc = "Lists CryptoKeyVersions."]
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
                                    filter: None,
                                    order_by: None,
                                    page_size: None,
                                    page_token: None,
                                    view: None,
                                }
                            }
                            #[doc = "Signs data using a CryptoKeyVersion with CryptoKey.purpose MAC, producing a tag that can be verified by another source with the same key."]
                            pub fn mac_sign(
                                &self,
                                request: crate::schemas::MacSignRequest,
                                name: impl Into<String>,
                            ) -> MacSignRequestBuilder {
                                MacSignRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                            #[doc = "Verifies MAC tag using a CryptoKeyVersion with CryptoKey.purpose MAC, and returns a response that indicates whether or not the verification was successful."]
                            pub fn mac_verify(
                                &self,
                                request: crate::schemas::MacVerifyRequest,
                                name: impl Into<String>,
                            ) -> MacVerifyRequestBuilder {
                                MacVerifyRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                            #[doc = "Update a CryptoKeyVersion's metadata. state may be changed between ENABLED and DISABLED using this method. See DestroyCryptoKeyVersion and RestoreCryptoKeyVersion to move between other states."]
                            pub fn patch(
                                &self,
                                request: crate::schemas::CryptoKeyVersion,
                                name: impl Into<String>,
                            ) -> PatchRequestBuilder {
                                PatchRequestBuilder {
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
                                    name: name.into(),
                                    update_mask: None,
                                }
                            }
                            #[doc = "Restore a CryptoKeyVersion in the DESTROY_SCHEDULED state. Upon restoration of the CryptoKeyVersion, state will be set to DISABLED, and destroy_time will be cleared."]
                            pub fn restore(
                                &self,
                                request: crate::schemas::RestoreCryptoKeyVersionRequest,
                                name: impl Into<String>,
                            ) -> RestoreRequestBuilder {
                                RestoreRequestBuilder {
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
                                    name: name.into(),
                                }
                            }
                        }
                        #[doc = "Created via [CryptoKeyVersionsActions::asymmetric_decrypt()](struct.CryptoKeyVersionsActions.html#method.asymmetric_decrypt)"]
                        #[derive(Debug, Clone)]
                        pub struct AsymmetricDecryptRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::AsymmetricDecryptRequest,
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
                        impl<'a> AsymmetricDecryptRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::AsymmetricDecryptResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::AsymmetricDecryptResponse, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":asymmetricDecrypt");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::asymmetric_sign()](struct.CryptoKeyVersionsActions.html#method.asymmetric_sign)"]
                        #[derive(Debug, Clone)]
                        pub struct AsymmetricSignRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::AsymmetricSignRequest,
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
                        impl<'a> AsymmetricSignRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::AsymmetricSignResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::AsymmetricSignResponse, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":asymmetricSign");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::create()](struct.CryptoKeyVersionsActions.html#method.create)"]
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::CryptoKeyVersion,
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/cryptoKeyVersions");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::destroy()](struct.CryptoKeyVersionsActions.html#method.destroy)"]
                        #[derive(Debug, Clone)]
                        pub struct DestroyRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::DestroyCryptoKeyVersionRequest,
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
                        impl<'a> DestroyRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":destroy");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::get()](struct.CryptoKeyVersionsActions.html#method.get)"]
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        #[doc = "Created via [CryptoKeyVersionsActions::get_public_key()](struct.CryptoKeyVersionsActions.html#method.get_public_key)"]
                        #[derive(Debug, Clone)]
                        pub struct GetPublicKeyRequestBuilder<'a> {
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
                        impl<'a> GetPublicKeyRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::PublicKey, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::PublicKey, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/publicKey");
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
                        #[doc = "Created via [CryptoKeyVersionsActions::import()](struct.CryptoKeyVersionsActions.html#method.import)"]
                        #[derive(Debug, Clone)]
                        pub struct ImportRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::ImportCryptoKeyVersionRequest,
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
                        impl<'a> ImportRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/cryptoKeyVersions:import");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::list()](struct.CryptoKeyVersionsActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , parent : String , filter : :: std :: option :: Option < String > , order_by : :: std :: option :: Option < String > , page_size : :: std :: option :: Option < i32 > , page_token : :: std :: option :: Option < String > , view : :: std :: option :: Option < crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: crypto_key_versions :: params :: ListView > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
                        impl<'a> ListRequestBuilder<'a> {
                            #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                            pub fn filter(mut self, value: impl Into<String>) -> Self {
                                self.filter = Some(value.into());
                                self
                            }
                            #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                                self.order_by = Some(value.into());
                                self
                            }
                            #[doc = "Optional. Optional limit on the number of CryptoKeyVersions to include in the response. Further CryptoKeyVersions can subsequently be obtained by including the ListCryptoKeyVersionsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "Optional. Optional pagination token, returned earlier via ListCryptoKeyVersionsResponse.next_page_token."]
                            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                                self.page_token = Some(value.into());
                                self
                            }
                            #[doc = "The fields to include in the response."]
                            pub fn view(
                                mut self,
                                value : crate :: resources :: projects :: locations :: key_rings :: crypto_keys :: crypto_key_versions :: params :: ListView,
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
                            #[doc = "\nExecute the request and yield each item in the `cryptoKeyVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                            pub fn stream_crypto_key_versions<T>(
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
                                self.stream_crypto_key_versions_with_fields(fields)
                            }
                            #[doc = "\nExecute the request and yield each item in the `cryptoKeyVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                            pub fn stream_crypto_key_versions_with_default_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::CryptoKeyVersion, crate::Error>,
                            > + 'a {
                                self.stream_crypto_key_versions_with_fields(None::<String>)
                            }
                            #[doc = "\nExecute the request and yield each item in the `cryptoKeyVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                            pub fn stream_crypto_key_versions_with_all_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::CryptoKeyVersion, crate::Error>,
                            > + 'a {
                                self.stream_crypto_key_versions_with_fields(Some("*"))
                            }
                            #[doc = "\nExecute the request and yield each item in the `cryptoKeyVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                            pub fn stream_crypto_key_versions_with_fields<T, F>(
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
                                    #[serde(rename = "cryptoKeyVersions")]
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
                                        concat!("nextPageToken,", "cryptoKeyVersions").to_owned();
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
                                    crate::schemas::ListCryptoKeyVersionsResponse,
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
                                    crate::schemas::ListCryptoKeyVersionsResponse,
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
                            ) -> Result<crate::schemas::ListCryptoKeyVersionsResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::ListCryptoKeyVersionsResponse, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/cryptoKeyVersions");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("filter", &self.filter)]);
                                req = req.query(&[("orderBy", &self.order_by)]);
                                req = req.query(&[("pageSize", &self.page_size)]);
                                req = req.query(&[("pageToken", &self.page_token)]);
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
                        #[doc = "Created via [CryptoKeyVersionsActions::mac_sign()](struct.CryptoKeyVersionsActions.html#method.mac_sign)"]
                        #[derive(Debug, Clone)]
                        pub struct MacSignRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::MacSignRequest,
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
                        impl<'a> MacSignRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::MacSignResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::MacSignResponse, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":macSign");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::mac_verify()](struct.CryptoKeyVersionsActions.html#method.mac_verify)"]
                        #[derive(Debug, Clone)]
                        pub struct MacVerifyRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::MacVerifyRequest,
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
                        impl<'a> MacVerifyRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::MacVerifyResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::MacVerifyResponse, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":macVerify");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                        #[doc = "Created via [CryptoKeyVersionsActions::patch()](struct.CryptoKeyVersionsActions.html#method.patch)"]
                        #[derive(Debug, Clone)]
                        pub struct PatchRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::CryptoKeyVersion,
                            name: String,
                            update_mask: ::std::option::Option<String>,
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
                        impl<'a> PatchRequestBuilder<'a> {
                            #[doc = "Required. List of fields to be updated in this request."]
                            pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                                self.update_mask = Some(value.into());
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                                req = req.query(&[("updateMask", &self.update_mask)]);
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
                        #[doc = "Created via [CryptoKeyVersionsActions::restore()](struct.CryptoKeyVersionsActions.html#method.restore)"]
                        #[derive(Debug, Clone)]
                        pub struct RestoreRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::RestoreCryptoKeyVersionRequest,
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
                        impl<'a> RestoreRequestBuilder<'a> {
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
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::CryptoKeyVersion, crate::Error>
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
                                let mut output = "https://cloudkms.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":restore");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
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
                }
                pub mod import_jobs {
                    pub mod params {}
                    pub struct ImportJobsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ImportJobsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Create a new ImportJob within a KeyRing. ImportJob.import_method is required."]
                        pub fn create(
                            &self,
                            request: crate::schemas::ImportJob,
                            parent: impl Into<String>,
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
                                parent: parent.into(),
                                import_job_id: None,
                            }
                        }
                        #[doc = "Returns metadata for a given ImportJob."]
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
                        #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                        pub fn get_iam_policy(
                            &self,
                            resource: impl Into<String>,
                        ) -> GetIamPolicyRequestBuilder {
                            GetIamPolicyRequestBuilder {
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
                                resource: resource.into(),
                                options_requested_policy_version: None,
                            }
                        }
                        #[doc = "Lists ImportJobs."]
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
                                filter: None,
                                order_by: None,
                                page_size: None,
                                page_token: None,
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
                    #[doc = "Created via [ImportJobsActions::create()](struct.ImportJobsActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::ImportJob,
                        parent: String,
                        import_job_id: ::std::option::Option<String>,
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
                        #[doc = "Required. It must be unique within a KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                        pub fn import_job_id(mut self, value: impl Into<String>) -> Self {
                            self.import_job_id = Some(value.into());
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
                        ) -> Result<crate::schemas::ImportJob, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ImportJob, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/importJobs");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                            req = req.query(&[("importJobId", &self.import_job_id)]);
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
                    #[doc = "Created via [ImportJobsActions::get()](struct.ImportJobsActions.html#method.get)"]
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
                        ) -> Result<crate::schemas::ImportJob, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ImportJob, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                    #[doc = "Created via [ImportJobsActions::get_iam_policy()](struct.ImportJobsActions.html#method.get_iam_policy)"]
                    #[derive(Debug, Clone)]
                    pub struct GetIamPolicyRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        resource: String,
                        options_requested_policy_version: ::std::option::Option<i32>,
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
                        #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                        pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                            self.options_requested_policy_version = Some(value);
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[(
                                "options.requestedPolicyVersion",
                                &self.options_requested_policy_version,
                            )]);
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
                    #[doc = "Created via [ImportJobsActions::list()](struct.ImportJobsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        filter: ::std::option::Option<String>,
                        order_by: ::std::option::Option<String>,
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
                        #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                        pub fn order_by(mut self, value: impl Into<String>) -> Self {
                            self.order_by = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Optional limit on the number of ImportJobs to include in the response. Further ImportJobs can subsequently be obtained by including the ListImportJobsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. Optional pagination token, returned earlier via ListImportJobsResponse.next_page_token."]
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
                        #[doc = "\nExecute the request and yield each item in the `importJobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_import_jobs<T>(
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
                            self.stream_import_jobs_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `importJobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_import_jobs_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::ImportJob, crate::Error>,
                        > + 'a {
                            self.stream_import_jobs_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `importJobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_import_jobs_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::ImportJob, crate::Error>,
                        > + 'a {
                            self.stream_import_jobs_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `importJobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_import_jobs_with_fields<T, F>(
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
                                #[serde(rename = "importJobs")]
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
                                    concat!("nextPageToken,", "importJobs").to_owned();
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
                            Item = Result<crate::schemas::ListImportJobsResponse, crate::Error>,
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
                            Item = Result<crate::schemas::ListImportJobsResponse, crate::Error>,
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
                        ) -> Result<crate::schemas::ListImportJobsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListImportJobsResponse, crate::Error>
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/importJobs");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("orderBy", &self.order_by)]);
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
                    #[doc = "Created via [ImportJobsActions::set_iam_policy()](struct.ImportJobsActions.html#method.set_iam_policy)"]
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [ImportJobsActions::test_iam_permissions()](struct.ImportJobsActions.html#method.test_iam_permissions)"]
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
                            let mut output = "https://cloudkms.googleapis.com/".to_owned();
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
