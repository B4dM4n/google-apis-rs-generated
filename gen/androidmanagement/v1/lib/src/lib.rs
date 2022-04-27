#![doc = "# Resources and Methods\n    * [enterprises](resources/enterprises/struct.EnterprisesActions.html)\n      * [*create*](resources/enterprises/struct.CreateRequestBuilder.html), [*delete*](resources/enterprises/struct.DeleteRequestBuilder.html), [*get*](resources/enterprises/struct.GetRequestBuilder.html), [*list*](resources/enterprises/struct.ListRequestBuilder.html), [*patch*](resources/enterprises/struct.PatchRequestBuilder.html)\n      * [applications](resources/enterprises/applications/struct.ApplicationsActions.html)\n        * [*get*](resources/enterprises/applications/struct.GetRequestBuilder.html)\n      * [devices](resources/enterprises/devices/struct.DevicesActions.html)\n        * [*delete*](resources/enterprises/devices/struct.DeleteRequestBuilder.html), [*get*](resources/enterprises/devices/struct.GetRequestBuilder.html), [*issueCommand*](resources/enterprises/devices/struct.IssueCommandRequestBuilder.html), [*list*](resources/enterprises/devices/struct.ListRequestBuilder.html), [*patch*](resources/enterprises/devices/struct.PatchRequestBuilder.html)\n        * [operations](resources/enterprises/devices/operations/struct.OperationsActions.html)\n          * [*cancel*](resources/enterprises/devices/operations/struct.CancelRequestBuilder.html), [*delete*](resources/enterprises/devices/operations/struct.DeleteRequestBuilder.html), [*get*](resources/enterprises/devices/operations/struct.GetRequestBuilder.html), [*list*](resources/enterprises/devices/operations/struct.ListRequestBuilder.html)\n      * [enrollment_tokens](resources/enterprises/enrollment_tokens/struct.EnrollmentTokensActions.html)\n        * [*create*](resources/enterprises/enrollment_tokens/struct.CreateRequestBuilder.html), [*delete*](resources/enterprises/enrollment_tokens/struct.DeleteRequestBuilder.html)\n      * [policies](resources/enterprises/policies/struct.PoliciesActions.html)\n        * [*delete*](resources/enterprises/policies/struct.DeleteRequestBuilder.html), [*get*](resources/enterprises/policies/struct.GetRequestBuilder.html), [*list*](resources/enterprises/policies/struct.ListRequestBuilder.html), [*patch*](resources/enterprises/policies/struct.PatchRequestBuilder.html)\n      * [web_apps](resources/enterprises/web_apps/struct.WebAppsActions.html)\n        * [*create*](resources/enterprises/web_apps/struct.CreateRequestBuilder.html), [*delete*](resources/enterprises/web_apps/struct.DeleteRequestBuilder.html), [*get*](resources/enterprises/web_apps/struct.GetRequestBuilder.html), [*list*](resources/enterprises/web_apps/struct.ListRequestBuilder.html), [*patch*](resources/enterprises/web_apps/struct.PatchRequestBuilder.html)\n      * [web_tokens](resources/enterprises/web_tokens/struct.WebTokensActions.html)\n        * [*create*](resources/enterprises/web_tokens/struct.CreateRequestBuilder.html)\n    * [signup_urls](resources/signup_urls/struct.SignupUrlsActions.html)\n      * [*create*](resources/signup_urls/struct.CreateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Manage Android devices and apps for your customers\n\n`https://www.googleapis.com/auth/androidmanagement`"]
    pub const ANDROIDMANAGEMENT: &str = "https://www.googleapis.com/auth/androidmanagement";
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
    pub struct AdvancedSecurityOverrides {
        #[doc = "Controls Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC). Enabling Common Criteria Mode increases certain security components on a device, including AES-GCM encryption of Bluetooth Long Term Keys, and Wi-Fi configuration stores.Warning: Common Criteria Mode enforces a strict security model typically only required for IT products used in national security systems and other highly sensitive organizations. Standard device use may be affected. Only enabled if required."]
        #[serde(
            rename = "commonCriteriaMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_criteria_mode:
            ::std::option::Option<crate::schemas::AdvancedSecurityOverridesCommonCriteriaMode>,
        #[doc = "Controls access to developer settings: developer options and safe boot. Replaces safeBootDisabled (deprecated) and debuggingFeaturesAllowed (deprecated)."]
        #[serde(
            rename = "developerSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_settings:
            ::std::option::Option<crate::schemas::AdvancedSecurityOverridesDeveloperSettings>,
        #[doc = "Whether Google Play Protect verification (https://support.google.com/accounts/answer/2812853) is enforced. Replaces ensureVerifyAppsEnabled (deprecated)."]
        #[serde(
            rename = "googlePlayProtectVerifyApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_play_protect_verify_apps: ::std::option::Option<
            crate::schemas::AdvancedSecurityOverridesGooglePlayProtectVerifyApps,
        >,
        #[doc = "Personal apps that can read work profile notifications using a NotificationListenerService (https://developer.android.com/reference/android/service/notification/NotificationListenerService). By default, no personal apps (aside from system apps) can read work notifications. Each value in the list must be a package name."]
        #[serde(
            rename = "personalAppsThatCanReadWorkNotifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub personal_apps_that_can_read_work_notifications: ::std::option::Option<Vec<String>>,
        #[doc = "The policy for untrusted apps (apps from unknown sources) enforced on the device. Replaces install_unknown_sources_allowed (deprecated)."]
        #[serde(
            rename = "untrustedAppsPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub untrusted_apps_policy:
            ::std::option::Option<crate::schemas::AdvancedSecurityOverridesUntrustedAppsPolicy>,
    }
    impl ::google_field_selector::FieldSelector for AdvancedSecurityOverrides {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvancedSecurityOverrides {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdvancedSecurityOverridesCommonCriteriaMode {
        #[doc = "Default. Disables Common Criteria Mode."]
        CommonCriteriaModeDisabled,
        #[doc = "Enables Common Criteria Mode."]
        CommonCriteriaModeEnabled,
        #[doc = "Unspecified. Defaults to COMMON_CRITERIA_MODE_DISABLED."]
        CommonCriteriaModeUnspecified,
    }
    impl AdvancedSecurityOverridesCommonCriteriaMode {
        pub fn as_str(self) -> &'static str {
            match self {
                AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeDisabled => {
                    "COMMON_CRITERIA_MODE_DISABLED"
                }
                AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeEnabled => {
                    "COMMON_CRITERIA_MODE_ENABLED"
                }
                AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeUnspecified => {
                    "COMMON_CRITERIA_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AdvancedSecurityOverridesCommonCriteriaMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdvancedSecurityOverridesCommonCriteriaMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AdvancedSecurityOverridesCommonCriteriaMode, ()> {
            Ok(match s {
                "COMMON_CRITERIA_MODE_DISABLED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeDisabled
                }
                "COMMON_CRITERIA_MODE_ENABLED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeEnabled
                }
                "COMMON_CRITERIA_MODE_UNSPECIFIED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AdvancedSecurityOverridesCommonCriteriaMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdvancedSecurityOverridesCommonCriteriaMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdvancedSecurityOverridesCommonCriteriaMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMON_CRITERIA_MODE_DISABLED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeDisabled
                }
                "COMMON_CRITERIA_MODE_ENABLED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeEnabled
                }
                "COMMON_CRITERIA_MODE_UNSPECIFIED" => {
                    AdvancedSecurityOverridesCommonCriteriaMode::CommonCriteriaModeUnspecified
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
    impl ::google_field_selector::FieldSelector for AdvancedSecurityOverridesCommonCriteriaMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvancedSecurityOverridesCommonCriteriaMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdvancedSecurityOverridesDeveloperSettings {
        #[doc = "Allows all developer settings. The user can access and optionally configure the settings."]
        DeveloperSettingsAllowed,
        #[doc = "Default. Disables all developer settings and prevents the user from accessing them."]
        DeveloperSettingsDisabled,
        #[doc = "Unspecified. Defaults to DEVELOPER_SETTINGS_DISABLED."]
        DeveloperSettingsUnspecified,
    }
    impl AdvancedSecurityOverridesDeveloperSettings {
        pub fn as_str(self) -> &'static str {
            match self {
                AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsAllowed => {
                    "DEVELOPER_SETTINGS_ALLOWED"
                }
                AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsDisabled => {
                    "DEVELOPER_SETTINGS_DISABLED"
                }
                AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsUnspecified => {
                    "DEVELOPER_SETTINGS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AdvancedSecurityOverridesDeveloperSettings {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdvancedSecurityOverridesDeveloperSettings {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AdvancedSecurityOverridesDeveloperSettings, ()> {
            Ok(match s {
                "DEVELOPER_SETTINGS_ALLOWED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsAllowed
                }
                "DEVELOPER_SETTINGS_DISABLED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsDisabled
                }
                "DEVELOPER_SETTINGS_UNSPECIFIED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AdvancedSecurityOverridesDeveloperSettings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdvancedSecurityOverridesDeveloperSettings {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdvancedSecurityOverridesDeveloperSettings {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVELOPER_SETTINGS_ALLOWED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsAllowed
                }
                "DEVELOPER_SETTINGS_DISABLED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsDisabled
                }
                "DEVELOPER_SETTINGS_UNSPECIFIED" => {
                    AdvancedSecurityOverridesDeveloperSettings::DeveloperSettingsUnspecified
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
    impl ::google_field_selector::FieldSelector for AdvancedSecurityOverridesDeveloperSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvancedSecurityOverridesDeveloperSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        #[doc = "Unspecified. Defaults to VERIFY_APPS_ENFORCED."]
        GooglePlayProtectVerifyAppsUnspecified,
        #[doc = "Default. Force-enables app verification."]
        VerifyAppsEnforced,
        #[doc = "Allows the user to choose whether to enable app verification."]
        VerifyAppsUserChoice,
    }
    impl AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        pub fn as_str(self) -> &'static str {
            match self { AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: GooglePlayProtectVerifyAppsUnspecified => "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED" , AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsEnforced => "VERIFY_APPS_ENFORCED" , AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsUserChoice => "VERIFY_APPS_USER_CHOICE" , }
        }
    }
    impl ::std::convert::AsRef<str> for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AdvancedSecurityOverridesGooglePlayProtectVerifyApps, ()>
        {
            Ok (match s { "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: GooglePlayProtectVerifyAppsUnspecified , "VERIFY_APPS_ENFORCED" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsEnforced , "VERIFY_APPS_USER_CHOICE" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsUserChoice , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: GooglePlayProtectVerifyAppsUnspecified , "VERIFY_APPS_ENFORCED" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsEnforced , "VERIFY_APPS_USER_CHOICE" => AdvancedSecurityOverridesGooglePlayProtectVerifyApps :: VerifyAppsUserChoice , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for AdvancedSecurityOverridesGooglePlayProtectVerifyApps
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvancedSecurityOverridesGooglePlayProtectVerifyApps {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdvancedSecurityOverridesUntrustedAppsPolicy {
        #[doc = "Allow untrusted app installs on entire device."]
        AllowInstallDeviceWide,
        #[doc = "For devices with work profiles, allow untrusted app installs in the device's personal profile only."]
        AllowInstallInPersonalProfileOnly,
        #[doc = "Default. Disallow untrusted app installs on entire device."]
        DisallowInstall,
        #[doc = "Unspecified. Defaults to DISALLOW_INSTALL."]
        UntrustedAppsPolicyUnspecified,
    }
    impl AdvancedSecurityOverridesUntrustedAppsPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallDeviceWide => {
                    "ALLOW_INSTALL_DEVICE_WIDE"
                }
                AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallInPersonalProfileOnly => {
                    "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY"
                }
                AdvancedSecurityOverridesUntrustedAppsPolicy::DisallowInstall => "DISALLOW_INSTALL",
                AdvancedSecurityOverridesUntrustedAppsPolicy::UntrustedAppsPolicyUnspecified => {
                    "UNTRUSTED_APPS_POLICY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AdvancedSecurityOverridesUntrustedAppsPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdvancedSecurityOverridesUntrustedAppsPolicy {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AdvancedSecurityOverridesUntrustedAppsPolicy, ()> {
            Ok(match s {
                "ALLOW_INSTALL_DEVICE_WIDE" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallDeviceWide
                }
                "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallInPersonalProfileOnly
                }
                "DISALLOW_INSTALL" => AdvancedSecurityOverridesUntrustedAppsPolicy::DisallowInstall,
                "UNTRUSTED_APPS_POLICY_UNSPECIFIED" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::UntrustedAppsPolicyUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AdvancedSecurityOverridesUntrustedAppsPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdvancedSecurityOverridesUntrustedAppsPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdvancedSecurityOverridesUntrustedAppsPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALLOW_INSTALL_DEVICE_WIDE" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallDeviceWide
                }
                "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::AllowInstallInPersonalProfileOnly
                }
                "DISALLOW_INSTALL" => AdvancedSecurityOverridesUntrustedAppsPolicy::DisallowInstall,
                "UNTRUSTED_APPS_POLICY_UNSPECIFIED" => {
                    AdvancedSecurityOverridesUntrustedAppsPolicy::UntrustedAppsPolicyUnspecified
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
    impl ::google_field_selector::FieldSelector for AdvancedSecurityOverridesUntrustedAppsPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvancedSecurityOverridesUntrustedAppsPolicy {
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
    pub struct AlwaysOnVpnPackage {
        #[doc = "Disallows networking when the VPN is not connected."]
        #[serde(
            rename = "lockdownEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lockdown_enabled: ::std::option::Option<bool>,
        #[doc = "The package name of the VPN app."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AlwaysOnVpnPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AlwaysOnVpnPackage {
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
    pub struct ApiLevelCondition {
        #[doc = "The minimum desired Android Framework API level. If the device doesn't meet the minimum requirement, this condition is satisfied. Must be greater than zero."]
        #[serde(
            rename = "minApiLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_api_level: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ApiLevelCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiLevelCondition {
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
    pub struct AppTrackInfo {
        #[doc = "The track name associated with the trackId, set in the Play Console. The name is modifiable from Play Console."]
        #[serde(
            rename = "trackAlias",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub track_alias: ::std::option::Option<String>,
        #[doc = "The unmodifiable unique track identifier, taken from the releaseTrackId in the URL of the Play Console page that displays the app’s track information."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub track_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AppTrackInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppTrackInfo {
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
    pub struct AppVersion {
        #[doc = "If the value is True, it indicates that this version is a production track."]
        #[serde(
            rename = "production",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub production: ::std::option::Option<bool>,
        #[doc = "Track identifiers that the app version is published in. This does not include the production track (see production instead)."]
        #[serde(
            rename = "trackIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub track_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Unique increasing identifier for the app version."]
        #[serde(
            rename = "versionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_code: ::std::option::Option<i32>,
        #[doc = "The string used in the Play store by the app developer to identify the version. The string is not necessarily unique or localized (for example, the string could be \"1.4\")."]
        #[serde(
            rename = "versionString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_string: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AppVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Application {
        #[doc = "Whether this app is free, free with in-app purchases, or paid. If the pricing is unspecified, this means the app is not generally available anymore (even though it might still be available to people who own it)."]
        #[serde(
            rename = "appPricing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_pricing: ::std::option::Option<crate::schemas::ApplicationAppPricing>,
        #[doc = "Application tracks visible to the enterprise."]
        #[serde(
            rename = "appTracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_tracks: ::std::option::Option<Vec<crate::schemas::AppTrackInfo>>,
        #[doc = "Versions currently available for this app."]
        #[serde(
            rename = "appVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_versions: ::std::option::Option<Vec<crate::schemas::AppVersion>>,
        #[doc = "The name of the author of the apps (for example, the app developer)."]
        #[serde(
            rename = "author",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub author: ::std::option::Option<String>,
        #[doc = "The countries which this app is available in as per ISO 3166-1 alpha-2."]
        #[serde(
            rename = "availableCountries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_countries: ::std::option::Option<Vec<String>>,
        #[doc = "The app category (e.g. RACING, SOCIAL, etc.)"]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "The content rating for this app."]
        #[serde(
            rename = "contentRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_rating: ::std::option::Option<crate::schemas::ApplicationContentRating>,
        #[doc = "The localized promotional description, if available."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "How and to whom the package is made available."]
        #[serde(
            rename = "distributionChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution_channel:
            ::std::option::Option<crate::schemas::ApplicationDistributionChannel>,
        #[doc = "Noteworthy features (if any) of this app."]
        #[serde(
            rename = "features",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub features: ::std::option::Option<Vec<crate::schemas::ApplicationFeaturesItems>>,
        #[doc = "Full app description, if available."]
        #[serde(
            rename = "fullDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_description: ::std::option::Option<String>,
        #[doc = "A link to an image that can be used as an icon for the app. This image is suitable for use up to a pixel size of 512 x 512."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The set of managed properties available to be pre-configured for the app."]
        #[serde(
            rename = "managedProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_properties: ::std::option::Option<Vec<crate::schemas::ManagedProperty>>,
        #[doc = "The minimum Android SDK necessary to run the app."]
        #[serde(
            rename = "minAndroidSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_android_sdk_version: ::std::option::Option<i32>,
        #[doc = "The name of the app in the form enterprises/{enterprise}/applications/{package_name}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The permissions required by the app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<crate::schemas::ApplicationPermission>>,
        #[doc = "A link to the (consumer) Google Play details page for the app."]
        #[serde(
            rename = "playStoreUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub play_store_url: ::std::option::Option<String>,
        #[doc = "A localised description of the recent changes made to the app."]
        #[serde(
            rename = "recentChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recent_changes: ::std::option::Option<String>,
        #[doc = "A list of screenshot links representing the app."]
        #[serde(
            rename = "screenshotUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screenshot_urls: ::std::option::Option<Vec<String>>,
        #[doc = "A link to a smaller image that can be used as an icon for the app. This image is suitable for use up to a pixel size of 128 x 128."]
        #[serde(
            rename = "smallIconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub small_icon_url: ::std::option::Option<String>,
        #[doc = "The title of the app. Localized."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Output only. The approximate time (within 7 days) the app was last published."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Application {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Application {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationAppPricing {
        #[doc = "Unknown pricing, used to denote an approved app that is not generally available."]
        AppPricingUnspecified,
        #[doc = "The app is free."]
        Free,
        #[doc = "The app is free, but offers in-app purchases."]
        FreeWithInAppPurchase,
        #[doc = "The app is paid."]
        Paid,
    }
    impl ApplicationAppPricing {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationAppPricing::AppPricingUnspecified => "APP_PRICING_UNSPECIFIED",
                ApplicationAppPricing::Free => "FREE",
                ApplicationAppPricing::FreeWithInAppPurchase => "FREE_WITH_IN_APP_PURCHASE",
                ApplicationAppPricing::Paid => "PAID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationAppPricing {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationAppPricing {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationAppPricing, ()> {
            Ok(match s {
                "APP_PRICING_UNSPECIFIED" => ApplicationAppPricing::AppPricingUnspecified,
                "FREE" => ApplicationAppPricing::Free,
                "FREE_WITH_IN_APP_PURCHASE" => ApplicationAppPricing::FreeWithInAppPurchase,
                "PAID" => ApplicationAppPricing::Paid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationAppPricing {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationAppPricing {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationAppPricing {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_PRICING_UNSPECIFIED" => ApplicationAppPricing::AppPricingUnspecified,
                "FREE" => ApplicationAppPricing::Free,
                "FREE_WITH_IN_APP_PURCHASE" => ApplicationAppPricing::FreeWithInAppPurchase,
                "PAID" => ApplicationAppPricing::Paid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationAppPricing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationAppPricing {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationContentRating {
        #[doc = "Unspecified."]
        ContentRatingUnspecified,
        #[doc = "Content suitable for ages 18 and above only."]
        EighteenYears,
        #[doc = "Content suitable for ages 7 and above only."]
        SevenYears,
        #[doc = "Content suitable for ages 16 and above only."]
        SixteenYears,
        #[doc = "Content suitable for ages 3 and above only."]
        ThreeYears,
        #[doc = "Content suitable for ages 12 and above only."]
        TwelveYears,
    }
    impl ApplicationContentRating {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationContentRating::ContentRatingUnspecified => "CONTENT_RATING_UNSPECIFIED",
                ApplicationContentRating::EighteenYears => "EIGHTEEN_YEARS",
                ApplicationContentRating::SevenYears => "SEVEN_YEARS",
                ApplicationContentRating::SixteenYears => "SIXTEEN_YEARS",
                ApplicationContentRating::ThreeYears => "THREE_YEARS",
                ApplicationContentRating::TwelveYears => "TWELVE_YEARS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationContentRating {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationContentRating {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationContentRating, ()> {
            Ok(match s {
                "CONTENT_RATING_UNSPECIFIED" => ApplicationContentRating::ContentRatingUnspecified,
                "EIGHTEEN_YEARS" => ApplicationContentRating::EighteenYears,
                "SEVEN_YEARS" => ApplicationContentRating::SevenYears,
                "SIXTEEN_YEARS" => ApplicationContentRating::SixteenYears,
                "THREE_YEARS" => ApplicationContentRating::ThreeYears,
                "TWELVE_YEARS" => ApplicationContentRating::TwelveYears,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationContentRating {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationContentRating {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationContentRating {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_RATING_UNSPECIFIED" => ApplicationContentRating::ContentRatingUnspecified,
                "EIGHTEEN_YEARS" => ApplicationContentRating::EighteenYears,
                "SEVEN_YEARS" => ApplicationContentRating::SevenYears,
                "SIXTEEN_YEARS" => ApplicationContentRating::SixteenYears,
                "THREE_YEARS" => ApplicationContentRating::ThreeYears,
                "TWELVE_YEARS" => ApplicationContentRating::TwelveYears,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationContentRating {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationContentRating {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationDistributionChannel {
        #[doc = "Unspecified."]
        DistributionChannelUnspecified,
        #[doc = "Package is a private app (restricted to an enterprise) but hosted by Google."]
        PrivateGoogleHosted,
        #[doc = "Private app (restricted to an enterprise) and is privately hosted."]
        PrivateSelfHosted,
        #[doc = "Package is available through the Play store and not restricted to a specific enterprise."]
        PublicGoogleHosted,
    }
    impl ApplicationDistributionChannel {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationDistributionChannel::DistributionChannelUnspecified => {
                    "DISTRIBUTION_CHANNEL_UNSPECIFIED"
                }
                ApplicationDistributionChannel::PrivateGoogleHosted => "PRIVATE_GOOGLE_HOSTED",
                ApplicationDistributionChannel::PrivateSelfHosted => "PRIVATE_SELF_HOSTED",
                ApplicationDistributionChannel::PublicGoogleHosted => "PUBLIC_GOOGLE_HOSTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationDistributionChannel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationDistributionChannel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationDistributionChannel, ()> {
            Ok(match s {
                "DISTRIBUTION_CHANNEL_UNSPECIFIED" => {
                    ApplicationDistributionChannel::DistributionChannelUnspecified
                }
                "PRIVATE_GOOGLE_HOSTED" => ApplicationDistributionChannel::PrivateGoogleHosted,
                "PRIVATE_SELF_HOSTED" => ApplicationDistributionChannel::PrivateSelfHosted,
                "PUBLIC_GOOGLE_HOSTED" => ApplicationDistributionChannel::PublicGoogleHosted,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationDistributionChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationDistributionChannel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationDistributionChannel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISTRIBUTION_CHANNEL_UNSPECIFIED" => {
                    ApplicationDistributionChannel::DistributionChannelUnspecified
                }
                "PRIVATE_GOOGLE_HOSTED" => ApplicationDistributionChannel::PrivateGoogleHosted,
                "PRIVATE_SELF_HOSTED" => ApplicationDistributionChannel::PrivateSelfHosted,
                "PUBLIC_GOOGLE_HOSTED" => ApplicationDistributionChannel::PublicGoogleHosted,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationDistributionChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationDistributionChannel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationFeaturesItems {
        #[doc = "Unspecified."]
        AppFeatureUnspecified,
        #[doc = "The app is a VPN."]
        VpnApp,
    }
    impl ApplicationFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationFeaturesItems::AppFeatureUnspecified => "APP_FEATURE_UNSPECIFIED",
                ApplicationFeaturesItems::VpnApp => "VPN_APP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationFeaturesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationFeaturesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationFeaturesItems, ()> {
            Ok(match s {
                "APP_FEATURE_UNSPECIFIED" => ApplicationFeaturesItems::AppFeatureUnspecified,
                "VPN_APP" => ApplicationFeaturesItems::VpnApp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationFeaturesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationFeaturesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_FEATURE_UNSPECIFIED" => ApplicationFeaturesItems::AppFeatureUnspecified,
                "VPN_APP" => ApplicationFeaturesItems::VpnApp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationFeaturesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationFeaturesItems {
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
    pub struct ApplicationEvent {
        #[doc = "The creation time of the event."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "App event type."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type: ::std::option::Option<crate::schemas::ApplicationEventEventType>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationEventEventType {
        #[doc = "This value is disallowed."]
        ApplicationEventTypeUnspecified,
        #[doc = "The app was changed, for example, a component was enabled or disabled."]
        Changed,
        #[doc = "The app data was cleared."]
        DataCleared,
        #[doc = "The app was installed."]
        Installed,
        #[doc = "The app was pinned to the foreground."]
        Pinned,
        #[doc = "The app was removed."]
        Removed,
        #[doc = "A new version of the app has been installed, replacing the old version."]
        Replaced,
        #[doc = "The app was restarted."]
        Restarted,
        #[doc = "The app was unpinned."]
        Unpinned,
    }
    impl ApplicationEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationEventEventType::ApplicationEventTypeUnspecified => {
                    "APPLICATION_EVENT_TYPE_UNSPECIFIED"
                }
                ApplicationEventEventType::Changed => "CHANGED",
                ApplicationEventEventType::DataCleared => "DATA_CLEARED",
                ApplicationEventEventType::Installed => "INSTALLED",
                ApplicationEventEventType::Pinned => "PINNED",
                ApplicationEventEventType::Removed => "REMOVED",
                ApplicationEventEventType::Replaced => "REPLACED",
                ApplicationEventEventType::Restarted => "RESTARTED",
                ApplicationEventEventType::Unpinned => "UNPINNED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationEventEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationEventEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationEventEventType, ()> {
            Ok(match s {
                "APPLICATION_EVENT_TYPE_UNSPECIFIED" => {
                    ApplicationEventEventType::ApplicationEventTypeUnspecified
                }
                "CHANGED" => ApplicationEventEventType::Changed,
                "DATA_CLEARED" => ApplicationEventEventType::DataCleared,
                "INSTALLED" => ApplicationEventEventType::Installed,
                "PINNED" => ApplicationEventEventType::Pinned,
                "REMOVED" => ApplicationEventEventType::Removed,
                "REPLACED" => ApplicationEventEventType::Replaced,
                "RESTARTED" => ApplicationEventEventType::Restarted,
                "UNPINNED" => ApplicationEventEventType::Unpinned,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_EVENT_TYPE_UNSPECIFIED" => {
                    ApplicationEventEventType::ApplicationEventTypeUnspecified
                }
                "CHANGED" => ApplicationEventEventType::Changed,
                "DATA_CLEARED" => ApplicationEventEventType::DataCleared,
                "INSTALLED" => ApplicationEventEventType::Installed,
                "PINNED" => ApplicationEventEventType::Pinned,
                "REMOVED" => ApplicationEventEventType::Removed,
                "REPLACED" => ApplicationEventEventType::Replaced,
                "RESTARTED" => ApplicationEventEventType::Restarted,
                "UNPINNED" => ApplicationEventEventType::Unpinned,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationEventEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationEventEventType {
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
    pub struct ApplicationPermission {
        #[doc = "A longer description of the permission, providing more detail on what it affects. Localized."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The name of the permission. Localized."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "An opaque string uniquely identifying the permission. Not localized."]
        #[serde(
            rename = "permissionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationPermission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPermission {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ApplicationPolicy {
        #[doc = "List of the app’s track IDs that a device belonging to the enterprise can access. If the list contains multiple track IDs, devices receive the latest version among all accessible tracks. If the list contains no track IDs, devices only have access to the app’s production track. More details about each track are available in AppTrackInfo."]
        #[serde(
            rename = "accessibleTrackIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accessible_track_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies whether the app is allowed networking when the VPN is not connected and alwaysOnVpnPackage.lockdownEnabled is enabled. If set to VPN_LOCKDOWN_ENFORCED, the app is not allowed networking, and if set to VPN_LOCKDOWN_EXEMPTION, the app is allowed networking. Only supported on devices running Android 10 and above. If this is not supported by the device, the device will contain a NonComplianceDetail with non_compliance_reason set to API_LEVEL and a fieldPath. If this is not applicable to the app, the device will contain a NonComplianceDetail with non_compliance_reason set to UNSUPPORTED and a fieldPath. The fieldPath is set to applications[i].alwaysOnVpnLockdownExemption, where i is the index of the package in the applications policy."]
        #[serde(
            rename = "alwaysOnVpnLockdownExemption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub always_on_vpn_lockdown_exemption:
            ::std::option::Option<crate::schemas::ApplicationPolicyAlwaysOnVpnLockdownExemption>,
        #[doc = "Controls the auto-update mode for the app."]
        #[serde(
            rename = "autoUpdateMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_update_mode:
            ::std::option::Option<crate::schemas::ApplicationPolicyAutoUpdateMode>,
        #[doc = "Controls whether the app can communicate with itself across a device’s work and personal profiles, subject to user consent."]
        #[serde(
            rename = "connectedWorkAndPersonalApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connected_work_and_personal_app:
            ::std::option::Option<crate::schemas::ApplicationPolicyConnectedWorkAndPersonalApp>,
        #[doc = "The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps."]
        #[serde(
            rename = "defaultPermissionPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_permission_policy:
            ::std::option::Option<crate::schemas::ApplicationPolicyDefaultPermissionPolicy>,
        #[doc = "The scopes delegated to the app from Android Device Policy."]
        #[serde(
            rename = "delegatedScopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delegated_scopes:
            ::std::option::Option<Vec<crate::schemas::ApplicationPolicyDelegatedScopesItems>>,
        #[doc = "Whether the app is disabled. When disabled, the app data is still preserved."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Configuration to enable this app as an extension app, with the capability of interacting with Android Device Policy offline.This field can be set for at most one app."]
        #[serde(
            rename = "extensionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extension_config: ::std::option::Option<crate::schemas::ExtensionConfig>,
        #[doc = "The type of installation to perform."]
        #[serde(
            rename = "installType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_type: ::std::option::Option<crate::schemas::ApplicationPolicyInstallType>,
        #[doc = "Whether the app is allowed to lock itself in full-screen mode. DEPRECATED. Use InstallType KIOSK or kioskCustomLauncherEnabled to to configure a dedicated device."]
        #[serde(
            rename = "lockTaskAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lock_task_allowed: ::std::option::Option<bool>,
        #[doc = "Managed configuration applied to the app. The format for the configuration is dictated by the ManagedProperty values supported by the app. Each field name in the managed configuration must match the key field of the ManagedProperty. The field value must be compatible with the type of the ManagedProperty: *type* *JSON value* BOOL true or false STRING string INTEGER number CHOICE string MULTISELECT array of strings HIDDEN string BUNDLE_ARRAY array of objects "]
        #[serde(
            rename = "managedConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The managed configurations template for the app, saved from the managed configurations iframe. This field is ignored if managed_configuration is set."]
        #[serde(
            rename = "managedConfigurationTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_configuration_template:
            ::std::option::Option<crate::schemas::ManagedConfigurationTemplate>,
        #[doc = "The minimum version of the app that runs on the device. If set, the device attempts to update the app to at least this version code. If the app is not up-to-date, the device will contain a NonComplianceDetail with non_compliance_reason set to APP_NOT_UPDATED. The app must already be published to Google Play with a version code greater than or equal to this value. At most 20 apps may specify a minimum version code per policy."]
        #[serde(
            rename = "minimumVersionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_version_code: ::std::option::Option<i32>,
        #[doc = "The package name of the app. For example, com.google.android.youtube for the YouTube app."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Explicit permission grants or denials for the app. These values override the default_permission_policy and permission_grants which apply to all apps."]
        #[serde(
            rename = "permissionGrants",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_grants: ::std::option::Option<Vec<crate::schemas::PermissionGrant>>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyAlwaysOnVpnLockdownExemption {
        #[doc = "Unspecified. Defaults to VPN_LOCKDOWN_ENFORCED."]
        AlwaysOnVpnLockdownExemptionUnspecified,
        #[doc = "The app respects the always-on VPN lockdown setting."]
        VpnLockdownEnforced,
        #[doc = "The app is exempt from the always-on VPN lockdown setting."]
        VpnLockdownExemption,
    }
    impl ApplicationPolicyAlwaysOnVpnLockdownExemption {
        pub fn as_str(self) -> &'static str {
            match self { ApplicationPolicyAlwaysOnVpnLockdownExemption :: AlwaysOnVpnLockdownExemptionUnspecified => "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED" , ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownEnforced => "VPN_LOCKDOWN_ENFORCED" , ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownExemption => "VPN_LOCKDOWN_EXEMPTION" , }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ApplicationPolicyAlwaysOnVpnLockdownExemption, ()> {
            Ok (match s { "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: AlwaysOnVpnLockdownExemptionUnspecified , "VPN_LOCKDOWN_ENFORCED" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownEnforced , "VPN_LOCKDOWN_EXEMPTION" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownExemption , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: AlwaysOnVpnLockdownExemptionUnspecified , "VPN_LOCKDOWN_ENFORCED" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownEnforced , "VPN_LOCKDOWN_EXEMPTION" => ApplicationPolicyAlwaysOnVpnLockdownExemption :: VpnLockdownExemption , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyAlwaysOnVpnLockdownExemption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyAutoUpdateMode {
        #[doc = "The app is automatically updated with low priority to minimize the impact on the user.The app is updated when all of the following constraints are met: The device is not actively used. The device is connected to an unmetered network. The device is charging.The device is notified about a new update within 24 hours after it is published by the developer, after which the app is updated the next time the constraints above are met."]
        AutoUpdateDefault,
        #[doc = "The app is updated as soon as possible. No constraints are applied.The device is notified immediately about a new update after it becomes available."]
        AutoUpdateHighPriority,
        #[doc = "Unspecified. Defaults to AUTO_UPDATE_DEFAULT."]
        AutoUpdateModeUnspecified,
        #[doc = "The app is not automatically updated for a maximum of 90 days after the app becomes out of date.90 days after the app becomes out of date, the latest available version is installed automatically with low priority (see AUTO_UPDATE_DEFAULT). After the app is updated it is not automatically updated again until 90 days after it becomes out of date again.The user can still manually update the app from the Play Store at any time."]
        AutoUpdatePostponed,
    }
    impl ApplicationPolicyAutoUpdateMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyAutoUpdateMode::AutoUpdateDefault => "AUTO_UPDATE_DEFAULT",
                ApplicationPolicyAutoUpdateMode::AutoUpdateHighPriority => {
                    "AUTO_UPDATE_HIGH_PRIORITY"
                }
                ApplicationPolicyAutoUpdateMode::AutoUpdateModeUnspecified => {
                    "AUTO_UPDATE_MODE_UNSPECIFIED"
                }
                ApplicationPolicyAutoUpdateMode::AutoUpdatePostponed => "AUTO_UPDATE_POSTPONED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyAutoUpdateMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyAutoUpdateMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationPolicyAutoUpdateMode, ()> {
            Ok(match s {
                "AUTO_UPDATE_DEFAULT" => ApplicationPolicyAutoUpdateMode::AutoUpdateDefault,
                "AUTO_UPDATE_HIGH_PRIORITY" => {
                    ApplicationPolicyAutoUpdateMode::AutoUpdateHighPriority
                }
                "AUTO_UPDATE_MODE_UNSPECIFIED" => {
                    ApplicationPolicyAutoUpdateMode::AutoUpdateModeUnspecified
                }
                "AUTO_UPDATE_POSTPONED" => ApplicationPolicyAutoUpdateMode::AutoUpdatePostponed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyAutoUpdateMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyAutoUpdateMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyAutoUpdateMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO_UPDATE_DEFAULT" => ApplicationPolicyAutoUpdateMode::AutoUpdateDefault,
                "AUTO_UPDATE_HIGH_PRIORITY" => {
                    ApplicationPolicyAutoUpdateMode::AutoUpdateHighPriority
                }
                "AUTO_UPDATE_MODE_UNSPECIFIED" => {
                    ApplicationPolicyAutoUpdateMode::AutoUpdateModeUnspecified
                }
                "AUTO_UPDATE_POSTPONED" => ApplicationPolicyAutoUpdateMode::AutoUpdatePostponed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyAutoUpdateMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyAutoUpdateMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyConnectedWorkAndPersonalApp {
        #[doc = "Allows the app to communicate across profiles after receiving user consent."]
        ConnectedWorkAndPersonalAppAllowed,
        #[doc = "Default. Prevents the app from communicating cross-profile."]
        ConnectedWorkAndPersonalAppDisallowed,
        #[doc = "Unspecified. Defaults to CONNECTED_WORK_AND_PERSONAL_APPS_DISALLOWED."]
        ConnectedWorkAndPersonalAppUnspecified,
    }
    impl ApplicationPolicyConnectedWorkAndPersonalApp {
        pub fn as_str(self) -> &'static str {
            match self { ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppAllowed => "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED" , ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppDisallowed => "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED" , ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppUnspecified => "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyConnectedWorkAndPersonalApp {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ApplicationPolicyConnectedWorkAndPersonalApp, ()> {
            Ok (match s { "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppAllowed , "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppDisallowed , "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppAllowed , "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppDisallowed , "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED" => ApplicationPolicyConnectedWorkAndPersonalApp :: ConnectedWorkAndPersonalAppUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyConnectedWorkAndPersonalApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyDefaultPermissionPolicy {
        #[doc = "Automatically deny a permission."]
        Deny,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
    }
    impl ApplicationPolicyDefaultPermissionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyDefaultPermissionPolicy::Deny => "DENY",
                ApplicationPolicyDefaultPermissionPolicy::Grant => "GRANT",
                ApplicationPolicyDefaultPermissionPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                ApplicationPolicyDefaultPermissionPolicy::Prompt => "PROMPT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyDefaultPermissionPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyDefaultPermissionPolicy {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ApplicationPolicyDefaultPermissionPolicy, ()> {
            Ok(match s {
                "DENY" => ApplicationPolicyDefaultPermissionPolicy::Deny,
                "GRANT" => ApplicationPolicyDefaultPermissionPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    ApplicationPolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => ApplicationPolicyDefaultPermissionPolicy::Prompt,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyDefaultPermissionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyDefaultPermissionPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyDefaultPermissionPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DENY" => ApplicationPolicyDefaultPermissionPolicy::Deny,
                "GRANT" => ApplicationPolicyDefaultPermissionPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    ApplicationPolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => ApplicationPolicyDefaultPermissionPolicy::Prompt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyDefaultPermissionPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyDefaultPermissionPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyDelegatedScopesItems {
        #[doc = "Grants access to blocking uninstallation."]
        BlockUninstall,
        #[doc = "Grants access to certificate installation and management."]
        CertInstall,
        #[doc = "No delegation scope specified."]
        DelegatedScopeUnspecified,
        #[doc = "Grants access for enabling system apps."]
        EnableSystemApp,
        #[doc = "Grants access to managed configurations management."]
        ManagedConfigurations,
        #[doc = "Grants access to package access state."]
        PackageAccess,
        #[doc = "Grants access to permission policy and permission grant state."]
        PermissionGrant,
    }
    impl ApplicationPolicyDelegatedScopesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyDelegatedScopesItems::BlockUninstall => "BLOCK_UNINSTALL",
                ApplicationPolicyDelegatedScopesItems::CertInstall => "CERT_INSTALL",
                ApplicationPolicyDelegatedScopesItems::DelegatedScopeUnspecified => {
                    "DELEGATED_SCOPE_UNSPECIFIED"
                }
                ApplicationPolicyDelegatedScopesItems::EnableSystemApp => "ENABLE_SYSTEM_APP",
                ApplicationPolicyDelegatedScopesItems::ManagedConfigurations => {
                    "MANAGED_CONFIGURATIONS"
                }
                ApplicationPolicyDelegatedScopesItems::PackageAccess => "PACKAGE_ACCESS",
                ApplicationPolicyDelegatedScopesItems::PermissionGrant => "PERMISSION_GRANT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyDelegatedScopesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyDelegatedScopesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationPolicyDelegatedScopesItems, ()> {
            Ok(match s {
                "BLOCK_UNINSTALL" => ApplicationPolicyDelegatedScopesItems::BlockUninstall,
                "CERT_INSTALL" => ApplicationPolicyDelegatedScopesItems::CertInstall,
                "DELEGATED_SCOPE_UNSPECIFIED" => {
                    ApplicationPolicyDelegatedScopesItems::DelegatedScopeUnspecified
                }
                "ENABLE_SYSTEM_APP" => ApplicationPolicyDelegatedScopesItems::EnableSystemApp,
                "MANAGED_CONFIGURATIONS" => {
                    ApplicationPolicyDelegatedScopesItems::ManagedConfigurations
                }
                "PACKAGE_ACCESS" => ApplicationPolicyDelegatedScopesItems::PackageAccess,
                "PERMISSION_GRANT" => ApplicationPolicyDelegatedScopesItems::PermissionGrant,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyDelegatedScopesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyDelegatedScopesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyDelegatedScopesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLOCK_UNINSTALL" => ApplicationPolicyDelegatedScopesItems::BlockUninstall,
                "CERT_INSTALL" => ApplicationPolicyDelegatedScopesItems::CertInstall,
                "DELEGATED_SCOPE_UNSPECIFIED" => {
                    ApplicationPolicyDelegatedScopesItems::DelegatedScopeUnspecified
                }
                "ENABLE_SYSTEM_APP" => ApplicationPolicyDelegatedScopesItems::EnableSystemApp,
                "MANAGED_CONFIGURATIONS" => {
                    ApplicationPolicyDelegatedScopesItems::ManagedConfigurations
                }
                "PACKAGE_ACCESS" => ApplicationPolicyDelegatedScopesItems::PackageAccess,
                "PERMISSION_GRANT" => ApplicationPolicyDelegatedScopesItems::PermissionGrant,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyDelegatedScopesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyDelegatedScopesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyInstallType {
        #[doc = "The app is available to install."]
        Available,
        #[doc = "The app is blocked and can't be installed. If the app was installed under a previous policy, it will be uninstalled."]
        Blocked,
        #[doc = "The app is automatically installed and can't be removed by the user."]
        ForceInstalled,
        #[doc = "Unspecified. Defaults to AVAILABLE."]
        InstallTypeUnspecified,
        #[doc = "The app is automatically installed in kiosk mode: it's set as the preferred home intent and whitelisted for lock task mode. Device setup won't complete until the app is installed. After installation, users won't be able to remove the app. You can only set this installType for one app per policy. When this is present in the policy, status bar will be automatically disabled."]
        Kiosk,
        #[doc = "The app is automatically installed and can be removed by the user."]
        Preinstalled,
        #[doc = "The app is automatically installed and can't be removed by the user and will prevent setup from completion until installation is complete."]
        RequiredForSetup,
    }
    impl ApplicationPolicyInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyInstallType::Available => "AVAILABLE",
                ApplicationPolicyInstallType::Blocked => "BLOCKED",
                ApplicationPolicyInstallType::ForceInstalled => "FORCE_INSTALLED",
                ApplicationPolicyInstallType::InstallTypeUnspecified => "INSTALL_TYPE_UNSPECIFIED",
                ApplicationPolicyInstallType::Kiosk => "KIOSK",
                ApplicationPolicyInstallType::Preinstalled => "PREINSTALLED",
                ApplicationPolicyInstallType::RequiredForSetup => "REQUIRED_FOR_SETUP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationPolicyInstallType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationPolicyInstallType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationPolicyInstallType, ()> {
            Ok(match s {
                "AVAILABLE" => ApplicationPolicyInstallType::Available,
                "BLOCKED" => ApplicationPolicyInstallType::Blocked,
                "FORCE_INSTALLED" => ApplicationPolicyInstallType::ForceInstalled,
                "INSTALL_TYPE_UNSPECIFIED" => ApplicationPolicyInstallType::InstallTypeUnspecified,
                "KIOSK" => ApplicationPolicyInstallType::Kiosk,
                "PREINSTALLED" => ApplicationPolicyInstallType::Preinstalled,
                "REQUIRED_FOR_SETUP" => ApplicationPolicyInstallType::RequiredForSetup,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyInstallType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyInstallType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVAILABLE" => ApplicationPolicyInstallType::Available,
                "BLOCKED" => ApplicationPolicyInstallType::Blocked,
                "FORCE_INSTALLED" => ApplicationPolicyInstallType::ForceInstalled,
                "INSTALL_TYPE_UNSPECIFIED" => ApplicationPolicyInstallType::InstallTypeUnspecified,
                "KIOSK" => ApplicationPolicyInstallType::Kiosk,
                "PREINSTALLED" => ApplicationPolicyInstallType::Preinstalled,
                "REQUIRED_FOR_SETUP" => ApplicationPolicyInstallType::RequiredForSetup,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationPolicyInstallType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationPolicyInstallType {
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
    pub struct ApplicationReport {
        #[doc = "The source of the package."]
        #[serde(
            rename = "applicationSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_source:
            ::std::option::Option<crate::schemas::ApplicationReportApplicationSource>,
        #[doc = "The display name of the app."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "List of app events. The most recent 20 events are stored in the list."]
        #[serde(
            rename = "events",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub events: ::std::option::Option<Vec<crate::schemas::ApplicationEvent>>,
        #[doc = "The package name of the app that installed this app."]
        #[serde(
            rename = "installerPackageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installer_package_name: ::std::option::Option<String>,
        #[doc = "List of keyed app states reported by the app."]
        #[serde(
            rename = "keyedAppStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyed_app_states: ::std::option::Option<Vec<crate::schemas::KeyedAppState>>,
        #[doc = "Package name of the app."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The SHA-256 hash of the app's APK file, which can be used to verify the app hasn't been modified. Each byte of the hash value is represented as a two-digit hexadecimal number."]
        #[serde(
            rename = "packageSha256Hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_sha_256_hash: ::std::option::Option<String>,
        #[doc = "The SHA-1 hash of each android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the app package. Each byte of each hash value is represented as a two-digit hexadecimal number."]
        #[serde(
            rename = "signingKeyCertFingerprints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signing_key_cert_fingerprints: ::std::option::Option<Vec<String>>,
        #[doc = "Application state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ApplicationReportState>,
        #[doc = "The app version code, which can be used to determine whether one version is more recent than another."]
        #[serde(
            rename = "versionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_code: ::std::option::Option<i32>,
        #[doc = "The app version as displayed to the user."]
        #[serde(
            rename = "versionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationReportApplicationSource {
        #[doc = "The app was sideloaded from an unspecified source."]
        ApplicationSourceUnspecified,
        #[doc = "The app was installed from the Google Play Store."]
        InstalledFromPlayStore,
        #[doc = "This is a system app from the device's factory image."]
        SystemAppFactoryVersion,
        #[doc = "This is an updated system app."]
        SystemAppUpdatedVersion,
    }
    impl ApplicationReportApplicationSource {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReportApplicationSource::ApplicationSourceUnspecified => {
                    "APPLICATION_SOURCE_UNSPECIFIED"
                }
                ApplicationReportApplicationSource::InstalledFromPlayStore => {
                    "INSTALLED_FROM_PLAY_STORE"
                }
                ApplicationReportApplicationSource::SystemAppFactoryVersion => {
                    "SYSTEM_APP_FACTORY_VERSION"
                }
                ApplicationReportApplicationSource::SystemAppUpdatedVersion => {
                    "SYSTEM_APP_UPDATED_VERSION"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationReportApplicationSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationReportApplicationSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationReportApplicationSource, ()> {
            Ok(match s {
                "APPLICATION_SOURCE_UNSPECIFIED" => {
                    ApplicationReportApplicationSource::ApplicationSourceUnspecified
                }
                "INSTALLED_FROM_PLAY_STORE" => {
                    ApplicationReportApplicationSource::InstalledFromPlayStore
                }
                "SYSTEM_APP_FACTORY_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppFactoryVersion
                }
                "SYSTEM_APP_UPDATED_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppUpdatedVersion
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationReportApplicationSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReportApplicationSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReportApplicationSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_SOURCE_UNSPECIFIED" => {
                    ApplicationReportApplicationSource::ApplicationSourceUnspecified
                }
                "INSTALLED_FROM_PLAY_STORE" => {
                    ApplicationReportApplicationSource::InstalledFromPlayStore
                }
                "SYSTEM_APP_FACTORY_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppFactoryVersion
                }
                "SYSTEM_APP_UPDATED_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppUpdatedVersion
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
    impl ::google_field_selector::FieldSelector for ApplicationReportApplicationSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReportApplicationSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationReportState {
        #[doc = "App state is unspecified"]
        ApplicationStateUnspecified,
        #[doc = "App is installed on the device"]
        Installed,
        #[doc = "App was removed from the device"]
        Removed,
    }
    impl ApplicationReportState {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReportState::ApplicationStateUnspecified => {
                    "APPLICATION_STATE_UNSPECIFIED"
                }
                ApplicationReportState::Installed => "INSTALLED",
                ApplicationReportState::Removed => "REMOVED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationReportState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationReportState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationReportState, ()> {
            Ok(match s {
                "APPLICATION_STATE_UNSPECIFIED" => {
                    ApplicationReportState::ApplicationStateUnspecified
                }
                "INSTALLED" => ApplicationReportState::Installed,
                "REMOVED" => ApplicationReportState::Removed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationReportState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReportState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReportState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_STATE_UNSPECIFIED" => {
                    ApplicationReportState::ApplicationStateUnspecified
                }
                "INSTALLED" => ApplicationReportState::Installed,
                "REMOVED" => ApplicationReportState::Removed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationReportState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReportState {
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
    pub struct ApplicationReportingSettings {
        #[doc = "Whether removed apps are included in application reports."]
        #[serde(
            rename = "includeRemovedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_removed_apps: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationReportingSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReportingSettings {
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
    pub struct BlockAction {
        #[doc = "Number of days the policy is non-compliant before the device or work profile is blocked. To block access immediately, set to 0. blockAfterDays must be less than wipeAfterDays."]
        #[serde(
            rename = "blockAfterDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_after_days: ::std::option::Option<i32>,
        #[doc = "Specifies the scope of this BlockAction. Only applicable to devices that are company-owned."]
        #[serde(
            rename = "blockScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_scope: ::std::option::Option<crate::schemas::BlockActionBlockScope>,
    }
    impl ::google_field_selector::FieldSelector for BlockAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BlockAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BlockActionBlockScope {
        #[doc = "Block action is applied to the entire device, including apps in the personal profile."]
        BlockScopeDevice,
        #[doc = "Unspecified. Defaults to BLOCK_SCOPE_WORK_PROFILE."]
        BlockScopeUnspecified,
        #[doc = "Block action is only applied to apps in the work profile. Apps in the personal profile are unaffected."]
        BlockScopeWorkProfile,
    }
    impl BlockActionBlockScope {
        pub fn as_str(self) -> &'static str {
            match self {
                BlockActionBlockScope::BlockScopeDevice => "BLOCK_SCOPE_DEVICE",
                BlockActionBlockScope::BlockScopeUnspecified => "BLOCK_SCOPE_UNSPECIFIED",
                BlockActionBlockScope::BlockScopeWorkProfile => "BLOCK_SCOPE_WORK_PROFILE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BlockActionBlockScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BlockActionBlockScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BlockActionBlockScope, ()> {
            Ok(match s {
                "BLOCK_SCOPE_DEVICE" => BlockActionBlockScope::BlockScopeDevice,
                "BLOCK_SCOPE_UNSPECIFIED" => BlockActionBlockScope::BlockScopeUnspecified,
                "BLOCK_SCOPE_WORK_PROFILE" => BlockActionBlockScope::BlockScopeWorkProfile,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BlockActionBlockScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BlockActionBlockScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BlockActionBlockScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLOCK_SCOPE_DEVICE" => BlockActionBlockScope::BlockScopeDevice,
                "BLOCK_SCOPE_UNSPECIFIED" => BlockActionBlockScope::BlockScopeUnspecified,
                "BLOCK_SCOPE_WORK_PROFILE" => BlockActionBlockScope::BlockScopeWorkProfile,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BlockActionBlockScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BlockActionBlockScope {
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
    pub struct ChoosePrivateKeyRule {
        #[doc = "The package names to which this rule applies. The hash of the signing certificate for each app is verified against the hash provided by Play. If no package names are specified, then the alias is provided to all apps that call KeyChain.choosePrivateKeyAlias (https://developer.android.com/reference/android/security/KeyChain#choosePrivateKeyAlias%28android.app.Activity,%20android.security.KeyChainAliasCallback,%20java.lang.String[],%20java.security.Principal[],%20java.lang.String,%20int,%20java.lang.String%29) or any overloads (but not without calling KeyChain.choosePrivateKeyAlias, even on Android 11 and above). Any app with the same Android UID as a package specified here will have access when they call KeyChain.choosePrivateKeyAlias."]
        #[serde(
            rename = "packageNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_names: ::std::option::Option<Vec<String>>,
        #[doc = "The alias of the private key to be used."]
        #[serde(
            rename = "privateKeyAlias",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_key_alias: ::std::option::Option<String>,
        #[doc = "The URL pattern to match against the URL of the request. If not set or empty, it matches all URLs. This uses the regular expression syntax of java.util.regex.Pattern."]
        #[serde(
            rename = "urlPattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_pattern: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ChoosePrivateKeyRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChoosePrivateKeyRule {
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
        #[doc = "The timestamp at which the command was created. The timestamp is automatically generated by the server."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The duration for which the command is valid. The command will expire if not executed by the device during this time. The default duration if unspecified is ten minutes. There is no maximum duration."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller."]
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_code: ::std::option::Option<crate::schemas::CommandErrorCode>,
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies the new password."]
        #[serde(
            rename = "newPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_password: ::std::option::Option<String>,
        #[doc = "The type of the command."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CommandType>,
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies flags."]
        #[serde(
            rename = "resetPasswordFlags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reset_password_flags:
            ::std::option::Option<Vec<crate::schemas::CommandResetPasswordFlagsItems>>,
        #[doc = "The resource name of the user that owns the device in the form enterprises/{enterpriseId}/users/{userId}. This is automatically generated by the server based on the device the command is sent to."]
        #[serde(
            rename = "userName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_name: ::std::option::Option<String>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommandErrorCode {
        #[doc = "The API level of the device does not support this command."]
        ApiLevel,
        #[doc = "There was no error."]
        CommandErrorCodeUnspecified,
        #[doc = "The command has an invalid parameter value."]
        InvalidValue,
        #[doc = "The management mode (profile owner, device owner, etc.) does not support the command."]
        ManagementMode,
        #[doc = "An unknown error occurred."]
        Unknown,
        #[doc = "The device doesn't support the command. Updating Android Device Policy to the latest version may resolve the issue."]
        Unsupported,
    }
    impl CommandErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                CommandErrorCode::ApiLevel => "API_LEVEL",
                CommandErrorCode::CommandErrorCodeUnspecified => "COMMAND_ERROR_CODE_UNSPECIFIED",
                CommandErrorCode::InvalidValue => "INVALID_VALUE",
                CommandErrorCode::ManagementMode => "MANAGEMENT_MODE",
                CommandErrorCode::Unknown => "UNKNOWN",
                CommandErrorCode::Unsupported => "UNSUPPORTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommandErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommandErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommandErrorCode, ()> {
            Ok(match s {
                "API_LEVEL" => CommandErrorCode::ApiLevel,
                "COMMAND_ERROR_CODE_UNSPECIFIED" => CommandErrorCode::CommandErrorCodeUnspecified,
                "INVALID_VALUE" => CommandErrorCode::InvalidValue,
                "MANAGEMENT_MODE" => CommandErrorCode::ManagementMode,
                "UNKNOWN" => CommandErrorCode::Unknown,
                "UNSUPPORTED" => CommandErrorCode::Unsupported,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommandErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_LEVEL" => CommandErrorCode::ApiLevel,
                "COMMAND_ERROR_CODE_UNSPECIFIED" => CommandErrorCode::CommandErrorCodeUnspecified,
                "INVALID_VALUE" => CommandErrorCode::InvalidValue,
                "MANAGEMENT_MODE" => CommandErrorCode::ManagementMode,
                "UNKNOWN" => CommandErrorCode::Unknown,
                "UNSUPPORTED" => CommandErrorCode::Unsupported,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommandErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommandErrorCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommandType {
        #[doc = "This value is disallowed."]
        CommandTypeUnspecified,
        #[doc = "Lock the device, as if the lock screen timeout had expired."]
        Lock,
        #[doc = "Reboot the device. Only supported on fully managed devices running Android 7.0 (API level 24) or higher."]
        Reboot,
        #[doc = "Removes the work profile and all policies from a company-owned Android 8.0+ device, relinquishing the device for personal use. Apps and data associated with the personal profile(s) are preserved. The device will be deleted from the server after it acknowledges the command."]
        RelinquishOwnership,
        #[doc = "Reset the user's password."]
        ResetPassword,
    }
    impl CommandType {
        pub fn as_str(self) -> &'static str {
            match self {
                CommandType::CommandTypeUnspecified => "COMMAND_TYPE_UNSPECIFIED",
                CommandType::Lock => "LOCK",
                CommandType::Reboot => "REBOOT",
                CommandType::RelinquishOwnership => "RELINQUISH_OWNERSHIP",
                CommandType::ResetPassword => "RESET_PASSWORD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommandType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommandType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommandType, ()> {
            Ok(match s {
                "COMMAND_TYPE_UNSPECIFIED" => CommandType::CommandTypeUnspecified,
                "LOCK" => CommandType::Lock,
                "REBOOT" => CommandType::Reboot,
                "RELINQUISH_OWNERSHIP" => CommandType::RelinquishOwnership,
                "RESET_PASSWORD" => CommandType::ResetPassword,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommandType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMAND_TYPE_UNSPECIFIED" => CommandType::CommandTypeUnspecified,
                "LOCK" => CommandType::Lock,
                "REBOOT" => CommandType::Reboot,
                "RELINQUISH_OWNERSHIP" => CommandType::RelinquishOwnership,
                "RESET_PASSWORD" => CommandType::ResetPassword,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommandType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommandType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommandResetPasswordFlagsItems {
        #[doc = "Don't ask for user credentials on device boot."]
        DoNotAskCredentialsOnBoot,
        #[doc = "Lock the device after password reset."]
        LockNow,
        #[doc = "Don't allow other admins to change the password again until the user has entered it."]
        RequireEntry,
        #[doc = "This value is ignored."]
        ResetPasswordFlagUnspecified,
    }
    impl CommandResetPasswordFlagsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CommandResetPasswordFlagsItems::DoNotAskCredentialsOnBoot => {
                    "DO_NOT_ASK_CREDENTIALS_ON_BOOT"
                }
                CommandResetPasswordFlagsItems::LockNow => "LOCK_NOW",
                CommandResetPasswordFlagsItems::RequireEntry => "REQUIRE_ENTRY",
                CommandResetPasswordFlagsItems::ResetPasswordFlagUnspecified => {
                    "RESET_PASSWORD_FLAG_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommandResetPasswordFlagsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommandResetPasswordFlagsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommandResetPasswordFlagsItems, ()> {
            Ok(match s {
                "DO_NOT_ASK_CREDENTIALS_ON_BOOT" => {
                    CommandResetPasswordFlagsItems::DoNotAskCredentialsOnBoot
                }
                "LOCK_NOW" => CommandResetPasswordFlagsItems::LockNow,
                "REQUIRE_ENTRY" => CommandResetPasswordFlagsItems::RequireEntry,
                "RESET_PASSWORD_FLAG_UNSPECIFIED" => {
                    CommandResetPasswordFlagsItems::ResetPasswordFlagUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommandResetPasswordFlagsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandResetPasswordFlagsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandResetPasswordFlagsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DO_NOT_ASK_CREDENTIALS_ON_BOOT" => {
                    CommandResetPasswordFlagsItems::DoNotAskCredentialsOnBoot
                }
                "LOCK_NOW" => CommandResetPasswordFlagsItems::LockNow,
                "REQUIRE_ENTRY" => CommandResetPasswordFlagsItems::RequireEntry,
                "RESET_PASSWORD_FLAG_UNSPECIFIED" => {
                    CommandResetPasswordFlagsItems::ResetPasswordFlagUnspecified
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
    impl ::google_field_selector::FieldSelector for CommandResetPasswordFlagsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommandResetPasswordFlagsItems {
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
    pub struct CommonCriteriaModeInfo {
        #[doc = "Whether Common Criteria Mode is enabled."]
        #[serde(
            rename = "commonCriteriaModeStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_criteria_mode_status:
            ::std::option::Option<crate::schemas::CommonCriteriaModeInfoCommonCriteriaModeStatus>,
    }
    impl ::google_field_selector::FieldSelector for CommonCriteriaModeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonCriteriaModeInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommonCriteriaModeInfoCommonCriteriaModeStatus {
        #[doc = "Common Criteria Mode is currently disabled."]
        CommonCriteriaModeDisabled,
        #[doc = "Common Criteria Mode is currently enabled."]
        CommonCriteriaModeEnabled,
        #[doc = "Unknown status."]
        CommonCriteriaModeStatusUnknown,
    }
    impl CommonCriteriaModeInfoCommonCriteriaModeStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeDisabled => {
                    "COMMON_CRITERIA_MODE_DISABLED"
                }
                CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeEnabled => {
                    "COMMON_CRITERIA_MODE_ENABLED"
                }
                CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeStatusUnknown => {
                    "COMMON_CRITERIA_MODE_STATUS_UNKNOWN"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CommonCriteriaModeInfoCommonCriteriaModeStatus, ()> {
            Ok(match s {
                "COMMON_CRITERIA_MODE_DISABLED" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeDisabled
                }
                "COMMON_CRITERIA_MODE_ENABLED" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeEnabled
                }
                "COMMON_CRITERIA_MODE_STATUS_UNKNOWN" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeStatusUnknown
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMON_CRITERIA_MODE_DISABLED" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeDisabled
                }
                "COMMON_CRITERIA_MODE_ENABLED" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeEnabled
                }
                "COMMON_CRITERIA_MODE_STATUS_UNKNOWN" => {
                    CommonCriteriaModeInfoCommonCriteriaModeStatus::CommonCriteriaModeStatusUnknown
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
    impl ::google_field_selector::FieldSelector for CommonCriteriaModeInfoCommonCriteriaModeStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonCriteriaModeInfoCommonCriteriaModeStatus {
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
    pub struct ComplianceRule {
        #[doc = "A condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement."]
        #[serde(
            rename = "apiLevelCondition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_level_condition: ::std::option::Option<crate::schemas::ApiLevelCondition>,
        #[doc = "If set to true, the rule includes a mitigating action to disable apps so that the device is effectively disabled, but app data is preserved. If the device is running an app in locked task mode, the app will be closed and a UI showing the reason for non-compliance will be displayed."]
        #[serde(
            rename = "disableApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_apps: ::std::option::Option<bool>,
        #[doc = "A condition which is satisfied if there exists any matching NonComplianceDetail for the device."]
        #[serde(
            rename = "nonComplianceDetailCondition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_detail_condition:
            ::std::option::Option<crate::schemas::NonComplianceDetailCondition>,
        #[doc = "If set, the rule includes a mitigating action to disable apps specified in the list, but app data is preserved."]
        #[serde(
            rename = "packageNamesToDisable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_names_to_disable: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ComplianceRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComplianceRule {
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
    pub struct ContactInfo {
        #[doc = "Email address for a point of contact, which will be used to send important announcements related to managed Google Play."]
        #[serde(
            rename = "contactEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_email: ::std::option::Option<String>,
        #[doc = "The email of the data protection officer. The email is validated but not verified."]
        #[serde(
            rename = "dataProtectionOfficerEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_protection_officer_email: ::std::option::Option<String>,
        #[doc = "The name of the data protection officer."]
        #[serde(
            rename = "dataProtectionOfficerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_protection_officer_name: ::std::option::Option<String>,
        #[doc = "The phone number of the data protection officer The phone number is validated but not verified."]
        #[serde(
            rename = "dataProtectionOfficerPhone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_protection_officer_phone: ::std::option::Option<String>,
        #[doc = "The email of the EU representative. The email is validated but not verified."]
        #[serde(
            rename = "euRepresentativeEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eu_representative_email: ::std::option::Option<String>,
        #[doc = "The name of the EU representative."]
        #[serde(
            rename = "euRepresentativeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eu_representative_name: ::std::option::Option<String>,
        #[doc = "The phone number of the EU representative. The phone number is validated but not verified."]
        #[serde(
            rename = "euRepresentativePhone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eu_representative_phone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContactInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactInfo {
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
    pub struct ContentProviderEndpoint {
        #[doc = "This feature is not generally available."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Required. This feature is not generally available."]
        #[serde(
            rename = "signingCertsSha256",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signing_certs_sha_256: ::std::option::Option<Vec<String>>,
        #[doc = "This feature is not generally available."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContentProviderEndpoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContentProviderEndpoint {
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
    pub struct CrossProfilePolicies {
        #[doc = "Whether text copied from one profile (personal or work) can be pasted in the other profile."]
        #[serde(
            rename = "crossProfileCopyPaste",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_profile_copy_paste:
            ::std::option::Option<crate::schemas::CrossProfilePoliciesCrossProfileCopyPaste>,
        #[doc = "Whether data from one profile (personal or work) can be shared with apps in the other profile. Specifically controls simple data sharing via intents. Management of other cross-profile communication channels, such as contact search, copy/paste, or connected work & personal apps, are configured separately."]
        #[serde(
            rename = "crossProfileDataSharing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_profile_data_sharing:
            ::std::option::Option<crate::schemas::CrossProfilePoliciesCrossProfileDataSharing>,
        #[doc = "Whether contacts stored in the work profile can be shown in personal profile contact searches and incoming calls."]
        #[serde(
            rename = "showWorkContactsInPersonalProfile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_work_contacts_in_personal_profile: ::std::option::Option<
            crate::schemas::CrossProfilePoliciesShowWorkContactsInPersonalProfile,
        >,
    }
    impl ::google_field_selector::FieldSelector for CrossProfilePolicies {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CrossProfilePolicies {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CrossProfilePoliciesCrossProfileCopyPaste {
        #[doc = "Default. Prevents users from pasting into the personal profile text copied from the work profile. Text copied from the personal profile can be pasted into the work profile, and text copied from the work profile can be pasted into the work profile."]
        CopyFromWorkToPersonalDisallowed,
        #[doc = "Text copied in either profile can be pasted in the other profile."]
        CrossProfileCopyPasteAllowed,
        #[doc = "Unspecified. Defaults to COPY_FROM_WORK_TO_PERSONAL_DISALLOWED"]
        CrossProfileCopyPasteUnspecified,
    }
    impl CrossProfilePoliciesCrossProfileCopyPaste {
        pub fn as_str(self) -> &'static str {
            match self {
                CrossProfilePoliciesCrossProfileCopyPaste::CopyFromWorkToPersonalDisallowed => {
                    "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED"
                }
                CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteAllowed => {
                    "CROSS_PROFILE_COPY_PASTE_ALLOWED"
                }
                CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteUnspecified => {
                    "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CrossProfilePoliciesCrossProfileCopyPaste {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CrossProfilePoliciesCrossProfileCopyPaste {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CrossProfilePoliciesCrossProfileCopyPaste, ()> {
            Ok(match s {
                "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CopyFromWorkToPersonalDisallowed
                }
                "CROSS_PROFILE_COPY_PASTE_ALLOWED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteAllowed
                }
                "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CrossProfilePoliciesCrossProfileCopyPaste {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CrossProfilePoliciesCrossProfileCopyPaste {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CrossProfilePoliciesCrossProfileCopyPaste {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CopyFromWorkToPersonalDisallowed
                }
                "CROSS_PROFILE_COPY_PASTE_ALLOWED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteAllowed
                }
                "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED" => {
                    CrossProfilePoliciesCrossProfileCopyPaste::CrossProfileCopyPasteUnspecified
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
    impl ::google_field_selector::FieldSelector for CrossProfilePoliciesCrossProfileCopyPaste {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CrossProfilePoliciesCrossProfileCopyPaste {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CrossProfilePoliciesCrossProfileDataSharing {
        #[doc = "Data from either profile can be shared with the other profile."]
        CrossProfileDataSharingAllowed,
        #[doc = "Prevents data from being shared from both the personal profile to the work profile and the work profile to the personal profile."]
        CrossProfileDataSharingDisallowed,
        #[doc = "Unspecified. Defaults to DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED."]
        CrossProfileDataSharingUnspecified,
        #[doc = "Default. Prevents users from sharing data from the work profile to apps in the personal profile. Personal data can be shared with work apps."]
        DataSharingFromWorkToPersonalDisallowed,
    }
    impl CrossProfilePoliciesCrossProfileDataSharing {
        pub fn as_str(self) -> &'static str {
            match self { CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingAllowed => "CROSS_PROFILE_DATA_SHARING_ALLOWED" , CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingDisallowed => "CROSS_PROFILE_DATA_SHARING_DISALLOWED" , CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingUnspecified => "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED" , CrossProfilePoliciesCrossProfileDataSharing :: DataSharingFromWorkToPersonalDisallowed => "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CrossProfilePoliciesCrossProfileDataSharing {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CrossProfilePoliciesCrossProfileDataSharing {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CrossProfilePoliciesCrossProfileDataSharing, ()> {
            Ok (match s { "CROSS_PROFILE_DATA_SHARING_ALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingAllowed , "CROSS_PROFILE_DATA_SHARING_DISALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingDisallowed , "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingUnspecified , "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: DataSharingFromWorkToPersonalDisallowed , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for CrossProfilePoliciesCrossProfileDataSharing {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CrossProfilePoliciesCrossProfileDataSharing {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CrossProfilePoliciesCrossProfileDataSharing {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CROSS_PROFILE_DATA_SHARING_ALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingAllowed , "CROSS_PROFILE_DATA_SHARING_DISALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingDisallowed , "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED" => CrossProfilePoliciesCrossProfileDataSharing :: CrossProfileDataSharingUnspecified , "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED" => CrossProfilePoliciesCrossProfileDataSharing :: DataSharingFromWorkToPersonalDisallowed , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for CrossProfilePoliciesCrossProfileDataSharing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CrossProfilePoliciesCrossProfileDataSharing {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        #[doc = "Default. Allows work profile contacts to appear in personal profile contact searches and incoming calls"]
        ShowWorkContactsInPersonalProfileAllowed,
        #[doc = "Prevents work profile contacts from appearing in personal profile contact searches and incoming calls"]
        ShowWorkContactsInPersonalProfileDisallowed,
        #[doc = "Unspecified. Defaults to SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED."]
        ShowWorkContactsInPersonalProfileUnspecified,
    }
    impl CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        pub fn as_str(self) -> &'static str {
            match self { CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileAllowed => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED" , CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileDisallowed => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED" , CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileUnspecified => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CrossProfilePoliciesShowWorkContactsInPersonalProfile, ()>
        {
            Ok (match s { "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileAllowed , "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileDisallowed , "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CrossProfilePoliciesShowWorkContactsInPersonalProfile {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileAllowed , "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileDisallowed , "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED" => CrossProfilePoliciesShowWorkContactsInPersonalProfile :: ShowWorkContactsInPersonalProfileUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for CrossProfilePoliciesShowWorkContactsInPersonalProfile
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for CrossProfilePoliciesShowWorkContactsInPersonalProfile
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
    pub struct Date {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Device {
        #[doc = "The API level of the Android platform version running on the device."]
        #[serde(
            rename = "apiLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_level: ::std::option::Option<i32>,
        #[doc = "Reports for apps installed on the device. This information is only available when application_reports_enabled is true in the device's policy."]
        #[serde(
            rename = "applicationReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_reports: ::std::option::Option<Vec<crate::schemas::ApplicationReport>>,
        #[doc = "The password requirements currently applied to the device. The applied requirements may be slightly different from those specified in passwordPolicies in some cases. fieldPath is set based on passwordPolicies."]
        #[serde(
            rename = "appliedPasswordPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_password_policies:
            ::std::option::Option<Vec<crate::schemas::PasswordRequirements>>,
        #[doc = "The name of the policy currently applied to the device."]
        #[serde(
            rename = "appliedPolicyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_policy_name: ::std::option::Option<String>,
        #[doc = "The version of the policy currently applied to the device."]
        #[serde(
            rename = "appliedPolicyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub applied_policy_version: ::std::option::Option<i64>,
        #[doc = "The state currently applied to the device."]
        #[serde(
            rename = "appliedState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_state: ::std::option::Option<crate::schemas::DeviceAppliedState>,
        #[doc = "Information about Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC).This information is only available if statusReportingSettings.commonCriteriaModeEnabled is true in the device's policy."]
        #[serde(
            rename = "commonCriteriaModeInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_criteria_mode_info:
            ::std::option::Option<crate::schemas::CommonCriteriaModeInfo>,
        #[doc = "Device settings information. This information is only available if deviceSettingsEnabled is true in the device's policy."]
        #[serde(
            rename = "deviceSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_settings: ::std::option::Option<crate::schemas::DeviceSettings>,
        #[doc = "If the device state is DISABLED, an optional message that is displayed on the device indicating the reason the device is disabled. This field can be modified by a patch request."]
        #[serde(
            rename = "disabledReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled_reason: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "Detailed information about displays on the device. This information is only available if displayInfoEnabled is true in the device's policy."]
        #[serde(
            rename = "displays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub displays: ::std::option::Option<Vec<crate::schemas::Display>>,
        #[doc = "The time of device enrollment."]
        #[serde(
            rename = "enrollmentTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enrollment_time: ::std::option::Option<String>,
        #[doc = "If the device was enrolled with an enrollment token with additional data provided, this field contains that data."]
        #[serde(
            rename = "enrollmentTokenData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enrollment_token_data: ::std::option::Option<String>,
        #[doc = "If the device was enrolled with an enrollment token, this field contains the name of the token."]
        #[serde(
            rename = "enrollmentTokenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enrollment_token_name: ::std::option::Option<String>,
        #[doc = "Detailed information about the device hardware."]
        #[serde(
            rename = "hardwareInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hardware_info: ::std::option::Option<crate::schemas::HardwareInfo>,
        #[doc = "Hardware status samples in chronological order. This information is only available if hardwareStatusEnabled is true in the device's policy."]
        #[serde(
            rename = "hardwareStatusSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hardware_status_samples: ::std::option::Option<Vec<crate::schemas::HardwareStatus>>,
        #[doc = "Deprecated."]
        #[serde(
            rename = "lastPolicyComplianceReportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_policy_compliance_report_time: ::std::option::Option<String>,
        #[doc = "The last time the device fetched its policy."]
        #[serde(
            rename = "lastPolicySyncTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_policy_sync_time: ::std::option::Option<String>,
        #[doc = "The last time the device sent a status report."]
        #[serde(
            rename = "lastStatusReportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_status_report_time: ::std::option::Option<String>,
        #[doc = "The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported."]
        #[serde(
            rename = "managementMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub management_mode: ::std::option::Option<crate::schemas::DeviceManagementMode>,
        #[doc = "Events related to memory and storage measurements in chronological order. This information is only available if memoryInfoEnabled is true in the device's policy."]
        #[serde(
            rename = "memoryEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_events: ::std::option::Option<Vec<crate::schemas::MemoryEvent>>,
        #[doc = "Memory information: contains information about device memory and storage."]
        #[serde(
            rename = "memoryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_info: ::std::option::Option<crate::schemas::MemoryInfo>,
        #[doc = "The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Device network information. This information is only available if networkInfoEnabled is true in the device's policy."]
        #[serde(
            rename = "networkInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_info: ::std::option::Option<crate::schemas::NetworkInfo>,
        #[doc = "Details about policy settings that the device is not compliant with."]
        #[serde(
            rename = "nonComplianceDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_details: ::std::option::Option<Vec<crate::schemas::NonComplianceDetail>>,
        #[doc = "Ownership of the managed device."]
        #[serde(
            rename = "ownership",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ownership: ::std::option::Option<crate::schemas::DeviceOwnership>,
        #[doc = "Whether the device is compliant with its policy."]
        #[serde(
            rename = "policyCompliant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_compliant: ::std::option::Option<bool>,
        #[doc = "The name of the policy applied to the device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device's user is applied. This field can be modified by a patch request. You can specify only the policyId when calling enterprises.devices.patch, as long as the policyId doesn’t contain any slashes. The rest of the policy name is inferred."]
        #[serde(
            rename = "policyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_name: ::std::option::Option<String>,
        #[doc = "Power management events on the device in chronological order. This information is only available if powerManagementEventsEnabled is true in the device's policy."]
        #[serde(
            rename = "powerManagementEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub power_management_events:
            ::std::option::Option<Vec<crate::schemas::PowerManagementEvent>>,
        #[doc = "If the same physical device has been enrolled multiple times, this field contains its previous device names. The serial number is used as the unique identifier to determine if the same physical device has enrolled previously. The names are in chronological order."]
        #[serde(
            rename = "previousDeviceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_device_names: ::std::option::Option<Vec<String>>,
        #[doc = "Device's security posture value that reflects how secure the device is."]
        #[serde(
            rename = "securityPosture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_posture: ::std::option::Option<crate::schemas::SecurityPosture>,
        #[doc = "Detailed information about the device software. This information is only available if softwareInfoEnabled is true in the device's policy."]
        #[serde(
            rename = "softwareInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_info: ::std::option::Option<crate::schemas::SoftwareInfo>,
        #[doc = "The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::DeviceState>,
        #[doc = "Map of selected system properties name and value related to the device. This information is only available if systemPropertiesEnabled is true in the device's policy."]
        #[serde(
            rename = "systemProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_properties: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The user who owns the device."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
        #[doc = "The resource name of the user that owns this device in the form enterprises/{enterpriseId}/users/{userId}."]
        #[serde(
            rename = "userName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Device {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Device {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceAppliedState {
        #[doc = "The device is active."]
        Active,
        #[doc = "The device was deleted. This state is never returned by an API call, but is used in the final status report when the device acknowledges the deletion. If the device is deleted via the API call, this state is published to Pub/Sub. If the user deletes the work profile or resets the device, the device state will remain unknown to the server."]
        Deleted,
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[doc = "The device is disabled."]
        Disabled,
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl DeviceAppliedState {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceAppliedState::Active => "ACTIVE",
                DeviceAppliedState::Deleted => "DELETED",
                DeviceAppliedState::DeviceStateUnspecified => "DEVICE_STATE_UNSPECIFIED",
                DeviceAppliedState::Disabled => "DISABLED",
                DeviceAppliedState::Provisioning => "PROVISIONING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceAppliedState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceAppliedState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceAppliedState, ()> {
            Ok(match s {
                "ACTIVE" => DeviceAppliedState::Active,
                "DELETED" => DeviceAppliedState::Deleted,
                "DEVICE_STATE_UNSPECIFIED" => DeviceAppliedState::DeviceStateUnspecified,
                "DISABLED" => DeviceAppliedState::Disabled,
                "PROVISIONING" => DeviceAppliedState::Provisioning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceAppliedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceAppliedState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceAppliedState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DeviceAppliedState::Active,
                "DELETED" => DeviceAppliedState::Deleted,
                "DEVICE_STATE_UNSPECIFIED" => DeviceAppliedState::DeviceStateUnspecified,
                "DISABLED" => DeviceAppliedState::Disabled,
                "PROVISIONING" => DeviceAppliedState::Provisioning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceAppliedState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceAppliedState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceManagementMode {
        #[doc = "Device owner. Android Device Policy has full control over the device."]
        DeviceOwner,
        #[doc = "This value is disallowed."]
        ManagementModeUnspecified,
        #[doc = "Profile owner. Android Device Policy has control over a managed profile on the device."]
        ProfileOwner,
    }
    impl DeviceManagementMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceManagementMode::DeviceOwner => "DEVICE_OWNER",
                DeviceManagementMode::ManagementModeUnspecified => "MANAGEMENT_MODE_UNSPECIFIED",
                DeviceManagementMode::ProfileOwner => "PROFILE_OWNER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceManagementMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceManagementMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceManagementMode, ()> {
            Ok(match s {
                "DEVICE_OWNER" => DeviceManagementMode::DeviceOwner,
                "MANAGEMENT_MODE_UNSPECIFIED" => DeviceManagementMode::ManagementModeUnspecified,
                "PROFILE_OWNER" => DeviceManagementMode::ProfileOwner,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceManagementMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceManagementMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceManagementMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_OWNER" => DeviceManagementMode::DeviceOwner,
                "MANAGEMENT_MODE_UNSPECIFIED" => DeviceManagementMode::ManagementModeUnspecified,
                "PROFILE_OWNER" => DeviceManagementMode::ProfileOwner,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceManagementMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceManagementMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceOwnership {
        #[doc = "Device is company-owned."]
        CompanyOwned,
        #[doc = "Ownership is unspecified."]
        OwnershipUnspecified,
        #[doc = "Device is personally-owned."]
        PersonallyOwned,
    }
    impl DeviceOwnership {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceOwnership::CompanyOwned => "COMPANY_OWNED",
                DeviceOwnership::OwnershipUnspecified => "OWNERSHIP_UNSPECIFIED",
                DeviceOwnership::PersonallyOwned => "PERSONALLY_OWNED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceOwnership {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceOwnership {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceOwnership, ()> {
            Ok(match s {
                "COMPANY_OWNED" => DeviceOwnership::CompanyOwned,
                "OWNERSHIP_UNSPECIFIED" => DeviceOwnership::OwnershipUnspecified,
                "PERSONALLY_OWNED" => DeviceOwnership::PersonallyOwned,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceOwnership {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceOwnership {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceOwnership {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPANY_OWNED" => DeviceOwnership::CompanyOwned,
                "OWNERSHIP_UNSPECIFIED" => DeviceOwnership::OwnershipUnspecified,
                "PERSONALLY_OWNED" => DeviceOwnership::PersonallyOwned,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceOwnership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceOwnership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceState {
        #[doc = "The device is active."]
        Active,
        #[doc = "The device was deleted. This state is never returned by an API call, but is used in the final status report when the device acknowledges the deletion. If the device is deleted via the API call, this state is published to Pub/Sub. If the user deletes the work profile or resets the device, the device state will remain unknown to the server."]
        Deleted,
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[doc = "The device is disabled."]
        Disabled,
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl DeviceState {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceState::Active => "ACTIVE",
                DeviceState::Deleted => "DELETED",
                DeviceState::DeviceStateUnspecified => "DEVICE_STATE_UNSPECIFIED",
                DeviceState::Disabled => "DISABLED",
                DeviceState::Provisioning => "PROVISIONING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceState, ()> {
            Ok(match s {
                "ACTIVE" => DeviceState::Active,
                "DELETED" => DeviceState::Deleted,
                "DEVICE_STATE_UNSPECIFIED" => DeviceState::DeviceStateUnspecified,
                "DISABLED" => DeviceState::Disabled,
                "PROVISIONING" => DeviceState::Provisioning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DeviceState::Active,
                "DELETED" => DeviceState::Deleted,
                "DEVICE_STATE_UNSPECIFIED" => DeviceState::DeviceStateUnspecified,
                "DISABLED" => DeviceState::Disabled,
                "PROVISIONING" => DeviceState::Provisioning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceState {
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
    pub struct DeviceSettings {
        #[doc = "Whether ADB (https://developer.android.com/studio/command-line/adb.html) is enabled on the device."]
        #[serde(
            rename = "adbEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adb_enabled: ::std::option::Option<bool>,
        #[doc = "Whether developer mode is enabled on the device."]
        #[serde(
            rename = "developmentSettingsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub development_settings_enabled: ::std::option::Option<bool>,
        #[doc = "Encryption status from DevicePolicyManager."]
        #[serde(
            rename = "encryptionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_status:
            ::std::option::Option<crate::schemas::DeviceSettingsEncryptionStatus>,
        #[doc = "Whether the device is secured with PIN/password."]
        #[serde(
            rename = "isDeviceSecure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_device_secure: ::std::option::Option<bool>,
        #[doc = "Whether the storage encryption is enabled."]
        #[serde(
            rename = "isEncrypted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_encrypted: ::std::option::Option<bool>,
        #[doc = "Whether installing apps from unknown sources is enabled."]
        #[serde(
            rename = "unknownSourcesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unknown_sources_enabled: ::std::option::Option<bool>,
        #[doc = "Whether Google Play Protect verification (https://support.google.com/accounts/answer/2812853) is enforced on the device."]
        #[serde(
            rename = "verifyAppsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verify_apps_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DeviceSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceSettingsEncryptionStatus {
        #[doc = "Encryption is not currently active, but is currently being activated."]
        Activating,
        #[doc = "Encryption is active."]
        Active,
        #[doc = "Encryption is active, but an encryption key is not set by the user."]
        ActiveDefaultKey,
        #[doc = "Encryption is active, and the encryption key is tied to the user profile."]
        ActivePerUser,
        #[doc = "Unspecified. No device should have this type."]
        EncryptionStatusUnspecified,
        #[doc = "Encryption is supported by the device, but is not currently active."]
        Inactive,
        #[doc = "Encryption is not supported by the device."]
        Unsupported,
    }
    impl DeviceSettingsEncryptionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceSettingsEncryptionStatus::Activating => "ACTIVATING",
                DeviceSettingsEncryptionStatus::Active => "ACTIVE",
                DeviceSettingsEncryptionStatus::ActiveDefaultKey => "ACTIVE_DEFAULT_KEY",
                DeviceSettingsEncryptionStatus::ActivePerUser => "ACTIVE_PER_USER",
                DeviceSettingsEncryptionStatus::EncryptionStatusUnspecified => {
                    "ENCRYPTION_STATUS_UNSPECIFIED"
                }
                DeviceSettingsEncryptionStatus::Inactive => "INACTIVE",
                DeviceSettingsEncryptionStatus::Unsupported => "UNSUPPORTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceSettingsEncryptionStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceSettingsEncryptionStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceSettingsEncryptionStatus, ()> {
            Ok(match s {
                "ACTIVATING" => DeviceSettingsEncryptionStatus::Activating,
                "ACTIVE" => DeviceSettingsEncryptionStatus::Active,
                "ACTIVE_DEFAULT_KEY" => DeviceSettingsEncryptionStatus::ActiveDefaultKey,
                "ACTIVE_PER_USER" => DeviceSettingsEncryptionStatus::ActivePerUser,
                "ENCRYPTION_STATUS_UNSPECIFIED" => {
                    DeviceSettingsEncryptionStatus::EncryptionStatusUnspecified
                }
                "INACTIVE" => DeviceSettingsEncryptionStatus::Inactive,
                "UNSUPPORTED" => DeviceSettingsEncryptionStatus::Unsupported,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceSettingsEncryptionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceSettingsEncryptionStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceSettingsEncryptionStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVATING" => DeviceSettingsEncryptionStatus::Activating,
                "ACTIVE" => DeviceSettingsEncryptionStatus::Active,
                "ACTIVE_DEFAULT_KEY" => DeviceSettingsEncryptionStatus::ActiveDefaultKey,
                "ACTIVE_PER_USER" => DeviceSettingsEncryptionStatus::ActivePerUser,
                "ENCRYPTION_STATUS_UNSPECIFIED" => {
                    DeviceSettingsEncryptionStatus::EncryptionStatusUnspecified
                }
                "INACTIVE" => DeviceSettingsEncryptionStatus::Inactive,
                "UNSUPPORTED" => DeviceSettingsEncryptionStatus::Unsupported,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceSettingsEncryptionStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceSettingsEncryptionStatus {
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
    pub struct Display {
        #[doc = "Display density expressed as dots-per-inch."]
        #[serde(
            rename = "density",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub density: ::std::option::Option<i32>,
        #[doc = "Unique display id."]
        #[serde(
            rename = "displayId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_id: ::std::option::Option<i32>,
        #[doc = "Display height in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Name of the display."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Refresh rate of the display in frames per second."]
        #[serde(
            rename = "refreshRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_rate: ::std::option::Option<i32>,
        #[doc = "State of the display."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::DisplayState>,
        #[doc = "Display width in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Display {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Display {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DisplayState {
        #[doc = "This value is disallowed."]
        DisplayStateUnspecified,
        #[doc = "Display is dozing in a low power state"]
        Doze,
        #[doc = "Display is off."]
        Off,
        #[doc = "Display is on."]
        On,
        #[doc = "Display is dozing in a suspended low power state."]
        Suspended,
    }
    impl DisplayState {
        pub fn as_str(self) -> &'static str {
            match self {
                DisplayState::DisplayStateUnspecified => "DISPLAY_STATE_UNSPECIFIED",
                DisplayState::Doze => "DOZE",
                DisplayState::Off => "OFF",
                DisplayState::On => "ON",
                DisplayState::Suspended => "SUSPENDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DisplayState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DisplayState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DisplayState, ()> {
            Ok(match s {
                "DISPLAY_STATE_UNSPECIFIED" => DisplayState::DisplayStateUnspecified,
                "DOZE" => DisplayState::Doze,
                "OFF" => DisplayState::Off,
                "ON" => DisplayState::On,
                "SUSPENDED" => DisplayState::Suspended,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DisplayState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DisplayState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DisplayState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_STATE_UNSPECIFIED" => DisplayState::DisplayStateUnspecified,
                "DOZE" => DisplayState::Doze,
                "OFF" => DisplayState::Off,
                "ON" => DisplayState::On,
                "SUSPENDED" => DisplayState::Suspended,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DisplayState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisplayState {
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
    pub struct EnrollmentToken {
        #[doc = "Optional, arbitrary data associated with the enrollment token. This could contain, for example, the ID of an org unit the device is assigned to after enrollment. After a device enrolls with the token, this data will be exposed in the enrollment_token_data field of the Device resource. The data must be 1024 characters or less; otherwise, the creation request will fail."]
        #[serde(
            rename = "additionalData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_data: ::std::option::Option<String>,
        #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
        #[serde(
            rename = "allowPersonalUsage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_personal_usage:
            ::std::option::Option<crate::schemas::EnrollmentTokenAllowPersonalUsage>,
        #[doc = "The length of time the enrollment token is valid, ranging from 1 minute to 90 days. If not specified, the default duration is 1 hour."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "The expiration time of the token. This is a read-only field generated by the server."]
        #[serde(
            rename = "expirationTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiration_timestamp: ::std::option::Option<String>,
        #[doc = "The name of the enrollment token, which is generated by the server during creation, in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Whether the enrollment token is for one time use only. If the flag is set to true, only one device can use it for registration."]
        #[serde(
            rename = "oneTimeOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub one_time_only: ::std::option::Option<bool>,
        #[doc = "The name of the policy initially applied to the enrolled device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device’s user is applied. If user_name is also not specified, enterprises/{enterpriseId}/policies/default is applied by default. When updating this field, you can specify only the policyId as long as the policyId doesn’t contain any slashes. The rest of the policy name will be inferred."]
        #[serde(
            rename = "policyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_name: ::std::option::Option<String>,
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON."]
        #[serde(
            rename = "qrCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qr_code: ::std::option::Option<String>,
        #[doc = "The user associated with this enrollment token. If it's specified when the enrollment token is created and the user does not exist, the user will be created. This field must not contain personally identifiable information. Only the account_identifier field needs to be set."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
        #[doc = "The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnrollmentToken {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnrollmentToken {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnrollmentTokenAllowPersonalUsage {
        #[doc = "Personal usage restriction is not specified"]
        AllowPersonalUsageUnspecified,
        #[doc = "Personal usage is allowed"]
        PersonalUsageAllowed,
        #[doc = "Personal usage is disallowed"]
        PersonalUsageDisallowed,
    }
    impl EnrollmentTokenAllowPersonalUsage {
        pub fn as_str(self) -> &'static str {
            match self {
                EnrollmentTokenAllowPersonalUsage::AllowPersonalUsageUnspecified => {
                    "ALLOW_PERSONAL_USAGE_UNSPECIFIED"
                }
                EnrollmentTokenAllowPersonalUsage::PersonalUsageAllowed => "PERSONAL_USAGE_ALLOWED",
                EnrollmentTokenAllowPersonalUsage::PersonalUsageDisallowed => {
                    "PERSONAL_USAGE_DISALLOWED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnrollmentTokenAllowPersonalUsage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnrollmentTokenAllowPersonalUsage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnrollmentTokenAllowPersonalUsage, ()> {
            Ok(match s {
                "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => {
                    EnrollmentTokenAllowPersonalUsage::AllowPersonalUsageUnspecified
                }
                "PERSONAL_USAGE_ALLOWED" => EnrollmentTokenAllowPersonalUsage::PersonalUsageAllowed,
                "PERSONAL_USAGE_DISALLOWED" => {
                    EnrollmentTokenAllowPersonalUsage::PersonalUsageDisallowed
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnrollmentTokenAllowPersonalUsage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnrollmentTokenAllowPersonalUsage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnrollmentTokenAllowPersonalUsage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => {
                    EnrollmentTokenAllowPersonalUsage::AllowPersonalUsageUnspecified
                }
                "PERSONAL_USAGE_ALLOWED" => EnrollmentTokenAllowPersonalUsage::PersonalUsageAllowed,
                "PERSONAL_USAGE_DISALLOWED" => {
                    EnrollmentTokenAllowPersonalUsage::PersonalUsageDisallowed
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
    impl ::google_field_selector::FieldSelector for EnrollmentTokenAllowPersonalUsage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnrollmentTokenAllowPersonalUsage {
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
    pub struct Enterprise {
        #[doc = "Deprecated and unused."]
        #[serde(
            rename = "appAutoApprovalEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_auto_approval_enabled: ::std::option::Option<bool>,
        #[doc = "The enterprise contact info of an EMM-managed enterprise."]
        #[serde(
            rename = "contactInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_info: ::std::option::Option<crate::schemas::ContactInfo>,
        #[doc = "The types of Google Pub/Sub notifications enabled for the enterprise."]
        #[serde(
            rename = "enabledNotificationTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled_notification_types:
            ::std::option::Option<Vec<crate::schemas::EnterpriseEnabledNotificationTypesItems>>,
        #[doc = "The name of the enterprise displayed to users."]
        #[serde(
            rename = "enterpriseDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enterprise_display_name: ::std::option::Option<String>,
        #[doc = "An image displayed as a logo during device provisioning. Supported types are: image/bmp, image/gif, image/x-ico, image/jpeg, image/png, image/webp, image/vnd.wap.wbmp, image/x-adobe-dng."]
        #[serde(
            rename = "logo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo: ::std::option::Option<crate::schemas::ExternalData>,
        #[doc = "The name of the enterprise which is generated by the server during creation, in the form enterprises/{enterpriseId}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A color in RGB format that indicates the predominant color to display in the device management app UI. The color components are stored as follows: (red << 16) | (green << 8) | blue, where the value of each component is between 0 and 255, inclusive."]
        #[serde(
            rename = "primaryColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_color: ::std::option::Option<i32>,
        #[doc = "The topic which Pub/Sub notifications are published to, in the form projects/{project}/topics/{topic}. This field is only required if Pub/Sub notifications are enabled."]
        #[serde(
            rename = "pubsubTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pubsub_topic: ::std::option::Option<String>,
        #[doc = "Sign-in details of the enterprise."]
        #[serde(
            rename = "signinDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signin_details: ::std::option::Option<Vec<crate::schemas::SigninDetail>>,
        #[doc = "Terms and conditions that must be accepted when provisioning a device for this enterprise. A page of terms is generated for each value in this list."]
        #[serde(
            rename = "termsAndConditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub terms_and_conditions: ::std::option::Option<Vec<crate::schemas::TermsAndConditions>>,
    }
    impl ::google_field_selector::FieldSelector for Enterprise {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Enterprise {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnterpriseEnabledNotificationTypesItems {
        #[doc = "A notification sent when a device command has completed."]
        Command,
        #[doc = "Deprecated."]
        ComplianceReport,
        #[doc = "A notification sent when a device enrolls."]
        Enrollment,
        #[doc = "This value is ignored."]
        NotificationTypeUnspecified,
        #[doc = "A notification sent when a device issues a status report."]
        StatusReport,
        #[doc = "A notification sent when device sends BatchUsageLogEvents."]
        UsageLogs,
    }
    impl EnterpriseEnabledNotificationTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                EnterpriseEnabledNotificationTypesItems::Command => "COMMAND",
                EnterpriseEnabledNotificationTypesItems::ComplianceReport => "COMPLIANCE_REPORT",
                EnterpriseEnabledNotificationTypesItems::Enrollment => "ENROLLMENT",
                EnterpriseEnabledNotificationTypesItems::NotificationTypeUnspecified => {
                    "NOTIFICATION_TYPE_UNSPECIFIED"
                }
                EnterpriseEnabledNotificationTypesItems::StatusReport => "STATUS_REPORT",
                EnterpriseEnabledNotificationTypesItems::UsageLogs => "USAGE_LOGS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnterpriseEnabledNotificationTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnterpriseEnabledNotificationTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnterpriseEnabledNotificationTypesItems, ()> {
            Ok(match s {
                "COMMAND" => EnterpriseEnabledNotificationTypesItems::Command,
                "COMPLIANCE_REPORT" => EnterpriseEnabledNotificationTypesItems::ComplianceReport,
                "ENROLLMENT" => EnterpriseEnabledNotificationTypesItems::Enrollment,
                "NOTIFICATION_TYPE_UNSPECIFIED" => {
                    EnterpriseEnabledNotificationTypesItems::NotificationTypeUnspecified
                }
                "STATUS_REPORT" => EnterpriseEnabledNotificationTypesItems::StatusReport,
                "USAGE_LOGS" => EnterpriseEnabledNotificationTypesItems::UsageLogs,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnterpriseEnabledNotificationTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnterpriseEnabledNotificationTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnterpriseEnabledNotificationTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMAND" => EnterpriseEnabledNotificationTypesItems::Command,
                "COMPLIANCE_REPORT" => EnterpriseEnabledNotificationTypesItems::ComplianceReport,
                "ENROLLMENT" => EnterpriseEnabledNotificationTypesItems::Enrollment,
                "NOTIFICATION_TYPE_UNSPECIFIED" => {
                    EnterpriseEnabledNotificationTypesItems::NotificationTypeUnspecified
                }
                "STATUS_REPORT" => EnterpriseEnabledNotificationTypesItems::StatusReport,
                "USAGE_LOGS" => EnterpriseEnabledNotificationTypesItems::UsageLogs,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnterpriseEnabledNotificationTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnterpriseEnabledNotificationTypesItems {
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
    pub struct ExtensionConfig {
        #[doc = "Fully qualified class name of the receiver service class for Android Device Policy to notify the extension app of any local command status updates."]
        #[serde(
            rename = "notificationReceiver",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notification_receiver: ::std::option::Option<String>,
        #[doc = "Hex-encoded SHA-256 hash of the signing certificate of the extension app. Only hexadecimal string representations of 64 characters are valid.If not specified, the signature for the corresponding package name is obtained from the Play Store instead.If this list is empty, the signature of the extension app on the device must match the signature obtained from the Play Store for the app to be able to communicate with Android Device Policy.If this list is not empty, the signature of the extension app on the device must match one of the entries in this list for the app to be able to communicate with Android Device Policy.In production use cases, it is recommended to leave this empty."]
        #[serde(
            rename = "signingKeyFingerprintsSha256",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signing_key_fingerprints_sha_256: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ExtensionConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtensionConfig {
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
    pub struct ExternalData {
        #[doc = "The base-64 encoded SHA-256 hash of the content hosted at url. If the content doesn't match this hash, Android Device Policy won't use the data."]
        #[serde(
            rename = "sha256Hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256_hash: ::std::option::Option<String>,
        #[doc = "The absolute URL to the data, which must use either the http or https scheme. Android Device Policy doesn't provide any credentials in the GET request, so the URL must be publicly accessible. Including a long, random component in the URL may be used to prevent attackers from discovering the URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExternalData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalData {
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
    pub struct FreezePeriod {
        #[doc = "The end date (inclusive) of the freeze period. Must be no later than 90 days from the start date. If the end date is earlier than the start date, the freeze period is considered wrapping year-end. Note: year must not be set. For example, {\"month\": 1,\"date\": 30}."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The start date (inclusive) of the freeze period. Note: year must not be set. For example, {\"month\": 1,\"date\": 30}."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
    }
    impl ::google_field_selector::FieldSelector for FreezePeriod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FreezePeriod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HardwareInfo {
        #[doc = "Battery shutdown temperature thresholds in Celsius for each battery on the device."]
        #[serde(
            rename = "batteryShutdownTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_shutdown_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Battery throttling temperature thresholds in Celsius for each battery on the device."]
        #[serde(
            rename = "batteryThrottlingTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_throttling_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Brand of the device. For example, Google."]
        #[serde(
            rename = "brand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brand: ::std::option::Option<String>,
        #[doc = "CPU shutdown temperature thresholds in Celsius for each CPU on the device."]
        #[serde(
            rename = "cpuShutdownTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_shutdown_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "CPU throttling temperature thresholds in Celsius for each CPU on the device."]
        #[serde(
            rename = "cpuThrottlingTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_throttling_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Baseband version. For example, MDM9625_104662.22.05.34p."]
        #[serde(
            rename = "deviceBasebandVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_baseband_version: ::std::option::Option<String>,
        #[doc = "Output only. ID that uniquely identifies a personally-owned device in a particular organization. On the same physical device when enrolled with the same organization, this ID persists across setups and even factory resets. This ID is available on personally-owned devices with a work profile on devices running Android 12 and above."]
        #[serde(
            rename = "enterpriseSpecificId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enterprise_specific_id: ::std::option::Option<String>,
        #[doc = "GPU shutdown temperature thresholds in Celsius for each GPU on the device."]
        #[serde(
            rename = "gpuShutdownTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpu_shutdown_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "GPU throttling temperature thresholds in Celsius for each GPU on the device."]
        #[serde(
            rename = "gpuThrottlingTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpu_throttling_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Name of the hardware. For example, Angler."]
        #[serde(
            rename = "hardware",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hardware: ::std::option::Option<String>,
        #[doc = "Manufacturer. For example, Motorola."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "The model of the device. For example, Asus Nexus 7."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
        #[doc = "The device serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Device skin shutdown temperature thresholds in Celsius."]
        #[serde(
            rename = "skinShutdownTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skin_shutdown_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Device skin throttling temperature thresholds in Celsius."]
        #[serde(
            rename = "skinThrottlingTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skin_throttling_temperatures: ::std::option::Option<Vec<f32>>,
    }
    impl ::google_field_selector::FieldSelector for HardwareInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HardwareInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HardwareStatus {
        #[doc = "Current battery temperatures in Celsius for each battery on the device."]
        #[serde(
            rename = "batteryTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Current CPU temperatures in Celsius for each CPU on the device."]
        #[serde(
            rename = "cpuTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "CPU usages in percentage for each core available on the device. Usage is 0 for each unplugged core. Empty array implies that CPU usage is not supported in the system."]
        #[serde(
            rename = "cpuUsages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_usages: ::std::option::Option<Vec<f32>>,
        #[doc = "The time the measurements were taken."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Fan speeds in RPM for each fan on the device. Empty array means that there are no fans or fan speed is not supported on the system."]
        #[serde(
            rename = "fanSpeeds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fan_speeds: ::std::option::Option<Vec<f32>>,
        #[doc = "Current GPU temperatures in Celsius for each GPU on the device."]
        #[serde(
            rename = "gpuTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpu_temperatures: ::std::option::Option<Vec<f32>>,
        #[doc = "Current device skin temperatures in Celsius."]
        #[serde(
            rename = "skinTemperatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skin_temperatures: ::std::option::Option<Vec<f32>>,
    }
    impl ::google_field_selector::FieldSelector for HardwareStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HardwareStatus {
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
    pub struct IssueCommandResponse {}
    impl ::google_field_selector::FieldSelector for IssueCommandResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IssueCommandResponse {
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
    pub struct KeyedAppState {
        #[doc = "The creation time of the app state on the device."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Optionally, a machine-readable value to be read by the EMM. For example, setting values that the admin can choose to query against in the EMM console (e.g. “notify me if the battery_warning data < 10”)."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<String>,
        #[doc = "The key for the app state. Acts as a point of reference for what the app is providing state for. For example, when providing managed configuration feedback, this key could be the managed configuration key."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The time the app state was most recently updated."]
        #[serde(
            rename = "lastUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_time: ::std::option::Option<String>,
        #[doc = "Optionally, a free-form message string to explain the app state. If the state was triggered by a particular value (e.g. a managed configuration value), it should be included in the message."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "The severity of the app state."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::KeyedAppStateSeverity>,
    }
    impl ::google_field_selector::FieldSelector for KeyedAppState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyedAppState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KeyedAppStateSeverity {
        #[doc = "Error severity level. This should only be set for genuine error conditions that a management organization needs to take action to fix."]
        Error,
        #[doc = "Information severity level."]
        Info,
        #[doc = "Unspecified severity level."]
        SeverityUnspecified,
    }
    impl KeyedAppStateSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                KeyedAppStateSeverity::Error => "ERROR",
                KeyedAppStateSeverity::Info => "INFO",
                KeyedAppStateSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KeyedAppStateSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KeyedAppStateSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KeyedAppStateSeverity, ()> {
            Ok(match s {
                "ERROR" => KeyedAppStateSeverity::Error,
                "INFO" => KeyedAppStateSeverity::Info,
                "SEVERITY_UNSPECIFIED" => KeyedAppStateSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KeyedAppStateSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KeyedAppStateSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KeyedAppStateSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => KeyedAppStateSeverity::Error,
                "INFO" => KeyedAppStateSeverity::Info,
                "SEVERITY_UNSPECIFIED" => KeyedAppStateSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KeyedAppStateSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyedAppStateSeverity {
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
    pub struct KioskCustomization {
        #[doc = "Specifies whether the Settings app is allowed in kiosk mode."]
        #[serde(
            rename = "deviceSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_settings:
            ::std::option::Option<crate::schemas::KioskCustomizationDeviceSettings>,
        #[doc = "Sets the behavior of a device in kiosk mode when a user presses and holds (long-presses) the Power button."]
        #[serde(
            rename = "powerButtonActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub power_button_actions:
            ::std::option::Option<crate::schemas::KioskCustomizationPowerButtonActions>,
        #[doc = "Specifies whether system info and notifications are disabled in kiosk mode."]
        #[serde(
            rename = "statusBar",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_bar: ::std::option::Option<crate::schemas::KioskCustomizationStatusBar>,
        #[doc = "Specifies whether system error dialogs for crashed or unresponsive apps are blocked in kiosk mode. When blocked, the system will force-stop the app as if the user chooses the \"close app\" option on the UI."]
        #[serde(
            rename = "systemErrorWarnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_error_warnings:
            ::std::option::Option<crate::schemas::KioskCustomizationSystemErrorWarnings>,
        #[doc = "Specifies which navigation features are enabled (e.g. Home, Overview buttons) in kiosk mode."]
        #[serde(
            rename = "systemNavigation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_navigation:
            ::std::option::Option<crate::schemas::KioskCustomizationSystemNavigation>,
    }
    impl ::google_field_selector::FieldSelector for KioskCustomization {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomization {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KioskCustomizationDeviceSettings {
        #[doc = "Unspecified, defaults to SETTINGS_ACCESS_ALLOWED."]
        DeviceSettingsUnspecified,
        #[doc = "Access to the Settings app is allowed in kiosk mode."]
        SettingsAccessAllowed,
        #[doc = "Access to the Settings app is not allowed in kiosk mode."]
        SettingsAccessBlocked,
    }
    impl KioskCustomizationDeviceSettings {
        pub fn as_str(self) -> &'static str {
            match self {
                KioskCustomizationDeviceSettings::DeviceSettingsUnspecified => {
                    "DEVICE_SETTINGS_UNSPECIFIED"
                }
                KioskCustomizationDeviceSettings::SettingsAccessAllowed => {
                    "SETTINGS_ACCESS_ALLOWED"
                }
                KioskCustomizationDeviceSettings::SettingsAccessBlocked => {
                    "SETTINGS_ACCESS_BLOCKED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for KioskCustomizationDeviceSettings {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KioskCustomizationDeviceSettings {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KioskCustomizationDeviceSettings, ()> {
            Ok(match s {
                "DEVICE_SETTINGS_UNSPECIFIED" => {
                    KioskCustomizationDeviceSettings::DeviceSettingsUnspecified
                }
                "SETTINGS_ACCESS_ALLOWED" => {
                    KioskCustomizationDeviceSettings::SettingsAccessAllowed
                }
                "SETTINGS_ACCESS_BLOCKED" => {
                    KioskCustomizationDeviceSettings::SettingsAccessBlocked
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KioskCustomizationDeviceSettings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KioskCustomizationDeviceSettings {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KioskCustomizationDeviceSettings {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_SETTINGS_UNSPECIFIED" => {
                    KioskCustomizationDeviceSettings::DeviceSettingsUnspecified
                }
                "SETTINGS_ACCESS_ALLOWED" => {
                    KioskCustomizationDeviceSettings::SettingsAccessAllowed
                }
                "SETTINGS_ACCESS_BLOCKED" => {
                    KioskCustomizationDeviceSettings::SettingsAccessBlocked
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
    impl ::google_field_selector::FieldSelector for KioskCustomizationDeviceSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomizationDeviceSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KioskCustomizationPowerButtonActions {
        #[doc = "Unspecified, defaults to POWER_BUTTON_AVAILABLE."]
        PowerButtonActionsUnspecified,
        #[doc = "The power menu (e.g. Power off, Restart) is shown when a user long-presses the Power button of a device in kiosk mode."]
        PowerButtonAvailable,
        #[doc = "The power menu (e.g. Power off, Restart) is not shown when a user long-presses the Power button of a device in kiosk mode. Note: this may prevent users from turning off the device."]
        PowerButtonBlocked,
    }
    impl KioskCustomizationPowerButtonActions {
        pub fn as_str(self) -> &'static str {
            match self {
                KioskCustomizationPowerButtonActions::PowerButtonActionsUnspecified => {
                    "POWER_BUTTON_ACTIONS_UNSPECIFIED"
                }
                KioskCustomizationPowerButtonActions::PowerButtonAvailable => {
                    "POWER_BUTTON_AVAILABLE"
                }
                KioskCustomizationPowerButtonActions::PowerButtonBlocked => "POWER_BUTTON_BLOCKED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KioskCustomizationPowerButtonActions {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KioskCustomizationPowerButtonActions {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KioskCustomizationPowerButtonActions, ()> {
            Ok(match s {
                "POWER_BUTTON_ACTIONS_UNSPECIFIED" => {
                    KioskCustomizationPowerButtonActions::PowerButtonActionsUnspecified
                }
                "POWER_BUTTON_AVAILABLE" => {
                    KioskCustomizationPowerButtonActions::PowerButtonAvailable
                }
                "POWER_BUTTON_BLOCKED" => KioskCustomizationPowerButtonActions::PowerButtonBlocked,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KioskCustomizationPowerButtonActions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KioskCustomizationPowerButtonActions {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KioskCustomizationPowerButtonActions {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "POWER_BUTTON_ACTIONS_UNSPECIFIED" => {
                    KioskCustomizationPowerButtonActions::PowerButtonActionsUnspecified
                }
                "POWER_BUTTON_AVAILABLE" => {
                    KioskCustomizationPowerButtonActions::PowerButtonAvailable
                }
                "POWER_BUTTON_BLOCKED" => KioskCustomizationPowerButtonActions::PowerButtonBlocked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KioskCustomizationPowerButtonActions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomizationPowerButtonActions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KioskCustomizationStatusBar {
        #[doc = "System info and notifications are disabled in kiosk mode."]
        NotificationsAndSystemInfoDisabled,
        #[doc = "System info and notifications are shown on the status bar in kiosk mode.Note: For this policy to take effect, the device's home button must be enabled using kioskCustomization.systemNavigation."]
        NotificationsAndSystemInfoEnabled,
        #[doc = "Unspecified, defaults to INFO_AND_NOTIFICATIONS_DISABLED."]
        StatusBarUnspecified,
        #[doc = "Only system info is shown on the status bar."]
        SystemInfoOnly,
    }
    impl KioskCustomizationStatusBar {
        pub fn as_str(self) -> &'static str {
            match self {
                KioskCustomizationStatusBar::NotificationsAndSystemInfoDisabled => {
                    "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED"
                }
                KioskCustomizationStatusBar::NotificationsAndSystemInfoEnabled => {
                    "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED"
                }
                KioskCustomizationStatusBar::StatusBarUnspecified => "STATUS_BAR_UNSPECIFIED",
                KioskCustomizationStatusBar::SystemInfoOnly => "SYSTEM_INFO_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KioskCustomizationStatusBar {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KioskCustomizationStatusBar {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KioskCustomizationStatusBar, ()> {
            Ok(match s {
                "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED" => {
                    KioskCustomizationStatusBar::NotificationsAndSystemInfoDisabled
                }
                "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED" => {
                    KioskCustomizationStatusBar::NotificationsAndSystemInfoEnabled
                }
                "STATUS_BAR_UNSPECIFIED" => KioskCustomizationStatusBar::StatusBarUnspecified,
                "SYSTEM_INFO_ONLY" => KioskCustomizationStatusBar::SystemInfoOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KioskCustomizationStatusBar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KioskCustomizationStatusBar {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KioskCustomizationStatusBar {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED" => {
                    KioskCustomizationStatusBar::NotificationsAndSystemInfoDisabled
                }
                "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED" => {
                    KioskCustomizationStatusBar::NotificationsAndSystemInfoEnabled
                }
                "STATUS_BAR_UNSPECIFIED" => KioskCustomizationStatusBar::StatusBarUnspecified,
                "SYSTEM_INFO_ONLY" => KioskCustomizationStatusBar::SystemInfoOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KioskCustomizationStatusBar {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomizationStatusBar {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KioskCustomizationSystemErrorWarnings {
        #[doc = "All system error dialogs such as crash and app not responding (ANR) are displayed."]
        ErrorAndWarningsEnabled,
        #[doc = "All system error dialogs, such as crash and app not responding (ANR) are blocked. When blocked, the system force-stops the app as if the user closes the app from the UI."]
        ErrorAndWarningsMuted,
        #[doc = "Unspecified, defaults to ERROR_AND_WARNINGS_MUTED."]
        SystemErrorWarningsUnspecified,
    }
    impl KioskCustomizationSystemErrorWarnings {
        pub fn as_str(self) -> &'static str {
            match self {
                KioskCustomizationSystemErrorWarnings::ErrorAndWarningsEnabled => {
                    "ERROR_AND_WARNINGS_ENABLED"
                }
                KioskCustomizationSystemErrorWarnings::ErrorAndWarningsMuted => {
                    "ERROR_AND_WARNINGS_MUTED"
                }
                KioskCustomizationSystemErrorWarnings::SystemErrorWarningsUnspecified => {
                    "SYSTEM_ERROR_WARNINGS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for KioskCustomizationSystemErrorWarnings {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KioskCustomizationSystemErrorWarnings {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KioskCustomizationSystemErrorWarnings, ()> {
            Ok(match s {
                "ERROR_AND_WARNINGS_ENABLED" => {
                    KioskCustomizationSystemErrorWarnings::ErrorAndWarningsEnabled
                }
                "ERROR_AND_WARNINGS_MUTED" => {
                    KioskCustomizationSystemErrorWarnings::ErrorAndWarningsMuted
                }
                "SYSTEM_ERROR_WARNINGS_UNSPECIFIED" => {
                    KioskCustomizationSystemErrorWarnings::SystemErrorWarningsUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KioskCustomizationSystemErrorWarnings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KioskCustomizationSystemErrorWarnings {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KioskCustomizationSystemErrorWarnings {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR_AND_WARNINGS_ENABLED" => {
                    KioskCustomizationSystemErrorWarnings::ErrorAndWarningsEnabled
                }
                "ERROR_AND_WARNINGS_MUTED" => {
                    KioskCustomizationSystemErrorWarnings::ErrorAndWarningsMuted
                }
                "SYSTEM_ERROR_WARNINGS_UNSPECIFIED" => {
                    KioskCustomizationSystemErrorWarnings::SystemErrorWarningsUnspecified
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
    impl ::google_field_selector::FieldSelector for KioskCustomizationSystemErrorWarnings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomizationSystemErrorWarnings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KioskCustomizationSystemNavigation {
        #[doc = "Only the home button is enabled."]
        HomeButtonOnly,
        #[doc = "The home and Overview buttons are not accessible."]
        NavigationDisabled,
        #[doc = "Home and overview buttons are enabled."]
        NavigationEnabled,
        #[doc = "Unspecified, defaults to NAVIGATION_DISABLED."]
        SystemNavigationUnspecified,
    }
    impl KioskCustomizationSystemNavigation {
        pub fn as_str(self) -> &'static str {
            match self {
                KioskCustomizationSystemNavigation::HomeButtonOnly => "HOME_BUTTON_ONLY",
                KioskCustomizationSystemNavigation::NavigationDisabled => "NAVIGATION_DISABLED",
                KioskCustomizationSystemNavigation::NavigationEnabled => "NAVIGATION_ENABLED",
                KioskCustomizationSystemNavigation::SystemNavigationUnspecified => {
                    "SYSTEM_NAVIGATION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for KioskCustomizationSystemNavigation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KioskCustomizationSystemNavigation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KioskCustomizationSystemNavigation, ()> {
            Ok(match s {
                "HOME_BUTTON_ONLY" => KioskCustomizationSystemNavigation::HomeButtonOnly,
                "NAVIGATION_DISABLED" => KioskCustomizationSystemNavigation::NavigationDisabled,
                "NAVIGATION_ENABLED" => KioskCustomizationSystemNavigation::NavigationEnabled,
                "SYSTEM_NAVIGATION_UNSPECIFIED" => {
                    KioskCustomizationSystemNavigation::SystemNavigationUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KioskCustomizationSystemNavigation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KioskCustomizationSystemNavigation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KioskCustomizationSystemNavigation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HOME_BUTTON_ONLY" => KioskCustomizationSystemNavigation::HomeButtonOnly,
                "NAVIGATION_DISABLED" => KioskCustomizationSystemNavigation::NavigationDisabled,
                "NAVIGATION_ENABLED" => KioskCustomizationSystemNavigation::NavigationEnabled,
                "SYSTEM_NAVIGATION_UNSPECIFIED" => {
                    KioskCustomizationSystemNavigation::SystemNavigationUnspecified
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
    impl ::google_field_selector::FieldSelector for KioskCustomizationSystemNavigation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KioskCustomizationSystemNavigation {
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
    pub struct LaunchAppAction {
        #[doc = "Package name of app to be launched"]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LaunchAppAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LaunchAppAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListDevicesResponse {
        #[doc = "The list of devices."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices: ::std::option::Option<Vec<crate::schemas::Device>>,
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListDevicesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDevicesResponse {
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
    pub struct ListEnterprisesResponse {
        #[doc = "The list of enterprises."]
        #[serde(
            rename = "enterprises",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enterprises: ::std::option::Option<Vec<crate::schemas::Enterprise>>,
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListEnterprisesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEnterprisesResponse {
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListPoliciesResponse {
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of policies."]
        #[serde(
            rename = "policies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policies: ::std::option::Option<Vec<crate::schemas::Policy>>,
    }
    impl ::google_field_selector::FieldSelector for ListPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPoliciesResponse {
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
    pub struct ListWebAppsResponse {
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of web apps."]
        #[serde(
            rename = "webApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_apps: ::std::option::Option<Vec<crate::schemas::WebApp>>,
    }
    impl ::google_field_selector::FieldSelector for ListWebAppsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListWebAppsResponse {
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
    pub struct ManagedConfigurationTemplate {
        #[doc = "Optional, a map containing configuration variables defined for the configuration."]
        #[serde(
            rename = "configurationVariables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configuration_variables:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The ID of the managed configurations template."]
        #[serde(
            rename = "templateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedConfigurationTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedConfigurationTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ManagedProperty {
        #[doc = "The default value of the property. BUNDLE_ARRAY properties don't have a default value."]
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_value: ::std::option::Option<::serde_json::Value>,
        #[doc = "A longer description of the property, providing more detail of what it affects. Localized."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "For CHOICE or MULTISELECT properties, the list of possible entries."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<Vec<crate::schemas::ManagedPropertyEntry>>,
        #[doc = "The unique key that the app uses to identify the property, e.g. \"com.google.android.gm.fieldname\"."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "For BUNDLE_ARRAY properties, the list of nested properties. A BUNDLE_ARRAY property is at most two levels deep."]
        #[serde(
            rename = "nestedProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nested_properties: ::std::option::Option<Vec<crate::schemas::ManagedProperty>>,
        #[doc = "The type of the property."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ManagedPropertyType>,
        #[doc = "The name of the property. Localized."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedProperty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedPropertyType {
        #[doc = "A property of boolean type."]
        Bool,
        #[doc = "A bundle of properties"]
        Bundle,
        #[doc = "An array of property bundles."]
        BundleArray,
        #[doc = "A choice of one item from a set."]
        Choice,
        #[doc = "A hidden restriction of string type (the default value can be used to pass along information that can't be modified, such as a version code)."]
        Hidden,
        #[doc = "A property of integer type."]
        Integer,
        #[doc = "Not used."]
        ManagedPropertyTypeUnspecified,
        #[doc = "A choice of multiple items from a set."]
        Multiselect,
        #[doc = "A property of string type."]
        String,
    }
    impl ManagedPropertyType {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedPropertyType::Bool => "BOOL",
                ManagedPropertyType::Bundle => "BUNDLE",
                ManagedPropertyType::BundleArray => "BUNDLE_ARRAY",
                ManagedPropertyType::Choice => "CHOICE",
                ManagedPropertyType::Hidden => "HIDDEN",
                ManagedPropertyType::Integer => "INTEGER",
                ManagedPropertyType::ManagedPropertyTypeUnspecified => {
                    "MANAGED_PROPERTY_TYPE_UNSPECIFIED"
                }
                ManagedPropertyType::Multiselect => "MULTISELECT",
                ManagedPropertyType::String => "STRING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedPropertyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedPropertyType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedPropertyType, ()> {
            Ok(match s {
                "BOOL" => ManagedPropertyType::Bool,
                "BUNDLE" => ManagedPropertyType::Bundle,
                "BUNDLE_ARRAY" => ManagedPropertyType::BundleArray,
                "CHOICE" => ManagedPropertyType::Choice,
                "HIDDEN" => ManagedPropertyType::Hidden,
                "INTEGER" => ManagedPropertyType::Integer,
                "MANAGED_PROPERTY_TYPE_UNSPECIFIED" => {
                    ManagedPropertyType::ManagedPropertyTypeUnspecified
                }
                "MULTISELECT" => ManagedPropertyType::Multiselect,
                "STRING" => ManagedPropertyType::String,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedPropertyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedPropertyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedPropertyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => ManagedPropertyType::Bool,
                "BUNDLE" => ManagedPropertyType::Bundle,
                "BUNDLE_ARRAY" => ManagedPropertyType::BundleArray,
                "CHOICE" => ManagedPropertyType::Choice,
                "HIDDEN" => ManagedPropertyType::Hidden,
                "INTEGER" => ManagedPropertyType::Integer,
                "MANAGED_PROPERTY_TYPE_UNSPECIFIED" => {
                    ManagedPropertyType::ManagedPropertyTypeUnspecified
                }
                "MULTISELECT" => ManagedPropertyType::Multiselect,
                "STRING" => ManagedPropertyType::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManagedPropertyType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedPropertyType {
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
    pub struct ManagedPropertyEntry {
        #[doc = "The human-readable name of the value. Localized."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The machine-readable value of the entry, which should be used in the configuration. Not localized."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedPropertyEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedPropertyEntry {
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
    pub struct MemoryEvent {
        #[doc = "The number of free bytes in the medium, or for EXTERNAL_STORAGE_DETECTED, the total capacity in bytes of the storage medium."]
        #[serde(
            rename = "byteCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub byte_count: ::std::option::Option<i64>,
        #[doc = "The creation time of the event."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Event type."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type: ::std::option::Option<crate::schemas::MemoryEventEventType>,
    }
    impl ::google_field_selector::FieldSelector for MemoryEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MemoryEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MemoryEventEventType {
        #[doc = "A new external storage medium was detected. The reported byte count is the total capacity of the storage medium."]
        ExternalStorageDetected,
        #[doc = "Free space in an external storage medium was measured."]
        ExternalStorageMeasured,
        #[doc = "An external storage medium was removed. The reported byte count is zero."]
        ExternalStorageRemoved,
        #[doc = "Free space in internal storage was measured."]
        InternalStorageMeasured,
        #[doc = "Unspecified. No events have this type."]
        MemoryEventTypeUnspecified,
        #[doc = "Free space in RAM was measured."]
        RamMeasured,
    }
    impl MemoryEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                MemoryEventEventType::ExternalStorageDetected => "EXTERNAL_STORAGE_DETECTED",
                MemoryEventEventType::ExternalStorageMeasured => "EXTERNAL_STORAGE_MEASURED",
                MemoryEventEventType::ExternalStorageRemoved => "EXTERNAL_STORAGE_REMOVED",
                MemoryEventEventType::InternalStorageMeasured => "INTERNAL_STORAGE_MEASURED",
                MemoryEventEventType::MemoryEventTypeUnspecified => "MEMORY_EVENT_TYPE_UNSPECIFIED",
                MemoryEventEventType::RamMeasured => "RAM_MEASURED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MemoryEventEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MemoryEventEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MemoryEventEventType, ()> {
            Ok(match s {
                "EXTERNAL_STORAGE_DETECTED" => MemoryEventEventType::ExternalStorageDetected,
                "EXTERNAL_STORAGE_MEASURED" => MemoryEventEventType::ExternalStorageMeasured,
                "EXTERNAL_STORAGE_REMOVED" => MemoryEventEventType::ExternalStorageRemoved,
                "INTERNAL_STORAGE_MEASURED" => MemoryEventEventType::InternalStorageMeasured,
                "MEMORY_EVENT_TYPE_UNSPECIFIED" => MemoryEventEventType::MemoryEventTypeUnspecified,
                "RAM_MEASURED" => MemoryEventEventType::RamMeasured,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MemoryEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MemoryEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MemoryEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL_STORAGE_DETECTED" => MemoryEventEventType::ExternalStorageDetected,
                "EXTERNAL_STORAGE_MEASURED" => MemoryEventEventType::ExternalStorageMeasured,
                "EXTERNAL_STORAGE_REMOVED" => MemoryEventEventType::ExternalStorageRemoved,
                "INTERNAL_STORAGE_MEASURED" => MemoryEventEventType::InternalStorageMeasured,
                "MEMORY_EVENT_TYPE_UNSPECIFIED" => MemoryEventEventType::MemoryEventTypeUnspecified,
                "RAM_MEASURED" => MemoryEventEventType::RamMeasured,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MemoryEventEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MemoryEventEventType {
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
    pub struct MemoryInfo {
        #[doc = "Total internal storage on device in bytes."]
        #[serde(
            rename = "totalInternalStorage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_internal_storage: ::std::option::Option<i64>,
        #[doc = "Total RAM on device in bytes."]
        #[serde(
            rename = "totalRam",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_ram: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MemoryInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MemoryInfo {
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
    pub struct NetworkInfo {
        #[doc = "IMEI number of the GSM device. For example, A1000031212."]
        #[serde(
            rename = "imei",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub imei: ::std::option::Option<String>,
        #[doc = "MEID number of the CDMA device. For example, A00000292788E1."]
        #[serde(
            rename = "meid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub meid: ::std::option::Option<String>,
        #[doc = "Alphabetic name of current registered operator. For example, Vodafone."]
        #[serde(
            rename = "networkOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_operator_name: ::std::option::Option<String>,
        #[doc = "Provides telephony information associated with each SIM card on the device. Only supported on fully managed devices starting from Android API level 23."]
        #[serde(
            rename = "telephonyInfos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub telephony_infos: ::std::option::Option<Vec<crate::schemas::TelephonyInfo>>,
        #[doc = "Wi-Fi MAC address of the device. For example, 7c:11:11:11:11:11."]
        #[serde(
            rename = "wifiMacAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wifi_mac_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NetworkInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct NonComplianceDetail {
        #[doc = "If the policy setting could not be applied, the current value of the setting on the device."]
        #[serde(
            rename = "currentValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_value: ::std::option::Option<::serde_json::Value>,
        #[doc = "For settings with nested fields, if a particular nested field is out of compliance, this specifies the full path to the offending field. The path is formatted in the same way the policy JSON field would be referenced in JavaScript, that is: 1) For object-typed fields, the field name is followed by a dot then by a subfield name. 2) For array-typed fields, the field name is followed by the array index enclosed in brackets. For example, to indicate a problem with the url field in the externalData field in the 3rd application, the path would be applications[2].externalData.url"]
        #[serde(
            rename = "fieldPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_path: ::std::option::Option<String>,
        #[doc = "If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated."]
        #[serde(
            rename = "installationFailureReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installation_failure_reason:
            ::std::option::Option<crate::schemas::NonComplianceDetailInstallationFailureReason>,
        #[doc = "The reason the device is not in compliance with the setting."]
        #[serde(
            rename = "nonComplianceReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_reason:
            ::std::option::Option<crate::schemas::NonComplianceDetailNonComplianceReason>,
        #[doc = "The package name indicating which app is out of compliance, if applicable."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy field."]
        #[serde(
            rename = "settingName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub setting_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NonComplianceDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonComplianceDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonComplianceDetailInstallationFailureReason {
        #[doc = "The installation is still in progress."]
        InProgress,
        #[doc = "An unknown condition is preventing the app from being installed. Some potential reasons are that the device doesn't have enough storage, the device network connection is unreliable, or the installation is taking longer than expected. The installation will be retried automatically."]
        InstallationFailureReasonUnknown,
        #[doc = "This value is disallowed."]
        InstallationFailureReasonUnspecified,
        #[doc = "There are no licenses available to assign to the user."]
        NoLicensesRemaining,
        #[doc = "The app has not been approved by the admin."]
        NotApproved,
        #[doc = "The app is not available in the user's country."]
        NotAvailableInCountry,
        #[doc = "The app is incompatible with the device."]
        NotCompatibleWithDevice,
        #[doc = "The enterprise is no longer enrolled with Managed Google Play or the admin has not accepted the latest Managed Google Play Terms of Service."]
        NotEnrolled,
        #[doc = "The app was not found in Play."]
        NotFound,
        #[doc = "The app has new permissions that have not been accepted by the admin."]
        PermissionsNotAccepted,
        #[doc = "The user is no longer valid. The user may have been deleted or disabled."]
        UserInvalid,
    }
    impl NonComplianceDetailInstallationFailureReason {
        pub fn as_str(self) -> &'static str {
            match self { NonComplianceDetailInstallationFailureReason :: InProgress => "IN_PROGRESS" , NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnknown => "INSTALLATION_FAILURE_REASON_UNKNOWN" , NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnspecified => "INSTALLATION_FAILURE_REASON_UNSPECIFIED" , NonComplianceDetailInstallationFailureReason :: NoLicensesRemaining => "NO_LICENSES_REMAINING" , NonComplianceDetailInstallationFailureReason :: NotApproved => "NOT_APPROVED" , NonComplianceDetailInstallationFailureReason :: NotAvailableInCountry => "NOT_AVAILABLE_IN_COUNTRY" , NonComplianceDetailInstallationFailureReason :: NotCompatibleWithDevice => "NOT_COMPATIBLE_WITH_DEVICE" , NonComplianceDetailInstallationFailureReason :: NotEnrolled => "NOT_ENROLLED" , NonComplianceDetailInstallationFailureReason :: NotFound => "NOT_FOUND" , NonComplianceDetailInstallationFailureReason :: PermissionsNotAccepted => "PERMISSIONS_NOT_ACCEPTED" , NonComplianceDetailInstallationFailureReason :: UserInvalid => "USER_INVALID" , }
        }
    }
    impl ::std::convert::AsRef<str> for NonComplianceDetailInstallationFailureReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NonComplianceDetailInstallationFailureReason {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<NonComplianceDetailInstallationFailureReason, ()> {
            Ok (match s { "IN_PROGRESS" => NonComplianceDetailInstallationFailureReason :: InProgress , "INSTALLATION_FAILURE_REASON_UNKNOWN" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnknown , "INSTALLATION_FAILURE_REASON_UNSPECIFIED" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnspecified , "NO_LICENSES_REMAINING" => NonComplianceDetailInstallationFailureReason :: NoLicensesRemaining , "NOT_APPROVED" => NonComplianceDetailInstallationFailureReason :: NotApproved , "NOT_AVAILABLE_IN_COUNTRY" => NonComplianceDetailInstallationFailureReason :: NotAvailableInCountry , "NOT_COMPATIBLE_WITH_DEVICE" => NonComplianceDetailInstallationFailureReason :: NotCompatibleWithDevice , "NOT_ENROLLED" => NonComplianceDetailInstallationFailureReason :: NotEnrolled , "NOT_FOUND" => NonComplianceDetailInstallationFailureReason :: NotFound , "PERMISSIONS_NOT_ACCEPTED" => NonComplianceDetailInstallationFailureReason :: PermissionsNotAccepted , "USER_INVALID" => NonComplianceDetailInstallationFailureReason :: UserInvalid , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailInstallationFailureReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailInstallationFailureReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailInstallationFailureReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "IN_PROGRESS" => NonComplianceDetailInstallationFailureReason :: InProgress , "INSTALLATION_FAILURE_REASON_UNKNOWN" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnknown , "INSTALLATION_FAILURE_REASON_UNSPECIFIED" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnspecified , "NO_LICENSES_REMAINING" => NonComplianceDetailInstallationFailureReason :: NoLicensesRemaining , "NOT_APPROVED" => NonComplianceDetailInstallationFailureReason :: NotApproved , "NOT_AVAILABLE_IN_COUNTRY" => NonComplianceDetailInstallationFailureReason :: NotAvailableInCountry , "NOT_COMPATIBLE_WITH_DEVICE" => NonComplianceDetailInstallationFailureReason :: NotCompatibleWithDevice , "NOT_ENROLLED" => NonComplianceDetailInstallationFailureReason :: NotEnrolled , "NOT_FOUND" => NonComplianceDetailInstallationFailureReason :: NotFound , "PERMISSIONS_NOT_ACCEPTED" => NonComplianceDetailInstallationFailureReason :: PermissionsNotAccepted , "USER_INVALID" => NonComplianceDetailInstallationFailureReason :: UserInvalid , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for NonComplianceDetailInstallationFailureReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonComplianceDetailInstallationFailureReason {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonComplianceDetailNonComplianceReason {
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
    }
    impl NonComplianceDetailNonComplianceReason {
        pub fn as_str(self) -> &'static str {
            match self {
                NonComplianceDetailNonComplianceReason::ApiLevel => "API_LEVEL",
                NonComplianceDetailNonComplianceReason::AppIncompatible => "APP_INCOMPATIBLE",
                NonComplianceDetailNonComplianceReason::AppInstalled => "APP_INSTALLED",
                NonComplianceDetailNonComplianceReason::AppNotInstalled => "APP_NOT_INSTALLED",
                NonComplianceDetailNonComplianceReason::AppNotUpdated => "APP_NOT_UPDATED",
                NonComplianceDetailNonComplianceReason::InvalidValue => "INVALID_VALUE",
                NonComplianceDetailNonComplianceReason::ManagementMode => "MANAGEMENT_MODE",
                NonComplianceDetailNonComplianceReason::NonComplianceReasonUnspecified => {
                    "NON_COMPLIANCE_REASON_UNSPECIFIED"
                }
                NonComplianceDetailNonComplianceReason::Pending => "PENDING",
                NonComplianceDetailNonComplianceReason::Unsupported => "UNSUPPORTED",
                NonComplianceDetailNonComplianceReason::UserAction => "USER_ACTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NonComplianceDetailNonComplianceReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NonComplianceDetailNonComplianceReason {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NonComplianceDetailNonComplianceReason, ()> {
            Ok(match s {
                "API_LEVEL" => NonComplianceDetailNonComplianceReason::ApiLevel,
                "APP_INCOMPATIBLE" => NonComplianceDetailNonComplianceReason::AppIncompatible,
                "APP_INSTALLED" => NonComplianceDetailNonComplianceReason::AppInstalled,
                "APP_NOT_INSTALLED" => NonComplianceDetailNonComplianceReason::AppNotInstalled,
                "APP_NOT_UPDATED" => NonComplianceDetailNonComplianceReason::AppNotUpdated,
                "INVALID_VALUE" => NonComplianceDetailNonComplianceReason::InvalidValue,
                "MANAGEMENT_MODE" => NonComplianceDetailNonComplianceReason::ManagementMode,
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailNonComplianceReason::NonComplianceReasonUnspecified
                }
                "PENDING" => NonComplianceDetailNonComplianceReason::Pending,
                "UNSUPPORTED" => NonComplianceDetailNonComplianceReason::Unsupported,
                "USER_ACTION" => NonComplianceDetailNonComplianceReason::UserAction,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailNonComplianceReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailNonComplianceReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailNonComplianceReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_LEVEL" => NonComplianceDetailNonComplianceReason::ApiLevel,
                "APP_INCOMPATIBLE" => NonComplianceDetailNonComplianceReason::AppIncompatible,
                "APP_INSTALLED" => NonComplianceDetailNonComplianceReason::AppInstalled,
                "APP_NOT_INSTALLED" => NonComplianceDetailNonComplianceReason::AppNotInstalled,
                "APP_NOT_UPDATED" => NonComplianceDetailNonComplianceReason::AppNotUpdated,
                "INVALID_VALUE" => NonComplianceDetailNonComplianceReason::InvalidValue,
                "MANAGEMENT_MODE" => NonComplianceDetailNonComplianceReason::ManagementMode,
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailNonComplianceReason::NonComplianceReasonUnspecified
                }
                "PENDING" => NonComplianceDetailNonComplianceReason::Pending,
                "UNSUPPORTED" => NonComplianceDetailNonComplianceReason::Unsupported,
                "USER_ACTION" => NonComplianceDetailNonComplianceReason::UserAction,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NonComplianceDetailNonComplianceReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonComplianceDetailNonComplianceReason {
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
    pub struct NonComplianceDetailCondition {
        #[doc = "The reason the device is not in compliance with the setting. If not set, then this condition matches any reason."]
        #[serde(
            rename = "nonComplianceReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_reason:
            ::std::option::Option<crate::schemas::NonComplianceDetailConditionNonComplianceReason>,
        #[doc = "The package name of the app that's out of compliance. If not set, then this condition matches any package name."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy field. If not set, then this condition matches any setting name."]
        #[serde(
            rename = "settingName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub setting_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NonComplianceDetailCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonComplianceDetailCondition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonComplianceDetailConditionNonComplianceReason {
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
    }
    impl NonComplianceDetailConditionNonComplianceReason {
        pub fn as_str(self) -> &'static str {
            match self {
                NonComplianceDetailConditionNonComplianceReason::ApiLevel => "API_LEVEL",
                NonComplianceDetailConditionNonComplianceReason::AppIncompatible => {
                    "APP_INCOMPATIBLE"
                }
                NonComplianceDetailConditionNonComplianceReason::AppInstalled => "APP_INSTALLED",
                NonComplianceDetailConditionNonComplianceReason::AppNotInstalled => {
                    "APP_NOT_INSTALLED"
                }
                NonComplianceDetailConditionNonComplianceReason::AppNotUpdated => "APP_NOT_UPDATED",
                NonComplianceDetailConditionNonComplianceReason::InvalidValue => "INVALID_VALUE",
                NonComplianceDetailConditionNonComplianceReason::ManagementMode => {
                    "MANAGEMENT_MODE"
                }
                NonComplianceDetailConditionNonComplianceReason::NonComplianceReasonUnspecified => {
                    "NON_COMPLIANCE_REASON_UNSPECIFIED"
                }
                NonComplianceDetailConditionNonComplianceReason::Pending => "PENDING",
                NonComplianceDetailConditionNonComplianceReason::Unsupported => "UNSUPPORTED",
                NonComplianceDetailConditionNonComplianceReason::UserAction => "USER_ACTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NonComplianceDetailConditionNonComplianceReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NonComplianceDetailConditionNonComplianceReason {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<NonComplianceDetailConditionNonComplianceReason, ()> {
            Ok(match s {
                "API_LEVEL" => NonComplianceDetailConditionNonComplianceReason::ApiLevel,
                "APP_INCOMPATIBLE" => {
                    NonComplianceDetailConditionNonComplianceReason::AppIncompatible
                }
                "APP_INSTALLED" => NonComplianceDetailConditionNonComplianceReason::AppInstalled,
                "APP_NOT_INSTALLED" => {
                    NonComplianceDetailConditionNonComplianceReason::AppNotInstalled
                }
                "APP_NOT_UPDATED" => NonComplianceDetailConditionNonComplianceReason::AppNotUpdated,
                "INVALID_VALUE" => NonComplianceDetailConditionNonComplianceReason::InvalidValue,
                "MANAGEMENT_MODE" => {
                    NonComplianceDetailConditionNonComplianceReason::ManagementMode
                }
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailConditionNonComplianceReason::NonComplianceReasonUnspecified
                }
                "PENDING" => NonComplianceDetailConditionNonComplianceReason::Pending,
                "UNSUPPORTED" => NonComplianceDetailConditionNonComplianceReason::Unsupported,
                "USER_ACTION" => NonComplianceDetailConditionNonComplianceReason::UserAction,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailConditionNonComplianceReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailConditionNonComplianceReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailConditionNonComplianceReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_LEVEL" => NonComplianceDetailConditionNonComplianceReason::ApiLevel,
                "APP_INCOMPATIBLE" => {
                    NonComplianceDetailConditionNonComplianceReason::AppIncompatible
                }
                "APP_INSTALLED" => NonComplianceDetailConditionNonComplianceReason::AppInstalled,
                "APP_NOT_INSTALLED" => {
                    NonComplianceDetailConditionNonComplianceReason::AppNotInstalled
                }
                "APP_NOT_UPDATED" => NonComplianceDetailConditionNonComplianceReason::AppNotUpdated,
                "INVALID_VALUE" => NonComplianceDetailConditionNonComplianceReason::InvalidValue,
                "MANAGEMENT_MODE" => {
                    NonComplianceDetailConditionNonComplianceReason::ManagementMode
                }
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailConditionNonComplianceReason::NonComplianceReasonUnspecified
                }
                "PENDING" => NonComplianceDetailConditionNonComplianceReason::Pending,
                "UNSUPPORTED" => NonComplianceDetailConditionNonComplianceReason::Unsupported,
                "USER_ACTION" => NonComplianceDetailConditionNonComplianceReason::UserAction,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NonComplianceDetailConditionNonComplianceReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonComplianceDetailConditionNonComplianceReason {
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
    pub struct OncCertificateProvider {
        #[doc = "This feature is not generally available."]
        #[serde(
            rename = "certificateReferences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub certificate_references: ::std::option::Option<Vec<String>>,
        #[doc = "This feature is not generally available."]
        #[serde(
            rename = "contentProviderEndpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_provider_endpoint:
            ::std::option::Option<crate::schemas::ContentProviderEndpoint>,
    }
    impl ::google_field_selector::FieldSelector for OncCertificateProvider {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OncCertificateProvider {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
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
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
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
    pub struct PackageNameList {
        #[doc = "A list of package names."]
        #[serde(
            rename = "packageNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for PackageNameList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageNameList {
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
    pub struct PasswordRequirements {
        #[doc = "Number of incorrect device-unlock passwords that can be entered before a device is wiped. A value of 0 means there is no restriction."]
        #[serde(
            rename = "maximumFailedPasswordsForWipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum_failed_passwords_for_wipe: ::std::option::Option<i32>,
        #[doc = "Password expiration timeout."]
        #[serde(
            rename = "passwordExpirationTimeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_expiration_timeout: ::std::option::Option<String>,
        #[doc = "The length of the password history. After setting this field, the user won't be able to enter a new password that is the same as any password in the history. A value of 0 means there is no restriction."]
        #[serde(
            rename = "passwordHistoryLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_history_length: ::std::option::Option<i32>,
        #[doc = "The minimum allowed password length. A value of 0 means there is no restriction. Only enforced when password_quality is NUMERIC, NUMERIC_COMPLEX, ALPHABETIC, ALPHANUMERIC, or COMPLEX."]
        #[serde(
            rename = "passwordMinimumLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_length: ::std::option::Option<i32>,
        #[doc = "Minimum number of letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumLetters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_letters: ::std::option::Option<i32>,
        #[doc = "Minimum number of lower case letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumLowerCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_lower_case: ::std::option::Option<i32>,
        #[doc = "Minimum number of non-letter characters (numerical digits or symbols) required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumNonLetter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_non_letter: ::std::option::Option<i32>,
        #[doc = "Minimum number of numerical digits required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumNumeric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_numeric: ::std::option::Option<i32>,
        #[doc = "Minimum number of symbols required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumSymbols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_symbols: ::std::option::Option<i32>,
        #[doc = "Minimum number of upper case letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(
            rename = "passwordMinimumUpperCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_minimum_upper_case: ::std::option::Option<i32>,
        #[doc = "The required password quality."]
        #[serde(
            rename = "passwordQuality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_quality:
            ::std::option::Option<crate::schemas::PasswordRequirementsPasswordQuality>,
        #[doc = "The scope that the password requirement applies to."]
        #[serde(
            rename = "passwordScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_scope:
            ::std::option::Option<crate::schemas::PasswordRequirementsPasswordScope>,
        #[doc = "The length of time after a device or work profile is unlocked using a strong form of authentication (password, PIN, pattern) that it can be unlocked using any other authentication method (e.g. fingerprint, trust agents, face). After the specified time period elapses, only strong forms of authentication can be used to unlock the device or work profile."]
        #[serde(
            rename = "requirePasswordUnlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_password_unlock:
            ::std::option::Option<crate::schemas::PasswordRequirementsRequirePasswordUnlock>,
    }
    impl ::google_field_selector::FieldSelector for PasswordRequirements {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasswordRequirements {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PasswordRequirementsPasswordQuality {
        #[doc = "The password must contain alphabetic (or symbol) characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. See PasswordQuality for details."]
        Alphabetic,
        #[doc = "The password must contain both numeric and alphabetic (or symbol) characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. See PasswordQuality for details."]
        Alphanumeric,
        #[doc = "The device must be secured with a low-security biometric recognition technology, at minimum. This includes technologies that can recognize the identity of an individual that are roughly equivalent to a 3-digit PIN (false detection is less than 1 in 1,000).This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_LOW for application. See PasswordQuality for details."]
        BiometricWeak,
        #[doc = "The password must meet the minimum requirements specified in passwordMinimumLength, passwordMinimumLetters, passwordMinimumSymbols, etc. For example, if passwordMinimumSymbols is 2, the password must contain at least two symbols.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. In this case, the requirements in passwordMinimumLength, passwordMinimumLetters, passwordMinimumSymbols, etc are not applied. See PasswordQuality for details."]
        Complex,
        #[doc = "Define the high password complexity band as:On Android 12 and above: PIN with no repeating (4444) or ordered (1234, 4321, 2468) sequences, length at least 8 alphabetic, length at least 6 alphanumeric, length at least 6This sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details."]
        ComplexityHigh,
        #[doc = "Define the low password complexity band as: pattern PIN with repeating (4444) or ordered (1234, 4321, 2468) sequencesThis sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details."]
        ComplexityLow,
        #[doc = "Define the medium password complexity band as: PIN with no repeating (4444) or ordered (1234, 4321, 2468) sequences, length at least 4 alphabetic, length at least 4 alphanumeric, length at least 4This sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details."]
        ComplexityMedium,
        #[doc = "The password must contain numeric characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_MEDIUM for application. See PasswordQuality for details."]
        Numeric,
        #[doc = "The password must contain numeric characters with no repeating (4444) or ordered (1234, 4321, 2468) sequences.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_MEDIUM for application. See PasswordQuality for details."]
        NumericComplex,
        #[doc = "There are no password requirements."]
        PasswordQualityUnspecified,
        #[doc = "A password is required, but there are no restrictions on what the password must contain.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_LOW for application. See PasswordQuality for details."]
        Something,
    }
    impl PasswordRequirementsPasswordQuality {
        pub fn as_str(self) -> &'static str {
            match self {
                PasswordRequirementsPasswordQuality::Alphabetic => "ALPHABETIC",
                PasswordRequirementsPasswordQuality::Alphanumeric => "ALPHANUMERIC",
                PasswordRequirementsPasswordQuality::BiometricWeak => "BIOMETRIC_WEAK",
                PasswordRequirementsPasswordQuality::Complex => "COMPLEX",
                PasswordRequirementsPasswordQuality::ComplexityHigh => "COMPLEXITY_HIGH",
                PasswordRequirementsPasswordQuality::ComplexityLow => "COMPLEXITY_LOW",
                PasswordRequirementsPasswordQuality::ComplexityMedium => "COMPLEXITY_MEDIUM",
                PasswordRequirementsPasswordQuality::Numeric => "NUMERIC",
                PasswordRequirementsPasswordQuality::NumericComplex => "NUMERIC_COMPLEX",
                PasswordRequirementsPasswordQuality::PasswordQualityUnspecified => {
                    "PASSWORD_QUALITY_UNSPECIFIED"
                }
                PasswordRequirementsPasswordQuality::Something => "SOMETHING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PasswordRequirementsPasswordQuality {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PasswordRequirementsPasswordQuality {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PasswordRequirementsPasswordQuality, ()> {
            Ok(match s {
                "ALPHABETIC" => PasswordRequirementsPasswordQuality::Alphabetic,
                "ALPHANUMERIC" => PasswordRequirementsPasswordQuality::Alphanumeric,
                "BIOMETRIC_WEAK" => PasswordRequirementsPasswordQuality::BiometricWeak,
                "COMPLEX" => PasswordRequirementsPasswordQuality::Complex,
                "COMPLEXITY_HIGH" => PasswordRequirementsPasswordQuality::ComplexityHigh,
                "COMPLEXITY_LOW" => PasswordRequirementsPasswordQuality::ComplexityLow,
                "COMPLEXITY_MEDIUM" => PasswordRequirementsPasswordQuality::ComplexityMedium,
                "NUMERIC" => PasswordRequirementsPasswordQuality::Numeric,
                "NUMERIC_COMPLEX" => PasswordRequirementsPasswordQuality::NumericComplex,
                "PASSWORD_QUALITY_UNSPECIFIED" => {
                    PasswordRequirementsPasswordQuality::PasswordQualityUnspecified
                }
                "SOMETHING" => PasswordRequirementsPasswordQuality::Something,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PasswordRequirementsPasswordQuality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasswordRequirementsPasswordQuality {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasswordRequirementsPasswordQuality {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHABETIC" => PasswordRequirementsPasswordQuality::Alphabetic,
                "ALPHANUMERIC" => PasswordRequirementsPasswordQuality::Alphanumeric,
                "BIOMETRIC_WEAK" => PasswordRequirementsPasswordQuality::BiometricWeak,
                "COMPLEX" => PasswordRequirementsPasswordQuality::Complex,
                "COMPLEXITY_HIGH" => PasswordRequirementsPasswordQuality::ComplexityHigh,
                "COMPLEXITY_LOW" => PasswordRequirementsPasswordQuality::ComplexityLow,
                "COMPLEXITY_MEDIUM" => PasswordRequirementsPasswordQuality::ComplexityMedium,
                "NUMERIC" => PasswordRequirementsPasswordQuality::Numeric,
                "NUMERIC_COMPLEX" => PasswordRequirementsPasswordQuality::NumericComplex,
                "PASSWORD_QUALITY_UNSPECIFIED" => {
                    PasswordRequirementsPasswordQuality::PasswordQualityUnspecified
                }
                "SOMETHING" => PasswordRequirementsPasswordQuality::Something,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PasswordRequirementsPasswordQuality {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasswordRequirementsPasswordQuality {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PasswordRequirementsPasswordScope {
        #[doc = "The password requirements are only applied to the device."]
        ScopeDevice,
        #[doc = "The password requirements are only applied to the work profile."]
        ScopeProfile,
        #[doc = "The scope is unspecified. The password requirements are applied to the work profile for work profile devices and the whole device for fully managed or dedicated devices."]
        ScopeUnspecified,
    }
    impl PasswordRequirementsPasswordScope {
        pub fn as_str(self) -> &'static str {
            match self {
                PasswordRequirementsPasswordScope::ScopeDevice => "SCOPE_DEVICE",
                PasswordRequirementsPasswordScope::ScopeProfile => "SCOPE_PROFILE",
                PasswordRequirementsPasswordScope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PasswordRequirementsPasswordScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PasswordRequirementsPasswordScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PasswordRequirementsPasswordScope, ()> {
            Ok(match s {
                "SCOPE_DEVICE" => PasswordRequirementsPasswordScope::ScopeDevice,
                "SCOPE_PROFILE" => PasswordRequirementsPasswordScope::ScopeProfile,
                "SCOPE_UNSPECIFIED" => PasswordRequirementsPasswordScope::ScopeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PasswordRequirementsPasswordScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasswordRequirementsPasswordScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasswordRequirementsPasswordScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_DEVICE" => PasswordRequirementsPasswordScope::ScopeDevice,
                "SCOPE_PROFILE" => PasswordRequirementsPasswordScope::ScopeProfile,
                "SCOPE_UNSPECIFIED" => PasswordRequirementsPasswordScope::ScopeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PasswordRequirementsPasswordScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasswordRequirementsPasswordScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PasswordRequirementsRequirePasswordUnlock {
        #[doc = "The timeout period is set to 24 hours."]
        RequireEveryDay,
        #[doc = "Unspecified. Defaults to USE_DEFAULT_DEVICE_TIMEOUT."]
        RequirePasswordUnlockUnspecified,
        #[doc = "The timeout period is set to the device’s default."]
        UseDefaultDeviceTimeout,
    }
    impl PasswordRequirementsRequirePasswordUnlock {
        pub fn as_str(self) -> &'static str {
            match self {
                PasswordRequirementsRequirePasswordUnlock::RequireEveryDay => "REQUIRE_EVERY_DAY",
                PasswordRequirementsRequirePasswordUnlock::RequirePasswordUnlockUnspecified => {
                    "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED"
                }
                PasswordRequirementsRequirePasswordUnlock::UseDefaultDeviceTimeout => {
                    "USE_DEFAULT_DEVICE_TIMEOUT"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PasswordRequirementsRequirePasswordUnlock {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PasswordRequirementsRequirePasswordUnlock {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PasswordRequirementsRequirePasswordUnlock, ()> {
            Ok(match s {
                "REQUIRE_EVERY_DAY" => PasswordRequirementsRequirePasswordUnlock::RequireEveryDay,
                "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED" => {
                    PasswordRequirementsRequirePasswordUnlock::RequirePasswordUnlockUnspecified
                }
                "USE_DEFAULT_DEVICE_TIMEOUT" => {
                    PasswordRequirementsRequirePasswordUnlock::UseDefaultDeviceTimeout
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PasswordRequirementsRequirePasswordUnlock {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasswordRequirementsRequirePasswordUnlock {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasswordRequirementsRequirePasswordUnlock {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REQUIRE_EVERY_DAY" => PasswordRequirementsRequirePasswordUnlock::RequireEveryDay,
                "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED" => {
                    PasswordRequirementsRequirePasswordUnlock::RequirePasswordUnlockUnspecified
                }
                "USE_DEFAULT_DEVICE_TIMEOUT" => {
                    PasswordRequirementsRequirePasswordUnlock::UseDefaultDeviceTimeout
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
    impl ::google_field_selector::FieldSelector for PasswordRequirementsRequirePasswordUnlock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasswordRequirementsRequirePasswordUnlock {
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
    pub struct PermissionGrant {
        #[doc = "The Android permission or group, e.g. android.permission.READ_CALENDAR or android.permission_group.CALENDAR."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "The policy for granting the permission."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::PermissionGrantPolicy>,
    }
    impl ::google_field_selector::FieldSelector for PermissionGrant {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PermissionGrant {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PermissionGrantPolicy {
        #[doc = "Automatically deny a permission."]
        Deny,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
    }
    impl PermissionGrantPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionGrantPolicy::Deny => "DENY",
                PermissionGrantPolicy::Grant => "GRANT",
                PermissionGrantPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                PermissionGrantPolicy::Prompt => "PROMPT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PermissionGrantPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PermissionGrantPolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PermissionGrantPolicy, ()> {
            Ok(match s {
                "DENY" => PermissionGrantPolicy::Deny,
                "GRANT" => PermissionGrantPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PermissionGrantPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PermissionGrantPolicy::Prompt,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PermissionGrantPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PermissionGrantPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PermissionGrantPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DENY" => PermissionGrantPolicy::Deny,
                "GRANT" => PermissionGrantPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PermissionGrantPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PermissionGrantPolicy::Prompt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PermissionGrantPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PermissionGrantPolicy {
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
    pub struct PersistentPreferredActivity {
        #[doc = "The intent actions to match in the filter. If any actions are included in the filter, then an intent's action must be one of those values for it to match. If no actions are included, the intent action is ignored."]
        #[serde(
            rename = "actions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actions: ::std::option::Option<Vec<String>>,
        #[doc = "The intent categories to match in the filter. An intent includes the categories that it requires, all of which must be included in the filter in order to match. In other words, adding a category to the filter has no impact on matching unless that category is specified in the intent."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "The activity that should be the default intent handler. This should be an Android component name, e.g. com.android.enterprise.app/.MainActivity. Alternatively, the value may be the package name of an app, which causes Android Device Policy to choose an appropriate activity from the app to handle the intent."]
        #[serde(
            rename = "receiverActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub receiver_activity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PersistentPreferredActivity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersistentPreferredActivity {
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
    pub struct PersonalApplicationPolicy {
        #[doc = "The type of installation to perform."]
        #[serde(
            rename = "installType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_type:
            ::std::option::Option<crate::schemas::PersonalApplicationPolicyInstallType>,
        #[doc = "The package name of the application."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PersonalApplicationPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonalApplicationPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PersonalApplicationPolicyInstallType {
        #[doc = "The app is available to install in the personal profile."]
        Available,
        #[doc = "The app is blocked and can't be installed in the personal profile."]
        Blocked,
        #[doc = "Unspecified. Defaults to AVAILABLE."]
        InstallTypeUnspecified,
    }
    impl PersonalApplicationPolicyInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                PersonalApplicationPolicyInstallType::Available => "AVAILABLE",
                PersonalApplicationPolicyInstallType::Blocked => "BLOCKED",
                PersonalApplicationPolicyInstallType::InstallTypeUnspecified => {
                    "INSTALL_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PersonalApplicationPolicyInstallType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PersonalApplicationPolicyInstallType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PersonalApplicationPolicyInstallType, ()> {
            Ok(match s {
                "AVAILABLE" => PersonalApplicationPolicyInstallType::Available,
                "BLOCKED" => PersonalApplicationPolicyInstallType::Blocked,
                "INSTALL_TYPE_UNSPECIFIED" => {
                    PersonalApplicationPolicyInstallType::InstallTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PersonalApplicationPolicyInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PersonalApplicationPolicyInstallType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PersonalApplicationPolicyInstallType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVAILABLE" => PersonalApplicationPolicyInstallType::Available,
                "BLOCKED" => PersonalApplicationPolicyInstallType::Blocked,
                "INSTALL_TYPE_UNSPECIFIED" => {
                    PersonalApplicationPolicyInstallType::InstallTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for PersonalApplicationPolicyInstallType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonalApplicationPolicyInstallType {
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
    pub struct PersonalUsagePolicies {
        #[doc = "Account types that can't be managed by the user."]
        #[serde(
            rename = "accountTypesWithManagementDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_types_with_management_disabled: ::std::option::Option<Vec<String>>,
        #[doc = "Whether camera is disabled."]
        #[serde(
            rename = "cameraDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub camera_disabled: ::std::option::Option<bool>,
        #[doc = "Controls how long the work profile can stay off. The duration must be at least 3 days."]
        #[serde(
            rename = "maxDaysWithWorkOff",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_days_with_work_off: ::std::option::Option<i32>,
        #[doc = "Policy applied to applications in the personal profile."]
        #[serde(
            rename = "personalApplications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub personal_applications:
            ::std::option::Option<Vec<crate::schemas::PersonalApplicationPolicy>>,
        #[doc = "Used together with personalApplications to control how apps in the personal profile are allowed or blocked."]
        #[serde(
            rename = "personalPlayStoreMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub personal_play_store_mode:
            ::std::option::Option<crate::schemas::PersonalUsagePoliciesPersonalPlayStoreMode>,
        #[doc = "Whether screen capture is disabled."]
        #[serde(
            rename = "screenCaptureDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_capture_disabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for PersonalUsagePolicies {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonalUsagePolicies {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PersonalUsagePoliciesPersonalPlayStoreMode {
        #[doc = "Only apps explicitly specified in personalApplications with installType set to AVAILABLE are allowed to be installed in the personal profile."]
        Allowlist,
        #[doc = "All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications."]
        Blacklist,
        #[doc = "All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications."]
        Blocklist,
        #[doc = "Unspecified. Defaults to BLOCKLIST."]
        PlayStoreModeUnspecified,
    }
    impl PersonalUsagePoliciesPersonalPlayStoreMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PersonalUsagePoliciesPersonalPlayStoreMode::Allowlist => "ALLOWLIST",
                PersonalUsagePoliciesPersonalPlayStoreMode::Blacklist => "BLACKLIST",
                PersonalUsagePoliciesPersonalPlayStoreMode::Blocklist => "BLOCKLIST",
                PersonalUsagePoliciesPersonalPlayStoreMode::PlayStoreModeUnspecified => {
                    "PLAY_STORE_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PersonalUsagePoliciesPersonalPlayStoreMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PersonalUsagePoliciesPersonalPlayStoreMode, ()> {
            Ok(match s {
                "ALLOWLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Allowlist,
                "BLACKLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Blacklist,
                "BLOCKLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Blocklist,
                "PLAY_STORE_MODE_UNSPECIFIED" => {
                    PersonalUsagePoliciesPersonalPlayStoreMode::PlayStoreModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALLOWLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Allowlist,
                "BLACKLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Blacklist,
                "BLOCKLIST" => PersonalUsagePoliciesPersonalPlayStoreMode::Blocklist,
                "PLAY_STORE_MODE_UNSPECIFIED" => {
                    PersonalUsagePoliciesPersonalPlayStoreMode::PlayStoreModeUnspecified
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
    impl ::google_field_selector::FieldSelector for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonalUsagePoliciesPersonalPlayStoreMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Policy {
        #[doc = "Account types that can't be managed by the user."]
        #[serde(
            rename = "accountTypesWithManagementDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_types_with_management_disabled: ::std::option::Option<Vec<String>>,
        #[doc = "Whether adding new users and profiles is disabled."]
        #[serde(
            rename = "addUserDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_user_disabled: ::std::option::Option<bool>,
        #[doc = "Whether adjusting the master volume is disabled. Also mutes the device."]
        #[serde(
            rename = "adjustVolumeDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adjust_volume_disabled: ::std::option::Option<bool>,
        #[doc = "Security policies set to secure values by default. To maintain the security posture of a device, we don't recommend overriding any of the default values."]
        #[serde(
            rename = "advancedSecurityOverrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advanced_security_overrides:
            ::std::option::Option<crate::schemas::AdvancedSecurityOverrides>,
        #[doc = "Configuration for an always-on VPN connection. Use with vpn_config_disabled to prevent modification of this setting."]
        #[serde(
            rename = "alwaysOnVpnPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub always_on_vpn_package: ::std::option::Option<crate::schemas::AlwaysOnVpnPackage>,
        #[doc = "The app tracks for Android Device Policy the device can access. The device receives the latest version among all accessible tracks. If no tracks are specified, then the device only uses the production track."]
        #[serde(
            rename = "androidDevicePolicyTracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device_policy_tracks:
            ::std::option::Option<Vec<crate::schemas::PolicyAndroidDevicePolicyTracksItems>>,
        #[doc = "Deprecated. Use autoUpdateMode instead.When autoUpdateMode is set to AUTO_UPDATE_POSTPONED or AUTO_UPDATE_HIGH_PRIORITY, this field has no effect.The app auto update policy, which controls when automatic app updates can be applied."]
        #[serde(
            rename = "appAutoUpdatePolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_auto_update_policy:
            ::std::option::Option<crate::schemas::PolicyAppAutoUpdatePolicy>,
        #[doc = "Policy applied to apps."]
        #[serde(
            rename = "applications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applications: ::std::option::Option<Vec<crate::schemas::ApplicationPolicy>>,
        #[doc = "Whether auto date, time, and time zone are enabled on a company-owned device. If this is set, then autoTimeRequired is ignored."]
        #[serde(
            rename = "autoDateAndTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_date_and_time_zone:
            ::std::option::Option<crate::schemas::PolicyAutoDateAndTimeZone>,
        #[doc = "Whether auto time is required, which prevents the user from manually setting the date and time. If autoDateAndTimeZone is set, this field is ignored."]
        #[serde(
            rename = "autoTimeRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_time_required: ::std::option::Option<bool>,
        #[doc = "Whether applications other than the ones configured in applications are blocked from being installed. When set, applications that were installed under a previous policy but no longer appear in the policy are automatically uninstalled."]
        #[serde(
            rename = "blockApplicationsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_applications_enabled: ::std::option::Option<bool>,
        #[doc = "Whether configuring bluetooth is disabled."]
        #[serde(
            rename = "bluetoothConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bluetooth_config_disabled: ::std::option::Option<bool>,
        #[doc = "Whether bluetooth contact sharing is disabled."]
        #[serde(
            rename = "bluetoothContactSharingDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bluetooth_contact_sharing_disabled: ::std::option::Option<bool>,
        #[doc = "Whether bluetooth is disabled. Prefer this setting over bluetooth_config_disabled because bluetooth_config_disabled can be bypassed by the user."]
        #[serde(
            rename = "bluetoothDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bluetooth_disabled: ::std::option::Option<bool>,
        #[doc = "Controls the use of the camera and whether the user has access to the camera access toggle."]
        #[serde(
            rename = "cameraAccess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub camera_access: ::std::option::Option<crate::schemas::PolicyCameraAccess>,
        #[doc = "If camera_access is set to any value other than CAMERA_ACCESS_UNSPECIFIED, this has no effect. Otherwise this field controls whether cameras are disabled: If true, all cameras are disabled, otherwise they are available. For fully managed devices this field applies for all apps on the device. For work profiles, this field applies only to apps in the work profile, and the camera access of apps outside the work profile is unaffected."]
        #[serde(
            rename = "cameraDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub camera_disabled: ::std::option::Option<bool>,
        #[doc = "Whether configuring cell broadcast is disabled."]
        #[serde(
            rename = "cellBroadcastsConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_broadcasts_config_disabled: ::std::option::Option<bool>,
        #[doc = "Rules for determining apps' access to private keys. See ChoosePrivateKeyRule for details."]
        #[serde(
            rename = "choosePrivateKeyRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub choose_private_key_rules:
            ::std::option::Option<Vec<crate::schemas::ChoosePrivateKeyRule>>,
        #[doc = "Rules declaring which mitigating actions to take when a device is not compliant with its policy. When the conditions for multiple rules are satisfied, all of the mitigating actions for the rules are taken. There is a maximum limit of 100 rules. Use policy enforcement rules instead."]
        #[serde(
            rename = "complianceRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance_rules: ::std::option::Option<Vec<crate::schemas::ComplianceRule>>,
        #[doc = "Whether creating windows besides app windows is disabled."]
        #[serde(
            rename = "createWindowsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_windows_disabled: ::std::option::Option<bool>,
        #[doc = "Whether configuring user credentials is disabled."]
        #[serde(
            rename = "credentialsConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credentials_config_disabled: ::std::option::Option<bool>,
        #[doc = "Cross-profile policies applied on the device."]
        #[serde(
            rename = "crossProfilePolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_profile_policies: ::std::option::Option<crate::schemas::CrossProfilePolicies>,
        #[doc = "Whether roaming data services are disabled."]
        #[serde(
            rename = "dataRoamingDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_roaming_disabled: ::std::option::Option<bool>,
        #[doc = "Whether the user is allowed to enable debugging features."]
        #[serde(
            rename = "debuggingFeaturesAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debugging_features_allowed: ::std::option::Option<bool>,
        #[doc = "The default permission policy for runtime permission requests."]
        #[serde(
            rename = "defaultPermissionPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_permission_policy:
            ::std::option::Option<crate::schemas::PolicyDefaultPermissionPolicy>,
        #[doc = "The device owner information to be shown on the lock screen."]
        #[serde(
            rename = "deviceOwnerLockScreenInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_owner_lock_screen_info: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "Whether encryption is enabled"]
        #[serde(
            rename = "encryptionPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_policy: ::std::option::Option<crate::schemas::PolicyEncryptionPolicy>,
        #[doc = "Whether app verification is force-enabled."]
        #[serde(
            rename = "ensureVerifyAppsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ensure_verify_apps_enabled: ::std::option::Option<bool>,
        #[doc = "Whether factory resetting from settings is disabled."]
        #[serde(
            rename = "factoryResetDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub factory_reset_disabled: ::std::option::Option<bool>,
        #[doc = "Email addresses of device administrators for factory reset protection. When the device is factory reset, it will require one of these admins to log in with the Google account email and password to unlock the device. If no admins are specified, the device won't provide factory reset protection."]
        #[serde(
            rename = "frpAdminEmails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frp_admin_emails: ::std::option::Option<Vec<String>>,
        #[doc = "Whether the user is allowed to have fun. Controls whether the Easter egg game in Settings is disabled."]
        #[serde(
            rename = "funDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fun_disabled: ::std::option::Option<bool>,
        #[doc = "Whether user installation of apps is disabled."]
        #[serde(
            rename = "installAppsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_apps_disabled: ::std::option::Option<bool>,
        #[doc = "This field has no effect."]
        #[serde(
            rename = "installUnknownSourcesAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_unknown_sources_allowed: ::std::option::Option<bool>,
        #[doc = "Whether the keyguard is disabled."]
        #[serde(
            rename = "keyguardDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyguard_disabled: ::std::option::Option<bool>,
        #[doc = "Disabled keyguard customizations, such as widgets."]
        #[serde(
            rename = "keyguardDisabledFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyguard_disabled_features:
            ::std::option::Option<Vec<crate::schemas::PolicyKeyguardDisabledFeaturesItems>>,
        #[doc = "Whether the kiosk custom launcher is enabled. This replaces the home screen with a launcher that locks down the device to the apps installed via the applications setting. Apps appear on a single page in alphabetical order. Use kioskCustomization to further configure the kiosk device behavior."]
        #[serde(
            rename = "kioskCustomLauncherEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kiosk_custom_launcher_enabled: ::std::option::Option<bool>,
        #[doc = "Settings controlling the behavior of a device in kiosk mode. To enable kiosk mode, set kioskCustomLauncherEnabled to true or specify an app in the policy with installType KIOSK."]
        #[serde(
            rename = "kioskCustomization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kiosk_customization: ::std::option::Option<crate::schemas::KioskCustomization>,
        #[doc = "The degree of location detection enabled."]
        #[serde(
            rename = "locationMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_mode: ::std::option::Option<crate::schemas::PolicyLocationMode>,
        #[doc = "A message displayed to the user in the device administators settings screen."]
        #[serde(
            rename = "longSupportMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_support_message: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "Maximum time in milliseconds for user activity until the device locks. A value of 0 means there is no restriction."]
        #[serde(
            rename = "maximumTimeToLock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_time_to_lock: ::std::option::Option<i64>,
        #[doc = "Controls the use of the microphone and whether the user has access to the microphone access toggle. This applies only on fully managed devices."]
        #[serde(
            rename = "microphoneAccess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub microphone_access: ::std::option::Option<crate::schemas::PolicyMicrophoneAccess>,
        #[doc = "The minimum allowed Android API level."]
        #[serde(
            rename = "minimumApiLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_api_level: ::std::option::Option<i32>,
        #[doc = "Whether configuring mobile networks is disabled."]
        #[serde(
            rename = "mobileNetworksConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_networks_config_disabled: ::std::option::Option<bool>,
        #[doc = "Whether adding or removing accounts is disabled."]
        #[serde(
            rename = "modifyAccountsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub modify_accounts_disabled: ::std::option::Option<bool>,
        #[doc = "Whether the user mounting physical external media is disabled."]
        #[serde(
            rename = "mountPhysicalMediaDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mount_physical_media_disabled: ::std::option::Option<bool>,
        #[doc = "The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Whether the network escape hatch is enabled. If a network connection can't be made at boot time, the escape hatch prompts the user to temporarily connect to a network in order to refresh the device policy. After applying policy, the temporary network will be forgotten and the device will continue booting. This prevents being unable to connect to a network if there is no suitable network in the last policy and the device boots into an app in lock task mode, or the user is otherwise unable to reach device settings.Note: Setting wifiConfigDisabled to true will override this setting under specific circumstances. Please see wifiConfigDisabled for further details."]
        #[serde(
            rename = "networkEscapeHatchEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_escape_hatch_enabled: ::std::option::Option<bool>,
        #[doc = "Whether resetting network settings is disabled."]
        #[serde(
            rename = "networkResetDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_reset_disabled: ::std::option::Option<bool>,
        #[doc = "This feature is not generally available."]
        #[serde(
            rename = "oncCertificateProviders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub onc_certificate_providers:
            ::std::option::Option<Vec<crate::schemas::OncCertificateProvider>>,
        #[doc = "Network configuration for the device. See configure networks for more information."]
        #[serde(
            rename = "openNetworkConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_network_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Whether using NFC to beam data from apps is disabled."]
        #[serde(
            rename = "outgoingBeamDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outgoing_beam_disabled: ::std::option::Option<bool>,
        #[doc = "Whether outgoing calls are disabled."]
        #[serde(
            rename = "outgoingCallsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outgoing_calls_disabled: ::std::option::Option<bool>,
        #[doc = "Password requirement policies. Different policies can be set for work profile or fully managed devices by setting the password_scope field in the policy."]
        #[serde(
            rename = "passwordPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_policies: ::std::option::Option<Vec<crate::schemas::PasswordRequirements>>,
        #[doc = "Password requirements. The field password_requirements.require_password_unlock must not be set. DEPRECATED - Use passwordPolicies.Note:Complexity-based values of PasswordQuality, that is, COMPLEXITY_LOW, COMPLEXITY_MEDIUM, and COMPLEXITY_HIGH, cannot be used here."]
        #[serde(
            rename = "passwordRequirements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_requirements: ::std::option::Option<crate::schemas::PasswordRequirements>,
        #[doc = "Explicit permission or group grants or denials for all apps. These values override the default_permission_policy."]
        #[serde(
            rename = "permissionGrants",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_grants: ::std::option::Option<Vec<crate::schemas::PermissionGrant>>,
        #[doc = "Specifies permitted accessibility services. If the field is not set, any accessibility service can be used. If the field is set, only the accessibility services in this list and the system's built-in accessibility service can be used. In particular, if the field is set to empty, only the system's built-in accessibility servicess can be used. This can be set on fully managed devices and on work profiles. When applied to a work profile, this affects both the personal profile and the work profile."]
        #[serde(
            rename = "permittedAccessibilityServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permitted_accessibility_services:
            ::std::option::Option<crate::schemas::PackageNameList>,
        #[doc = "If present, only the input methods provided by packages in this list are permitted. If this field is present, but the list is empty, then only system input methods are permitted."]
        #[serde(
            rename = "permittedInputMethods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permitted_input_methods: ::std::option::Option<crate::schemas::PackageNameList>,
        #[doc = "Default intent handler activities."]
        #[serde(
            rename = "persistentPreferredActivities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub persistent_preferred_activities:
            ::std::option::Option<Vec<crate::schemas::PersistentPreferredActivity>>,
        #[doc = "Policies managing personal usage on a company-owned device."]
        #[serde(
            rename = "personalUsagePolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub personal_usage_policies: ::std::option::Option<crate::schemas::PersonalUsagePolicies>,
        #[doc = "This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy."]
        #[serde(
            rename = "playStoreMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub play_store_mode: ::std::option::Option<crate::schemas::PolicyPlayStoreMode>,
        #[doc = "Rules that define the behavior when a particular policy can not be applied on device"]
        #[serde(
            rename = "policyEnforcementRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_enforcement_rules:
            ::std::option::Option<Vec<crate::schemas::PolicyEnforcementRule>>,
        #[doc = "Controls whether preferential network service is enabled on the work profile. For example, an organization may have an agreement with a carrier that all of the work data from its employees' devices will be sent via a network service dedicated for enterprise use. An example of a supported preferential network service is the enterprise slice on 5G networks. This has no effect on fully managed devices."]
        #[serde(
            rename = "preferentialNetworkService",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferential_network_service:
            ::std::option::Option<crate::schemas::PolicyPreferentialNetworkService>,
        #[doc = "Allows showing UI on a device for a user to choose a private key alias if there are no matching rules in ChoosePrivateKeyRules. For devices below Android P, setting this may leave enterprise keys vulnerable."]
        #[serde(
            rename = "privateKeySelectionEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_key_selection_enabled: ::std::option::Option<bool>,
        #[doc = "The network-independent global HTTP proxy. Typically proxies should be configured per-network in open_network_configuration. However for unusual configurations like general internal filtering a global HTTP proxy may be useful. If the proxy is not accessible, network access may break. The global proxy is only a recommendation and some apps may ignore it."]
        #[serde(
            rename = "recommendedGlobalProxy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recommended_global_proxy: ::std::option::Option<crate::schemas::ProxyInfo>,
        #[doc = "Whether removing other users is disabled."]
        #[serde(
            rename = "removeUserDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remove_user_disabled: ::std::option::Option<bool>,
        #[doc = "Whether rebooting the device into safe boot is disabled."]
        #[serde(
            rename = "safeBootDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe_boot_disabled: ::std::option::Option<bool>,
        #[doc = "Whether screen capture is disabled."]
        #[serde(
            rename = "screenCaptureDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_capture_disabled: ::std::option::Option<bool>,
        #[doc = "Whether changing the user icon is disabled."]
        #[serde(
            rename = "setUserIconDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub set_user_icon_disabled: ::std::option::Option<bool>,
        #[doc = "Whether changing the wallpaper is disabled."]
        #[serde(
            rename = "setWallpaperDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub set_wallpaper_disabled: ::std::option::Option<bool>,
        #[doc = "Action to take during the setup process. At most one action may be specified."]
        #[serde(
            rename = "setupActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub setup_actions: ::std::option::Option<Vec<crate::schemas::SetupAction>>,
        #[doc = "Whether location sharing is disabled."]
        #[serde(
            rename = "shareLocationDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub share_location_disabled: ::std::option::Option<bool>,
        #[doc = "A message displayed to the user in the settings screen wherever functionality has been disabled by the admin. If the message is longer than 200 characters it may be truncated."]
        #[serde(
            rename = "shortSupportMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_support_message: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "Flag to skip hints on the first use. Enterprise admin can enable the system recommendation for apps to skip their user tutorial and other introductory hints on first start-up."]
        #[serde(
            rename = "skipFirstUseHintsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_first_use_hints_enabled: ::std::option::Option<bool>,
        #[doc = "Whether sending and receiving SMS messages is disabled."]
        #[serde(
            rename = "smsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sms_disabled: ::std::option::Option<bool>,
        #[doc = "Whether the status bar is disabled. This disables notifications, quick settings, and other screen overlays that allow escape from full-screen mode. DEPRECATED. To disable the status bar on a kiosk device, use InstallType KIOSK or kioskCustomLauncherEnabled."]
        #[serde(
            rename = "statusBarDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_bar_disabled: ::std::option::Option<bool>,
        #[doc = "Status reporting settings"]
        #[serde(
            rename = "statusReportingSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_reporting_settings:
            ::std::option::Option<crate::schemas::StatusReportingSettings>,
        #[doc = "The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn't lock itself while it stays on."]
        #[serde(
            rename = "stayOnPluggedModes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stay_on_plugged_modes:
            ::std::option::Option<Vec<crate::schemas::PolicyStayOnPluggedModesItems>>,
        #[doc = "The system update policy, which controls how OS updates are applied. If the update type is WINDOWED, the update window will automatically apply to Play app updates as well."]
        #[serde(
            rename = "systemUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_update: ::std::option::Option<crate::schemas::SystemUpdate>,
        #[doc = "Whether configuring tethering and portable hotspots is disabled."]
        #[serde(
            rename = "tetheringConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tethering_config_disabled: ::std::option::Option<bool>,
        #[doc = "Whether user uninstallation of applications is disabled."]
        #[serde(
            rename = "uninstallAppsDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uninstall_apps_disabled: ::std::option::Option<bool>,
        #[doc = "If microphone_access is set to any value other than MICROPHONE_ACCESS_UNSPECIFIED, this has no effect. Otherwise this field controls whether microphones are disabled: If true, all microphones are disabled, otherwise they are available. This is available only on fully managed devices."]
        #[serde(
            rename = "unmuteMicrophoneDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unmute_microphone_disabled: ::std::option::Option<bool>,
        #[doc = "Configuration of device activity logging."]
        #[serde(
            rename = "usageLog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_log: ::std::option::Option<crate::schemas::UsageLog>,
        #[doc = "Whether transferring files over USB is disabled."]
        #[serde(
            rename = "usbFileTransferDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usb_file_transfer_disabled: ::std::option::Option<bool>,
        #[doc = "Whether USB storage is enabled. Deprecated."]
        #[serde(
            rename = "usbMassStorageEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usb_mass_storage_enabled: ::std::option::Option<bool>,
        #[doc = "The version of the policy. This is a read-only field. The version is incremented each time the policy is updated."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub version: ::std::option::Option<i64>,
        #[doc = "Whether configuring VPN is disabled."]
        #[serde(
            rename = "vpnConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vpn_config_disabled: ::std::option::Option<bool>,
        #[doc = "Whether configuring Wi-Fi access points is disabled.Note: If a network connection can't be made at boot time and configuring Wi-Fi is disabled then network escape hatch will be shown in order to refresh the device policy (see networkEscapeHatchEnabled)."]
        #[serde(
            rename = "wifiConfigDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wifi_config_disabled: ::std::option::Option<bool>,
        #[doc = "DEPRECATED - Use wifi_config_disabled."]
        #[serde(
            rename = "wifiConfigsLockdownEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wifi_configs_lockdown_enabled: ::std::option::Option<bool>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyAndroidDevicePolicyTracksItems {
        #[doc = "This value is ignored."]
        AppTrackUnspecified,
        #[doc = "The beta track, which provides the latest beta release."]
        Beta,
        #[doc = "The production track, which provides the latest stable release."]
        Production,
    }
    impl PolicyAndroidDevicePolicyTracksItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyAndroidDevicePolicyTracksItems::AppTrackUnspecified => {
                    "APP_TRACK_UNSPECIFIED"
                }
                PolicyAndroidDevicePolicyTracksItems::Beta => "BETA",
                PolicyAndroidDevicePolicyTracksItems::Production => "PRODUCTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyAndroidDevicePolicyTracksItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyAndroidDevicePolicyTracksItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyAndroidDevicePolicyTracksItems, ()> {
            Ok(match s {
                "APP_TRACK_UNSPECIFIED" => {
                    PolicyAndroidDevicePolicyTracksItems::AppTrackUnspecified
                }
                "BETA" => PolicyAndroidDevicePolicyTracksItems::Beta,
                "PRODUCTION" => PolicyAndroidDevicePolicyTracksItems::Production,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyAndroidDevicePolicyTracksItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAndroidDevicePolicyTracksItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyAndroidDevicePolicyTracksItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_TRACK_UNSPECIFIED" => {
                    PolicyAndroidDevicePolicyTracksItems::AppTrackUnspecified
                }
                "BETA" => PolicyAndroidDevicePolicyTracksItems::Beta,
                "PRODUCTION" => PolicyAndroidDevicePolicyTracksItems::Production,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyAndroidDevicePolicyTracksItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyAndroidDevicePolicyTracksItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyAppAutoUpdatePolicy {
        #[doc = "Apps are auto-updated at any time. Data charges may apply."]
        Always,
        #[doc = "The auto-update policy is not set. Equivalent to CHOICE_TO_THE_USER."]
        AppAutoUpdatePolicyUnspecified,
        #[doc = "The user can control auto-updates."]
        ChoiceToTheUser,
        #[doc = "Apps are never auto-updated."]
        Never,
        #[doc = "Apps are auto-updated over Wi-Fi only."]
        WifiOnly,
    }
    impl PolicyAppAutoUpdatePolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyAppAutoUpdatePolicy::Always => "ALWAYS",
                PolicyAppAutoUpdatePolicy::AppAutoUpdatePolicyUnspecified => {
                    "APP_AUTO_UPDATE_POLICY_UNSPECIFIED"
                }
                PolicyAppAutoUpdatePolicy::ChoiceToTheUser => "CHOICE_TO_THE_USER",
                PolicyAppAutoUpdatePolicy::Never => "NEVER",
                PolicyAppAutoUpdatePolicy::WifiOnly => "WIFI_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyAppAutoUpdatePolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyAppAutoUpdatePolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyAppAutoUpdatePolicy, ()> {
            Ok(match s {
                "ALWAYS" => PolicyAppAutoUpdatePolicy::Always,
                "APP_AUTO_UPDATE_POLICY_UNSPECIFIED" => {
                    PolicyAppAutoUpdatePolicy::AppAutoUpdatePolicyUnspecified
                }
                "CHOICE_TO_THE_USER" => PolicyAppAutoUpdatePolicy::ChoiceToTheUser,
                "NEVER" => PolicyAppAutoUpdatePolicy::Never,
                "WIFI_ONLY" => PolicyAppAutoUpdatePolicy::WifiOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyAppAutoUpdatePolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAppAutoUpdatePolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyAppAutoUpdatePolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALWAYS" => PolicyAppAutoUpdatePolicy::Always,
                "APP_AUTO_UPDATE_POLICY_UNSPECIFIED" => {
                    PolicyAppAutoUpdatePolicy::AppAutoUpdatePolicyUnspecified
                }
                "CHOICE_TO_THE_USER" => PolicyAppAutoUpdatePolicy::ChoiceToTheUser,
                "NEVER" => PolicyAppAutoUpdatePolicy::Never,
                "WIFI_ONLY" => PolicyAppAutoUpdatePolicy::WifiOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyAppAutoUpdatePolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyAppAutoUpdatePolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyAutoDateAndTimeZone {
        #[doc = "Enforce auto date, time, and time zone on the device."]
        AutoDateAndTimeZoneEnforced,
        #[doc = "Unspecified. Defaults to AUTO_DATE_AND_TIME_ZONE_USER_CHOICE."]
        AutoDateAndTimeZoneUnspecified,
        #[doc = "Auto date, time, and time zone are left to user's choice."]
        AutoDateAndTimeZoneUserChoice,
    }
    impl PolicyAutoDateAndTimeZone {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneEnforced => {
                    "AUTO_DATE_AND_TIME_ZONE_ENFORCED"
                }
                PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUnspecified => {
                    "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED"
                }
                PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUserChoice => {
                    "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyAutoDateAndTimeZone {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyAutoDateAndTimeZone {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyAutoDateAndTimeZone, ()> {
            Ok(match s {
                "AUTO_DATE_AND_TIME_ZONE_ENFORCED" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneEnforced
                }
                "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUnspecified
                }
                "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUserChoice
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyAutoDateAndTimeZone {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAutoDateAndTimeZone {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyAutoDateAndTimeZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO_DATE_AND_TIME_ZONE_ENFORCED" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneEnforced
                }
                "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUnspecified
                }
                "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE" => {
                    PolicyAutoDateAndTimeZone::AutoDateAndTimeZoneUserChoice
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
    impl ::google_field_selector::FieldSelector for PolicyAutoDateAndTimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyAutoDateAndTimeZone {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyCameraAccess {
        #[doc = "The field camera_disabled is ignored. All cameras on the device are disabled (for fully managed devices, this applies device-wide and for work profiles this applies only to the work profile).There are no explicit restrictions placed on the camera access toggle on Android 12 and above: on fully managed devices, the camera access toggle has no effect as all cameras are disabled. On devices with a work profile, this toggle has no effect on apps in the work profile, but it affects apps outside the work profile."]
        CameraAccessDisabled,
        #[doc = "The field camera_disabled is ignored. All cameras on the device are available. On fully managed devices running Android 12 and above, the user is unable to use the camera access toggle. On devices which are not fully managed or which run Android 11 or below, this is equivalent to CAMERA_ACCESS_USER_CHOICE."]
        CameraAccessEnforced,
        #[doc = "If camera_disabled is true, this is equivalent to CAMERA_ACCESS_DISABLED. Otherwise, this is equivalent to CAMERA_ACCESS_USER_CHOICE."]
        CameraAccessUnspecified,
        #[doc = "The field camera_disabled is ignored. This is the default device behaviour: all cameras on the device are available. On Android 12 and above, the user can use the camera access toggle."]
        CameraAccessUserChoice,
    }
    impl PolicyCameraAccess {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyCameraAccess::CameraAccessDisabled => "CAMERA_ACCESS_DISABLED",
                PolicyCameraAccess::CameraAccessEnforced => "CAMERA_ACCESS_ENFORCED",
                PolicyCameraAccess::CameraAccessUnspecified => "CAMERA_ACCESS_UNSPECIFIED",
                PolicyCameraAccess::CameraAccessUserChoice => "CAMERA_ACCESS_USER_CHOICE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyCameraAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyCameraAccess {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyCameraAccess, ()> {
            Ok(match s {
                "CAMERA_ACCESS_DISABLED" => PolicyCameraAccess::CameraAccessDisabled,
                "CAMERA_ACCESS_ENFORCED" => PolicyCameraAccess::CameraAccessEnforced,
                "CAMERA_ACCESS_UNSPECIFIED" => PolicyCameraAccess::CameraAccessUnspecified,
                "CAMERA_ACCESS_USER_CHOICE" => PolicyCameraAccess::CameraAccessUserChoice,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyCameraAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyCameraAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyCameraAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CAMERA_ACCESS_DISABLED" => PolicyCameraAccess::CameraAccessDisabled,
                "CAMERA_ACCESS_ENFORCED" => PolicyCameraAccess::CameraAccessEnforced,
                "CAMERA_ACCESS_UNSPECIFIED" => PolicyCameraAccess::CameraAccessUnspecified,
                "CAMERA_ACCESS_USER_CHOICE" => PolicyCameraAccess::CameraAccessUserChoice,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyCameraAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyCameraAccess {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyDefaultPermissionPolicy {
        #[doc = "Automatically deny a permission."]
        Deny,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
    }
    impl PolicyDefaultPermissionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyDefaultPermissionPolicy::Deny => "DENY",
                PolicyDefaultPermissionPolicy::Grant => "GRANT",
                PolicyDefaultPermissionPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                PolicyDefaultPermissionPolicy::Prompt => "PROMPT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyDefaultPermissionPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyDefaultPermissionPolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyDefaultPermissionPolicy, ()> {
            Ok(match s {
                "DENY" => PolicyDefaultPermissionPolicy::Deny,
                "GRANT" => PolicyDefaultPermissionPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PolicyDefaultPermissionPolicy::Prompt,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyDefaultPermissionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyDefaultPermissionPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyDefaultPermissionPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DENY" => PolicyDefaultPermissionPolicy::Deny,
                "GRANT" => PolicyDefaultPermissionPolicy::Grant,
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PolicyDefaultPermissionPolicy::Prompt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyDefaultPermissionPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyDefaultPermissionPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyEncryptionPolicy {
        #[doc = "Encryption required with password required to boot"]
        EnabledWithPassword,
        #[doc = "Encryption required but no password required to boot"]
        EnabledWithoutPassword,
        #[doc = "This value is ignored, i.e. no encryption required"]
        EncryptionPolicyUnspecified,
    }
    impl PolicyEncryptionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyEncryptionPolicy::EnabledWithPassword => "ENABLED_WITH_PASSWORD",
                PolicyEncryptionPolicy::EnabledWithoutPassword => "ENABLED_WITHOUT_PASSWORD",
                PolicyEncryptionPolicy::EncryptionPolicyUnspecified => {
                    "ENCRYPTION_POLICY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyEncryptionPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyEncryptionPolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyEncryptionPolicy, ()> {
            Ok(match s {
                "ENABLED_WITH_PASSWORD" => PolicyEncryptionPolicy::EnabledWithPassword,
                "ENABLED_WITHOUT_PASSWORD" => PolicyEncryptionPolicy::EnabledWithoutPassword,
                "ENCRYPTION_POLICY_UNSPECIFIED" => {
                    PolicyEncryptionPolicy::EncryptionPolicyUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyEncryptionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyEncryptionPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyEncryptionPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED_WITH_PASSWORD" => PolicyEncryptionPolicy::EnabledWithPassword,
                "ENABLED_WITHOUT_PASSWORD" => PolicyEncryptionPolicy::EnabledWithoutPassword,
                "ENCRYPTION_POLICY_UNSPECIFIED" => {
                    PolicyEncryptionPolicy::EncryptionPolicyUnspecified
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
    impl ::google_field_selector::FieldSelector for PolicyEncryptionPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyEncryptionPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyKeyguardDisabledFeaturesItems {
        #[doc = "Disable all current and future keyguard customizations."]
        AllFeatures,
        #[doc = "Disable all biometric authentication on secure keyguard screens."]
        Biometrics,
        #[doc = "Disable the camera on secure keyguard screens (e.g. PIN)."]
        Camera,
        #[doc = "Disable fingerprint sensor on secure keyguard screens."]
        DisableFingerprint,
        #[doc = "On devices running Android 6 and below, disables text entry into notifications on secure keyguard screens. Has no effect on Android 7 and above."]
        DisableRemoteInput,
        #[doc = "Disable face authentication on secure keyguard screens."]
        Face,
        #[doc = "Disable iris authentication on secure keyguard screens."]
        Iris,
        #[doc = "This value is ignored."]
        KeyguardDisabledFeatureUnspecified,
        #[doc = "Disable showing all notifications on secure keyguard screens."]
        Notifications,
        #[doc = "Ignore trust agent state on secure keyguard screens."]
        TrustAgents,
        #[doc = "Disable unredacted notifications on secure keyguard screens."]
        UnredactedNotifications,
    }
    impl PolicyKeyguardDisabledFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyKeyguardDisabledFeaturesItems::AllFeatures => "ALL_FEATURES",
                PolicyKeyguardDisabledFeaturesItems::Biometrics => "BIOMETRICS",
                PolicyKeyguardDisabledFeaturesItems::Camera => "CAMERA",
                PolicyKeyguardDisabledFeaturesItems::DisableFingerprint => "DISABLE_FINGERPRINT",
                PolicyKeyguardDisabledFeaturesItems::DisableRemoteInput => "DISABLE_REMOTE_INPUT",
                PolicyKeyguardDisabledFeaturesItems::Face => "FACE",
                PolicyKeyguardDisabledFeaturesItems::Iris => "IRIS",
                PolicyKeyguardDisabledFeaturesItems::KeyguardDisabledFeatureUnspecified => {
                    "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED"
                }
                PolicyKeyguardDisabledFeaturesItems::Notifications => "NOTIFICATIONS",
                PolicyKeyguardDisabledFeaturesItems::TrustAgents => "TRUST_AGENTS",
                PolicyKeyguardDisabledFeaturesItems::UnredactedNotifications => {
                    "UNREDACTED_NOTIFICATIONS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyKeyguardDisabledFeaturesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyKeyguardDisabledFeaturesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyKeyguardDisabledFeaturesItems, ()> {
            Ok(match s {
                "ALL_FEATURES" => PolicyKeyguardDisabledFeaturesItems::AllFeatures,
                "BIOMETRICS" => PolicyKeyguardDisabledFeaturesItems::Biometrics,
                "CAMERA" => PolicyKeyguardDisabledFeaturesItems::Camera,
                "DISABLE_FINGERPRINT" => PolicyKeyguardDisabledFeaturesItems::DisableFingerprint,
                "DISABLE_REMOTE_INPUT" => PolicyKeyguardDisabledFeaturesItems::DisableRemoteInput,
                "FACE" => PolicyKeyguardDisabledFeaturesItems::Face,
                "IRIS" => PolicyKeyguardDisabledFeaturesItems::Iris,
                "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED" => {
                    PolicyKeyguardDisabledFeaturesItems::KeyguardDisabledFeatureUnspecified
                }
                "NOTIFICATIONS" => PolicyKeyguardDisabledFeaturesItems::Notifications,
                "TRUST_AGENTS" => PolicyKeyguardDisabledFeaturesItems::TrustAgents,
                "UNREDACTED_NOTIFICATIONS" => {
                    PolicyKeyguardDisabledFeaturesItems::UnredactedNotifications
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyKeyguardDisabledFeaturesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyKeyguardDisabledFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyKeyguardDisabledFeaturesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_FEATURES" => PolicyKeyguardDisabledFeaturesItems::AllFeatures,
                "BIOMETRICS" => PolicyKeyguardDisabledFeaturesItems::Biometrics,
                "CAMERA" => PolicyKeyguardDisabledFeaturesItems::Camera,
                "DISABLE_FINGERPRINT" => PolicyKeyguardDisabledFeaturesItems::DisableFingerprint,
                "DISABLE_REMOTE_INPUT" => PolicyKeyguardDisabledFeaturesItems::DisableRemoteInput,
                "FACE" => PolicyKeyguardDisabledFeaturesItems::Face,
                "IRIS" => PolicyKeyguardDisabledFeaturesItems::Iris,
                "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED" => {
                    PolicyKeyguardDisabledFeaturesItems::KeyguardDisabledFeatureUnspecified
                }
                "NOTIFICATIONS" => PolicyKeyguardDisabledFeaturesItems::Notifications,
                "TRUST_AGENTS" => PolicyKeyguardDisabledFeaturesItems::TrustAgents,
                "UNREDACTED_NOTIFICATIONS" => {
                    PolicyKeyguardDisabledFeaturesItems::UnredactedNotifications
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
    impl ::google_field_selector::FieldSelector for PolicyKeyguardDisabledFeaturesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyKeyguardDisabledFeaturesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyLocationMode {
        #[doc = "On Android 8 and below, only the network location provider is enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        BatterySaving,
        #[doc = "On Android 8 and below, all location detection methods are enabled, including GPS, networks, and other sensors. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        HighAccuracy,
        #[doc = "Disable location setting on the device."]
        LocationDisabled,
        #[doc = "Enable location setting on the device."]
        LocationEnforced,
        #[doc = "Defaults to LOCATION_USER_CHOICE."]
        LocationModeUnspecified,
        #[doc = "Location setting is not restricted on the device. No specific behavior is set or enforced."]
        LocationUserChoice,
        #[doc = "On Android 8 and below, location setting and accuracy are disabled. On Android 9 and above, this is equivalent to LOCATION_DISABLED."]
        Off,
        #[doc = "On Android 8 and below, only GPS and other sensors are enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        SensorsOnly,
    }
    impl PolicyLocationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyLocationMode::BatterySaving => "BATTERY_SAVING",
                PolicyLocationMode::HighAccuracy => "HIGH_ACCURACY",
                PolicyLocationMode::LocationDisabled => "LOCATION_DISABLED",
                PolicyLocationMode::LocationEnforced => "LOCATION_ENFORCED",
                PolicyLocationMode::LocationModeUnspecified => "LOCATION_MODE_UNSPECIFIED",
                PolicyLocationMode::LocationUserChoice => "LOCATION_USER_CHOICE",
                PolicyLocationMode::Off => "OFF",
                PolicyLocationMode::SensorsOnly => "SENSORS_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyLocationMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyLocationMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyLocationMode, ()> {
            Ok(match s {
                "BATTERY_SAVING" => PolicyLocationMode::BatterySaving,
                "HIGH_ACCURACY" => PolicyLocationMode::HighAccuracy,
                "LOCATION_DISABLED" => PolicyLocationMode::LocationDisabled,
                "LOCATION_ENFORCED" => PolicyLocationMode::LocationEnforced,
                "LOCATION_MODE_UNSPECIFIED" => PolicyLocationMode::LocationModeUnspecified,
                "LOCATION_USER_CHOICE" => PolicyLocationMode::LocationUserChoice,
                "OFF" => PolicyLocationMode::Off,
                "SENSORS_ONLY" => PolicyLocationMode::SensorsOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyLocationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyLocationMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyLocationMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BATTERY_SAVING" => PolicyLocationMode::BatterySaving,
                "HIGH_ACCURACY" => PolicyLocationMode::HighAccuracy,
                "LOCATION_DISABLED" => PolicyLocationMode::LocationDisabled,
                "LOCATION_ENFORCED" => PolicyLocationMode::LocationEnforced,
                "LOCATION_MODE_UNSPECIFIED" => PolicyLocationMode::LocationModeUnspecified,
                "LOCATION_USER_CHOICE" => PolicyLocationMode::LocationUserChoice,
                "OFF" => PolicyLocationMode::Off,
                "SENSORS_ONLY" => PolicyLocationMode::SensorsOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyLocationMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyLocationMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyMicrophoneAccess {
        #[doc = "The field unmute_microphone_disabled is ignored. The microphone on the device is disabled (for fully managed devices, this applies device-wide).The microphone access toggle has no effect as the microphone is disabled."]
        MicrophoneAccessDisabled,
        #[doc = "The field unmute_microphone_disabled is ignored. The microphone on the device is available. On devices running Android 12 and above, the user is unable to use the microphone access toggle. On devices which run Android 11 or below, this is equivalent to MICROPHONE_ACCESS_USER_CHOICE."]
        MicrophoneAccessEnforced,
        #[doc = "If unmute_microphone_disabled is true, this is equivalent to MICROPHONE_ACCESS_DISABLED. Otherwise, this is equivalent to MICROPHONE_ACCESS_USER_CHOICE."]
        MicrophoneAccessUnspecified,
        #[doc = "The field unmute_microphone_disabled is ignored. This is the default device behaviour: the microphone on the device is available. On Android 12 and above, the user can use the microphone access toggle."]
        MicrophoneAccessUserChoice,
    }
    impl PolicyMicrophoneAccess {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyMicrophoneAccess::MicrophoneAccessDisabled => "MICROPHONE_ACCESS_DISABLED",
                PolicyMicrophoneAccess::MicrophoneAccessEnforced => "MICROPHONE_ACCESS_ENFORCED",
                PolicyMicrophoneAccess::MicrophoneAccessUnspecified => {
                    "MICROPHONE_ACCESS_UNSPECIFIED"
                }
                PolicyMicrophoneAccess::MicrophoneAccessUserChoice => {
                    "MICROPHONE_ACCESS_USER_CHOICE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyMicrophoneAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyMicrophoneAccess {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyMicrophoneAccess, ()> {
            Ok(match s {
                "MICROPHONE_ACCESS_DISABLED" => PolicyMicrophoneAccess::MicrophoneAccessDisabled,
                "MICROPHONE_ACCESS_ENFORCED" => PolicyMicrophoneAccess::MicrophoneAccessEnforced,
                "MICROPHONE_ACCESS_UNSPECIFIED" => {
                    PolicyMicrophoneAccess::MicrophoneAccessUnspecified
                }
                "MICROPHONE_ACCESS_USER_CHOICE" => {
                    PolicyMicrophoneAccess::MicrophoneAccessUserChoice
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyMicrophoneAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyMicrophoneAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyMicrophoneAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MICROPHONE_ACCESS_DISABLED" => PolicyMicrophoneAccess::MicrophoneAccessDisabled,
                "MICROPHONE_ACCESS_ENFORCED" => PolicyMicrophoneAccess::MicrophoneAccessEnforced,
                "MICROPHONE_ACCESS_UNSPECIFIED" => {
                    PolicyMicrophoneAccess::MicrophoneAccessUnspecified
                }
                "MICROPHONE_ACCESS_USER_CHOICE" => {
                    PolicyMicrophoneAccess::MicrophoneAccessUserChoice
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
    impl ::google_field_selector::FieldSelector for PolicyMicrophoneAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyMicrophoneAccess {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyPlayStoreMode {
        #[doc = "All apps are available and any app that should not be on the device should be explicitly marked as 'BLOCKED' in the applications policy."]
        Blacklist,
        #[doc = "Unspecified. Defaults to WHITELIST."]
        PlayStoreModeUnspecified,
        #[doc = "Only apps that are in the policy are available and any app not in the policy will be automatically uninstalled from the device."]
        Whitelist,
    }
    impl PolicyPlayStoreMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyPlayStoreMode::Blacklist => "BLACKLIST",
                PolicyPlayStoreMode::PlayStoreModeUnspecified => "PLAY_STORE_MODE_UNSPECIFIED",
                PolicyPlayStoreMode::Whitelist => "WHITELIST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyPlayStoreMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyPlayStoreMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyPlayStoreMode, ()> {
            Ok(match s {
                "BLACKLIST" => PolicyPlayStoreMode::Blacklist,
                "PLAY_STORE_MODE_UNSPECIFIED" => PolicyPlayStoreMode::PlayStoreModeUnspecified,
                "WHITELIST" => PolicyPlayStoreMode::Whitelist,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyPlayStoreMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyPlayStoreMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyPlayStoreMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLACKLIST" => PolicyPlayStoreMode::Blacklist,
                "PLAY_STORE_MODE_UNSPECIFIED" => PolicyPlayStoreMode::PlayStoreModeUnspecified,
                "WHITELIST" => PolicyPlayStoreMode::Whitelist,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyPlayStoreMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyPlayStoreMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyPreferentialNetworkService {
        #[doc = "Preferential network service is disabled on the work profile."]
        PreferentialNetworkServiceDisabled,
        #[doc = "Preferential network service is enabled on the work profile."]
        PreferentialNetworkServiceEnabled,
        #[doc = "Unspecified. Defaults to PREFERENTIAL_NETWORK_SERVICES_DISABLED."]
        PreferentialNetworkServiceUnspecified,
    }
    impl PolicyPreferentialNetworkService {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyPreferentialNetworkService::PreferentialNetworkServiceDisabled => {
                    "PREFERENTIAL_NETWORK_SERVICE_DISABLED"
                }
                PolicyPreferentialNetworkService::PreferentialNetworkServiceEnabled => {
                    "PREFERENTIAL_NETWORK_SERVICE_ENABLED"
                }
                PolicyPreferentialNetworkService::PreferentialNetworkServiceUnspecified => {
                    "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyPreferentialNetworkService {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyPreferentialNetworkService {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyPreferentialNetworkService, ()> {
            Ok(match s {
                "PREFERENTIAL_NETWORK_SERVICE_DISABLED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceDisabled
                }
                "PREFERENTIAL_NETWORK_SERVICE_ENABLED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceEnabled
                }
                "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyPreferentialNetworkService {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyPreferentialNetworkService {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyPreferentialNetworkService {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PREFERENTIAL_NETWORK_SERVICE_DISABLED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceDisabled
                }
                "PREFERENTIAL_NETWORK_SERVICE_ENABLED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceEnabled
                }
                "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED" => {
                    PolicyPreferentialNetworkService::PreferentialNetworkServiceUnspecified
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
    impl ::google_field_selector::FieldSelector for PolicyPreferentialNetworkService {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyPreferentialNetworkService {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyStayOnPluggedModesItems {
        #[doc = "Power source is an AC charger."]
        Ac,
        #[doc = "This value is ignored."]
        BatteryPluggedModeUnspecified,
        #[doc = "Power source is a USB port."]
        Usb,
        #[doc = "Power source is wireless."]
        Wireless,
    }
    impl PolicyStayOnPluggedModesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyStayOnPluggedModesItems::Ac => "AC",
                PolicyStayOnPluggedModesItems::BatteryPluggedModeUnspecified => {
                    "BATTERY_PLUGGED_MODE_UNSPECIFIED"
                }
                PolicyStayOnPluggedModesItems::Usb => "USB",
                PolicyStayOnPluggedModesItems::Wireless => "WIRELESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyStayOnPluggedModesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyStayOnPluggedModesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyStayOnPluggedModesItems, ()> {
            Ok(match s {
                "AC" => PolicyStayOnPluggedModesItems::Ac,
                "BATTERY_PLUGGED_MODE_UNSPECIFIED" => {
                    PolicyStayOnPluggedModesItems::BatteryPluggedModeUnspecified
                }
                "USB" => PolicyStayOnPluggedModesItems::Usb,
                "WIRELESS" => PolicyStayOnPluggedModesItems::Wireless,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyStayOnPluggedModesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyStayOnPluggedModesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyStayOnPluggedModesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AC" => PolicyStayOnPluggedModesItems::Ac,
                "BATTERY_PLUGGED_MODE_UNSPECIFIED" => {
                    PolicyStayOnPluggedModesItems::BatteryPluggedModeUnspecified
                }
                "USB" => PolicyStayOnPluggedModesItems::Usb,
                "WIRELESS" => PolicyStayOnPluggedModesItems::Wireless,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyStayOnPluggedModesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyStayOnPluggedModesItems {
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
    pub struct PolicyEnforcementRule {
        #[doc = "An action to block access to apps and data on a fully managed device or in a work profile. This action also triggers a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified."]
        #[serde(
            rename = "blockAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_action: ::std::option::Option<crate::schemas::BlockAction>,
        #[doc = "The top-level policy to enforce. For example, applications or passwordPolicies."]
        #[serde(
            rename = "settingName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub setting_name: ::std::option::Option<String>,
        #[doc = "An action to reset a fully managed device or delete a work profile. Note: blockAction must also be specified."]
        #[serde(
            rename = "wipeAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wipe_action: ::std::option::Option<crate::schemas::WipeAction>,
    }
    impl ::google_field_selector::FieldSelector for PolicyEnforcementRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyEnforcementRule {
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
    pub struct PostureDetail {
        #[doc = "Corresponding admin-facing advice to mitigate this security risk and improve the security posture of the device."]
        #[serde(
            rename = "advice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advice: ::std::option::Option<Vec<crate::schemas::UserFacingMessage>>,
        #[doc = "A specific security risk that negatively affects the security posture of the device."]
        #[serde(
            rename = "securityRisk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_risk: ::std::option::Option<crate::schemas::PostureDetailSecurityRisk>,
    }
    impl ::google_field_selector::FieldSelector for PostureDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PostureDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PostureDetailSecurityRisk {
        #[doc = "SafetyNet detects that the device is running a compromised OS (basicIntegrity check fails)."]
        CompromisedOs,
        #[doc = "SafetyNet detects that the device does not have a strong guarantee of system integrity, such as a hardware-backed keystore (https://developer.android.com/training/articles/security-key-attestation)."]
        HardwareBackedEvaluationFailed,
        #[doc = "Unspecified."]
        SecurityRiskUnspecified,
        #[doc = "SafetyNet detects that the device is running an unknown OS (basicIntegrity check succeeds but ctsProfileMatch fails)."]
        UnknownOs,
    }
    impl PostureDetailSecurityRisk {
        pub fn as_str(self) -> &'static str {
            match self {
                PostureDetailSecurityRisk::CompromisedOs => "COMPROMISED_OS",
                PostureDetailSecurityRisk::HardwareBackedEvaluationFailed => {
                    "HARDWARE_BACKED_EVALUATION_FAILED"
                }
                PostureDetailSecurityRisk::SecurityRiskUnspecified => "SECURITY_RISK_UNSPECIFIED",
                PostureDetailSecurityRisk::UnknownOs => "UNKNOWN_OS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PostureDetailSecurityRisk {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PostureDetailSecurityRisk {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PostureDetailSecurityRisk, ()> {
            Ok(match s {
                "COMPROMISED_OS" => PostureDetailSecurityRisk::CompromisedOs,
                "HARDWARE_BACKED_EVALUATION_FAILED" => {
                    PostureDetailSecurityRisk::HardwareBackedEvaluationFailed
                }
                "SECURITY_RISK_UNSPECIFIED" => PostureDetailSecurityRisk::SecurityRiskUnspecified,
                "UNKNOWN_OS" => PostureDetailSecurityRisk::UnknownOs,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PostureDetailSecurityRisk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PostureDetailSecurityRisk {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostureDetailSecurityRisk {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPROMISED_OS" => PostureDetailSecurityRisk::CompromisedOs,
                "HARDWARE_BACKED_EVALUATION_FAILED" => {
                    PostureDetailSecurityRisk::HardwareBackedEvaluationFailed
                }
                "SECURITY_RISK_UNSPECIFIED" => PostureDetailSecurityRisk::SecurityRiskUnspecified,
                "UNKNOWN_OS" => PostureDetailSecurityRisk::UnknownOs,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PostureDetailSecurityRisk {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PostureDetailSecurityRisk {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PowerManagementEvent {
        #[doc = "For BATTERY_LEVEL_COLLECTED events, the battery level as a percentage."]
        #[serde(
            rename = "batteryLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_level: ::std::option::Option<f32>,
        #[doc = "The creation time of the event."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Event type."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type: ::std::option::Option<crate::schemas::PowerManagementEventEventType>,
    }
    impl ::google_field_selector::FieldSelector for PowerManagementEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PowerManagementEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PowerManagementEventEventType {
        #[doc = "Battery level was measured."]
        BatteryLevelCollected,
        #[doc = "The device entered low-power mode."]
        BatteryLow,
        #[doc = "The device exited low-power mode."]
        BatteryOkay,
        #[doc = "The device booted."]
        BootCompleted,
        #[doc = "The device started charging."]
        PowerConnected,
        #[doc = "The device stopped charging."]
        PowerDisconnected,
        #[doc = "Unspecified. No events have this type."]
        PowerManagementEventTypeUnspecified,
        #[doc = "The device shut down."]
        Shutdown,
    }
    impl PowerManagementEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                PowerManagementEventEventType::BatteryLevelCollected => "BATTERY_LEVEL_COLLECTED",
                PowerManagementEventEventType::BatteryLow => "BATTERY_LOW",
                PowerManagementEventEventType::BatteryOkay => "BATTERY_OKAY",
                PowerManagementEventEventType::BootCompleted => "BOOT_COMPLETED",
                PowerManagementEventEventType::PowerConnected => "POWER_CONNECTED",
                PowerManagementEventEventType::PowerDisconnected => "POWER_DISCONNECTED",
                PowerManagementEventEventType::PowerManagementEventTypeUnspecified => {
                    "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED"
                }
                PowerManagementEventEventType::Shutdown => "SHUTDOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PowerManagementEventEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PowerManagementEventEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PowerManagementEventEventType, ()> {
            Ok(match s {
                "BATTERY_LEVEL_COLLECTED" => PowerManagementEventEventType::BatteryLevelCollected,
                "BATTERY_LOW" => PowerManagementEventEventType::BatteryLow,
                "BATTERY_OKAY" => PowerManagementEventEventType::BatteryOkay,
                "BOOT_COMPLETED" => PowerManagementEventEventType::BootCompleted,
                "POWER_CONNECTED" => PowerManagementEventEventType::PowerConnected,
                "POWER_DISCONNECTED" => PowerManagementEventEventType::PowerDisconnected,
                "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED" => {
                    PowerManagementEventEventType::PowerManagementEventTypeUnspecified
                }
                "SHUTDOWN" => PowerManagementEventEventType::Shutdown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PowerManagementEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PowerManagementEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PowerManagementEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BATTERY_LEVEL_COLLECTED" => PowerManagementEventEventType::BatteryLevelCollected,
                "BATTERY_LOW" => PowerManagementEventEventType::BatteryLow,
                "BATTERY_OKAY" => PowerManagementEventEventType::BatteryOkay,
                "BOOT_COMPLETED" => PowerManagementEventEventType::BootCompleted,
                "POWER_CONNECTED" => PowerManagementEventEventType::PowerConnected,
                "POWER_DISCONNECTED" => PowerManagementEventEventType::PowerDisconnected,
                "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED" => {
                    PowerManagementEventEventType::PowerManagementEventTypeUnspecified
                }
                "SHUTDOWN" => PowerManagementEventEventType::Shutdown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PowerManagementEventEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PowerManagementEventEventType {
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
    pub struct ProxyInfo {
        #[doc = "For a direct proxy, the hosts for which the proxy is bypassed. The host names may contain wildcards such as *.example.com."]
        #[serde(
            rename = "excludedHosts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_hosts: ::std::option::Option<Vec<String>>,
        #[doc = "The host of the direct proxy."]
        #[serde(
            rename = "host",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host: ::std::option::Option<String>,
        #[doc = "The URI of the PAC script used to configure the proxy."]
        #[serde(
            rename = "pacUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pac_uri: ::std::option::Option<String>,
        #[doc = "The port of the direct proxy."]
        #[serde(
            rename = "port",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub port: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ProxyInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProxyInfo {
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
    pub struct SecurityPosture {
        #[doc = "Device's security posture value."]
        #[serde(
            rename = "devicePosture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_posture: ::std::option::Option<crate::schemas::SecurityPostureDevicePosture>,
        #[doc = "Additional details regarding the security posture of the device."]
        #[serde(
            rename = "postureDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posture_details: ::std::option::Option<Vec<crate::schemas::PostureDetail>>,
    }
    impl ::google_field_selector::FieldSelector for SecurityPosture {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SecurityPosture {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SecurityPostureDevicePosture {
        #[doc = "This device may be more vulnerable to malicious actors than is recommended for use with corporate data."]
        AtRisk,
        #[doc = "Unspecified. There is no posture detail for this posture value."]
        PostureUnspecified,
        #[doc = "This device may be compromised and corporate data may be accessible to unauthorized actors."]
        PotentiallyCompromised,
        #[doc = "This device is secure."]
        Secure,
    }
    impl SecurityPostureDevicePosture {
        pub fn as_str(self) -> &'static str {
            match self {
                SecurityPostureDevicePosture::AtRisk => "AT_RISK",
                SecurityPostureDevicePosture::PostureUnspecified => "POSTURE_UNSPECIFIED",
                SecurityPostureDevicePosture::PotentiallyCompromised => "POTENTIALLY_COMPROMISED",
                SecurityPostureDevicePosture::Secure => "SECURE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SecurityPostureDevicePosture {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SecurityPostureDevicePosture {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SecurityPostureDevicePosture, ()> {
            Ok(match s {
                "AT_RISK" => SecurityPostureDevicePosture::AtRisk,
                "POSTURE_UNSPECIFIED" => SecurityPostureDevicePosture::PostureUnspecified,
                "POTENTIALLY_COMPROMISED" => SecurityPostureDevicePosture::PotentiallyCompromised,
                "SECURE" => SecurityPostureDevicePosture::Secure,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SecurityPostureDevicePosture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SecurityPostureDevicePosture {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SecurityPostureDevicePosture {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AT_RISK" => SecurityPostureDevicePosture::AtRisk,
                "POSTURE_UNSPECIFIED" => SecurityPostureDevicePosture::PostureUnspecified,
                "POTENTIALLY_COMPROMISED" => SecurityPostureDevicePosture::PotentiallyCompromised,
                "SECURE" => SecurityPostureDevicePosture::Secure,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SecurityPostureDevicePosture {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SecurityPostureDevicePosture {
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
    pub struct SetupAction {
        #[doc = "Description of this action."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "An action to launch an app. The app will be launched with an intent containing an extra with key com.google.android.apps.work.clouddpc.EXTRA_LAUNCHED_AS_SETUP_ACTION set to the boolean value true to indicate that this is a setup action flow."]
        #[serde(
            rename = "launchApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_app: ::std::option::Option<crate::schemas::LaunchAppAction>,
        #[doc = "Title of this action."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<crate::schemas::UserFacingMessage>,
    }
    impl ::google_field_selector::FieldSelector for SetupAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetupAction {
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
    pub struct SigninDetail {
        #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
        #[serde(
            rename = "allowPersonalUsage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_personal_usage:
            ::std::option::Option<crate::schemas::SigninDetailAllowPersonalUsage>,
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON. This is a read-only field generated by the server."]
        #[serde(
            rename = "qrCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qr_code: ::std::option::Option<String>,
        #[doc = "An enterprise wide enrollment token used to trigger custom sign-in flow. This is a read-only field generated by the server."]
        #[serde(
            rename = "signinEnrollmentToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signin_enrollment_token: ::std::option::Option<String>,
        #[doc = "Sign-in URL for authentication when device is provisioned with a sign-in enrollment token. The sign-in endpoint should finish authentication flow with a URL in the form of https://enterprise.google.com/android/enroll?et= for a successful login, or https://enterprise.google.com/android/enroll/invalid for a failed login."]
        #[serde(
            rename = "signinUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signin_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SigninDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SigninDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SigninDetailAllowPersonalUsage {
        #[doc = "Personal usage restriction is not specified"]
        AllowPersonalUsageUnspecified,
        #[doc = "Personal usage is allowed"]
        PersonalUsageAllowed,
        #[doc = "Personal usage is disallowed"]
        PersonalUsageDisallowed,
    }
    impl SigninDetailAllowPersonalUsage {
        pub fn as_str(self) -> &'static str {
            match self {
                SigninDetailAllowPersonalUsage::AllowPersonalUsageUnspecified => {
                    "ALLOW_PERSONAL_USAGE_UNSPECIFIED"
                }
                SigninDetailAllowPersonalUsage::PersonalUsageAllowed => "PERSONAL_USAGE_ALLOWED",
                SigninDetailAllowPersonalUsage::PersonalUsageDisallowed => {
                    "PERSONAL_USAGE_DISALLOWED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for SigninDetailAllowPersonalUsage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SigninDetailAllowPersonalUsage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SigninDetailAllowPersonalUsage, ()> {
            Ok(match s {
                "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => {
                    SigninDetailAllowPersonalUsage::AllowPersonalUsageUnspecified
                }
                "PERSONAL_USAGE_ALLOWED" => SigninDetailAllowPersonalUsage::PersonalUsageAllowed,
                "PERSONAL_USAGE_DISALLOWED" => {
                    SigninDetailAllowPersonalUsage::PersonalUsageDisallowed
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SigninDetailAllowPersonalUsage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SigninDetailAllowPersonalUsage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SigninDetailAllowPersonalUsage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => {
                    SigninDetailAllowPersonalUsage::AllowPersonalUsageUnspecified
                }
                "PERSONAL_USAGE_ALLOWED" => SigninDetailAllowPersonalUsage::PersonalUsageAllowed,
                "PERSONAL_USAGE_DISALLOWED" => {
                    SigninDetailAllowPersonalUsage::PersonalUsageDisallowed
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
    impl ::google_field_selector::FieldSelector for SigninDetailAllowPersonalUsage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SigninDetailAllowPersonalUsage {
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
    pub struct SignupUrl {
        #[doc = "The name of the resource. Use this value in the signupUrl field when calling enterprises.create to complete the enterprise signup flow."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A URL where an enterprise admin can register their enterprise. The page can't be rendered in an iframe."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SignupUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SignupUrl {
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
    pub struct SoftwareInfo {
        #[doc = "Android build ID string meant for displaying to the user. For example, shamu-userdebug 6.0.1 MOB30I 2756745 dev-keys."]
        #[serde(
            rename = "androidBuildNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_build_number: ::std::option::Option<String>,
        #[doc = "Build time."]
        #[serde(
            rename = "androidBuildTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_build_time: ::std::option::Option<String>,
        #[doc = "The Android Device Policy app version code."]
        #[serde(
            rename = "androidDevicePolicyVersionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device_policy_version_code: ::std::option::Option<i32>,
        #[doc = "The Android Device Policy app version as displayed to the user."]
        #[serde(
            rename = "androidDevicePolicyVersionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device_policy_version_name: ::std::option::Option<String>,
        #[doc = "The user-visible Android version string. For example, 6.0.1."]
        #[serde(
            rename = "androidVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_version: ::std::option::Option<String>,
        #[doc = "The system bootloader version number, e.g. 0.6.7."]
        #[serde(
            rename = "bootloaderVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bootloader_version: ::std::option::Option<String>,
        #[doc = "SHA-256 hash of android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the system package, which can be used to verify that the system build hasn't been modified."]
        #[serde(
            rename = "deviceBuildSignature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_build_signature: ::std::option::Option<String>,
        #[doc = "Kernel version, for example, 2.6.32.9-g103d848."]
        #[serde(
            rename = "deviceKernelVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_kernel_version: ::std::option::Option<String>,
        #[doc = "An IETF BCP 47 language code for the primary locale on the device."]
        #[serde(
            rename = "primaryLanguageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_language_code: ::std::option::Option<String>,
        #[doc = "Security patch level, e.g. 2016-05-01."]
        #[serde(
            rename = "securityPatchLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_patch_level: ::std::option::Option<String>,
        #[doc = "Information about a potential pending system update."]
        #[serde(
            rename = "systemUpdateInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_update_info: ::std::option::Option<crate::schemas::SystemUpdateInfo>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareInfo {
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
    pub struct StatusReportingSettings {
        #[doc = "Application reporting settings. Only applicable if application_reports_enabled is true."]
        #[serde(
            rename = "applicationReportingSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_reporting_settings:
            ::std::option::Option<crate::schemas::ApplicationReportingSettings>,
        #[doc = "Whether app reports are enabled."]
        #[serde(
            rename = "applicationReportsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_reports_enabled: ::std::option::Option<bool>,
        #[doc = "Whether Common Criteria Mode reporting is enabled."]
        #[serde(
            rename = "commonCriteriaModeEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_criteria_mode_enabled: ::std::option::Option<bool>,
        #[doc = "Whether device settings reporting is enabled."]
        #[serde(
            rename = "deviceSettingsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_settings_enabled: ::std::option::Option<bool>,
        #[doc = "Whether displays reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        #[serde(
            rename = "displayInfoEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_info_enabled: ::std::option::Option<bool>,
        #[doc = "Whether hardware status reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        #[serde(
            rename = "hardwareStatusEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hardware_status_enabled: ::std::option::Option<bool>,
        #[doc = "Whether memory event reporting is enabled."]
        #[serde(
            rename = "memoryInfoEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_info_enabled: ::std::option::Option<bool>,
        #[doc = "Whether network info reporting is enabled."]
        #[serde(
            rename = "networkInfoEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_info_enabled: ::std::option::Option<bool>,
        #[doc = "Whether power management event reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        #[serde(
            rename = "powerManagementEventsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub power_management_events_enabled: ::std::option::Option<bool>,
        #[doc = "Whether software info reporting is enabled."]
        #[serde(
            rename = "softwareInfoEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_info_enabled: ::std::option::Option<bool>,
        #[doc = "Whether system properties reporting is enabled."]
        #[serde(
            rename = "systemPropertiesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_properties_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for StatusReportingSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StatusReportingSettings {
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
    pub struct SystemUpdate {
        #[doc = "If the type is WINDOWED, the end of the maintenance window, measured as the number of minutes after midnight in device's local time. This value must be between 0 and 1439, inclusive. If this value is less than start_minutes, then the maintenance window spans midnight. If the maintenance window specified is smaller than 30 minutes, the actual window is extended to 30 minutes beyond the start time."]
        #[serde(
            rename = "endMinutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_minutes: ::std::option::Option<i32>,
        #[doc = "An annually repeating time period in which over-the-air (OTA) system updates are postponed to freeze the OS version running on a device. To prevent freezing the device indefinitely, each freeze period must be separated by at least 60 days."]
        #[serde(
            rename = "freezePeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freeze_periods: ::std::option::Option<Vec<crate::schemas::FreezePeriod>>,
        #[doc = "The type of system update to configure."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SystemUpdateType>,
        #[doc = "If the type is WINDOWED, the start of the maintenance window, measured as the number of minutes after midnight in the device's local time. This value must be between 0 and 1439, inclusive."]
        #[serde(
            rename = "startMinutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_minutes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SystemUpdate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemUpdate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SystemUpdateType {
        #[doc = "Install automatically as soon as an update is available."]
        Automatic,
        #[doc = "Postpone automatic install up to a maximum of 30 days."]
        Postpone,
        #[doc = "Follow the default update behavior for the device, which typically requires the user to accept system updates."]
        SystemUpdateTypeUnspecified,
        #[doc = "Install automatically within a daily maintenance window. This also configures Play apps to be updated within the window. This is strongly recommended for kiosk devices because this is the only way apps persistently pinned to the foreground can be updated by Play.If autoUpdateMode is set to AUTO_UPDATE_HIGH_PRIORITY for an app, then the maintenance window is ignored for that app and it is updated as soon as possible even outside of the maintenance window."]
        Windowed,
    }
    impl SystemUpdateType {
        pub fn as_str(self) -> &'static str {
            match self {
                SystemUpdateType::Automatic => "AUTOMATIC",
                SystemUpdateType::Postpone => "POSTPONE",
                SystemUpdateType::SystemUpdateTypeUnspecified => "SYSTEM_UPDATE_TYPE_UNSPECIFIED",
                SystemUpdateType::Windowed => "WINDOWED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SystemUpdateType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SystemUpdateType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SystemUpdateType, ()> {
            Ok(match s {
                "AUTOMATIC" => SystemUpdateType::Automatic,
                "POSTPONE" => SystemUpdateType::Postpone,
                "SYSTEM_UPDATE_TYPE_UNSPECIFIED" => SystemUpdateType::SystemUpdateTypeUnspecified,
                "WINDOWED" => SystemUpdateType::Windowed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SystemUpdateType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SystemUpdateType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SystemUpdateType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTOMATIC" => SystemUpdateType::Automatic,
                "POSTPONE" => SystemUpdateType::Postpone,
                "SYSTEM_UPDATE_TYPE_UNSPECIFIED" => SystemUpdateType::SystemUpdateTypeUnspecified,
                "WINDOWED" => SystemUpdateType::Windowed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SystemUpdateType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemUpdateType {
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
    pub struct SystemUpdateInfo {
        #[doc = "The time when the update was first available. A zero value indicates that this field is not set. This field is set only if an update is available (that is, updateStatus is neither UPDATE_STATUS_UNKNOWN nor UP_TO_DATE)."]
        #[serde(
            rename = "updateReceivedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_received_time: ::std::option::Option<String>,
        #[doc = "The status of an update: whether an update exists and what type it is."]
        #[serde(
            rename = "updateStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_status: ::std::option::Option<crate::schemas::SystemUpdateInfoUpdateStatus>,
    }
    impl ::google_field_selector::FieldSelector for SystemUpdateInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemUpdateInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SystemUpdateInfoUpdateStatus {
        #[doc = "There is a pending OS update available."]
        OsUpdateAvailable,
        #[doc = "There is a pending security update available."]
        SecurityUpdateAvailable,
        #[doc = "There is a pending system update available, but its type is not known."]
        UnknownUpdateAvailable,
        #[doc = "There is no pending system update available on the device."]
        UpToDate,
        #[doc = "It is unknown whether there is a pending system update. This happens when, for example, the device API level is less than 26, or if the version of Android Device Policy is outdated."]
        UpdateStatusUnknown,
    }
    impl SystemUpdateInfoUpdateStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                SystemUpdateInfoUpdateStatus::OsUpdateAvailable => "OS_UPDATE_AVAILABLE",
                SystemUpdateInfoUpdateStatus::SecurityUpdateAvailable => {
                    "SECURITY_UPDATE_AVAILABLE"
                }
                SystemUpdateInfoUpdateStatus::UnknownUpdateAvailable => "UNKNOWN_UPDATE_AVAILABLE",
                SystemUpdateInfoUpdateStatus::UpToDate => "UP_TO_DATE",
                SystemUpdateInfoUpdateStatus::UpdateStatusUnknown => "UPDATE_STATUS_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SystemUpdateInfoUpdateStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SystemUpdateInfoUpdateStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SystemUpdateInfoUpdateStatus, ()> {
            Ok(match s {
                "OS_UPDATE_AVAILABLE" => SystemUpdateInfoUpdateStatus::OsUpdateAvailable,
                "SECURITY_UPDATE_AVAILABLE" => {
                    SystemUpdateInfoUpdateStatus::SecurityUpdateAvailable
                }
                "UNKNOWN_UPDATE_AVAILABLE" => SystemUpdateInfoUpdateStatus::UnknownUpdateAvailable,
                "UP_TO_DATE" => SystemUpdateInfoUpdateStatus::UpToDate,
                "UPDATE_STATUS_UNKNOWN" => SystemUpdateInfoUpdateStatus::UpdateStatusUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SystemUpdateInfoUpdateStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SystemUpdateInfoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SystemUpdateInfoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OS_UPDATE_AVAILABLE" => SystemUpdateInfoUpdateStatus::OsUpdateAvailable,
                "SECURITY_UPDATE_AVAILABLE" => {
                    SystemUpdateInfoUpdateStatus::SecurityUpdateAvailable
                }
                "UNKNOWN_UPDATE_AVAILABLE" => SystemUpdateInfoUpdateStatus::UnknownUpdateAvailable,
                "UP_TO_DATE" => SystemUpdateInfoUpdateStatus::UpToDate,
                "UPDATE_STATUS_UNKNOWN" => SystemUpdateInfoUpdateStatus::UpdateStatusUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SystemUpdateInfoUpdateStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemUpdateInfoUpdateStatus {
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
    pub struct TelephonyInfo {
        #[doc = "The carrier name associated with this SIM card."]
        #[serde(
            rename = "carrierName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub carrier_name: ::std::option::Option<String>,
        #[doc = "The phone number associated with this SIM card."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TelephonyInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TelephonyInfo {
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
    pub struct TermsAndConditions {
        #[doc = "A well-formatted HTML string. It will be parsed on the client with android.text.Html#fromHtml."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<crate::schemas::UserFacingMessage>,
        #[doc = "A short header which appears above the HTML content."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::UserFacingMessage>,
    }
    impl ::google_field_selector::FieldSelector for TermsAndConditions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TermsAndConditions {
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
    pub struct UsageLog {
        #[doc = "Specifies which log types are enabled. Note that users will receive on-device messaging when usage logging is enabled."]
        #[serde(
            rename = "enabledLogTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled_log_types:
            ::std::option::Option<Vec<crate::schemas::UsageLogEnabledLogTypesItems>>,
        #[doc = "Specifies which of the enabled log types can be uploaded over mobile data. By default logs are queued for upload when the device connects to WiFi."]
        #[serde(
            rename = "uploadOnCellularAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upload_on_cellular_allowed:
            ::std::option::Option<Vec<crate::schemas::UsageLogUploadOnCellularAllowedItems>>,
    }
    impl ::google_field_selector::FieldSelector for UsageLog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageLog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UsageLogEnabledLogTypesItems {
        #[doc = "This value is not used."]
        LogTypeUnspecified,
        #[doc = "Enable logging of on-device network events, like DNS lookups and TCP connections. See event for a complete description of the logged network events. Supported for fully managed devices on Android 8 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only network events from the work profile are logged."]
        NetworkActivityLogs,
        #[doc = "Enable logging of on-device security events, like when the device password is incorrectly entered or removable storage is mounted. See event for a complete description of the logged security events. Supported for fully managed devices on Android 7.0 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only security events from the work profile are logged."]
        SecurityLogs,
    }
    impl UsageLogEnabledLogTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                UsageLogEnabledLogTypesItems::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
                UsageLogEnabledLogTypesItems::NetworkActivityLogs => "NETWORK_ACTIVITY_LOGS",
                UsageLogEnabledLogTypesItems::SecurityLogs => "SECURITY_LOGS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UsageLogEnabledLogTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UsageLogEnabledLogTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UsageLogEnabledLogTypesItems, ()> {
            Ok(match s {
                "LOG_TYPE_UNSPECIFIED" => UsageLogEnabledLogTypesItems::LogTypeUnspecified,
                "NETWORK_ACTIVITY_LOGS" => UsageLogEnabledLogTypesItems::NetworkActivityLogs,
                "SECURITY_LOGS" => UsageLogEnabledLogTypesItems::SecurityLogs,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UsageLogEnabledLogTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UsageLogEnabledLogTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UsageLogEnabledLogTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOG_TYPE_UNSPECIFIED" => UsageLogEnabledLogTypesItems::LogTypeUnspecified,
                "NETWORK_ACTIVITY_LOGS" => UsageLogEnabledLogTypesItems::NetworkActivityLogs,
                "SECURITY_LOGS" => UsageLogEnabledLogTypesItems::SecurityLogs,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UsageLogEnabledLogTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageLogEnabledLogTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UsageLogUploadOnCellularAllowedItems {
        #[doc = "This value is not used."]
        LogTypeUnspecified,
        #[doc = "Enable logging of on-device network events, like DNS lookups and TCP connections. See event for a complete description of the logged network events. Supported for fully managed devices on Android 8 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only network events from the work profile are logged."]
        NetworkActivityLogs,
        #[doc = "Enable logging of on-device security events, like when the device password is incorrectly entered or removable storage is mounted. See event for a complete description of the logged security events. Supported for fully managed devices on Android 7.0 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only security events from the work profile are logged."]
        SecurityLogs,
    }
    impl UsageLogUploadOnCellularAllowedItems {
        pub fn as_str(self) -> &'static str {
            match self {
                UsageLogUploadOnCellularAllowedItems::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
                UsageLogUploadOnCellularAllowedItems::NetworkActivityLogs => {
                    "NETWORK_ACTIVITY_LOGS"
                }
                UsageLogUploadOnCellularAllowedItems::SecurityLogs => "SECURITY_LOGS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UsageLogUploadOnCellularAllowedItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UsageLogUploadOnCellularAllowedItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UsageLogUploadOnCellularAllowedItems, ()> {
            Ok(match s {
                "LOG_TYPE_UNSPECIFIED" => UsageLogUploadOnCellularAllowedItems::LogTypeUnspecified,
                "NETWORK_ACTIVITY_LOGS" => {
                    UsageLogUploadOnCellularAllowedItems::NetworkActivityLogs
                }
                "SECURITY_LOGS" => UsageLogUploadOnCellularAllowedItems::SecurityLogs,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UsageLogUploadOnCellularAllowedItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UsageLogUploadOnCellularAllowedItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UsageLogUploadOnCellularAllowedItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOG_TYPE_UNSPECIFIED" => UsageLogUploadOnCellularAllowedItems::LogTypeUnspecified,
                "NETWORK_ACTIVITY_LOGS" => {
                    UsageLogUploadOnCellularAllowedItems::NetworkActivityLogs
                }
                "SECURITY_LOGS" => UsageLogUploadOnCellularAllowedItems::SecurityLogs,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UsageLogUploadOnCellularAllowedItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageLogUploadOnCellularAllowedItems {
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
    pub struct User {
        #[doc = "A unique identifier you create for this user, such as user342 or asset#44418. This field must be set when the user is created and can't be updated. This field must not contain personally identifiable information (PII). This identifier must be 1024 characters or less; otherwise, the update policy request will fail."]
        #[serde(
            rename = "accountIdentifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_identifier: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for User {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for User {
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
    pub struct UserFacingMessage {
        #[doc = "The default message displayed if no localized message is specified or the user's locale doesn't match with any of the localized messages. A default message must be provided if any localized messages are provided."]
        #[serde(
            rename = "defaultMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_message: ::std::option::Option<String>,
        #[doc = "A map containing pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr."]
        #[serde(
            rename = "localizedMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub localized_messages: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for UserFacingMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserFacingMessage {
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
    pub struct WebApp {
        #[doc = "The display mode of the web app."]
        #[serde(
            rename = "displayMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_mode: ::std::option::Option<crate::schemas::WebAppDisplayMode>,
        #[doc = "A list of icons for the web app. Must have at least one element."]
        #[serde(
            rename = "icons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icons: ::std::option::Option<Vec<crate::schemas::WebAppIcon>>,
        #[doc = "The name of the web app, which is generated by the server during creation in the form enterprises/{enterpriseId}/webApps/{packageName}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The start URL, i.e. the URL that should load when the user opens the application."]
        #[serde(
            rename = "startUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_url: ::std::option::Option<String>,
        #[doc = "The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon)."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The current version of the app.Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date."]
        #[serde(
            rename = "versionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub version_code: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for WebApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WebAppDisplayMode {
        #[doc = "Not used."]
        DisplayModeUnspecified,
        #[doc = "Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area."]
        FullScreen,
        #[doc = "Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL."]
        MinimalUi,
        #[doc = "Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible."]
        Standalone,
    }
    impl WebAppDisplayMode {
        pub fn as_str(self) -> &'static str {
            match self {
                WebAppDisplayMode::DisplayModeUnspecified => "DISPLAY_MODE_UNSPECIFIED",
                WebAppDisplayMode::FullScreen => "FULL_SCREEN",
                WebAppDisplayMode::MinimalUi => "MINIMAL_UI",
                WebAppDisplayMode::Standalone => "STANDALONE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WebAppDisplayMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WebAppDisplayMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WebAppDisplayMode, ()> {
            Ok(match s {
                "DISPLAY_MODE_UNSPECIFIED" => WebAppDisplayMode::DisplayModeUnspecified,
                "FULL_SCREEN" => WebAppDisplayMode::FullScreen,
                "MINIMAL_UI" => WebAppDisplayMode::MinimalUi,
                "STANDALONE" => WebAppDisplayMode::Standalone,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WebAppDisplayMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WebAppDisplayMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WebAppDisplayMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_MODE_UNSPECIFIED" => WebAppDisplayMode::DisplayModeUnspecified,
                "FULL_SCREEN" => WebAppDisplayMode::FullScreen,
                "MINIMAL_UI" => WebAppDisplayMode::MinimalUi,
                "STANDALONE" => WebAppDisplayMode::Standalone,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WebAppDisplayMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebAppDisplayMode {
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
    pub struct WebAppIcon {
        #[doc = "The actual bytes of the image in a base64url encoded string (c.f. RFC4648, section 5 \"Base 64 Encoding with URL and Filename Safe Alphabet\"). - The image type can be png or jpg. - The image should ideally be square. - The image should ideally have a size of 512x512. "]
        #[serde(
            rename = "imageData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_data: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WebAppIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebAppIcon {
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
    pub struct WebToken {
        #[doc = "The features to enable. Use this if you want to control exactly which feature(s) will be activated; leave empty to allow all features.Restrictions / things to note: - If no features are listed here, all features are enabled — this is the default behavior where you give access to all features to your admins. - This must not contain any FEATURE_UNSPECIFIED values. - Repeated values are ignored "]
        #[serde(
            rename = "enabledFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled_features:
            ::std::option::Option<Vec<crate::schemas::WebTokenEnabledFeaturesItems>>,
        #[doc = "The name of the web token, which is generated by the server during creation in the form enterprises/{enterpriseId}/webTokens/{webTokenId}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The URL of the parent frame hosting the iframe with the embedded UI. To prevent XSS, the iframe may not be hosted at other URLs. The URL must use the https scheme."]
        #[serde(
            rename = "parentFrameUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_frame_url: ::std::option::Option<String>,
        #[doc = "Permissions available to an admin in the embedded UI. An admin must have all of these permissions in order to view the UI. This field is deprecated."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<crate::schemas::WebTokenPermissionsItems>>,
        #[doc = "The token value which is used in the hosting page to generate the iframe with the embedded UI. This is a read-only field generated by the server."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WebToken {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebToken {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WebTokenEnabledFeaturesItems {
        #[doc = "Unspecified feature."]
        FeatureUnspecified,
        #[doc = "The managed configurations page (https://developers.google.com/android/management/managed-configurations-iframe)."]
        ManagedConfigurations,
        #[doc = "The Managed Play search apps page (https://developers.google.com/android/management/apps#search-apps)."]
        PlaySearch,
        #[doc = "The private apps page (https://developers.google.com/android/management/apps#private-apps)."]
        PrivateApps,
        #[doc = "The organize apps page (https://developers.google.com/android/management/apps#organize-apps)."]
        StoreBuilder,
        #[doc = "The Web Apps page (https://developers.google.com/android/management/apps#web-apps)."]
        WebApps,
        #[doc = "The zero-touch iframe (https://developers.google.com/android/management/zero-touch-iframe)."]
        ZeroTouchCustomerManagement,
    }
    impl WebTokenEnabledFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                WebTokenEnabledFeaturesItems::FeatureUnspecified => "FEATURE_UNSPECIFIED",
                WebTokenEnabledFeaturesItems::ManagedConfigurations => "MANAGED_CONFIGURATIONS",
                WebTokenEnabledFeaturesItems::PlaySearch => "PLAY_SEARCH",
                WebTokenEnabledFeaturesItems::PrivateApps => "PRIVATE_APPS",
                WebTokenEnabledFeaturesItems::StoreBuilder => "STORE_BUILDER",
                WebTokenEnabledFeaturesItems::WebApps => "WEB_APPS",
                WebTokenEnabledFeaturesItems::ZeroTouchCustomerManagement => {
                    "ZERO_TOUCH_CUSTOMER_MANAGEMENT"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for WebTokenEnabledFeaturesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WebTokenEnabledFeaturesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WebTokenEnabledFeaturesItems, ()> {
            Ok(match s {
                "FEATURE_UNSPECIFIED" => WebTokenEnabledFeaturesItems::FeatureUnspecified,
                "MANAGED_CONFIGURATIONS" => WebTokenEnabledFeaturesItems::ManagedConfigurations,
                "PLAY_SEARCH" => WebTokenEnabledFeaturesItems::PlaySearch,
                "PRIVATE_APPS" => WebTokenEnabledFeaturesItems::PrivateApps,
                "STORE_BUILDER" => WebTokenEnabledFeaturesItems::StoreBuilder,
                "WEB_APPS" => WebTokenEnabledFeaturesItems::WebApps,
                "ZERO_TOUCH_CUSTOMER_MANAGEMENT" => {
                    WebTokenEnabledFeaturesItems::ZeroTouchCustomerManagement
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WebTokenEnabledFeaturesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WebTokenEnabledFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WebTokenEnabledFeaturesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEATURE_UNSPECIFIED" => WebTokenEnabledFeaturesItems::FeatureUnspecified,
                "MANAGED_CONFIGURATIONS" => WebTokenEnabledFeaturesItems::ManagedConfigurations,
                "PLAY_SEARCH" => WebTokenEnabledFeaturesItems::PlaySearch,
                "PRIVATE_APPS" => WebTokenEnabledFeaturesItems::PrivateApps,
                "STORE_BUILDER" => WebTokenEnabledFeaturesItems::StoreBuilder,
                "WEB_APPS" => WebTokenEnabledFeaturesItems::WebApps,
                "ZERO_TOUCH_CUSTOMER_MANAGEMENT" => {
                    WebTokenEnabledFeaturesItems::ZeroTouchCustomerManagement
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
    impl ::google_field_selector::FieldSelector for WebTokenEnabledFeaturesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebTokenEnabledFeaturesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WebTokenPermissionsItems {
        #[doc = "The permission to approve apps for the enterprise."]
        ApproveApps,
        #[doc = "This value is ignored."]
        WebTokenPermissionUnspecified,
    }
    impl WebTokenPermissionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                WebTokenPermissionsItems::ApproveApps => "APPROVE_APPS",
                WebTokenPermissionsItems::WebTokenPermissionUnspecified => {
                    "WEB_TOKEN_PERMISSION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for WebTokenPermissionsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WebTokenPermissionsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WebTokenPermissionsItems, ()> {
            Ok(match s {
                "APPROVE_APPS" => WebTokenPermissionsItems::ApproveApps,
                "WEB_TOKEN_PERMISSION_UNSPECIFIED" => {
                    WebTokenPermissionsItems::WebTokenPermissionUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WebTokenPermissionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WebTokenPermissionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WebTokenPermissionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPROVE_APPS" => WebTokenPermissionsItems::ApproveApps,
                "WEB_TOKEN_PERMISSION_UNSPECIFIED" => {
                    WebTokenPermissionsItems::WebTokenPermissionUnspecified
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
    impl ::google_field_selector::FieldSelector for WebTokenPermissionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebTokenPermissionsItems {
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
    pub struct WipeAction {
        #[doc = "Whether the factory-reset protection data is preserved on the device. This setting doesn’t apply to work profiles."]
        #[serde(
            rename = "preserveFrp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preserve_frp: ::std::option::Option<bool>,
        #[doc = "Number of days the policy is non-compliant before the device or work profile is wiped. wipeAfterDays must be greater than blockAfterDays."]
        #[serde(
            rename = "wipeAfterDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wipe_after_days: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for WipeAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WipeAction {
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
    #[doc = "Actions that can be performed on the enterprises resource"]
    pub fn enterprises(&self) -> crate::resources::enterprises::EnterprisesActions {
        crate::resources::enterprises::EnterprisesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the signup_urls resource"]
    pub fn signup_urls(&self) -> crate::resources::signup_urls::SignupUrlsActions {
        crate::resources::signup_urls::SignupUrlsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod enterprises {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListView {
                #[doc = "Includes name and enterprise_display_name fields."]
                Basic,
                #[doc = "The API will default to the BASIC view for the List method."]
                EnterpriseViewUnspecified,
            }
            impl ListView {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListView::Basic => "BASIC",
                        ListView::EnterpriseViewUnspecified => "ENTERPRISE_VIEW_UNSPECIFIED",
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
                        "BASIC" => ListView::Basic,
                        "ENTERPRISE_VIEW_UNSPECIFIED" => ListView::EnterpriseViewUnspecified,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListView {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListView {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "BASIC" => ListView::Basic,
                        "ENTERPRISE_VIEW_UNSPECIFIED" => ListView::EnterpriseViewUnspecified,
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
        pub struct EnterprisesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EnterprisesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates an enterprise. This is the last step in the enterprise signup flow."]
            pub fn create(&self, request: crate::schemas::Enterprise) -> CreateRequestBuilder {
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
                    agreement_accepted: None,
                    enterprise_token: None,
                    project_id: None,
                    signup_url_name: None,
                }
            }
            #[doc = "Deletes an enterprise. Only available for EMM-managed enterprises."]
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
            #[doc = "Gets an enterprise."]
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
            #[doc = "Lists EMM-managed enterprises. Only BASIC fields are returned."]
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
                    page_size: None,
                    page_token: None,
                    project_id: None,
                    view: None,
                }
            }
            #[doc = "Updates an enterprise."]
            pub fn patch(
                &self,
                request: crate::schemas::Enterprise,
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
            #[doc = "Actions that can be performed on the applications resource"]
            pub fn applications(
                &self,
            ) -> crate::resources::enterprises::applications::ApplicationsActions {
                crate::resources::enterprises::applications::ApplicationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the devices resource"]
            pub fn devices(&self) -> crate::resources::enterprises::devices::DevicesActions {
                crate::resources::enterprises::devices::DevicesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the enrollment_tokens resource"]
            pub fn enrollment_tokens(
                &self,
            ) -> crate::resources::enterprises::enrollment_tokens::EnrollmentTokensActions
            {
                crate::resources::enterprises::enrollment_tokens::EnrollmentTokensActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the policies resource"]
            pub fn policies(&self) -> crate::resources::enterprises::policies::PoliciesActions {
                crate::resources::enterprises::policies::PoliciesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the web_apps resource"]
            pub fn web_apps(&self) -> crate::resources::enterprises::web_apps::WebAppsActions {
                crate::resources::enterprises::web_apps::WebAppsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the web_tokens resource"]
            pub fn web_tokens(
                &self,
            ) -> crate::resources::enterprises::web_tokens::WebTokensActions {
                crate::resources::enterprises::web_tokens::WebTokensActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [EnterprisesActions::create()](struct.EnterprisesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Enterprise,
            agreement_accepted: Option<bool>,
            enterprise_token: Option<String>,
            project_id: Option<String>,
            signup_url_name: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = "Whether the enterprise admin has seen and agreed to the managed Google Play Agreement (https://www.android.com/enterprise/terms/). Do not set this field for any customer-managed enterprise (https://developers.google.com/android/management/create-enterprise#customer-managed_enterprises). Set this to field to true for all EMM-managed enterprises (https://developers.google.com/android/management/create-enterprise#emm-managed_enterprises)."]
            pub fn agreement_accepted(mut self, value: bool) -> Self {
                self.agreement_accepted = Some(value);
                self
            }
            #[doc = "The enterprise token appended to the callback URL. Set this when creating a customer-managed enterprise (https://developers.google.com/android/management/create-enterprise#customer-managed_enterprises) and not when creating a deprecated EMM-managed enterprise (https://developers.google.com/android/management/create-enterprise#emm-managed_enterprises)."]
            pub fn enterprise_token(mut self, value: impl Into<String>) -> Self {
                self.enterprise_token = Some(value.into());
                self
            }
            #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
            pub fn project_id(mut self, value: impl Into<String>) -> Self {
                self.project_id = Some(value.into());
                self
            }
            #[doc = "The name of the SignupUrl used to sign up for the enterprise. Set this when creating a customer-managed enterprise (https://developers.google.com/android/management/create-enterprise#customer-managed_enterprises) and not when creating a deprecated EMM-managed enterprise (https://developers.google.com/android/management/create-enterprise#emm-managed_enterprises)."]
            pub fn signup_url_name(mut self, value: impl Into<String>) -> Self {
                self.signup_url_name = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/enterprises");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("agreementAccepted", &self.agreement_accepted)]);
                req = req.query(&[("enterpriseToken", &self.enterprise_token)]);
                req = req.query(&[("projectId", &self.project_id)]);
                req = req.query(&[("signupUrlName", &self.signup_url_name)]);
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [EnterprisesActions::delete()](struct.EnterprisesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [EnterprisesActions::get()](struct.EnterprisesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [EnterprisesActions::list()](struct.EnterprisesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            page_size: Option<i32>,
            page_token: Option<String>,
            project_id: Option<String>,
            view: Option<crate::resources::enterprises::params::ListView>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results returned by the server."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Required. The Cloud project ID of the EMM managing the enterprises."]
            pub fn project_id(mut self, value: impl Into<String>) -> Self {
                self.project_id = Some(value.into());
                self
            }
            #[doc = "Specifies which Enterprise fields to return. This method only supports BASIC."]
            pub fn view(mut self, value: crate::resources::enterprises::params::ListView) -> Self {
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_enterprises<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_enterprises_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_enterprises_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Enterprise> {
                self.iter_enterprises_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_enterprises_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Enterprise> {
                self.iter_enterprises_with_fields(Some("*"))
            }
            pub fn iter_enterprises_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "enterprises").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "enterprises")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListEnterprisesResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListEnterprisesResponse> {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
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
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListEnterprisesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListEnterprisesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/enterprises");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("projectId", &self.project_id)]);
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[doc = "Created via [EnterprisesActions::patch()](struct.EnterprisesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Enterprise,
            name: String,
            update_mask: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> PatchRequestBuilder<'a> {
            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Enterprise, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        pub mod applications {
            pub mod params {}
            pub struct ApplicationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ApplicationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets info about an application."]
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
                        language_code: None,
                    }
                }
            }
            #[doc = "Created via [ApplicationsActions::get()](struct.ApplicationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                language_code: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "The preferred language for localized application info, as a BCP47 tag (e.g. \"en-US\", \"de\"). If not specified the default language of the application will be used."]
                pub fn language_code(mut self, value: impl Into<String>) -> Self {
                    self.language_code = Some(value.into());
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Application, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Application, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("languageCode", &self.language_code)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod devices {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum DeleteWipeDataFlagsItems {
                    #[doc = "Preserve the factory reset protection data on the device."]
                    PreserveResetProtectionData,
                    #[doc = "This value is ignored."]
                    WipeDataFlagUnspecified,
                    #[doc = "Additionally wipe the device's external storage (such as SD cards)."]
                    WipeExternalStorage,
                }
                impl DeleteWipeDataFlagsItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            DeleteWipeDataFlagsItems::PreserveResetProtectionData => {
                                "PRESERVE_RESET_PROTECTION_DATA"
                            }
                            DeleteWipeDataFlagsItems::WipeDataFlagUnspecified => {
                                "WIPE_DATA_FLAG_UNSPECIFIED"
                            }
                            DeleteWipeDataFlagsItems::WipeExternalStorage => {
                                "WIPE_EXTERNAL_STORAGE"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for DeleteWipeDataFlagsItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for DeleteWipeDataFlagsItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<DeleteWipeDataFlagsItems, ()> {
                        Ok(match s {
                            "PRESERVE_RESET_PROTECTION_DATA" => {
                                DeleteWipeDataFlagsItems::PreserveResetProtectionData
                            }
                            "WIPE_DATA_FLAG_UNSPECIFIED" => {
                                DeleteWipeDataFlagsItems::WipeDataFlagUnspecified
                            }
                            "WIPE_EXTERNAL_STORAGE" => {
                                DeleteWipeDataFlagsItems::WipeExternalStorage
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for DeleteWipeDataFlagsItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for DeleteWipeDataFlagsItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for DeleteWipeDataFlagsItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PRESERVE_RESET_PROTECTION_DATA" => {
                                DeleteWipeDataFlagsItems::PreserveResetProtectionData
                            }
                            "WIPE_DATA_FLAG_UNSPECIFIED" => {
                                DeleteWipeDataFlagsItems::WipeDataFlagUnspecified
                            }
                            "WIPE_EXTERNAL_STORAGE" => {
                                DeleteWipeDataFlagsItems::WipeExternalStorage
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
                impl ::google_field_selector::FieldSelector for DeleteWipeDataFlagsItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for DeleteWipeDataFlagsItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct DevicesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DevicesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes a device. This operation wipes the device."]
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
                        wipe_data_flags: None,
                        wipe_reason_message: None,
                    }
                }
                #[doc = "Gets a device."]
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
                #[doc = "Issues a command to a device. The Operation resource returned contains a Command in its metadata field. Use the get operation method to get the status of the command."]
                pub fn issue_command(
                    &self,
                    request: crate::schemas::Command,
                    name: impl Into<String>,
                ) -> IssueCommandRequestBuilder {
                    IssueCommandRequestBuilder {
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
                #[doc = "Lists devices for a given enterprise."]
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
                #[doc = "Updates a device."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Device,
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
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::enterprises::devices::operations::OperationsActions
                {
                    crate::resources::enterprises::devices::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [DevicesActions::delete()](struct.DevicesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                wipe_data_flags: Option<
                    Vec<crate::resources::enterprises::devices::params::DeleteWipeDataFlagsItems>,
                >,
                wipe_reason_message: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> DeleteRequestBuilder<'a> {
                #[doc = "Optional flags that control the device wiping behavior."]
                pub fn wipe_data_flags(
                    mut self,
                    value : impl Into < Vec < crate :: resources :: enterprises :: devices :: params :: DeleteWipeDataFlagsItems > >,
                ) -> Self {
                    self.wipe_data_flags = Some(value.into());
                    self
                }
                #[doc = "Optional. A short message displayed to the user before wiping the work profile on personal devices. This has no effect on company owned devices. The maximum message length is 200 characters."]
                pub fn wipe_reason_message(mut self, value: impl Into<String>) -> Self {
                    self.wipe_reason_message = Some(value.into());
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    for value in self.wipe_data_flags.iter().flatten() {
                        req = req.query(&[("wipeDataFlags", value)]);
                    }
                    req = req.query(&[("wipeReasonMessage", &self.wipe_reason_message)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [DevicesActions::get()](struct.DevicesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Device, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Device, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [DevicesActions::issue_command()](struct.DevicesActions.html#method.issue_command)"]
            #[derive(Debug, Clone)]
            pub struct IssueCommandRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Command,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> IssueCommandRequestBuilder<'a> {
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":issueCommand");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [DevicesActions::list()](struct.DevicesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results returned by the server."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_devices<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_devices_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_devices_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Device> {
                    self.iter_devices_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_devices_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Device> {
                    self.iter_devices_with_fields(Some("*"))
                }
                pub fn iter_devices_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "devices").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "devices")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDevicesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDevicesResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
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
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListDevicesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDevicesResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/devices");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [DevicesActions::patch()](struct.DevicesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Device,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Device, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Device, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OperationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."]
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
                    #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED."]
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
                    #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
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
                }
                #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                #[derive(Debug, Clone)]
                pub struct CancelRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                        output.push_str("v1/");
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_operations<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_operations_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_operations_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation>
                    {
                        self.iter_operations_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_operations_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation>
                    {
                        self.iter_operations_with_fields(Some("*"))
                    }
                    pub fn iter_operations_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
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
                        crate::iter::PageItemIter::new(self, "operations")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
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
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
        }
        pub mod enrollment_tokens {
            pub mod params {}
            pub struct EnrollmentTokensActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> EnrollmentTokensActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates an enrollment token for a given enterprise."]
                pub fn create(
                    &self,
                    request: crate::schemas::EnrollmentToken,
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
                #[doc = "Deletes an enrollment token. This operation invalidates the token, preventing its future use."]
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
            }
            #[doc = "Created via [EnrollmentTokensActions::create()](struct.EnrollmentTokensActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::EnrollmentToken,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::EnrollmentToken, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::EnrollmentToken, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/enrollmentTokens");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [EnrollmentTokensActions::delete()](struct.EnrollmentTokensActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod policies {
            pub mod params {}
            pub struct PoliciesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PoliciesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes a policy. This operation is only permitted if no devices are currently referencing the policy."]
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
                #[doc = "Gets a policy."]
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
                #[doc = "Lists policies for a given enterprise."]
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
                #[doc = "Updates or creates a policy."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Policy,
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
            }
            #[doc = "Created via [PoliciesActions::delete()](struct.PoliciesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [PoliciesActions::get()](struct.PoliciesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [PoliciesActions::list()](struct.PoliciesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results returned by the server."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_policies<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_policies_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_policies_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Policy> {
                    self.iter_policies_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_policies_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Policy> {
                    self.iter_policies_with_fields(Some("*"))
                }
                pub fn iter_policies_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "policies").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "policies")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListPoliciesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListPoliciesResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
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
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListPoliciesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListPoliciesResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/policies");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [PoliciesActions::patch()](struct.PoliciesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Policy,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod web_apps {
            pub mod params {}
            pub struct WebAppsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> WebAppsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a web app."]
                pub fn create(
                    &self,
                    request: crate::schemas::WebApp,
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
                #[doc = "Deletes a web app."]
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
                #[doc = "Gets a web app."]
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
                #[doc = "Lists web apps for a given enterprise."]
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
                #[doc = "Updates a web app."]
                pub fn patch(
                    &self,
                    request: crate::schemas::WebApp,
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
            }
            #[doc = "Created via [WebAppsActions::create()](struct.WebAppsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::WebApp,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/webApps");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [WebAppsActions::delete()](struct.WebAppsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [WebAppsActions::get()](struct.WebAppsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [WebAppsActions::list()](struct.WebAppsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The requested page size. This is a hint and the actual page size in the response may be different."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results returned by the server."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_web_apps<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_web_apps_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_web_apps_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::WebApp> {
                    self.iter_web_apps_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_web_apps_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::WebApp> {
                    self.iter_web_apps_with_fields(Some("*"))
                }
                pub fn iter_web_apps_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "webApps").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "webApps")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListWebAppsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListWebAppsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
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
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListWebAppsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListWebAppsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/webApps");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [WebAppsActions::patch()](struct.WebAppsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::WebApp,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::WebApp, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod web_tokens {
            pub mod params {}
            pub struct WebTokensActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> WebTokensActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a web token to access an embeddable managed Google Play web UI for a given enterprise."]
                pub fn create(
                    &self,
                    request: crate::schemas::WebToken,
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
            }
            #[doc = "Created via [WebTokensActions::create()](struct.WebTokensActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::WebToken,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::WebToken, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::WebToken, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/webTokens");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
    }
    pub mod signup_urls {
        pub mod params {}
        pub struct SignupUrlsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SignupUrlsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates an enterprise signup URL."]
            pub fn create(&self) -> CreateRequestBuilder {
                CreateRequestBuilder {
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
                    callback_url: None,
                    project_id: None,
                }
            }
        }
        #[doc = "Created via [SignupUrlsActions::create()](struct.SignupUrlsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            callback_url: Option<String>,
            project_id: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = "The callback URL that the admin will be redirected to after successfully creating an enterprise. Before redirecting there the system will add a query parameter to this URL named enterpriseToken which will contain an opaque token to be used for the create enterprise request. The URL will be parsed then reformatted in order to add the enterpriseToken parameter, so there may be some minor formatting changes."]
            pub fn callback_url(mut self, value: impl Into<String>) -> Self {
                self.callback_url = Some(value.into());
                self
            }
            #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
            pub fn project_id(mut self, value: impl Into<String>) -> Self {
                self.project_id = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::SignupUrl, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SignupUrl, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/signupUrls");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("callbackUrl", &self.callback_url)]);
                req = req.query(&[("projectId", &self.project_id)]);
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
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
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
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
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
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
                        let written = body.read(rem_buf)?;
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
            Ok(bytes_written)
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
