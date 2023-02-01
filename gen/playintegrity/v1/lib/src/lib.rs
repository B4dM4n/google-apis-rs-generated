#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [v_1](resources/v_1/struct.V1Actions.html)\n  * [*decodeIntegrityToken*](resources/v_1/struct.DecodeIntegrityTokenRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Private Service: https://www.googleapis.com/auth/playintegrity\n\n`https://www.googleapis.com/auth/playintegrity`"]
    pub const PLAYINTEGRITY: &str = "https://www.googleapis.com/auth/playintegrity";
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
    pub struct AccountActivity {
        #[doc = "Required. Indicates the activity level of the account."]
        #[serde(
            rename = "activityLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity_level: ::std::option::Option<crate::schemas::AccountActivityActivityLevel>,
    }
    impl ::google_field_selector::FieldSelector for AccountActivity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountActivity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountActivityActivityLevel {
        #[doc = "Activity level has not been set."]
        ActivityLevelUnspecified,
        #[doc = "Google Play store activity is typical for the user account or accounts on the device."]
        TypicalBasic,
        #[doc = "Google Play store activity is typical for the user account or accounts on the device, with harder to replicate signals."]
        TypicalStrong,
        #[doc = "Account activity level is not evaluated because one of the prerequisite conditions is not met (e.g., device is not trusted, the user does not have Play app license)"]
        Unevaluated,
        #[doc = "Google Play does not have sufficient activity for the user account on the device. The account may be new, or it may lack activity on Google Play."]
        Unknown,
        #[doc = "Google Play store activity is unusual for at least one of the user accounts on the device. Google Play recommends checking that this is a real user."]
        Unusual,
    }
    impl AccountActivityActivityLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountActivityActivityLevel::ActivityLevelUnspecified => {
                    "ACTIVITY_LEVEL_UNSPECIFIED"
                }
                AccountActivityActivityLevel::TypicalBasic => "TYPICAL_BASIC",
                AccountActivityActivityLevel::TypicalStrong => "TYPICAL_STRONG",
                AccountActivityActivityLevel::Unevaluated => "UNEVALUATED",
                AccountActivityActivityLevel::Unknown => "UNKNOWN",
                AccountActivityActivityLevel::Unusual => "UNUSUAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AccountActivityActivityLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AccountActivityActivityLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AccountActivityActivityLevel, ()> {
            Ok(match s {
                "ACTIVITY_LEVEL_UNSPECIFIED" => {
                    AccountActivityActivityLevel::ActivityLevelUnspecified
                }
                "TYPICAL_BASIC" => AccountActivityActivityLevel::TypicalBasic,
                "TYPICAL_STRONG" => AccountActivityActivityLevel::TypicalStrong,
                "UNEVALUATED" => AccountActivityActivityLevel::Unevaluated,
                "UNKNOWN" => AccountActivityActivityLevel::Unknown,
                "UNUSUAL" => AccountActivityActivityLevel::Unusual,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AccountActivityActivityLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountActivityActivityLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountActivityActivityLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVITY_LEVEL_UNSPECIFIED" => {
                    AccountActivityActivityLevel::ActivityLevelUnspecified
                }
                "TYPICAL_BASIC" => AccountActivityActivityLevel::TypicalBasic,
                "TYPICAL_STRONG" => AccountActivityActivityLevel::TypicalStrong,
                "UNEVALUATED" => AccountActivityActivityLevel::Unevaluated,
                "UNKNOWN" => AccountActivityActivityLevel::Unknown,
                "UNUSUAL" => AccountActivityActivityLevel::Unusual,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AccountActivityActivityLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountActivityActivityLevel {
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
    pub struct AccountDetails {
        #[doc = "Details about the account activity for the user in the scope."]
        #[serde(
            rename = "accountActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_activity: ::std::option::Option<crate::schemas::AccountActivity>,
        #[doc = "Required. Details about the licensing status of the user for the app in the scope."]
        #[serde(
            rename = "appLicensingVerdict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_licensing_verdict:
            ::std::option::Option<crate::schemas::AccountDetailsAppLicensingVerdict>,
    }
    impl ::google_field_selector::FieldSelector for AccountDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountDetailsAppLicensingVerdict {
        #[doc = "The app and certificate match the versions distributed by Play."]
        Licensed,
        #[doc = "Licensing details were not evaluated since a necessary requirement was missed. For example DeviceIntegrity did not meet the minimum bar or the application was not a known Play version."]
        Unevaluated,
        #[doc = "Play does not have sufficient information to evaluate licensing details"]
        Unknown,
        #[doc = "The certificate or package name does not match Google Play records."]
        Unlicensed,
    }
    impl AccountDetailsAppLicensingVerdict {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountDetailsAppLicensingVerdict::Licensed => "LICENSED",
                AccountDetailsAppLicensingVerdict::Unevaluated => "UNEVALUATED",
                AccountDetailsAppLicensingVerdict::Unknown => "UNKNOWN",
                AccountDetailsAppLicensingVerdict::Unlicensed => "UNLICENSED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AccountDetailsAppLicensingVerdict {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AccountDetailsAppLicensingVerdict {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AccountDetailsAppLicensingVerdict, ()> {
            Ok(match s {
                "LICENSED" => AccountDetailsAppLicensingVerdict::Licensed,
                "UNEVALUATED" => AccountDetailsAppLicensingVerdict::Unevaluated,
                "UNKNOWN" => AccountDetailsAppLicensingVerdict::Unknown,
                "UNLICENSED" => AccountDetailsAppLicensingVerdict::Unlicensed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AccountDetailsAppLicensingVerdict {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountDetailsAppLicensingVerdict {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountDetailsAppLicensingVerdict {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LICENSED" => AccountDetailsAppLicensingVerdict::Licensed,
                "UNEVALUATED" => AccountDetailsAppLicensingVerdict::Unevaluated,
                "UNKNOWN" => AccountDetailsAppLicensingVerdict::Unknown,
                "UNLICENSED" => AccountDetailsAppLicensingVerdict::Unlicensed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AccountDetailsAppLicensingVerdict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountDetailsAppLicensingVerdict {
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
    pub struct AppIntegrity {
        #[doc = "Required. Details about the app recognition verdict"]
        #[serde(
            rename = "appRecognitionVerdict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_recognition_verdict:
            ::std::option::Option<crate::schemas::AppIntegrityAppRecognitionVerdict>,
        #[doc = "The SHA256 hash of the requesting app’s signing certificates (base64 web-safe encoded). Set iff app_recognition_verdict != UNEVALUATED."]
        #[serde(
            rename = "certificateSha256Digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub certificate_sha_256_digest: ::std::option::Option<Vec<String>>,
        #[doc = "Package name of the application under attestation. Set iff app_recognition_verdict != UNEVALUATED."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Version code of the application. Set iff app_recognition_verdict != UNEVALUATED."]
        #[serde(
            rename = "versionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub version_code: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AppIntegrity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppIntegrity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppIntegrityAppRecognitionVerdict {
        #[doc = "The app and certificate match the versions distributed by Play."]
        PlayRecognized,
        #[doc = "Application integrity was not evaluated since a necessary requirement was missed. For example DeviceIntegrity did not meet the minimum bar."]
        Unevaluated,
        #[doc = "Play does not have sufficient information to evaluate app integrity"]
        Unknown,
        #[doc = "The certificate or package name does not match Google Play records."]
        UnrecognizedVersion,
    }
    impl AppIntegrityAppRecognitionVerdict {
        pub fn as_str(self) -> &'static str {
            match self {
                AppIntegrityAppRecognitionVerdict::PlayRecognized => "PLAY_RECOGNIZED",
                AppIntegrityAppRecognitionVerdict::Unevaluated => "UNEVALUATED",
                AppIntegrityAppRecognitionVerdict::Unknown => "UNKNOWN",
                AppIntegrityAppRecognitionVerdict::UnrecognizedVersion => "UNRECOGNIZED_VERSION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AppIntegrityAppRecognitionVerdict {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AppIntegrityAppRecognitionVerdict {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AppIntegrityAppRecognitionVerdict, ()> {
            Ok(match s {
                "PLAY_RECOGNIZED" => AppIntegrityAppRecognitionVerdict::PlayRecognized,
                "UNEVALUATED" => AppIntegrityAppRecognitionVerdict::Unevaluated,
                "UNKNOWN" => AppIntegrityAppRecognitionVerdict::Unknown,
                "UNRECOGNIZED_VERSION" => AppIntegrityAppRecognitionVerdict::UnrecognizedVersion,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AppIntegrityAppRecognitionVerdict {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppIntegrityAppRecognitionVerdict {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppIntegrityAppRecognitionVerdict {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLAY_RECOGNIZED" => AppIntegrityAppRecognitionVerdict::PlayRecognized,
                "UNEVALUATED" => AppIntegrityAppRecognitionVerdict::Unevaluated,
                "UNKNOWN" => AppIntegrityAppRecognitionVerdict::Unknown,
                "UNRECOGNIZED_VERSION" => AppIntegrityAppRecognitionVerdict::UnrecognizedVersion,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AppIntegrityAppRecognitionVerdict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppIntegrityAppRecognitionVerdict {
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
    pub struct DecodeIntegrityTokenRequest {
        #[doc = "Encoded integrity token."]
        #[serde(
            rename = "integrityToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrity_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DecodeIntegrityTokenRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DecodeIntegrityTokenRequest {
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
    pub struct DecodeIntegrityTokenResponse {
        #[doc = "Plain token payload generated from the decoded integrity token."]
        #[serde(
            rename = "tokenPayloadExternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token_payload_external: ::std::option::Option<crate::schemas::TokenPayloadExternal>,
    }
    impl ::google_field_selector::FieldSelector for DecodeIntegrityTokenResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DecodeIntegrityTokenResponse {
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
    pub struct DeviceIntegrity {
        #[doc = "Details about the integrity of the device the app is running on"]
        #[serde(
            rename = "deviceRecognitionVerdict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_recognition_verdict: ::std::option::Option<
            Vec<crate::schemas::DeviceIntegrityDeviceRecognitionVerdictItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for DeviceIntegrity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceIntegrity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceIntegrityDeviceRecognitionVerdictItems {
        #[doc = "App is running on a device that passes basic system integrity checks, but may not meet Android platform compatibility requirements and may not be approved to run Google Play services."]
        MeetsBasicIntegrity,
        #[doc = "App is running on GMS Android device with Google Play services."]
        MeetsDeviceIntegrity,
        #[doc = "App is running on GMS Android device with Google Play services and has a strong guarantee of system integrity such as a hardware-backed keystore."]
        MeetsStrongIntegrity,
        #[doc = "App is running on an Android emulator with Google Play services which meets core Android compatibility requirements."]
        MeetsVirtualIntegrity,
        #[doc = "Play does not have sufficient information to evaluate device integrity"]
        Unknown,
    }
    impl DeviceIntegrityDeviceRecognitionVerdictItems {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceIntegrityDeviceRecognitionVerdictItems::MeetsBasicIntegrity => {
                    "MEETS_BASIC_INTEGRITY"
                }
                DeviceIntegrityDeviceRecognitionVerdictItems::MeetsDeviceIntegrity => {
                    "MEETS_DEVICE_INTEGRITY"
                }
                DeviceIntegrityDeviceRecognitionVerdictItems::MeetsStrongIntegrity => {
                    "MEETS_STRONG_INTEGRITY"
                }
                DeviceIntegrityDeviceRecognitionVerdictItems::MeetsVirtualIntegrity => {
                    "MEETS_VIRTUAL_INTEGRITY"
                }
                DeviceIntegrityDeviceRecognitionVerdictItems::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceIntegrityDeviceRecognitionVerdictItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceIntegrityDeviceRecognitionVerdictItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<DeviceIntegrityDeviceRecognitionVerdictItems, ()> {
            Ok(match s {
                "MEETS_BASIC_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsBasicIntegrity
                }
                "MEETS_DEVICE_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsDeviceIntegrity
                }
                "MEETS_STRONG_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsStrongIntegrity
                }
                "MEETS_VIRTUAL_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsVirtualIntegrity
                }
                "UNKNOWN" => DeviceIntegrityDeviceRecognitionVerdictItems::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceIntegrityDeviceRecognitionVerdictItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceIntegrityDeviceRecognitionVerdictItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceIntegrityDeviceRecognitionVerdictItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MEETS_BASIC_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsBasicIntegrity
                }
                "MEETS_DEVICE_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsDeviceIntegrity
                }
                "MEETS_STRONG_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsStrongIntegrity
                }
                "MEETS_VIRTUAL_INTEGRITY" => {
                    DeviceIntegrityDeviceRecognitionVerdictItems::MeetsVirtualIntegrity
                }
                "UNKNOWN" => DeviceIntegrityDeviceRecognitionVerdictItems::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceIntegrityDeviceRecognitionVerdictItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceIntegrityDeviceRecognitionVerdictItems {
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
    pub struct RequestDetails {
        #[doc = "Nonce that was provided in the request (which is base64 web-safe no-wrap)."]
        #[serde(
            rename = "nonce",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nonce: ::std::option::Option<String>,
        #[doc = "Request hash that was provided in the request."]
        #[serde(
            rename = "requestHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_hash: ::std::option::Option<String>,
        #[doc = "Required. Application package name this attestation was requested for. Note: This field makes no guarantees or promises on the caller integrity. For details on application integrity, check application_integrity."]
        #[serde(
            rename = "requestPackageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_package_name: ::std::option::Option<String>,
        #[doc = "Required. Timestamp, in milliseconds, of the integrity application request."]
        #[serde(
            rename = "timestampMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for RequestDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestDetails {
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
    pub struct TestingDetails {
        #[doc = "Required. Indicates that the information contained in this payload is a testing response that is statically overridden for a tester."]
        #[serde(
            rename = "isTestingResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_testing_response: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TestingDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestingDetails {
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
    pub struct TokenPayloadExternal {
        #[doc = "Required. Details about the Play Store account."]
        #[serde(
            rename = "accountDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_details: ::std::option::Option<crate::schemas::AccountDetails>,
        #[doc = "Required. Details about the application integrity."]
        #[serde(
            rename = "appIntegrity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_integrity: ::std::option::Option<crate::schemas::AppIntegrity>,
        #[doc = "Required. Details about the device integrity."]
        #[serde(
            rename = "deviceIntegrity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_integrity: ::std::option::Option<crate::schemas::DeviceIntegrity>,
        #[doc = "Required. Details about the integrity request."]
        #[serde(
            rename = "requestDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_details: ::std::option::Option<crate::schemas::RequestDetails>,
        #[doc = "Indicates that this payload is generated for testing purposes and contains any additional data that is linked with testing status."]
        #[serde(
            rename = "testingDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub testing_details: ::std::option::Option<crate::schemas::TestingDetails>,
    }
    impl ::google_field_selector::FieldSelector for TokenPayloadExternal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TokenPayloadExternal {
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
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::resources::v_1::V1Actions {
        crate::resources::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod v_1 {
        pub mod params {}
        pub struct V1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Decodes the integrity token and returns the token payload."]
            pub fn decode_integrity_token(
                &self,
                request: crate::schemas::DecodeIntegrityTokenRequest,
                package_name: impl Into<String>,
            ) -> DecodeIntegrityTokenRequestBuilder {
                DecodeIntegrityTokenRequestBuilder {
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
                    package_name: package_name.into(),
                }
            }
        }
        #[doc = "Created via [V1Actions::decode_integrity_token()](struct.V1Actions.html#method.decode_integrity_token)"]
        #[derive(Debug, Clone)]
        pub struct DecodeIntegrityTokenRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DecodeIntegrityTokenRequest,
            package_name: String,
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
        impl<'a> DecodeIntegrityTokenRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::DecodeIntegrityTokenResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DecodeIntegrityTokenResponse, crate::Error> {
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
                let mut output = "https://playintegrity.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.package_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":decodeIntegrityToken");
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
