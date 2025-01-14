#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [customers](resources/customers/struct.CustomersActions.html)\n  * [apps](resources/customers/apps/struct.AppsActions.html)\n    * [*countChromeAppRequests*](resources/customers/apps/struct.CountChromeAppRequestsRequestBuilder.html)\n    * [android](resources/customers/apps/android/struct.AndroidActions.html)\n      * [*get*](resources/customers/apps/android/struct.GetRequestBuilder.html)\n    * [chrome](resources/customers/apps/chrome/struct.ChromeActions.html)\n      * [*get*](resources/customers/apps/chrome/struct.GetRequestBuilder.html)\n    * [web](resources/customers/apps/web/struct.WebActions.html)\n      * [*get*](resources/customers/apps/web/struct.GetRequestBuilder.html)\n  * [reports](resources/customers/reports/struct.ReportsActions.html)\n    * [*countChromeDevicesReachingAutoExpirationDate*](resources/customers/reports/struct.CountChromeDevicesReachingAutoExpirationDateRequestBuilder.html), [*countChromeDevicesThatNeedAttention*](resources/customers/reports/struct.CountChromeDevicesThatNeedAttentionRequestBuilder.html), [*countChromeHardwareFleetDevices*](resources/customers/reports/struct.CountChromeHardwareFleetDevicesRequestBuilder.html), [*countChromeVersions*](resources/customers/reports/struct.CountChromeVersionsRequestBuilder.html), [*countInstalledApps*](resources/customers/reports/struct.CountInstalledAppsRequestBuilder.html), [*findInstalledAppDevices*](resources/customers/reports/struct.FindInstalledAppDevicesRequestBuilder.html)\n  * [telemetry](resources/customers/telemetry/struct.TelemetryActions.html)\n    * [devices](resources/customers/telemetry/devices/struct.DevicesActions.html)\n      * [*get*](resources/customers/telemetry/devices/struct.GetRequestBuilder.html), [*list*](resources/customers/telemetry/devices/struct.ListRequestBuilder.html)\n    * [events](resources/customers/telemetry/events/struct.EventsActions.html)\n      * [*list*](resources/customers/telemetry/events/struct.ListRequestBuilder.html)\n    * [users](resources/customers/telemetry/users/struct.UsersActions.html)\n      * [*get*](resources/customers/telemetry/users/struct.GetRequestBuilder.html), [*list*](resources/customers/telemetry/users/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See detailed information about apps installed on Chrome browsers and devices managed by your organization\n\n`https://www.googleapis.com/auth/chrome.management.appdetails.readonly`"]
    pub const CHROME_MANAGEMENT_APPDETAILS_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.appdetails.readonly";
    #[doc = "See reports about devices and Chrome browsers managed within your organization\n\n`https://www.googleapis.com/auth/chrome.management.reports.readonly`"]
    pub const CHROME_MANAGEMENT_REPORTS_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.reports.readonly";
    #[doc = "See basic device and telemetry information collected from Chrome OS devices or users managed within your organization\n\n`https://www.googleapis.com/auth/chrome.management.telemetry.readonly`"]
    pub const CHROME_MANAGEMENT_TELEMETRY_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.telemetry.readonly";
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
    pub struct GoogleChromeManagementV1AndroidAppInfo {
        #[doc = "Output only. Permissions requested by an Android app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1AndroidAppPermission>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AndroidAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AndroidAppInfo {
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
    pub struct GoogleChromeManagementV1AndroidAppPermission {
        #[doc = "Output only. The type of the permission."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AndroidAppPermission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AndroidAppPermission {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleChromeManagementV1AppDetails {
        #[doc = "Output only. Android app information."]
        #[serde(
            rename = "androidAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_app_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1AndroidAppInfo>,
        #[doc = "Output only. Unique store identifier for the item. Examples: “gmbmikajjgmnabiglmofipeabaddhgne” for the Save to Google Drive Chrome extension, “com.google.android.apps.docs” for the Google Drive Android app."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. Chrome Web Store app information."]
        #[serde(
            rename = "chromeAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chrome_app_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1ChromeAppInfo>,
        #[doc = "Output only. App’s description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. The uri for the detail page of the item."]
        #[serde(
            rename = "detailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail_uri: ::std::option::Option<String>,
        #[doc = "Output only. App’s display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. First published time."]
        #[serde(
            rename = "firstPublishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_publish_time: ::std::option::Option<String>,
        #[doc = "Output only. Home page or Website uri."]
        #[serde(
            rename = "homepageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub homepage_uri: ::std::option::Option<String>,
        #[doc = "Output only. A link to an image that can be used as an icon for the product."]
        #[serde(
            rename = "iconUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_uri: ::std::option::Option<String>,
        #[doc = "Output only. Indicates if the app has to be paid for OR has paid content."]
        #[serde(
            rename = "isPaidApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_paid_app: ::std::option::Option<bool>,
        #[doc = "Output only. Latest published time."]
        #[serde(
            rename = "latestPublishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_publish_time: ::std::option::Option<String>,
        #[doc = "Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "privacyPolicyUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privacy_policy_uri: ::std::option::Option<String>,
        #[doc = "Output only. The publisher of the item."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<String>,
        #[doc = "Output only. App type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleChromeManagementV1AppDetailsType>,
        #[doc = "Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app."]
        #[serde(
            rename = "reviewNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub review_number: ::std::option::Option<i64>,
        #[doc = "Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app."]
        #[serde(
            rename = "reviewRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub review_rating: ::std::option::Option<f32>,
        #[doc = "Output only. App version. A new revision is committed whenever a new version of the app is published."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Output only. Information about a partial service error if applicable."]
        #[serde(
            rename = "serviceError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AppDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AppDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1AppDetailsType {
        #[doc = "ARC++ app."]
        Android,
        #[doc = "App type unspecified."]
        AppItemTypeUnspecified,
        #[doc = "Chrome app."]
        Chrome,
        #[doc = "Web app."]
        Web,
    }
    impl GoogleChromeManagementV1AppDetailsType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1AppDetailsType::Android => "ANDROID",
                GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified => {
                    "APP_ITEM_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1AppDetailsType::Chrome => "CHROME",
                GoogleChromeManagementV1AppDetailsType::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1AppDetailsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1AppDetailsType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleChromeManagementV1AppDetailsType, ()> {
            Ok(match s {
                "ANDROID" => GoogleChromeManagementV1AppDetailsType::Android,
                "APP_ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified
                }
                "CHROME" => GoogleChromeManagementV1AppDetailsType::Chrome,
                "WEB" => GoogleChromeManagementV1AppDetailsType::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1AppDetailsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1AppDetailsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1AppDetailsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => GoogleChromeManagementV1AppDetailsType::Android,
                "APP_ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified
                }
                "CHROME" => GoogleChromeManagementV1AppDetailsType::Chrome,
                "WEB" => GoogleChromeManagementV1AppDetailsType::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AppDetailsType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AppDetailsType {
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
    pub struct GoogleChromeManagementV1AudioStatusReport {
        #[doc = "Output only. Active input device’s name."]
        #[serde(
            rename = "inputDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_device: ::std::option::Option<String>,
        #[doc = "Output only. Active input device’s gain in \\[0, 100\\]."]
        #[serde(
            rename = "inputGain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_gain: ::std::option::Option<i32>,
        #[doc = "Output only. Is active input device mute or not."]
        #[serde(
            rename = "inputMute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_mute: ::std::option::Option<bool>,
        #[doc = "Output only. Active output device’s name."]
        #[serde(
            rename = "outputDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_device: ::std::option::Option<String>,
        #[doc = "Output only. Is active output device mute or not."]
        #[serde(
            rename = "outputMute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_mute: ::std::option::Option<bool>,
        #[doc = "Output only. Active output device’s volume in \\[0, 100\\]."]
        #[serde(
            rename = "outputVolume",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_volume: ::std::option::Option<i32>,
        #[doc = "Output only. Timestamp of when the sample was collected on device."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AudioStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AudioStatusReport {
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
    pub struct GoogleChromeManagementV1BatteryInfo {
        #[doc = "Output only. Design capacity (mAmpere-hours)."]
        #[serde(
            rename = "designCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub design_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Designed minimum output voltage (mV)"]
        #[serde(
            rename = "designMinVoltage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub design_min_voltage: ::std::option::Option<i32>,
        #[doc = "Output only. The date the battery was manufactured."]
        #[serde(
            rename = "manufactureDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacture_date: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "Output only. Battery manufacturer."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "Output only. Battery serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Technology of the battery. Example: Li-ion"]
        #[serde(
            rename = "technology",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub technology: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatteryInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatteryInfo {
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
    pub struct GoogleChromeManagementV1BatterySampleReport {
        #[doc = "Output only. Battery charge percentage."]
        #[serde(
            rename = "chargeRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charge_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Battery current (mA)."]
        #[serde(
            rename = "current",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current: ::std::option::Option<i64>,
        #[doc = "Output only. The battery discharge rate measured in mW. Positive if the battery is being discharged, negative if it’s being charged."]
        #[serde(
            rename = "dischargeRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discharge_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Battery remaining capacity (mAmpere-hours)."]
        #[serde(
            rename = "remainingCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub remaining_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Battery status read from sysfs. Example: Discharging"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<String>,
        #[doc = "Output only. Temperature in Celsius degrees."]
        #[serde(
            rename = "temperature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temperature: ::std::option::Option<i32>,
        #[doc = "Output only. Battery voltage (millivolt)."]
        #[serde(
            rename = "voltage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub voltage: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatterySampleReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatterySampleReport {
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
    pub struct GoogleChromeManagementV1BatteryStatusReport {
        #[doc = "Output only. Battery health."]
        #[serde(
            rename = "batteryHealth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_health: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1BatteryStatusReportBatteryHealth,
        >,
        #[doc = "Output only. Cycle count."]
        #[serde(
            rename = "cycleCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cycle_count: ::std::option::Option<i32>,
        #[doc = "Output only. Full charge capacity (mAmpere-hours)."]
        #[serde(
            rename = "fullChargeCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub full_charge_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Sampling data for the battery sorted in a decreasing order of report_time."]
        #[serde(
            rename = "sample",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatterySampleReport>>,
        #[doc = "Output only. Battery serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatteryStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatteryStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        #[doc = "Battery is healthy."]
        BatteryHealthNormal,
        #[doc = "Health unknown."]
        BatteryHealthUnspecified,
        #[doc = "Battery is unhealthy and should be replaced."]
        BatteryReplaceNow,
        #[doc = "Battery is moderately unhealthy and should be replaced soon."]
        BatteryReplaceSoon,
    }
    impl GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal => "BATTERY_HEALTH_NORMAL" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified => "BATTERY_HEALTH_UNSPECIFIED" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow => "BATTERY_REPLACE_NOW" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon => "BATTERY_REPLACE_SOON" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BatteryStatusReportBatteryHealth, ()>
        {
            Ok (match s { "BATTERY_HEALTH_NORMAL" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal , "BATTERY_HEALTH_UNSPECIFIED" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified , "BATTERY_REPLACE_NOW" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow , "BATTERY_REPLACE_SOON" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BATTERY_HEALTH_NORMAL" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal , "BATTERY_HEALTH_UNSPECIFIED" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified , "BATTERY_REPLACE_NOW" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow , "BATTERY_REPLACE_SOON" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1BatteryStatusReportBatteryHealth
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1BatteryStatusReportBatteryHealth
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
    pub struct GoogleChromeManagementV1BootPerformanceReport {
        #[doc = "Total time to boot up."]
        #[serde(
            rename = "bootUpDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boot_up_duration: ::std::option::Option<String>,
        #[doc = "The timestamp when power came on."]
        #[serde(
            rename = "bootUpTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boot_up_time: ::std::option::Option<String>,
        #[doc = "Timestamp when the report was collected."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Total time since shutdown start to power off."]
        #[serde(
            rename = "shutdownDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shutdown_duration: ::std::option::Option<String>,
        #[doc = "The shutdown reason."]
        #[serde(
            rename = "shutdownReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shutdown_reason: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1BootPerformanceReportShutdownReason,
        >,
        #[doc = "The timestamp when shutdown."]
        #[serde(
            rename = "shutdownTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shutdown_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BootPerformanceReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BootPerformanceReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        #[doc = "Shutdown due to low battery."]
        LowBattery,
        #[doc = "Shutdown due to other reasons."]
        Other,
        #[doc = "Shutdown reason is not specified."]
        ShutdownReasonUnspecified,
        #[doc = "System update initiated."]
        SystemUpdate,
        #[doc = "User initiated."]
        UserRequest,
    }
    impl GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1BootPerformanceReportShutdownReason :: LowBattery => "LOW_BATTERY" , GoogleChromeManagementV1BootPerformanceReportShutdownReason :: Other => "OTHER" , GoogleChromeManagementV1BootPerformanceReportShutdownReason :: ShutdownReasonUnspecified => "SHUTDOWN_REASON_UNSPECIFIED" , GoogleChromeManagementV1BootPerformanceReportShutdownReason :: SystemUpdate => "SYSTEM_UPDATE" , GoogleChromeManagementV1BootPerformanceReportShutdownReason :: UserRequest => "USER_REQUEST" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BootPerformanceReportShutdownReason, ()>
        {
            Ok (match s { "LOW_BATTERY" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: LowBattery , "OTHER" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: Other , "SHUTDOWN_REASON_UNSPECIFIED" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: ShutdownReasonUnspecified , "SYSTEM_UPDATE" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: SystemUpdate , "USER_REQUEST" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: UserRequest , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BootPerformanceReportShutdownReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleChromeManagementV1BootPerformanceReportShutdownReason
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LOW_BATTERY" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: LowBattery , "OTHER" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: Other , "SHUTDOWN_REASON_UNSPECIFIED" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: ShutdownReasonUnspecified , "SYSTEM_UPDATE" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: SystemUpdate , "USER_REQUEST" => GoogleChromeManagementV1BootPerformanceReportShutdownReason :: UserRequest , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1BootPerformanceReportShutdownReason
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1BootPerformanceReportShutdownReason
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
    pub struct GoogleChromeManagementV1BrowserVersion {
        #[doc = "Output only. The release channel of the installed browser."]
        #[serde(
            rename = "channel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionChannel>,
        #[doc = "Output only. Count grouped by device_system and major version"]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Output only. Version of the system-specified operating system."]
        #[serde(
            rename = "deviceOsVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_os_version: ::std::option::Option<String>,
        #[doc = "Output only. The device operating system."]
        #[serde(
            rename = "system",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionSystem>,
        #[doc = "Output only. The full version of the installed browser."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionChannel {
        #[doc = "Beta release channel."]
        Beta,
        #[doc = "Canary release channel."]
        Canary,
        #[doc = "Dev release channel."]
        Dev,
        #[doc = "No release channel specified."]
        ReleaseChannelUnspecified,
        #[doc = "Stable release channel."]
        Stable,
    }
    impl GoogleChromeManagementV1BrowserVersionChannel {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionChannel::Beta => "BETA",
                GoogleChromeManagementV1BrowserVersionChannel::Canary => "CANARY",
                GoogleChromeManagementV1BrowserVersionChannel::Dev => "DEV",
                GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified => {
                    "RELEASE_CHANNEL_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionChannel::Stable => "STABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionChannel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionChannel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionChannel, ()> {
            Ok(match s {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionChannel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionChannel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionChannel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionSystem {
        #[doc = "No operating system specified."]
        DeviceSystemUnspecified,
        #[doc = "Android operating system."]
        SystemAndroid,
        #[doc = "ChromeOS operating system."]
        SystemCros,
        #[doc = "Apple iOS operating system."]
        SystemIos,
        #[doc = "Linux operating system."]
        SystemLinux,
        #[doc = "Apple macOS operating system."]
        SystemMac,
        #[doc = "Other operating system."]
        SystemOther,
        #[doc = "Microsoft Windows operating system."]
        SystemWindows,
    }
    impl GoogleChromeManagementV1BrowserVersionSystem {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified => {
                    "DEVICE_SYSTEM_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid => "SYSTEM_ANDROID",
                GoogleChromeManagementV1BrowserVersionSystem::SystemCros => "SYSTEM_CROS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemIos => "SYSTEM_IOS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemLinux => "SYSTEM_LINUX",
                GoogleChromeManagementV1BrowserVersionSystem::SystemMac => "SYSTEM_MAC",
                GoogleChromeManagementV1BrowserVersionSystem::SystemOther => "SYSTEM_OTHER",
                GoogleChromeManagementV1BrowserVersionSystem::SystemWindows => "SYSTEM_WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionSystem {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionSystem {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionSystem, ()> {
            Ok(match s {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionSystem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionSystem {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionSystem {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionSystem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionSystem {
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
    pub struct GoogleChromeManagementV1ChromeAppInfo {
        #[doc = "Output only. Whether the app or extension is built and maintained by Google. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "googleOwned",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_owned: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app or extension is in a published state in the Chrome Web Store."]
        #[serde(
            rename = "isCwsHosted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_cws_hosted: ::std::option::Option<bool>,
        #[doc = "Output only. Whether an app supports policy for extensions."]
        #[serde(
            rename = "isExtensionPolicySupported",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_extension_policy_supported: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app is only for Kiosk mode on ChromeOS devices"]
        #[serde(
            rename = "isKioskOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_kiosk_only: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app or extension is a theme."]
        #[serde(
            rename = "isTheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_theme: ::std::option::Option<bool>,
        #[doc = "Output only. Whether this app is enabled for Kiosk mode on ChromeOS devices"]
        #[serde(
            rename = "kioskEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kiosk_enabled: ::std::option::Option<bool>,
        #[doc = "Output only. The minimum number of users using this app."]
        #[serde(
            rename = "minUserCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_user_count: ::std::option::Option<i32>,
        #[doc = "Output only. Every custom permission requested by the app. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppPermission>>,
        #[doc = "Output only. Types of an item in the Chrome Web Store"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1ChromeAppInfoType>,
        #[doc = "Output only. Every permission giving access to domains or broad host patterns. ( e.g. www.google.com). This includes the matches from content scripts as well as hosts in the permissions node of the manifest. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "siteAccess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_access:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppSiteAccess>>,
        #[doc = "Output only. The app developer has enabled support for their app. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "supportEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1ChromeAppInfoType {
        #[doc = "Chrome Extensions."]
        Extension,
        #[doc = "Unspecified ItemType."]
        ItemTypeUnspecified,
        #[doc = "Any other type than extension."]
        Others,
    }
    impl GoogleChromeManagementV1ChromeAppInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1ChromeAppInfoType::Extension => "EXTENSION",
                GoogleChromeManagementV1ChromeAppInfoType::ItemTypeUnspecified => {
                    "ITEM_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1ChromeAppInfoType::Others => "OTHERS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1ChromeAppInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1ChromeAppInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1ChromeAppInfoType, ()> {
            Ok(match s {
                "EXTENSION" => GoogleChromeManagementV1ChromeAppInfoType::Extension,
                "ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1ChromeAppInfoType::ItemTypeUnspecified
                }
                "OTHERS" => GoogleChromeManagementV1ChromeAppInfoType::Others,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1ChromeAppInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1ChromeAppInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1ChromeAppInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTENSION" => GoogleChromeManagementV1ChromeAppInfoType::Extension,
                "ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1ChromeAppInfoType::ItemTypeUnspecified
                }
                "OTHERS" => GoogleChromeManagementV1ChromeAppInfoType::Others,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppInfoType {
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
    pub struct GoogleChromeManagementV1ChromeAppPermission {
        #[doc = "Output only. If available, whether this permissions grants the app/extension access to user data."]
        #[serde(
            rename = "accessUserData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_user_data: ::std::option::Option<bool>,
        #[doc = "Output only. If available, a URI to a page that has documentation for the current permission."]
        #[serde(
            rename = "documentationUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_uri: ::std::option::Option<String>,
        #[doc = "Output only. The type of the permission."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppPermission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppPermission {
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
    pub struct GoogleChromeManagementV1ChromeAppRequest {
        #[doc = "Output only. Format: app_details=customers/{customer_id}/apps/chrome/{app_id}"]
        #[serde(
            rename = "appDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_details: ::std::option::Option<String>,
        #[doc = "Output only. Unique store identifier for the app. Example: “gmbmikajjgmnabiglmofipeabaddhgne” for the Save to Google Drive Chrome extension."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. The uri for the detail page of the item."]
        #[serde(
            rename = "detailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail_uri: ::std::option::Option<String>,
        #[doc = "Output only. App’s display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. A link to an image that can be used as an icon for the product."]
        #[serde(
            rename = "iconUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_uri: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp of the most recently made request for this app."]
        #[serde(
            rename = "latestRequestTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_request_time: ::std::option::Option<String>,
        #[doc = "Output only. Total count of requests for this app."]
        #[serde(
            rename = "requestCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub request_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppRequest {
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
    pub struct GoogleChromeManagementV1ChromeAppSiteAccess {
        #[doc = "Output only. This can contain very specific hosts, or patterns like “\\*.com” for instance."]
        #[serde(
            rename = "hostMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_match: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppSiteAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppSiteAccess {
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
    pub struct GoogleChromeManagementV1CountChromeAppRequestsResponse {
        #[doc = "Token to specify the next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Count of requested apps matching request."]
        #[serde(
            rename = "requestedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_apps:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppRequest>>,
        #[doc = "Total number of matching app requests."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeAppRequestsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1CountChromeAppRequestsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1CountChromeAppRequestsResponse {
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
    pub struct GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse {
        #[doc = "The list of reports sorted by auto update expiration date in ascending order."]
        #[serde(
            rename = "deviceAueCountReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_aue_count_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1DeviceAueCountReport>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse
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
    pub struct GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse {
        #[doc = "Number of ChromeOS devices have not synced policies in the past 28 days."]
        #[serde(
            rename = "noRecentPolicySyncCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub no_recent_policy_sync_count: ::std::option::Option<i64>,
        #[doc = "Number of ChromeOS devices that have not seen any user activity in the past 28 days."]
        #[serde(
            rename = "noRecentUserActivityCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub no_recent_user_activity_count: ::std::option::Option<i64>,
        #[doc = "Number of devices whose OS version is not compliant."]
        #[serde(
            rename = "osVersionNotCompliantCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub os_version_not_compliant_count: ::std::option::Option<i64>,
        #[doc = "Number of devices that are pending an OS update."]
        #[serde(
            rename = "pendingUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pending_update: ::std::option::Option<i64>,
        #[doc = "Number of devices that are unable to apply a policy due to an OS version mismatch."]
        #[serde(
            rename = "unsupportedPolicyCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub unsupported_policy_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse
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
    pub struct GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse {
        #[doc = "The DeviceHardwareCountReport for device cpu type (for example Intel(R) Core(TM) i7-10610U CPU @ 1.80GHz)."]
        #[serde(
            rename = "cpuReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1DeviceHardwareCountReport>,
        >,
        #[doc = "The DeviceHardwareCountReport for device memory amount in gigabytes (for example 16)."]
        #[serde(
            rename = "memoryReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1DeviceHardwareCountReport>,
        >,
        #[doc = "The DeviceHardwareCountReport for device model type (for example Acer C7 Chromebook)."]
        #[serde(
            rename = "modelReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1DeviceHardwareCountReport>,
        >,
        #[doc = "The DeviceHardwareCountReport for device storage amount in gigabytes (for example 128)."]
        #[serde(
            rename = "storageReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1DeviceHardwareCountReport>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse
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
    pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
        #[doc = "List of all browser versions and their install counts."]
        #[serde(
            rename = "browserVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub browser_versions:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BrowserVersion>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number browser versions matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeVersionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountChromeVersionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1CountChromeVersionsResponse {
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
    pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
        #[doc = "List of installed apps matching request."]
        #[serde(
            rename = "installedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_apps:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1InstalledApp>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of installed apps matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CountInstalledAppsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountInstalledAppsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1CountInstalledAppsResponse {
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
    pub struct GoogleChromeManagementV1CpuInfo {
        #[doc = "Output only. Architecture type for the CPU. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A"]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1CpuInfoArchitecture>,
        #[doc = "Output only. Whether keylocker is configured.`TRUE` = Enabled; `FALSE` = disabled. Only reported if keylockerSupported = `TRUE`."]
        #[serde(
            rename = "keylockerConfigured",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keylocker_configured: ::std::option::Option<bool>,
        #[doc = "Output only. Whether keylocker is supported."]
        #[serde(
            rename = "keylockerSupported",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keylocker_supported: ::std::option::Option<bool>,
        #[doc = "Output only. The max CPU clock speed in kHz."]
        #[serde(
            rename = "maxClockSpeed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_clock_speed: ::std::option::Option<i32>,
        #[doc = "Output only. The CPU model name. Example: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz"]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1CpuInfoArchitecture {
        #[doc = "Architecture unknown."]
        ArchitectureUnspecified,
        #[doc = "x64 architecture"]
        X64,
    }
    impl GoogleChromeManagementV1CpuInfoArchitecture {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified => {
                    "ARCHITECTURE_UNSPECIFIED"
                }
                GoogleChromeManagementV1CpuInfoArchitecture::X64 => "X64",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1CpuInfoArchitecture {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1CpuInfoArchitecture {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1CpuInfoArchitecture, ()> {
            Ok(match s {
                "ARCHITECTURE_UNSPECIFIED" => {
                    GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified
                }
                "X64" => GoogleChromeManagementV1CpuInfoArchitecture::X64,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1CpuInfoArchitecture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1CpuInfoArchitecture {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1CpuInfoArchitecture {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHITECTURE_UNSPECIFIED" => {
                    GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified
                }
                "X64" => GoogleChromeManagementV1CpuInfoArchitecture::X64,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuInfoArchitecture {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuInfoArchitecture {
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
    pub struct GoogleChromeManagementV1CpuStatusReport {
        #[doc = "Output only. CPU temperature sample info per CPU core in Celsius"]
        #[serde(
            rename = "cpuTemperatureInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_temperature_info:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuTemperatureInfo>>,
        #[doc = "Output only. Sample of CPU utilization (0-100 percent)."]
        #[serde(
            rename = "cpuUtilizationPct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_utilization_pct: ::std::option::Option<i32>,
        #[doc = "Output only. The timestamp in milliseconds representing time at which this report was sampled."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuStatusReport {
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
    pub struct GoogleChromeManagementV1CpuTemperatureInfo {
        #[doc = "Output only. CPU label. Example: Core 0"]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "Output only. CPU temperature in Celsius."]
        #[serde(
            rename = "temperatureCelsius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temperature_celsius: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuTemperatureInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuTemperatureInfo {
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
    pub struct GoogleChromeManagementV1Device {
        #[doc = "Output only. The ID of the device that reported this Chrome browser information."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. The name of the machine within its local network."]
        #[serde(
            rename = "machine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1Device {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1Device {
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
    pub struct GoogleChromeManagementV1DeviceAueCountReport {
        #[doc = "Enum value of month corresponding to the auto update expiration date in UTC time zone. If the device is already expired, this field is empty."]
        #[serde(
            rename = "aueMonth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aue_month: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1DeviceAueCountReportAueMonth,
        >,
        #[doc = "Int value of year corresponding to the Auto Update Expiration date in UTC time zone. If the device is already expired, this field is empty."]
        #[serde(
            rename = "aueYear",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub aue_year: ::std::option::Option<i64>,
        #[doc = "Count of devices of this model."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Boolean value for whether or not the device has already expired."]
        #[serde(
            rename = "expired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expired: ::std::option::Option<bool>,
        #[doc = "Public model name of the devices."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DeviceAueCountReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DeviceAueCountReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        #[doc = "The month of April."]
        April,
        #[doc = "The month of August."]
        August,
        #[doc = "The month of December."]
        December,
        #[doc = "The month of February."]
        February,
        #[doc = "The month of January."]
        January,
        #[doc = "The month of July."]
        July,
        #[doc = "The month of June."]
        June,
        #[doc = "The month of March."]
        March,
        #[doc = "The month of May."]
        May,
        #[doc = "The unspecified month."]
        MonthUnspecified,
        #[doc = "The month of November."]
        November,
        #[doc = "The month of October."]
        October,
        #[doc = "The month of September."]
        September,
    }
    impl GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::April => "APRIL",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::August => "AUGUST",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::December => "DECEMBER",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::February => "FEBRUARY",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::January => "JANUARY",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::July => "JULY",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::June => "JUNE",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::March => "MARCH",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::May => "MAY",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::MonthUnspecified => {
                    "MONTH_UNSPECIFIED"
                }
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::November => "NOVEMBER",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::October => "OCTOBER",
                GoogleChromeManagementV1DeviceAueCountReportAueMonth::September => "SEPTEMBER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1DeviceAueCountReportAueMonth, ()>
        {
            Ok(match s {
                "APRIL" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::April,
                "AUGUST" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::August,
                "DECEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::December,
                "FEBRUARY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::February,
                "JANUARY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::January,
                "JULY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::July,
                "JUNE" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::June,
                "MARCH" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::March,
                "MAY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::May,
                "MONTH_UNSPECIFIED" => {
                    GoogleChromeManagementV1DeviceAueCountReportAueMonth::MonthUnspecified
                }
                "NOVEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::November,
                "OCTOBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::October,
                "SEPTEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::September,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APRIL" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::April,
                "AUGUST" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::August,
                "DECEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::December,
                "FEBRUARY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::February,
                "JANUARY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::January,
                "JULY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::July,
                "JUNE" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::June,
                "MARCH" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::March,
                "MAY" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::May,
                "MONTH_UNSPECIFIED" => {
                    GoogleChromeManagementV1DeviceAueCountReportAueMonth::MonthUnspecified
                }
                "NOVEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::November,
                "OCTOBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::October,
                "SEPTEMBER" => GoogleChromeManagementV1DeviceAueCountReportAueMonth::September,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1DeviceAueCountReportAueMonth
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DeviceAueCountReportAueMonth {
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
    pub struct GoogleChromeManagementV1DeviceHardwareCountReport {
        #[doc = "Public name of the hardware specification."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Count of devices with a unique hardware specification."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DeviceHardwareCountReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DeviceHardwareCountReport {
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
    pub struct GoogleChromeManagementV1DiskInfo {
        #[doc = "Output only. Number of bytes read since last boot."]
        #[serde(
            rename = "bytesReadThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bytes_read_this_session: ::std::option::Option<i64>,
        #[doc = "Output only. Number of bytes written since last boot."]
        #[serde(
            rename = "bytesWrittenThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bytes_written_this_session: ::std::option::Option<i64>,
        #[doc = "Output only. Time spent discarding since last boot. Discarding is writing to clear blocks which are no longer in use. Supported on kernels 4.18+."]
        #[serde(
            rename = "discardTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discard_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk health."]
        #[serde(
            rename = "health",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub health: ::std::option::Option<String>,
        #[doc = "Output only. Counts the time the disk and queue were busy, so unlike the fields above, parallel requests are not counted multiple times."]
        #[serde(
            rename = "ioTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub io_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk manufacturer."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "Output only. Disk model."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
        #[doc = "Output only. Disk type: eMMC / NVMe / ATA / SCSI."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. Time spent reading from disk since last boot."]
        #[serde(
            rename = "readTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Disk size."]
        #[serde(
            rename = "sizeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
        #[doc = "Output only. Disk volumes."]
        #[serde(
            rename = "volumeIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Time spent writing to disk since last boot."]
        #[serde(
            rename = "writeTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_time_this_session: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DiskInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DiskInfo {
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
    pub struct GoogleChromeManagementV1DisplayInfo {
        #[doc = "Output only. Represents the graphics card device id."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub device_id: ::std::option::Option<i64>,
        #[doc = "Output only. Indicates if display is internal or not."]
        #[serde(
            rename = "isInternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_internal: ::std::option::Option<bool>,
        #[doc = "Output only. Refresh rate in Hz."]
        #[serde(
            rename = "refreshRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Resolution height in pixels."]
        #[serde(
            rename = "resolutionHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_height: ::std::option::Option<i32>,
        #[doc = "Output only. Resolution width in pixels."]
        #[serde(
            rename = "resolutionWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DisplayInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DisplayInfo {
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
    pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
        #[doc = "A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1Device>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of devices matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1FindInstalledAppDevicesResponse {
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
    pub struct GoogleChromeManagementV1GraphicsAdapterInfo {
        #[doc = "Output only. Adapter name. Example: Mesa DRI Intel(R) UHD Graphics 620 (Kabylake GT2)."]
        #[serde(
            rename = "adapter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adapter: ::std::option::Option<String>,
        #[doc = "Output only. Represents the graphics card device id."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub device_id: ::std::option::Option<i64>,
        #[doc = "Output only. Version of the GPU driver."]
        #[serde(
            rename = "driverVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub driver_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsAdapterInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsAdapterInfo {
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
    pub struct GoogleChromeManagementV1GraphicsInfo {
        #[doc = "Output only. Information about the graphics adapter (GPU)."]
        #[serde(
            rename = "adapterInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adapter_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1GraphicsAdapterInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsInfo {
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
    pub struct GoogleChromeManagementV1GraphicsStatusReport {
        #[doc = "Output only. Information about the displays for the device."]
        #[serde(
            rename = "displays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub displays:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1DisplayInfo>>,
        #[doc = "Output only. Time at which the graphics data was reported."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsStatusReport {
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
    pub struct GoogleChromeManagementV1HttpsLatencyRoutineData {
        #[doc = "Output only. HTTPS latency if routine succeeded or failed because of HIGH_LATENCY or VERY_HIGH_LATENCY."]
        #[serde(
            rename = "latency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latency: ::std::option::Option<String>,
        #[doc = "Output only. HTTPS latency routine problem if a problem occurred."]
        #[serde(
            rename = "problem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub problem: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1HttpsLatencyRoutineDataProblem,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1HttpsLatencyRoutineData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1HttpsLatencyRoutineData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        #[doc = "One or more DNS resolutions resulted in a failure."]
        FailedDnsResolutions,
        #[doc = "One or more HTTPS requests resulted in a failure."]
        FailedHttpsRequests,
        #[doc = "Average HTTPS request latency time between 500ms and 1000ms is high."]
        HighLatency,
        #[doc = "HTTPS latency problem not specified."]
        HttpsLatencyProblemUnspecified,
        #[doc = "Average HTTPS request latency time greater than 1000ms is very high."]
        VeryHighLatency,
    }
    impl GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedDnsResolutions => "FAILED_DNS_RESOLUTIONS" , GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedHttpsRequests => "FAILED_HTTPS_REQUESTS" , GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HighLatency => "HIGH_LATENCY" , GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HttpsLatencyProblemUnspecified => "HTTPS_LATENCY_PROBLEM_UNSPECIFIED" , GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: VeryHighLatency => "VERY_HIGH_LATENCY" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1HttpsLatencyRoutineDataProblem, ()>
        {
            Ok (match s { "FAILED_DNS_RESOLUTIONS" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedDnsResolutions , "FAILED_HTTPS_REQUESTS" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedHttpsRequests , "HIGH_LATENCY" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HighLatency , "HTTPS_LATENCY_PROBLEM_UNSPECIFIED" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HttpsLatencyProblemUnspecified , "VERY_HIGH_LATENCY" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: VeryHighLatency , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "FAILED_DNS_RESOLUTIONS" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedDnsResolutions , "FAILED_HTTPS_REQUESTS" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: FailedHttpsRequests , "HIGH_LATENCY" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HighLatency , "HTTPS_LATENCY_PROBLEM_UNSPECIFIED" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: HttpsLatencyProblemUnspecified , "VERY_HIGH_LATENCY" => GoogleChromeManagementV1HttpsLatencyRoutineDataProblem :: VeryHighLatency , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1HttpsLatencyRoutineDataProblem
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
    pub struct GoogleChromeManagementV1InstalledApp {
        #[doc = "Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. How the app was installed."]
        #[serde(
            rename = "appInstallType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_install_type: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1InstalledAppAppInstallType,
        >,
        #[doc = "Output only. Source of the installed app."]
        #[serde(
            rename = "appSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_source:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppSource>,
        #[doc = "Output only. Type of the app."]
        #[serde(
            rename = "appType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppType>,
        #[doc = "Output only. Count of browser devices with this app installed."]
        #[serde(
            rename = "browserDeviceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub browser_device_count: ::std::option::Option<i64>,
        #[doc = "Output only. Description of the installed app."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. Whether the app is disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Output only. Name of the installed app."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. Homepage uri of the installed app."]
        #[serde(
            rename = "homepageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub homepage_uri: ::std::option::Option<String>,
        #[doc = "Output only. Count of ChromeOS users with this app installed."]
        #[serde(
            rename = "osUserCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub os_user_count: ::std::option::Option<i64>,
        #[doc = "Output only. Permissions of the installed app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppInstallType {
        #[doc = "Administrator app install type."]
        Admin,
        #[doc = "Application install type not specified."]
        AppInstallTypeUnspecified,
        #[doc = "Development app install type."]
        Development,
        #[doc = "Multiple app install types."]
        Multiple,
        #[doc = "Normal app install type."]
        Normal,
        #[doc = "Other app install type."]
        Other,
        #[doc = "Sideloaded app install type."]
        Sideload,
    }
    impl GoogleChromeManagementV1InstalledAppAppInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppInstallType::Admin => "ADMIN",
                GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified => {
                    "APP_INSTALL_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppInstallType::Development => "DEVELOPMENT",
                GoogleChromeManagementV1InstalledAppAppInstallType::Multiple => "MULTIPLE",
                GoogleChromeManagementV1InstalledAppAppInstallType::Normal => "NORMAL",
                GoogleChromeManagementV1InstalledAppAppInstallType::Other => "OTHER",
                GoogleChromeManagementV1InstalledAppAppInstallType::Sideload => "SIDELOAD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppInstallType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppInstallType, ()> {
            Ok(match s {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppSource {
        #[doc = "Application source not specified."]
        AppSourceUnspecified,
        #[doc = "Generally for extensions and Chrome apps."]
        ChromeWebstore,
        #[doc = "Play Store app."]
        PlayStore,
    }
    impl GoogleChromeManagementV1InstalledAppAppSource {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified => {
                    "APP_SOURCE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore => "CHROME_WEBSTORE",
                GoogleChromeManagementV1InstalledAppAppSource::PlayStore => "PLAY_STORE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppSource {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppSource, ()> {
            Ok(match s {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppType {
        #[doc = "ARC++ app."]
        AndroidApp,
        #[doc = "Chrome app."]
        App,
        #[doc = "App type not specified."]
        AppTypeUnspecified,
        #[doc = "Chrome extension."]
        Extension,
        #[doc = "Chrome hosted app."]
        HostedApp,
        #[doc = "Chrome theme."]
        Theme,
    }
    impl GoogleChromeManagementV1InstalledAppAppType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppType::AndroidApp => "ANDROID_APP",
                GoogleChromeManagementV1InstalledAppAppType::App => "APP",
                GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified => {
                    "APP_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppType::Extension => "EXTENSION",
                GoogleChromeManagementV1InstalledAppAppType::HostedApp => "HOSTED_APP",
                GoogleChromeManagementV1InstalledAppAppType::Theme => "THEME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppType, ()> {
            Ok(match s {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppType {
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
    pub struct GoogleChromeManagementV1ListTelemetryDevicesResponse {
        #[doc = "Telemetry devices returned in the response."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1TelemetryDevice>>,
        #[doc = "Token to specify next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1ListTelemetryDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ListTelemetryDevicesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1ListTelemetryDevicesResponse {
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
    pub struct GoogleChromeManagementV1ListTelemetryEventsResponse {
        #[doc = "Token to specify next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Telemetry events returned in the response."]
        #[serde(
            rename = "telemetryEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub telemetry_events:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1TelemetryEvent>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1ListTelemetryEventsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ListTelemetryEventsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1ListTelemetryEventsResponse {
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
    pub struct GoogleChromeManagementV1ListTelemetryUsersResponse {
        #[doc = "Token to specify next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Telemetry users returned in the response."]
        #[serde(
            rename = "telemetryUsers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub telemetry_users:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1TelemetryUser>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ListTelemetryUsersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ListTelemetryUsersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleChromeManagementV1ListTelemetryUsersResponse {
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
    pub struct GoogleChromeManagementV1MemoryInfo {
        #[doc = "Output only. Amount of available RAM in bytes."]
        #[serde(
            rename = "availableRamBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub available_ram_bytes: ::std::option::Option<i64>,
        #[doc = "Output only. Total memory encryption info for the device."]
        #[serde(
            rename = "totalMemoryEncryption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_memory_encryption: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TotalMemoryEncryptionInfo,
        >,
        #[doc = "Output only. Total RAM in bytes."]
        #[serde(
            rename = "totalRamBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_ram_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1MemoryInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1MemoryInfo {
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
    pub struct GoogleChromeManagementV1MemoryStatusReport {
        #[doc = "Output only. Number of page faults during this collection"]
        #[serde(
            rename = "pageFaults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_faults: ::std::option::Option<i32>,
        #[doc = "Output only. The timestamp in milliseconds representing time at which this report was sampled."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
        #[doc = "Output only. Amount of free RAM in bytes (unreliable due to Garbage Collection)."]
        #[serde(
            rename = "systemRamFreeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub system_ram_free_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1MemoryStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1MemoryStatusReport {
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
    pub struct GoogleChromeManagementV1NetworkDevice {
        #[doc = "Output only. The integrated circuit card ID associated with the device’s sim card."]
        #[serde(
            rename = "iccid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iccid: ::std::option::Option<String>,
        #[doc = "Output only. IMEI (if applicable) of the corresponding network device."]
        #[serde(
            rename = "imei",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub imei: ::std::option::Option<String>,
        #[doc = "Output only. MAC address (if applicable) of the corresponding network device."]
        #[serde(
            rename = "macAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mac_address: ::std::option::Option<String>,
        #[doc = "Output only. The mobile directory number associated with the device’s sim card."]
        #[serde(
            rename = "mdn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mdn: ::std::option::Option<String>,
        #[doc = "Output only. MEID (if applicable) of the corresponding network device."]
        #[serde(
            rename = "meid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub meid: ::std::option::Option<String>,
        #[doc = "Output only. Network device type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1NetworkDeviceType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkDevice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1NetworkDeviceType {
        #[doc = "Cellular device."]
        CellularDevice,
        #[doc = "Ethernet device."]
        EthernetDevice,
        #[doc = "Network device type not specified."]
        NetworkDeviceTypeUnspecified,
        #[doc = "Wifi device."]
        WifiDevice,
    }
    impl GoogleChromeManagementV1NetworkDeviceType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1NetworkDeviceType::CellularDevice => "CELLULAR_DEVICE",
                GoogleChromeManagementV1NetworkDeviceType::EthernetDevice => "ETHERNET_DEVICE",
                GoogleChromeManagementV1NetworkDeviceType::NetworkDeviceTypeUnspecified => {
                    "NETWORK_DEVICE_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1NetworkDeviceType::WifiDevice => "WIFI_DEVICE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1NetworkDeviceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1NetworkDeviceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1NetworkDeviceType, ()> {
            Ok(match s {
                "CELLULAR_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::CellularDevice,
                "ETHERNET_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::EthernetDevice,
                "NETWORK_DEVICE_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1NetworkDeviceType::NetworkDeviceTypeUnspecified
                }
                "WIFI_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::WifiDevice,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1NetworkDeviceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1NetworkDeviceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1NetworkDeviceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CELLULAR_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::CellularDevice,
                "ETHERNET_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::EthernetDevice,
                "NETWORK_DEVICE_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1NetworkDeviceType::NetworkDeviceTypeUnspecified
                }
                "WIFI_DEVICE" => GoogleChromeManagementV1NetworkDeviceType::WifiDevice,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkDeviceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkDeviceType {
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
    pub struct GoogleChromeManagementV1NetworkDiagnosticsReport {
        #[doc = "Output only. HTTPS latency test data."]
        #[serde(
            rename = "httpsLatencyData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub https_latency_data:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1HttpsLatencyRoutineData>,
        #[doc = "Output only. Timestamp of when the diagnostics were collected."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkDiagnosticsReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkDiagnosticsReport {
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
    pub struct GoogleChromeManagementV1NetworkInfo {
        #[doc = "Output only. List of network devices."]
        #[serde(
            rename = "networkDevices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_devices:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1NetworkDevice>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkInfo {
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
    pub struct GoogleChromeManagementV1NetworkStatusReport {
        #[doc = "Output only. Current connection state of the network."]
        #[serde(
            rename = "connectionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_state: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1NetworkStatusReportConnectionState,
        >,
        #[doc = "Output only. Network connection type."]
        #[serde(
            rename = "connectionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_type: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1NetworkStatusReportConnectionType,
        >,
        #[doc = "Output only. Whether the wifi encryption key is turned off."]
        #[serde(
            rename = "encryptionOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_on: ::std::option::Option<bool>,
        #[doc = "Output only. Gateway IP address."]
        #[serde(
            rename = "gatewayIpAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gateway_ip_address: ::std::option::Option<String>,
        #[doc = "Output only. Network connection guid."]
        #[serde(
            rename = "guid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub guid: ::std::option::Option<String>,
        #[doc = "Output only. LAN IP address."]
        #[serde(
            rename = "lanIpAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lan_ip_address: ::std::option::Option<String>,
        #[doc = "Output only. Receiving bit rate measured in Megabits per second."]
        #[serde(
            rename = "receivingBitRateMbps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub receiving_bit_rate_mbps: ::std::option::Option<i64>,
        #[doc = "Output only. Time at which the network state was reported."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
        #[doc = "Output only. Signal strength for wireless networks measured in decibels."]
        #[serde(
            rename = "signalStrengthDbm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signal_strength_dbm: ::std::option::Option<i32>,
        #[doc = "Output only. Transmission bit rate measured in Megabits per second."]
        #[serde(
            rename = "transmissionBitRateMbps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub transmission_bit_rate_mbps: ::std::option::Option<i64>,
        #[doc = "Output only. Transmission power measured in decibels."]
        #[serde(
            rename = "transmissionPowerDbm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transmission_power_dbm: ::std::option::Option<i32>,
        #[doc = "Output only. Wifi link quality. Value ranges from \\[0, 70\\]. 0 indicates no signal and 70 indicates a strong signal."]
        #[serde(
            rename = "wifiLinkQuality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub wifi_link_quality: ::std::option::Option<i64>,
        #[doc = "Output only. Wifi power management enabled"]
        #[serde(
            rename = "wifiPowerManagementEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wifi_power_management_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1NetworkStatusReportConnectionState {
        #[doc = "The network is connected and not in a detected portal state, but internet connectivity may not be available."]
        Connected,
        #[doc = "The network is in the process of connecting."]
        Connecting,
        #[doc = "Network connection state unspecified."]
        NetworkConnectionStateUnspecified,
        #[doc = "The network is not connected."]
        NotConnected,
        #[doc = "The network is connected and internet connectivity is available."]
        Online,
        #[doc = "The network is connected but a portal state was detected. Internet connectivity may be limited."]
        Portal,
    }
    impl GoogleChromeManagementV1NetworkStatusReportConnectionState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connected => "CONNECTED" , GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connecting => "CONNECTING" , GoogleChromeManagementV1NetworkStatusReportConnectionState :: NetworkConnectionStateUnspecified => "NETWORK_CONNECTION_STATE_UNSPECIFIED" , GoogleChromeManagementV1NetworkStatusReportConnectionState :: NotConnected => "NOT_CONNECTED" , GoogleChromeManagementV1NetworkStatusReportConnectionState :: Online => "ONLINE" , GoogleChromeManagementV1NetworkStatusReportConnectionState :: Portal => "PORTAL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1NetworkStatusReportConnectionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1NetworkStatusReportConnectionState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1NetworkStatusReportConnectionState, ()>
        {
            Ok (match s { "CONNECTED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connected , "CONNECTING" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connecting , "NETWORK_CONNECTION_STATE_UNSPECIFIED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: NetworkConnectionStateUnspecified , "NOT_CONNECTED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: NotConnected , "ONLINE" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Online , "PORTAL" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Portal , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1NetworkStatusReportConnectionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1NetworkStatusReportConnectionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1NetworkStatusReportConnectionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONNECTED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connected , "CONNECTING" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Connecting , "NETWORK_CONNECTION_STATE_UNSPECIFIED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: NetworkConnectionStateUnspecified , "NOT_CONNECTED" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: NotConnected , "ONLINE" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Online , "PORTAL" => GoogleChromeManagementV1NetworkStatusReportConnectionState :: Portal , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1NetworkStatusReportConnectionState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1NetworkStatusReportConnectionState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1NetworkStatusReportConnectionType {
        #[doc = "Cellular network connection."]
        Cellular,
        #[doc = "Ethernet network connection."]
        Ethernet,
        #[doc = "Network connection type unspecified"]
        NetworkTypeUnspecified,
        #[doc = "Tether network connection."]
        Tether,
        #[doc = "VPN network connection."]
        Vpn,
        #[doc = "Wifi network connection."]
        Wifi,
    }
    impl GoogleChromeManagementV1NetworkStatusReportConnectionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1NetworkStatusReportConnectionType :: Cellular => "CELLULAR" , GoogleChromeManagementV1NetworkStatusReportConnectionType :: Ethernet => "ETHERNET" , GoogleChromeManagementV1NetworkStatusReportConnectionType :: NetworkTypeUnspecified => "NETWORK_TYPE_UNSPECIFIED" , GoogleChromeManagementV1NetworkStatusReportConnectionType :: Tether => "TETHER" , GoogleChromeManagementV1NetworkStatusReportConnectionType :: Vpn => "VPN" , GoogleChromeManagementV1NetworkStatusReportConnectionType :: Wifi => "WIFI" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1NetworkStatusReportConnectionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1NetworkStatusReportConnectionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1NetworkStatusReportConnectionType, ()>
        {
            Ok (match s { "CELLULAR" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Cellular , "ETHERNET" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Ethernet , "NETWORK_TYPE_UNSPECIFIED" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: NetworkTypeUnspecified , "TETHER" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Tether , "VPN" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Vpn , "WIFI" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Wifi , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1NetworkStatusReportConnectionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1NetworkStatusReportConnectionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1NetworkStatusReportConnectionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CELLULAR" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Cellular , "ETHERNET" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Ethernet , "NETWORK_TYPE_UNSPECIFIED" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: NetworkTypeUnspecified , "TETHER" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Tether , "VPN" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Vpn , "WIFI" => GoogleChromeManagementV1NetworkStatusReportConnectionType :: Wifi , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1NetworkStatusReportConnectionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1NetworkStatusReportConnectionType
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
    pub struct GoogleChromeManagementV1OsUpdateStatus {
        #[doc = "Output only. Timestamp of the last reboot."]
        #[serde(
            rename = "lastRebootTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_reboot_time: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last update check."]
        #[serde(
            rename = "lastUpdateCheckTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_check_time: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last successful update."]
        #[serde(
            rename = "lastUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_time: ::std::option::Option<String>,
        #[doc = "Output only. New platform version of the os image being downloaded and applied. It is only set when update status is OS_IMAGE_DOWNLOAD_IN_PROGRESS or OS_UPDATE_NEED_REBOOT. Note this could be a dummy “0.0.0.0” for OS_UPDATE_NEED_REBOOT status for some edge cases, e.g. update engine is restarted without a reboot."]
        #[serde(
            rename = "newPlatformVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_platform_version: ::std::option::Option<String>,
        #[doc = "Output only. New requested platform version from the pending updated kiosk app."]
        #[serde(
            rename = "newRequestedPlatformVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_requested_platform_version: ::std::option::Option<String>,
        #[doc = "Output only. Current state of the os update."]
        #[serde(
            rename = "updateState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_state: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1OsUpdateStatusUpdateState,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1OsUpdateStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1OsUpdateStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1OsUpdateStatusUpdateState {
        #[doc = "OS has started download on device."]
        OsImageDownloadInProgress,
        #[doc = "OS has not started downloading."]
        OsImageDownloadNotStarted,
        #[doc = "Device needs reboot to finish upload."]
        OsUpdateNeedReboot,
        #[doc = "State unspecified."]
        UpdateStateUnspecified,
    }
    impl GoogleChromeManagementV1OsUpdateStatusUpdateState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress => {
                    "OS_IMAGE_DOWNLOAD_IN_PROGRESS"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted => {
                    "OS_IMAGE_DOWNLOAD_NOT_STARTED"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot => {
                    "OS_UPDATE_NEED_REBOOT"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified => {
                    "UPDATE_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1OsUpdateStatusUpdateState, ()> {
            Ok(match s {
                "OS_IMAGE_DOWNLOAD_IN_PROGRESS" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress
                }
                "OS_IMAGE_DOWNLOAD_NOT_STARTED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted
                }
                "OS_UPDATE_NEED_REBOOT" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot
                }
                "UPDATE_STATE_UNSPECIFIED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OS_IMAGE_DOWNLOAD_IN_PROGRESS" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress
                }
                "OS_IMAGE_DOWNLOAD_NOT_STARTED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted
                }
                "OS_UPDATE_NEED_REBOOT" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot
                }
                "UPDATE_STATE_UNSPECIFIED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified
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
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1OsUpdateStatusUpdateState {
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
    pub struct GoogleChromeManagementV1PeripheralsReport {
        #[doc = "Output only. Timestamp of when the report was collected."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Reports of all usb connected devices."]
        #[serde(
            rename = "usbPeripheralReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usb_peripheral_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1UsbPeripheralReport>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1PeripheralsReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1PeripheralsReport {
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
    pub struct GoogleChromeManagementV1StorageInfo {
        #[doc = "The available space for user data storage in the device in bytes."]
        #[serde(
            rename = "availableDiskBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub available_disk_bytes: ::std::option::Option<i64>,
        #[doc = "The total space for user data storage in the device in bytes."]
        #[serde(
            rename = "totalDiskBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_disk_bytes: ::std::option::Option<i64>,
        #[doc = "Information for disk volumes"]
        #[serde(
            rename = "volume",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1StorageInfoDiskVolume>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageInfo {
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
    pub struct GoogleChromeManagementV1StorageInfoDiskVolume {
        #[doc = "Free storage space in bytes."]
        #[serde(
            rename = "storageFreeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub storage_free_bytes: ::std::option::Option<i64>,
        #[doc = "Total storage space in bytes."]
        #[serde(
            rename = "storageTotalBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub storage_total_bytes: ::std::option::Option<i64>,
        #[doc = "Disk volume id."]
        #[serde(
            rename = "volumeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageInfoDiskVolume {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageInfoDiskVolume {
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
    pub struct GoogleChromeManagementV1StorageStatusReport {
        #[doc = "Output only. Reports on disk."]
        #[serde(
            rename = "disk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1DiskInfo>>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageStatusReport {
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
    pub struct GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent {}
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent
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
    pub struct GoogleChromeManagementV1TelemetryDevice {
        #[doc = "Output only. Audio reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "audioStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1AudioStatusReport>>,
        #[doc = "Output only. Information on battery specs for the device."]
        #[serde(
            rename = "batteryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_info:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatteryInfo>>,
        #[doc = "Output only. Battery reports collected periodically."]
        #[serde(
            rename = "batteryStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatteryStatusReport>>,
        #[doc = "Output only. Boot performance reports of the device."]
        #[serde(
            rename = "bootPerformanceReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boot_performance_report: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1BootPerformanceReport>,
        >,
        #[doc = "Output only. Information regarding CPU specs for the device."]
        #[serde(
            rename = "cpuInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_info: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuInfo>>,
        #[doc = "Output only. CPU status reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "cpuStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuStatusReport>>,
        #[doc = "Output only. Google Workspace Customer whose enterprise enrolled the device."]
        #[serde(
            rename = "customer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer: ::std::option::Option<String>,
        #[doc = "Output only. The unique Directory API ID of the device. This value is the same as the Admin Console’s Directory API ID in the ChromeOS Devices tab"]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. Contains information regarding Graphic peripherals for the device."]
        #[serde(
            rename = "graphicsInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub graphics_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1GraphicsInfo>,
        #[doc = "Output only. Graphics reports collected periodically."]
        #[serde(
            rename = "graphicsStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub graphics_status_report: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1GraphicsStatusReport>,
        >,
        #[doc = "Output only. Information regarding memory specs for the device."]
        #[serde(
            rename = "memoryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_info: ::std::option::Option<crate::schemas::GoogleChromeManagementV1MemoryInfo>,
        #[doc = "Output only. Memory status reports collected periodically sorted decreasing by report_time."]
        #[serde(
            rename = "memoryStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1MemoryStatusReport>>,
        #[doc = "Output only. Resource name of the device."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Network diagnostics collected periodically."]
        #[serde(
            rename = "networkDiagnosticsReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_diagnostics_report: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1NetworkDiagnosticsReport>,
        >,
        #[doc = "Output only. Network devices information."]
        #[serde(
            rename = "networkInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1NetworkInfo>,
        #[doc = "Output only. Network specs collected periodically."]
        #[serde(
            rename = "networkStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1NetworkStatusReport>>,
        #[doc = "Output only. Organization unit ID of the device."]
        #[serde(
            rename = "orgUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_unit_id: ::std::option::Option<String>,
        #[doc = "Output only. Contains relevant information regarding ChromeOS update status."]
        #[serde(
            rename = "osUpdateStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_update_status:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1OsUpdateStatus>>,
        #[doc = "Output only. Device serial number. This value is the same as the Admin Console’s Serial Number in the ChromeOS Devices tab."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Information of storage specs for the device."]
        #[serde(
            rename = "storageInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1StorageInfo>,
        #[doc = "Output only. Storage reports collected periodically."]
        #[serde(
            rename = "storageStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1StorageStatusReport>>,
        #[doc = "Output only. Information on Thunderbolt bus."]
        #[serde(
            rename = "thunderboltInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thunderbolt_info:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ThunderboltInfo>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryDevice {
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
    pub struct GoogleChromeManagementV1TelemetryDeviceInfo {
        #[doc = "Output only. The unique Directory API ID of the device. This value is the same as the Admin Console’s Directory API ID in the ChromeOS Devices tab."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. Organization unit ID of the device."]
        #[serde(
            rename = "orgUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_unit_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryDeviceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryDeviceInfo {
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
    pub struct GoogleChromeManagementV1TelemetryEvent {
        #[doc = "Output only. Payload for audio severe underrun event. Present only when the `event_type` field is `AUDIO_SEVERE_UNDERRUN`."]
        #[serde(
            rename = "audioSevereUnderrunEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_severe_underrun_event: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent,
        >,
        #[doc = "Output only. Information about the device associated with the event."]
        #[serde(
            rename = "device",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1TelemetryDeviceInfo>,
        #[doc = "The event type of the current event."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1TelemetryEventEventType>,
        #[doc = "Output only. Payload for HTTPS latency change event. Present only when `event_type` is `NETWORK_HTTPS_LATENCY_CHANGE`."]
        #[serde(
            rename = "httpsLatencyChangeEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub https_latency_change_event: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent,
        >,
        #[doc = "Output only. Resource name of the event."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Timestamp that represents when the event was reported."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Payload for usb peripherals event. Present only when the `event_type` field is either `USB_ADDED` or `USB_REMOVED`."]
        #[serde(
            rename = "usbPeripheralsEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usb_peripherals_event: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TelemetryUsbPeripheralsEvent,
        >,
        #[doc = "Output only. Information about the user associated with the event."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::GoogleChromeManagementV1TelemetryUserInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1TelemetryEventEventType {
        #[doc = "Triggered when a audio devices run out of buffer data for more than 5 seconds."]
        AudioSevereUnderrun,
        #[doc = "Event type unknown."]
        EventTypeUnspecified,
        #[doc = "Triggered when a new HTTPS latency problem was detected or the device has recovered form an existing HTTPS latency problem."]
        NetworkHttpsLatencyChange,
        #[doc = "Triggered when USB devices are added."]
        UsbAdded,
        #[doc = "Triggered when USB devices are removed."]
        UsbRemoved,
    }
    impl GoogleChromeManagementV1TelemetryEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1TelemetryEventEventType::AudioSevereUnderrun => {
                    "AUDIO_SEVERE_UNDERRUN"
                }
                GoogleChromeManagementV1TelemetryEventEventType::EventTypeUnspecified => {
                    "EVENT_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1TelemetryEventEventType::NetworkHttpsLatencyChange => {
                    "NETWORK_HTTPS_LATENCY_CHANGE"
                }
                GoogleChromeManagementV1TelemetryEventEventType::UsbAdded => "USB_ADDED",
                GoogleChromeManagementV1TelemetryEventEventType::UsbRemoved => "USB_REMOVED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1TelemetryEventEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1TelemetryEventEventType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1TelemetryEventEventType, ()> {
            Ok(match s {
                "AUDIO_SEVERE_UNDERRUN" => {
                    GoogleChromeManagementV1TelemetryEventEventType::AudioSevereUnderrun
                }
                "EVENT_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1TelemetryEventEventType::EventTypeUnspecified
                }
                "NETWORK_HTTPS_LATENCY_CHANGE" => {
                    GoogleChromeManagementV1TelemetryEventEventType::NetworkHttpsLatencyChange
                }
                "USB_ADDED" => GoogleChromeManagementV1TelemetryEventEventType::UsbAdded,
                "USB_REMOVED" => GoogleChromeManagementV1TelemetryEventEventType::UsbRemoved,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1TelemetryEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1TelemetryEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1TelemetryEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUDIO_SEVERE_UNDERRUN" => {
                    GoogleChromeManagementV1TelemetryEventEventType::AudioSevereUnderrun
                }
                "EVENT_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1TelemetryEventEventType::EventTypeUnspecified
                }
                "NETWORK_HTTPS_LATENCY_CHANGE" => {
                    GoogleChromeManagementV1TelemetryEventEventType::NetworkHttpsLatencyChange
                }
                "USB_ADDED" => GoogleChromeManagementV1TelemetryEventEventType::UsbAdded,
                "USB_REMOVED" => GoogleChromeManagementV1TelemetryEventEventType::UsbRemoved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryEventEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryEventEventType {
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
    pub struct GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent { # [doc = "HTTPS latency routine data that triggered the event."] # [serde (rename = "httpsLatencyRoutineData" , default , skip_serializing_if = "std::option::Option::is_none")] pub https_latency_routine_data : :: std :: option :: Option < crate :: schemas :: GoogleChromeManagementV1HttpsLatencyRoutineData > , # [doc = "Current HTTPS latency state."] # [serde (rename = "httpsLatencyState" , default , skip_serializing_if = "std::option::Option::is_none")] pub https_latency_state : :: std :: option :: Option < crate :: schemas :: GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState > , }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState {
        #[doc = "HTTPS latency state is unspecified."]
        HttpsLatencyStateUnspecified,
        #[doc = "HTTPS latency problem."]
        Problem,
        #[doc = "HTTPS latency recovered from a problem."]
        Recovery,
    }
    impl GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: HttpsLatencyStateUnspecified => "HTTPS_LATENCY_STATE_UNSPECIFIED" , GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Problem => "PROBLEM" , GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Recovery => "RECOVERY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState,
            (),
        > {
            Ok (match s { "HTTPS_LATENCY_STATE_UNSPECIFIED" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: HttpsLatencyStateUnspecified , "PROBLEM" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Problem , "RECOVERY" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Recovery , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "HTTPS_LATENCY_STATE_UNSPECIFIED" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: HttpsLatencyStateUnspecified , "PROBLEM" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Problem , "RECOVERY" => GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState :: Recovery , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyState
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
    pub struct GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {
        #[doc = "List of usb devices that were either added or removed."]
        #[serde(
            rename = "usbPeripheralReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usb_peripheral_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1UsbPeripheralReport>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TelemetryUsbPeripheralsEvent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {
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
    pub struct GoogleChromeManagementV1TelemetryUser {
        #[doc = "G Suite Customer whose enterprise enrolled the device."]
        #[serde(
            rename = "customer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer: ::std::option::Option<String>,
        #[doc = "Resource name of the user."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Organization unit of the user."]
        #[serde(
            rename = "orgUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_unit_id: ::std::option::Option<String>,
        #[doc = "Telemetry data collected from a managed user and device."]
        #[serde(
            rename = "userDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_device:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1TelemetryUserDevice>>,
        #[doc = "Email address of the user."]
        #[serde(
            rename = "userEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
        #[doc = "Directory ID of the user."]
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryUser {
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
    pub struct GoogleChromeManagementV1TelemetryUserDevice {
        #[doc = "Output only. Audio reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "audioStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1AudioStatusReport>>,
        #[doc = "The unique Directory API ID of the device. This value is the same as the Admin Console’s Directory API ID in the ChromeOS Devices tab."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. Peripherals reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "peripheralsReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub peripherals_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1PeripheralsReport>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryUserDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryUserDevice {
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
    pub struct GoogleChromeManagementV1TelemetryUserInfo {
        #[doc = "Output only. User’s email."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Output only. Organization unit ID of the user."]
        #[serde(
            rename = "orgUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_unit_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryUserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryUserInfo {
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
    pub struct GoogleChromeManagementV1ThunderboltInfo {
        #[doc = "Security level of the Thunderbolt bus."]
        #[serde(
            rename = "securityLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_level: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1ThunderboltInfoSecurityLevel,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ThunderboltInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ThunderboltInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        #[doc = "The firmware automatically creates tunnels for Thunderbolt."]
        ThunderboltSecurityDpOnlyLevel,
        #[doc = "Thunderbolt security level is not set."]
        ThunderboltSecurityLevelUnspecified,
        #[doc = "PCIE tunneling is disabled."]
        ThunderboltSecurityNoPcieLevel,
        #[doc = "All devices are automatically connected by the firmware. No user approval is needed."]
        ThunderboltSecurityNoneLevel,
        #[doc = "User is asked whether the device is allowed to be connected. In addition the device is sent a challenge that should match the expected one based on a random key written to the key sysfs attribute"]
        ThunderboltSecuritySecureLevel,
        #[doc = "The firmware automatically creates tunnels for the USB controller and Display Port in a dock. All PCIe links downstream of the dock are removed."]
        ThunderboltSecurityUsbOnlyLevel,
        #[doc = "User is asked whether the device is allowed to be connected."]
        ThunderboltSecurityUserLevel,
    }
    impl GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityDpOnlyLevel => "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityLevelUnspecified => "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoPcieLevel => "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoneLevel => "THUNDERBOLT_SECURITY_NONE_LEVEL" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecuritySecureLevel => "THUNDERBOLT_SECURITY_SECURE_LEVEL" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUsbOnlyLevel => "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL" , GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUserLevel => "THUNDERBOLT_SECURITY_USER_LEVEL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1ThunderboltInfoSecurityLevel, ()>
        {
            Ok (match s { "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityDpOnlyLevel , "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityLevelUnspecified , "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoPcieLevel , "THUNDERBOLT_SECURITY_NONE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoneLevel , "THUNDERBOLT_SECURITY_SECURE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecuritySecureLevel , "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUsbOnlyLevel , "THUNDERBOLT_SECURITY_USER_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUserLevel , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityDpOnlyLevel , "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityLevelUnspecified , "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoPcieLevel , "THUNDERBOLT_SECURITY_NONE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityNoneLevel , "THUNDERBOLT_SECURITY_SECURE_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecuritySecureLevel , "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUsbOnlyLevel , "THUNDERBOLT_SECURITY_USER_LEVEL" => GoogleChromeManagementV1ThunderboltInfoSecurityLevel :: ThunderboltSecurityUserLevel , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1ThunderboltInfoSecurityLevel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ThunderboltInfoSecurityLevel {
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
    pub struct GoogleChromeManagementV1TotalMemoryEncryptionInfo {
        #[doc = "Memory encryption algorithm."]
        #[serde(
            rename = "encryptionAlgorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_algorithm: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm,
        >,
        #[doc = "The state of memory encryption on the device."]
        #[serde(
            rename = "encryptionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_state: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState,
        >,
        #[doc = "The length of the encryption keys."]
        #[serde(
            rename = "keyLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub key_length: ::std::option::Option<i64>,
        #[doc = "The maximum number of keys that can be used for encryption."]
        #[serde(
            rename = "maxKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_keys: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TotalMemoryEncryptionInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TotalMemoryEncryptionInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm {
        #[doc = "The memory encryption algorithm is using the AES_XTS encryption algorithm with a 128 bit block cypher."]
        MemoryEncryptionAlgorithmAesXts128,
        #[doc = "The memory encryption algorithm is using the AES_XTS encryption algorithm with a 256 bit block cypher."]
        MemoryEncryptionAlgorithmAesXts256,
        #[doc = "The memory encryption algorithm being used is unknown."]
        MemoryEncryptionAlgorithmUnknown,
        #[doc = "Memory encryption algorithm is not set."]
        MemoryEncryptionAlgorithmUnspecified,
    }
    impl GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts128 => "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts256 => "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnknown => "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnspecified => "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm,
            (),
        > {
            Ok (match s { "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts128 , "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts256 , "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnknown , "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts128 , "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmAesXts256 , "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnknown , "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm :: MemoryEncryptionAlgorithmUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithm
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState {
        #[doc = "Memory encrpytion on the device is disabled."]
        MemoryEncryptionStateDisabled,
        #[doc = "Memory encryption on the device uses multi-key total memory encryption."]
        MemoryEncryptionStateMktme,
        #[doc = "Memory encryption on the device uses total memory encryption."]
        MemoryEncryptionStateTme,
        #[doc = "The memory encryption state is unknown."]
        MemoryEncryptionStateUnknown,
        #[doc = "Memory encryption state is not set."]
        MemoryEncryptionStateUnspecified,
    }
    impl GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateDisabled => "MEMORY_ENCRYPTION_STATE_DISABLED" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateMktme => "MEMORY_ENCRYPTION_STATE_MKTME" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateTme => "MEMORY_ENCRYPTION_STATE_TME" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnknown => "MEMORY_ENCRYPTION_STATE_UNKNOWN" , GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnspecified => "MEMORY_ENCRYPTION_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState,
            (),
        > {
            Ok (match s { "MEMORY_ENCRYPTION_STATE_DISABLED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateDisabled , "MEMORY_ENCRYPTION_STATE_MKTME" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateMktme , "MEMORY_ENCRYPTION_STATE_TME" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateTme , "MEMORY_ENCRYPTION_STATE_UNKNOWN" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnknown , "MEMORY_ENCRYPTION_STATE_UNSPECIFIED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "MEMORY_ENCRYPTION_STATE_DISABLED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateDisabled , "MEMORY_ENCRYPTION_STATE_MKTME" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateMktme , "MEMORY_ENCRYPTION_STATE_TME" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateTme , "MEMORY_ENCRYPTION_STATE_UNKNOWN" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnknown , "MEMORY_ENCRYPTION_STATE_UNSPECIFIED" => GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState :: MemoryEncryptionStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionState
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
    pub struct GoogleChromeManagementV1UsbPeripheralReport {
        #[doc = "Output only. Categories the device belongs to https://www.usb.org/defined-class-codes"]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Class ID https://www.usb.org/defined-class-codes"]
        #[serde(
            rename = "classId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub class_id: ::std::option::Option<i32>,
        #[doc = "Output only. Firmware version"]
        #[serde(
            rename = "firmwareVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub firmware_version: ::std::option::Option<String>,
        #[doc = "Output only. Device name, model name, or product name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Product ID"]
        #[serde(
            rename = "pid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pid: ::std::option::Option<i32>,
        #[doc = "Output only. Subclass ID https://www.usb.org/defined-class-codes"]
        #[serde(
            rename = "subclassId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subclass_id: ::std::option::Option<i32>,
        #[doc = "Output only. Vendor name"]
        #[serde(
            rename = "vendor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vendor: ::std::option::Option<String>,
        #[doc = "Output only. Vendor ID"]
        #[serde(
            rename = "vid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vid: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1UsbPeripheralReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1UsbPeripheralReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
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
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
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
    pub struct GoogleTypeDate {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
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
    impl ::google_field_selector::FieldSelector for GoogleTypeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDate {
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
    #[doc = "Actions that can be performed on the customers resource"]
    pub fn customers(&self) -> crate::resources::customers::CustomersActions {
        crate::resources::customers::CustomersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod customers {
        pub mod params {}
        pub struct CustomersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the apps resource"]
            pub fn apps(&self) -> crate::resources::customers::apps::AppsActions {
                crate::resources::customers::apps::AppsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the reports resource"]
            pub fn reports(&self) -> crate::resources::customers::reports::ReportsActions {
                crate::resources::customers::reports::ReportsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the telemetry resource"]
            pub fn telemetry(&self) -> crate::resources::customers::telemetry::TelemetryActions {
                crate::resources::customers::telemetry::TelemetryActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod apps {
            pub mod params {}
            pub struct AppsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AppsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generate summary of app installation requests."]
                pub fn count_chrome_app_requests(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeAppRequestsRequestBuilder {
                    CountChromeAppRequestsRequestBuilder {
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
                        customer: customer.into(),
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the android resource"]
                pub fn android(
                    &self,
                ) -> crate::resources::customers::apps::android::AndroidActions {
                    crate::resources::customers::apps::android::AndroidActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the chrome resource"]
                pub fn chrome(&self) -> crate::resources::customers::apps::chrome::ChromeActions {
                    crate::resources::customers::apps::chrome::ChromeActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the web resource"]
                pub fn web(&self) -> crate::resources::customers::apps::web::WebActions {
                    crate::resources::customers::apps::web::WebActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [AppsActions::count_chrome_app_requests()](struct.AppsActions.html#method.count_chrome_app_requests)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeAppRequestsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                order_by: ::std::option::Option<String>,
                org_unit_id: ::std::option::Option<String>,
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
            impl<'a> CountChromeAppRequestsRequestBuilder<'a> {
                #[doc = "Field used to order results. Supported fields: * request_count * latest_request_time"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 50, anything above will be coerced to 50."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                #[doc = "\nExecute the request and yield each item in the `requestedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_requested_apps<T>(
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
                    self.stream_requested_apps_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `requestedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_requested_apps_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1ChromeAppRequest,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_requested_apps_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `requestedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_requested_apps_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1ChromeAppRequest,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_requested_apps_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `requestedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_requested_apps_with_fields<T, F>(
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
                        #[serde(rename = "requestedApps")]
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
                        let mut selector = concat!("nextPageToken,", "requestedApps").to_owned();
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
                        crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                        crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/apps:countChromeAppRequests");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            impl<'a> crate::stream::StreamableMethod for CountChromeAppRequestsRequestBuilder<'a> {
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
            pub mod android {
                pub mod params {}
                pub struct AndroidActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AndroidActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [AndroidActions::get()](struct.AndroidActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
            pub mod chrome {
                pub mod params {}
                pub struct ChromeActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ChromeActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [ChromeActions::get()](struct.ChromeActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
            pub mod web {
                pub mod params {}
                pub struct WebActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> WebActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [WebActions::get()](struct.WebActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
        }
        pub mod reports {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum FindInstalledAppDevicesAppType {
                    #[doc = "ARC++ app."]
                    AndroidApp,
                    #[doc = "Chrome app."]
                    App,
                    #[doc = "App type not specified."]
                    AppTypeUnspecified,
                    #[doc = "Chrome extension."]
                    Extension,
                    #[doc = "Chrome hosted app."]
                    HostedApp,
                    #[doc = "Chrome theme."]
                    Theme,
                }
                impl FindInstalledAppDevicesAppType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            FindInstalledAppDevicesAppType::AndroidApp => "ANDROID_APP",
                            FindInstalledAppDevicesAppType::App => "APP",
                            FindInstalledAppDevicesAppType::AppTypeUnspecified => {
                                "APP_TYPE_UNSPECIFIED"
                            }
                            FindInstalledAppDevicesAppType::Extension => "EXTENSION",
                            FindInstalledAppDevicesAppType::HostedApp => "HOSTED_APP",
                            FindInstalledAppDevicesAppType::Theme => "THEME",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for FindInstalledAppDevicesAppType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for FindInstalledAppDevicesAppType {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<FindInstalledAppDevicesAppType, ()>
                    {
                        Ok(match s {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for FindInstalledAppDevicesAppType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for FindInstalledAppDevicesAppType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for FindInstalledAppDevicesAppType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for FindInstalledAppDevicesAppType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for FindInstalledAppDevicesAppType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ReportsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ReportsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generate report of the number of devices expiring in each month of the selected time frame. Devices are grouped by auto update expiration date and model. Further information can be found [here](https://support.google.com/chrome/a/answer/10564947)."]
                pub fn count_chrome_devices_reaching_auto_expiration_date(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeDevicesReachingAutoExpirationDateRequestBuilder {
                    CountChromeDevicesReachingAutoExpirationDateRequestBuilder {
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
                        customer: customer.into(),
                        max_aue_date: None,
                        min_aue_date: None,
                        org_unit_id: None,
                    }
                }
                #[doc = "Counts of ChromeOS devices that have not synced policies or have lacked user activity in the past 28 days, are out of date, or are not complaint. Further information can be found here https://support.google.com/chrome/a/answer/10564947"]
                pub fn count_chrome_devices_that_need_attention(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeDevicesThatNeedAttentionRequestBuilder {
                    CountChromeDevicesThatNeedAttentionRequestBuilder {
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
                        customer: customer.into(),
                        org_unit_id: None,
                        read_mask: None,
                    }
                }
                #[doc = "Counts of devices with a specific hardware specification from the requested hardware type (for example model name, processor type). Further information can be found here https://support.google.com/chrome/a/answer/10564947"]
                pub fn count_chrome_hardware_fleet_devices(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeHardwareFleetDevicesRequestBuilder {
                    CountChromeHardwareFleetDevicesRequestBuilder {
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
                        customer: customer.into(),
                        org_unit_id: None,
                        read_mask: None,
                    }
                }
                #[doc = "Generate report of installed Chrome versions."]
                pub fn count_chrome_versions(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeVersionsRequestBuilder {
                    CountChromeVersionsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of app installations."]
                pub fn count_installed_apps(
                    &self,
                    customer: impl Into<String>,
                ) -> CountInstalledAppsRequestBuilder {
                    CountInstalledAppsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of devices that have a specified app installed."]
                pub fn find_installed_app_devices(
                    &self,
                    customer: impl Into<String>,
                ) -> FindInstalledAppDevicesRequestBuilder {
                    FindInstalledAppDevicesRequestBuilder {
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
                        customer: customer.into(),
                        app_id: None,
                        app_type: None,
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [ReportsActions::count_chrome_devices_reaching_auto_expiration_date()](struct.ReportsActions.html#method.count_chrome_devices_reaching_auto_expiration_date)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeDevicesReachingAutoExpirationDateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                max_aue_date: ::std::option::Option<String>,
                min_aue_date: ::std::option::Option<String>,
                org_unit_id: ::std::option::Option<String>,
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
            impl<'a> CountChromeDevicesReachingAutoExpirationDateRequestBuilder<'a> {
                #[doc = "Optional. Maximum expiration date in format yyyy-mm-dd in UTC timezone. If included returns all devices that have already expired and devices with auto expiration date equal to or earlier than the maximum date."]
                pub fn max_aue_date(mut self, value: impl Into<String>) -> Self {
                    self.max_aue_date = Some(value.into());
                    self
                }
                #[doc = "Optional. Maximum expiration date in format yyyy-mm-dd in UTC timezone. If included returns all devices that have already expired and devices with auto expiration date equal to or later than the minimum date."]
                pub fn min_aue_date(mut self, value: impl Into<String>) -> Self {
                    self.min_aue_date = Some(value.into());
                    self
                }
                #[doc = "Optional. The organizational unit ID, if omitted, will return data for all organizational units."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse , crate :: Error >{
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeDevicesReachingAutoExpirationDate");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("maxAueDate", &self.max_aue_date)]);
                    req = req.query(&[("minAueDate", &self.min_aue_date)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            #[doc = "Created via [ReportsActions::count_chrome_devices_that_need_attention()](struct.ReportsActions.html#method.count_chrome_devices_that_need_attention)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeDevicesThatNeedAttentionRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                org_unit_id: ::std::option::Option<String>,
                read_mask: ::std::option::Option<String>,
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
            impl<'a> CountChromeDevicesThatNeedAttentionRequestBuilder<'a> {
                #[doc = "Optional. The ID of the organizational unit. If omitted, all data will be returned."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Required. Mask of the fields that should be populated in the returned report."]
                pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                    self.read_mask = Some(value.into());
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse , crate :: Error >{
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeDevicesThatNeedAttention");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
                    req = req.query(&[("readMask", &self.read_mask)]);
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
            #[doc = "Created via [ReportsActions::count_chrome_hardware_fleet_devices()](struct.ReportsActions.html#method.count_chrome_hardware_fleet_devices)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeHardwareFleetDevicesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                org_unit_id: ::std::option::Option<String>,
                read_mask: ::std::option::Option<String>,
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
            impl<'a> CountChromeHardwareFleetDevicesRequestBuilder<'a> {
                #[doc = "Optional. The ID of the organizational unit. If omitted, all data will be returned."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Required. Mask of the fields that should be populated in the returned report."]
                pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                    self.read_mask = Some(value.into());
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
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeHardwareFleetDevices");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
                    req = req.query(&[("readMask", &self.read_mask)]);
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
            #[doc = "Created via [ReportsActions::count_chrome_versions()](struct.ReportsActions.html#method.count_chrome_versions)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeVersionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: ::std::option::Option<String>,
                org_unit_id: ::std::option::Option<String>,
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
            impl<'a> CountChromeVersionsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                #[doc = "\nExecute the request and yield each item in the `browserVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_browser_versions<T>(
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
                    self.stream_browser_versions_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `browserVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_browser_versions_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1BrowserVersion,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_browser_versions_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `browserVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_browser_versions_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1BrowserVersion,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_browser_versions_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `browserVersions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_browser_versions_with_fields<T, F>(
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
                        #[serde(rename = "browserVersions")]
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
                        let mut selector = concat!("nextPageToken,", "browserVersions").to_owned();
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
                        crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                        crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeVersions");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            impl<'a> crate::stream::StreamableMethod for CountChromeVersionsRequestBuilder<'a> {
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
            #[doc = "Created via [ReportsActions::count_installed_apps()](struct.ReportsActions.html#method.count_installed_apps)"]
            #[derive(Debug, Clone)]
            pub struct CountInstalledAppsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: ::std::option::Option<String>,
                order_by: ::std::option::Option<String>,
                org_unit_id: ::std::option::Option<String>,
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
            impl<'a> CountInstalledAppsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name * app_id"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * app_id"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                #[doc = "\nExecute the request and yield each item in the `installedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_installed_apps<T>(
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
                    self.stream_installed_apps_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `installedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_installed_apps_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1InstalledApp,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_installed_apps_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `installedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_installed_apps_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleChromeManagementV1InstalledApp,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_installed_apps_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `installedApps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_installed_apps_with_fields<T, F>(
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
                        #[serde(rename = "installedApps")]
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
                        let mut selector = concat!("nextPageToken,", "installedApps").to_owned();
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
                        crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                        crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countInstalledApps");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            impl<'a> crate::stream::StreamableMethod for CountInstalledAppsRequestBuilder<'a> {
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
            #[doc = "Created via [ReportsActions::find_installed_app_devices()](struct.ReportsActions.html#method.find_installed_app_devices)"]
            #[derive(Debug, Clone)]
            pub struct FindInstalledAppDevicesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                app_id: ::std::option::Option<String>,
                app_type: ::std::option::Option<
                    crate::resources::customers::reports::params::FindInstalledAppDevicesAppType,
                >,
                filter: ::std::option::Option<String>,
                order_by: ::std::option::Option<String>,
                org_unit_id: ::std::option::Option<String>,
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
            impl<'a> FindInstalledAppDevicesRequestBuilder<'a> {
                #[doc = "Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
                pub fn app_id(mut self, value: impl Into<String>) -> Self {
                    self.app_id = Some(value.into());
                    self
                }
                #[doc = "Type of the app."]
                pub fn app_type(
                    mut self,
                    value : crate :: resources :: customers :: reports :: params :: FindInstalledAppDevicesAppType,
                ) -> Self {
                    self.app_type = Some(value);
                    self
                }
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * machine * device_id"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_devices<T>(
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
                    self.stream_devices_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_devices_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::GoogleChromeManagementV1Device, crate::Error>,
                > + 'a {
                    self.stream_devices_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_devices_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::GoogleChromeManagementV1Device, crate::Error>,
                > + 'a {
                    self.stream_devices_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_devices_with_fields<T, F>(
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
                        #[serde(rename = "devices")]
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
                        let mut selector = concat!("nextPageToken,", "devices").to_owned();
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
                        crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                        crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:findInstalledAppDevices");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("appId", &self.app_id)]);
                    req = req.query(&[("appType", &self.app_type)]);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            impl<'a> crate::stream::StreamableMethod for FindInstalledAppDevicesRequestBuilder<'a> {
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
        pub mod telemetry {
            pub mod params {}
            pub struct TelemetryActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> TelemetryActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the devices resource"]
                pub fn devices(
                    &self,
                ) -> crate::resources::customers::telemetry::devices::DevicesActions
                {
                    crate::resources::customers::telemetry::devices::DevicesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the events resource"]
                pub fn events(
                    &self,
                ) -> crate::resources::customers::telemetry::events::EventsActions {
                    crate::resources::customers::telemetry::events::EventsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the users resource"]
                pub fn users(&self) -> crate::resources::customers::telemetry::users::UsersActions {
                    crate::resources::customers::telemetry::users::UsersActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod devices {
                pub mod params {}
                pub struct DevicesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> DevicesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get telemetry device."]
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
                            read_mask: None,
                        }
                    }
                    #[doc = "List all telemetry devices."]
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
                            page_size: None,
                            page_token: None,
                            read_mask: None,
                        }
                    }
                }
                #[doc = "Created via [DevicesActions::get()](struct.DevicesActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    read_mask: ::std::option::Option<String>,
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
                    #[doc = "Required. Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1TelemetryDevice, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1TelemetryDevice, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
                #[doc = "Created via [DevicesActions::list()](struct.DevicesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
                    read_mask: ::std::option::Option<String>,
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
                    #[doc = "Optional. Only include resources that match the filter. Supported filter fields: - org_unit_id - serial_number - device_id "]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of results to return. Default value is 100. Maximum value is 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Token to specify next page in the list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Required. Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                    #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_devices<T>(
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
                        self.stream_devices_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_devices_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryDevice,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_devices_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_devices_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryDevice,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_devices_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `devices` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_devices_with_fields<T, F>(
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
                            #[serde(rename = "devices")]
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
                            let mut selector = concat!("nextPageToken,", "devices").to_owned();
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                    ) -> Result<
                        crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                        crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/telemetry/devices");
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
            pub mod events {
                pub mod params {}
                pub struct EventsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> EventsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "List telemetry events."]
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
                            page_size: None,
                            page_token: None,
                            read_mask: None,
                        }
                    }
                }
                #[doc = "Created via [EventsActions::list()](struct.EventsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
                    read_mask: ::std::option::Option<String>,
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
                    #[doc = "Optional. Only include resources that match the filter. Supported filter fields: - device_id - user_id - device_org_unit_id - user_org_unit_id - timestamp - event_type The “timestamp” filter accepts either Epoch milliseconds or RFC 3339 formatted time surrounded by simple double quotes."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Optional. Maximum number of results to return. Default value is 100. Maximum value is 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. Token to specify next page in the list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Required. Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                    #[doc = "\nExecute the request and yield each item in the `telemetryEvents` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_telemetry_events<T>(
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
                        self.stream_telemetry_events_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryEvents` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_telemetry_events_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryEvent,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_telemetry_events_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryEvents` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_telemetry_events_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryEvent,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_telemetry_events_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryEvents` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_telemetry_events_with_fields<T, F>(
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
                            #[serde(rename = "telemetryEvents")]
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
                                concat!("nextPageToken,", "telemetryEvents").to_owned();
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryEventsResponse,
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryEventsResponse,
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
                    ) -> Result<
                        crate::schemas::GoogleChromeManagementV1ListTelemetryEventsResponse,
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
                        crate::schemas::GoogleChromeManagementV1ListTelemetryEventsResponse,
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/telemetry/events");
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
            pub mod users {
                pub mod params {}
                pub struct UsersActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> UsersActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get telemetry user."]
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
                            read_mask: None,
                        }
                    }
                    #[doc = "List all telemetry users."]
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
                            page_size: None,
                            page_token: None,
                            read_mask: None,
                        }
                    }
                }
                #[doc = "Created via [UsersActions::get()](struct.UsersActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    read_mask: ::std::option::Option<String>,
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
                    #[doc = "Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1TelemetryUser, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1TelemetryUser, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
                #[doc = "Created via [UsersActions::list()](struct.UsersActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
                    read_mask: ::std::option::Option<String>,
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
                    #[doc = "Only include resources that match the filter. Supported filter fields: - user_id - user_org_unit_id "]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of results to return. Default value is 100. Maximum value is 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Token to specify next page in the list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                    #[doc = "\nExecute the request and yield each item in the `telemetryUsers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_telemetry_users<T>(
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
                        self.stream_telemetry_users_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryUsers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_telemetry_users_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryUser,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_telemetry_users_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryUsers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_telemetry_users_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleChromeManagementV1TelemetryUser,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_telemetry_users_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `telemetryUsers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_telemetry_users_with_fields<T, F>(
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
                            #[serde(rename = "telemetryUsers")]
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
                                concat!("nextPageToken,", "telemetryUsers").to_owned();
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryUsersResponse,
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
                            crate::schemas::GoogleChromeManagementV1ListTelemetryUsersResponse,
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
                    ) -> Result<
                        crate::schemas::GoogleChromeManagementV1ListTelemetryUsersResponse,
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
                        crate::schemas::GoogleChromeManagementV1ListTelemetryUsersResponse,
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/telemetry/users");
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
