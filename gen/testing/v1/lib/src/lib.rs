#![doc = "# Resources and Methods\n* [application_detail_service](resources/application_detail_service/struct.ApplicationDetailServiceActions.html)\n  * [*getApkDetails*](resources/application_detail_service/struct.GetApkDetailsRequestBuilder.html)\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [test_matrices](resources/projects/test_matrices/struct.TestMatricesActions.html)\n    * [*cancel*](resources/projects/test_matrices/struct.CancelRequestBuilder.html), [*create*](resources/projects/test_matrices/struct.CreateRequestBuilder.html), [*get*](resources/projects/test_matrices/struct.GetRequestBuilder.html)\n* [test_environment_catalog](resources/test_environment_catalog/struct.TestEnvironmentCatalogActions.html)\n  * [*get*](resources/test_environment_catalog/struct.GetRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud services and see the email address of your Google Account\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
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
    pub struct Account {
        #[doc = "An automatic google login account."]
        #[serde(
            rename = "googleAuto",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_auto: ::std::option::Option<crate::schemas::GoogleAuto>,
    }
    impl ::google_field_selector::FieldSelector for Account {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Account {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidDevice {
        #[doc = "Required. The id of the Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "androidModelId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_model_id: ::std::option::Option<String>,
        #[doc = "Required. The id of the Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "androidVersionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_version_id: ::std::option::Option<String>,
        #[doc = "Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "orientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientation: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidDevice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AndroidDeviceCatalog {
        #[doc = "The set of supported Android device models."]
        #[serde(
            rename = "models",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub models: ::std::option::Option<Vec<crate::schemas::AndroidModel>>,
        #[doc = "The set of supported runtime configurations."]
        #[serde(
            rename = "runtimeConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_configuration:
            ::std::option::Option<crate::schemas::AndroidRuntimeConfiguration>,
        #[doc = "The set of supported Android OS versions."]
        #[serde(
            rename = "versions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versions: ::std::option::Option<Vec<crate::schemas::AndroidVersion>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidDeviceCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidDeviceCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidDeviceList {
        #[doc = "Required. A list of Android devices."]
        #[serde(
            rename = "androidDevices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_devices: ::std::option::Option<Vec<crate::schemas::AndroidDevice>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidDeviceList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidDeviceList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidInstrumentationTest {
        #[doc = "The APK for the application under test."]
        #[serde(
            rename = "appApk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_apk: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(
            rename = "appBundle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_bundle: ::std::option::Option<crate::schemas::AppBundle>,
        #[doc = "The java package for the application under test. The default value is determined by examining the application's manifest."]
        #[serde(
            rename = "appPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_package_id: ::std::option::Option<String>,
        #[doc = "The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator."]
        #[serde(
            rename = "orchestratorOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orchestrator_option:
            ::std::option::Option<crate::schemas::AndroidInstrumentationTestOrchestratorOption>,
        #[doc = "The option to run tests in multiple shards in parallel."]
        #[serde(
            rename = "shardingOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sharding_option: ::std::option::Option<crate::schemas::ShardingOption>,
        #[doc = "Required. The APK containing the test code to be executed."]
        #[serde(
            rename = "testApk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_apk: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The java package for the test to be executed. The default value is determined by examining the application's manifest."]
        #[serde(
            rename = "testPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_package_id: ::std::option::Option<String>,
        #[doc = "The InstrumentationTestRunner class. The default value is determined by examining the application's manifest."]
        #[serde(
            rename = "testRunnerClass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_runner_class: ::std::option::Option<String>,
        #[doc = "Each target must be fully qualified with the package name or class name, in one of these formats: - \"package package_name\" - \"class package_name.class_name\" - \"class package_name.class_name#method_name\" If empty, all targets in the module will be run."]
        #[serde(
            rename = "testTargets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_targets: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidInstrumentationTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidInstrumentationTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AndroidInstrumentationTestOrchestratorOption {
        #[doc = "Run test without using orchestrator."]
        DoNotUseOrchestrator,
        #[doc = "Default value: the server will choose the mode. Currently implies that the test will run without the orchestrator. In the future, all instrumentation tests will be run with the orchestrator. Using the orchestrator is highly encouraged because of all the benefits it offers."]
        OrchestratorOptionUnspecified,
        #[doc = "Run test using orchestrator. ** Only compatible with AndroidJUnitRunner version 1.1 or higher! ** Recommended."]
        UseOrchestrator,
    }
    impl AndroidInstrumentationTestOrchestratorOption {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidInstrumentationTestOrchestratorOption::DoNotUseOrchestrator => {
                    "DO_NOT_USE_ORCHESTRATOR"
                }
                AndroidInstrumentationTestOrchestratorOption::OrchestratorOptionUnspecified => {
                    "ORCHESTRATOR_OPTION_UNSPECIFIED"
                }
                AndroidInstrumentationTestOrchestratorOption::UseOrchestrator => "USE_ORCHESTRATOR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AndroidInstrumentationTestOrchestratorOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AndroidInstrumentationTestOrchestratorOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AndroidInstrumentationTestOrchestratorOption, ()> {
            Ok(match s {
                "DO_NOT_USE_ORCHESTRATOR" => {
                    AndroidInstrumentationTestOrchestratorOption::DoNotUseOrchestrator
                }
                "ORCHESTRATOR_OPTION_UNSPECIFIED" => {
                    AndroidInstrumentationTestOrchestratorOption::OrchestratorOptionUnspecified
                }
                "USE_ORCHESTRATOR" => AndroidInstrumentationTestOrchestratorOption::UseOrchestrator,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AndroidInstrumentationTestOrchestratorOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidInstrumentationTestOrchestratorOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidInstrumentationTestOrchestratorOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DO_NOT_USE_ORCHESTRATOR" => {
                    AndroidInstrumentationTestOrchestratorOption::DoNotUseOrchestrator
                }
                "ORCHESTRATOR_OPTION_UNSPECIFIED" => {
                    AndroidInstrumentationTestOrchestratorOption::OrchestratorOptionUnspecified
                }
                "USE_ORCHESTRATOR" => AndroidInstrumentationTestOrchestratorOption::UseOrchestrator,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AndroidInstrumentationTestOrchestratorOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidInstrumentationTestOrchestratorOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidMatrix {
        #[doc = "Required. The ids of the set of Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "androidModelIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_model_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The ids of the set of Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "androidVersionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_version_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The set of locales the test device will enable for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "locales",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locales: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The set of orientations to test with. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "orientations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientations: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidModel {
        #[doc = "The company that this device is branded with. Example: \"Google\", \"Samsung\"."]
        #[serde(
            rename = "brand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brand: ::std::option::Option<String>,
        #[doc = "The name of the industrial design. This corresponds to android.os.Build.DEVICE."]
        #[serde(
            rename = "codename",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub codename: ::std::option::Option<String>,
        #[doc = "Whether this device is virtual or physical."]
        #[serde(
            rename = "form",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form: ::std::option::Option<crate::schemas::AndroidModelForm>,
        #[doc = "Whether this device is a phone, tablet, wearable, etc."]
        #[serde(
            rename = "formFactor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_factor: ::std::option::Option<crate::schemas::AndroidModelFormFactor>,
        #[doc = "The unique opaque id for this model. Use this for invoking the TestExecutionService."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "True if and only if tests with this model are recorded by stitching together screenshots. See use_low_spec_video_recording in device config."]
        #[serde(
            rename = "lowFpsVideoRecording",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_fps_video_recording: ::std::option::Option<bool>,
        #[doc = "The manufacturer of this device."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "The human-readable marketing name for this device model. Examples: \"Nexus 5\", \"Galaxy S5\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Screen density in DPI. This corresponds to ro.sf.lcd_density"]
        #[serde(
            rename = "screenDensity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_density: ::std::option::Option<i32>,
        #[doc = "Screen size in the horizontal (X) dimension measured in pixels."]
        #[serde(
            rename = "screenX",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_x: ::std::option::Option<i32>,
        #[doc = "Screen size in the vertical (Y) dimension measured in pixels."]
        #[serde(
            rename = "screenY",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_y: ::std::option::Option<i32>,
        #[doc = "The list of supported ABIs for this device. This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2. The most preferred ABI is the first element in the list. Elements are optionally prefixed by \"version_id:\" (where version_id is the id of an AndroidVersion), denoting an ABI that is supported only on a particular version."]
        #[serde(
            rename = "supportedAbis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_abis: ::std::option::Option<Vec<String>>,
        #[doc = "The set of Android versions this device supports."]
        #[serde(
            rename = "supportedVersionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_version_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
        #[doc = "URL of a thumbnail image (photo) of the device."]
        #[serde(
            rename = "thumbnailUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidModel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidModel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AndroidModelForm {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormUnspecified,
        #[doc = "Android virtual device using emulator in nested virtualization. Equivalent to Android Studio."]
        Emulator,
        #[doc = "Actual hardware."]
        Physical,
        #[doc = "Android virtual device using Compute Engine native virtualization. Firebase Test Lab only."]
        Virtual,
    }
    impl AndroidModelForm {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidModelForm::DeviceFormUnspecified => "DEVICE_FORM_UNSPECIFIED",
                AndroidModelForm::Emulator => "EMULATOR",
                AndroidModelForm::Physical => "PHYSICAL",
                AndroidModelForm::Virtual => "VIRTUAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AndroidModelForm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AndroidModelForm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AndroidModelForm, ()> {
            Ok(match s {
                "DEVICE_FORM_UNSPECIFIED" => AndroidModelForm::DeviceFormUnspecified,
                "EMULATOR" => AndroidModelForm::Emulator,
                "PHYSICAL" => AndroidModelForm::Physical,
                "VIRTUAL" => AndroidModelForm::Virtual,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AndroidModelForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidModelForm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidModelForm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_UNSPECIFIED" => AndroidModelForm::DeviceFormUnspecified,
                "EMULATOR" => AndroidModelForm::Emulator,
                "PHYSICAL" => AndroidModelForm::Physical,
                "VIRTUAL" => AndroidModelForm::Virtual,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AndroidModelForm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidModelForm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AndroidModelFormFactor {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormFactorUnspecified,
        #[doc = "This device has the shape of a phone."]
        Phone,
        #[doc = "This device has the shape of a tablet."]
        Tablet,
        #[doc = "This device has the shape of a watch or other wearable."]
        Wearable,
    }
    impl AndroidModelFormFactor {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidModelFormFactor::DeviceFormFactorUnspecified => {
                    "DEVICE_FORM_FACTOR_UNSPECIFIED"
                }
                AndroidModelFormFactor::Phone => "PHONE",
                AndroidModelFormFactor::Tablet => "TABLET",
                AndroidModelFormFactor::Wearable => "WEARABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AndroidModelFormFactor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AndroidModelFormFactor {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AndroidModelFormFactor, ()> {
            Ok(match s {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => {
                    AndroidModelFormFactor::DeviceFormFactorUnspecified
                }
                "PHONE" => AndroidModelFormFactor::Phone,
                "TABLET" => AndroidModelFormFactor::Tablet,
                "WEARABLE" => AndroidModelFormFactor::Wearable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AndroidModelFormFactor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidModelFormFactor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidModelFormFactor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => {
                    AndroidModelFormFactor::DeviceFormFactorUnspecified
                }
                "PHONE" => AndroidModelFormFactor::Phone,
                "TABLET" => AndroidModelFormFactor::Tablet,
                "WEARABLE" => AndroidModelFormFactor::Wearable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AndroidModelFormFactor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidModelFormFactor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidRoboTest {
        #[doc = "The APK for the application under test."]
        #[serde(
            rename = "appApk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_apk: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(
            rename = "appBundle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_bundle: ::std::option::Option<crate::schemas::AppBundle>,
        #[doc = "The initial activity that should be used to start the app."]
        #[serde(
            rename = "appInitialActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_initial_activity: ::std::option::Option<String>,
        #[doc = "The java package for the application under test. The default value is determined by examining the application's manifest."]
        #[serde(
            rename = "appPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_package_id: ::std::option::Option<String>,
        #[doc = "The max depth of the traversal stack Robo can explore. Needs to be at least 2 to make Robo explore the app beyond the first activity. Default is 50."]
        #[serde(
            rename = "maxDepth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_depth: ::std::option::Option<i32>,
        #[doc = "The max number of steps Robo can execute. Default is no limit."]
        #[serde(
            rename = "maxSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_steps: ::std::option::Option<i32>,
        #[doc = "A set of directives Robo should apply during the crawl. This allows users to customize the crawl. For example, the username and password for a test account can be provided."]
        #[serde(
            rename = "roboDirectives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub robo_directives: ::std::option::Option<Vec<crate::schemas::RoboDirective>>,
        #[doc = "The mode in which Robo should run. Most clients should allow the server to populate this field automatically."]
        #[serde(
            rename = "roboMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub robo_mode: ::std::option::Option<crate::schemas::AndroidRoboTestRoboMode>,
        #[doc = "A JSON file with a sequence of actions Robo should perform as a prologue for the crawl."]
        #[serde(
            rename = "roboScript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub robo_script: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The intents used to launch the app for the crawl. If none are provided, then the main launcher activity is launched. If some are provided, then only those provided are launched (the main launcher activity must be provided explicitly)."]
        #[serde(
            rename = "startingIntents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub starting_intents: ::std::option::Option<Vec<crate::schemas::RoboStartingIntent>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidRoboTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidRoboTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AndroidRoboTestRoboMode {
        #[doc = "This means that the server should choose the mode. Recommended."]
        RoboModeUnspecified,
        #[doc = "Runs Robo in UIAutomator-only mode without app resigning"]
        RoboVersion1,
        #[doc = "Runs Robo in standard Espresso with UIAutomator fallback"]
        RoboVersion2,
    }
    impl AndroidRoboTestRoboMode {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidRoboTestRoboMode::RoboModeUnspecified => "ROBO_MODE_UNSPECIFIED",
                AndroidRoboTestRoboMode::RoboVersion1 => "ROBO_VERSION_1",
                AndroidRoboTestRoboMode::RoboVersion2 => "ROBO_VERSION_2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AndroidRoboTestRoboMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AndroidRoboTestRoboMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AndroidRoboTestRoboMode, ()> {
            Ok(match s {
                "ROBO_MODE_UNSPECIFIED" => AndroidRoboTestRoboMode::RoboModeUnspecified,
                "ROBO_VERSION_1" => AndroidRoboTestRoboMode::RoboVersion1,
                "ROBO_VERSION_2" => AndroidRoboTestRoboMode::RoboVersion2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AndroidRoboTestRoboMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidRoboTestRoboMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidRoboTestRoboMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ROBO_MODE_UNSPECIFIED" => AndroidRoboTestRoboMode::RoboModeUnspecified,
                "ROBO_VERSION_1" => AndroidRoboTestRoboMode::RoboVersion1,
                "ROBO_VERSION_2" => AndroidRoboTestRoboMode::RoboVersion2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AndroidRoboTestRoboMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidRoboTestRoboMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidRuntimeConfiguration {
        #[doc = "The set of available locales."]
        #[serde(
            rename = "locales",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locales: ::std::option::Option<Vec<crate::schemas::Locale>>,
        #[doc = "The set of available orientations."]
        #[serde(
            rename = "orientations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientations: ::std::option::Option<Vec<crate::schemas::Orientation>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidRuntimeConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidRuntimeConfiguration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidTestLoop {
        #[doc = "The APK for the application under test."]
        #[serde(
            rename = "appApk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_apk: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(
            rename = "appBundle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_bundle: ::std::option::Option<crate::schemas::AppBundle>,
        #[doc = "The java package for the application under test. The default is determined by examining the application's manifest."]
        #[serde(
            rename = "appPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_package_id: ::std::option::Option<String>,
        #[doc = "The list of scenario labels that should be run during the test. The scenario labels should map to labels defined in the application's manifest. For example, player_experience and com.google.test.loops.player_experience add all of the loops labeled in the manifest with the com.google.test.loops.player_experience name to the execution. Scenarios can also be specified in the scenarios field."]
        #[serde(
            rename = "scenarioLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scenario_labels: ::std::option::Option<Vec<String>>,
        #[doc = "The list of scenarios that should be run during the test. The default is all test loops, derived from the application's manifest."]
        #[serde(
            rename = "scenarios",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scenarios: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for AndroidTestLoop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidTestLoop {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AndroidVersion {
        #[doc = "The API level for this Android version. Examples: 18, 19."]
        #[serde(
            rename = "apiLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_level: ::std::option::Option<i32>,
        #[doc = "The code name for this Android version. Examples: \"JellyBean\", \"KitKat\"."]
        #[serde(
            rename = "codeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code_name: ::std::option::Option<String>,
        #[doc = "Market share for this version."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<crate::schemas::Distribution>,
        #[doc = "An opaque id for this Android version. Use this id to invoke the TestExecutionService."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The date this Android version became available in the market."]
        #[serde(
            rename = "releaseDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub release_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
        #[doc = "A string representing this version of the Android OS. Examples: \"4.3\", \"4.4\"."]
        #[serde(
            rename = "versionString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_string: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Apk {
        #[doc = "The path to an APK to be installed on the device before the test begins."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The java package for the APK to be installed. Value is determined by examining the application's manifest."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Apk {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Apk {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ApkDetail {
        #[serde(
            rename = "apkManifest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apk_manifest: ::std::option::Option<crate::schemas::ApkManifest>,
    }
    impl ::google_field_selector::FieldSelector for ApkDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApkDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ApkManifest {
        #[doc = "User-readable name for the application."]
        #[serde(
            rename = "applicationLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_label: ::std::option::Option<String>,
        #[serde(
            rename = "intentFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intent_filters: ::std::option::Option<Vec<crate::schemas::IntentFilter>>,
        #[doc = "Maximum API level on which the application is designed to run."]
        #[serde(
            rename = "maxSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_sdk_version: ::std::option::Option<i32>,
        #[doc = "Minimum API level required for the application to run."]
        #[serde(
            rename = "minSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_sdk_version: ::std::option::Option<i32>,
        #[doc = "Full Java-style package name for this application, e.g. \"com.example.foo\"."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Specifies the API Level on which the application is designed to run."]
        #[serde(
            rename = "targetSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_sdk_version: ::std::option::Option<i32>,
        #[doc = "Permissions declared to be used by the application"]
        #[serde(
            rename = "usesPermission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uses_permission: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ApkManifest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApkManifest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppBundle {
        #[doc = ".aab file representing the app bundle under test."]
        #[serde(
            rename = "bundleLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_location: ::std::option::Option<crate::schemas::FileReference>,
    }
    impl ::google_field_selector::FieldSelector for AppBundle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppBundle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CancelTestMatrixResponse {
        #[doc = "The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect."]
        #[serde(
            rename = "testState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_state: ::std::option::Option<crate::schemas::CancelTestMatrixResponseTestState>,
    }
    impl ::google_field_selector::FieldSelector for CancelTestMatrixResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CancelTestMatrixResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CancelTestMatrixResponseTestState {
        #[doc = "The user cancelled the execution. Can only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
        Error,
        #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
        Invalid,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed. Can only be set on an execution."]
        Running,
        #[doc = "Do not use. For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution or matrix is being validated."]
        Validating,
    }
    impl CancelTestMatrixResponseTestState {
        pub fn as_str(self) -> &'static str {
            match self {
                CancelTestMatrixResponseTestState::Cancelled => "CANCELLED",
                CancelTestMatrixResponseTestState::Error => "ERROR",
                CancelTestMatrixResponseTestState::Finished => "FINISHED",
                CancelTestMatrixResponseTestState::IncompatibleArchitecture => {
                    "INCOMPATIBLE_ARCHITECTURE"
                }
                CancelTestMatrixResponseTestState::IncompatibleEnvironment => {
                    "INCOMPATIBLE_ENVIRONMENT"
                }
                CancelTestMatrixResponseTestState::Invalid => "INVALID",
                CancelTestMatrixResponseTestState::Pending => "PENDING",
                CancelTestMatrixResponseTestState::Running => "RUNNING",
                CancelTestMatrixResponseTestState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                CancelTestMatrixResponseTestState::UnsupportedEnvironment => {
                    "UNSUPPORTED_ENVIRONMENT"
                }
                CancelTestMatrixResponseTestState::Validating => "VALIDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CancelTestMatrixResponseTestState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CancelTestMatrixResponseTestState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CancelTestMatrixResponseTestState, ()> {
            Ok(match s {
                "CANCELLED" => CancelTestMatrixResponseTestState::Cancelled,
                "ERROR" => CancelTestMatrixResponseTestState::Error,
                "FINISHED" => CancelTestMatrixResponseTestState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => {
                    CancelTestMatrixResponseTestState::IncompatibleArchitecture
                }
                "INCOMPATIBLE_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::IncompatibleEnvironment
                }
                "INVALID" => CancelTestMatrixResponseTestState::Invalid,
                "PENDING" => CancelTestMatrixResponseTestState::Pending,
                "RUNNING" => CancelTestMatrixResponseTestState::Running,
                "TEST_STATE_UNSPECIFIED" => CancelTestMatrixResponseTestState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::UnsupportedEnvironment
                }
                "VALIDATING" => CancelTestMatrixResponseTestState::Validating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CancelTestMatrixResponseTestState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CancelTestMatrixResponseTestState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CancelTestMatrixResponseTestState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => CancelTestMatrixResponseTestState::Cancelled,
                "ERROR" => CancelTestMatrixResponseTestState::Error,
                "FINISHED" => CancelTestMatrixResponseTestState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => {
                    CancelTestMatrixResponseTestState::IncompatibleArchitecture
                }
                "INCOMPATIBLE_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::IncompatibleEnvironment
                }
                "INVALID" => CancelTestMatrixResponseTestState::Invalid,
                "PENDING" => CancelTestMatrixResponseTestState::Pending,
                "RUNNING" => CancelTestMatrixResponseTestState::Running,
                "TEST_STATE_UNSPECIFIED" => CancelTestMatrixResponseTestState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::UnsupportedEnvironment
                }
                "VALIDATING" => CancelTestMatrixResponseTestState::Validating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CancelTestMatrixResponseTestState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CancelTestMatrixResponseTestState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ClientInfo {
        #[doc = "The list of detailed information about client."]
        #[serde(
            rename = "clientInfoDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_info_details: ::std::option::Option<Vec<crate::schemas::ClientInfoDetail>>,
        #[doc = "Required. Client name, such as gcloud."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClientInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ClientInfoDetail {
        #[doc = "Required. The key of detailed client information."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Required. The value of detailed client information."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClientInfoDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientInfoDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceFile {
        #[doc = "A reference to an opaque binary blob file."]
        #[serde(
            rename = "obbFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub obb_file: ::std::option::Option<crate::schemas::ObbFile>,
        #[doc = "A reference to a regular file."]
        #[serde(
            rename = "regularFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regular_file: ::std::option::Option<crate::schemas::RegularFile>,
    }
    impl ::google_field_selector::FieldSelector for DeviceFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceIpBlock {
        #[doc = "The date this block was added to Firebase Test Lab"]
        #[serde(
            rename = "addedDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "An IP address block in CIDR notation eg: 34.68.194.64/29"]
        #[serde(
            rename = "block",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block: ::std::option::Option<String>,
        #[doc = "Whether this block is used by physical or virtual devices"]
        #[serde(
            rename = "form",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form: ::std::option::Option<crate::schemas::DeviceIpBlockForm>,
    }
    impl ::google_field_selector::FieldSelector for DeviceIpBlock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceIpBlock {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceIpBlockForm {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormUnspecified,
        #[doc = "Android virtual device using emulator in nested virtualization. Equivalent to Android Studio."]
        Emulator,
        #[doc = "Actual hardware."]
        Physical,
        #[doc = "Android virtual device using Compute Engine native virtualization. Firebase Test Lab only."]
        Virtual,
    }
    impl DeviceIpBlockForm {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceIpBlockForm::DeviceFormUnspecified => "DEVICE_FORM_UNSPECIFIED",
                DeviceIpBlockForm::Emulator => "EMULATOR",
                DeviceIpBlockForm::Physical => "PHYSICAL",
                DeviceIpBlockForm::Virtual => "VIRTUAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceIpBlockForm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceIpBlockForm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceIpBlockForm, ()> {
            Ok(match s {
                "DEVICE_FORM_UNSPECIFIED" => DeviceIpBlockForm::DeviceFormUnspecified,
                "EMULATOR" => DeviceIpBlockForm::Emulator,
                "PHYSICAL" => DeviceIpBlockForm::Physical,
                "VIRTUAL" => DeviceIpBlockForm::Virtual,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceIpBlockForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceIpBlockForm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceIpBlockForm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_UNSPECIFIED" => DeviceIpBlockForm::DeviceFormUnspecified,
                "EMULATOR" => DeviceIpBlockForm::Emulator,
                "PHYSICAL" => DeviceIpBlockForm::Physical,
                "VIRTUAL" => DeviceIpBlockForm::Virtual,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceIpBlockForm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceIpBlockForm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceIpBlockCatalog {
        #[doc = "The device IP blocks used by Firebase Test Lab"]
        #[serde(
            rename = "ipBlocks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_blocks: ::std::option::Option<Vec<crate::schemas::DeviceIpBlock>>,
    }
    impl ::google_field_selector::FieldSelector for DeviceIpBlockCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceIpBlockCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Distribution {
        #[doc = "Output only. The estimated fraction (0-1) of the total market with this configuration."]
        #[serde(
            rename = "marketShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub market_share: ::std::option::Option<f64>,
        #[doc = "Output only. The time this distribution was measured."]
        #[serde(
            rename = "measurementTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub measurement_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Distribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Distribution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Environment {
        #[doc = "An Android device which must be used with an Android test."]
        #[serde(
            rename = "androidDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device: ::std::option::Option<crate::schemas::AndroidDevice>,
        #[doc = "An iOS device which must be used with an iOS test."]
        #[serde(
            rename = "iosDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_device: ::std::option::Option<crate::schemas::IosDevice>,
    }
    impl ::google_field_selector::FieldSelector for Environment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Environment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EnvironmentMatrix {
        #[doc = "A list of Android devices; the test will be run only on the specified devices."]
        #[serde(
            rename = "androidDeviceList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device_list: ::std::option::Option<crate::schemas::AndroidDeviceList>,
        #[doc = "A matrix of Android devices."]
        #[serde(
            rename = "androidMatrix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_matrix: ::std::option::Option<crate::schemas::AndroidMatrix>,
        #[doc = "A list of iOS devices."]
        #[serde(
            rename = "iosDeviceList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_device_list: ::std::option::Option<crate::schemas::IosDeviceList>,
    }
    impl ::google_field_selector::FieldSelector for EnvironmentMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvironmentMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EnvironmentVariable {
        #[doc = "Key for the environment variable."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Value for the environment variable."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnvironmentVariable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvironmentVariable {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FileReference {
        #[doc = "A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding)"]
        #[serde(
            rename = "gcsPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FileReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetApkDetailsResponse {
        #[doc = "Details of the Android APK."]
        #[serde(
            rename = "apkDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apk_detail: ::std::option::Option<crate::schemas::ApkDetail>,
    }
    impl ::google_field_selector::FieldSelector for GetApkDetailsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetApkDetailsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct GoogleAuto {}
    impl ::google_field_selector::FieldSelector for GoogleAuto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAuto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudStorage {
        #[doc = "Required. The path to a directory in GCS that will eventually contain the results for this test. The requesting user must have write access on the bucket in the supplied path."]
        #[serde(
            rename = "gcsPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudStorage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudStorage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IntentFilter {
        #[doc = "The android:name value of the tag."]
        #[serde(
            rename = "actionNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_names: ::std::option::Option<Vec<String>>,
        #[doc = "The android:name value of the tag."]
        #[serde(
            rename = "categoryNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_names: ::std::option::Option<Vec<String>>,
        #[doc = "The android:mimeType value of the tag."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IntentFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntentFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosDevice {
        #[doc = "Required. The id of the iOS device to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "iosModelId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_model_id: ::std::option::Option<String>,
        #[doc = "Required. The id of the iOS major software version to be used. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "iosVersionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_version_id: ::std::option::Option<String>,
        #[doc = "Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(
            rename = "orientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientation: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IosDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosDevice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosDeviceCatalog {
        #[doc = "The set of supported iOS device models."]
        #[serde(
            rename = "models",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub models: ::std::option::Option<Vec<crate::schemas::IosModel>>,
        #[doc = "The set of supported runtime configurations."]
        #[serde(
            rename = "runtimeConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_configuration: ::std::option::Option<crate::schemas::IosRuntimeConfiguration>,
        #[doc = "The set of supported iOS software versions."]
        #[serde(
            rename = "versions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versions: ::std::option::Option<Vec<crate::schemas::IosVersion>>,
        #[doc = "The set of supported Xcode versions."]
        #[serde(
            rename = "xcodeVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xcode_versions: ::std::option::Option<Vec<crate::schemas::XcodeVersion>>,
    }
    impl ::google_field_selector::FieldSelector for IosDeviceCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosDeviceCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosDeviceFile {
        #[doc = "The bundle id of the app where this file lives. iOS apps sandbox their own filesystem, so app files must specify which app installed on the device."]
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<String>,
        #[doc = "The source file"]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "Location of the file on the device, inside the app's sandboxed filesystem"]
        #[serde(
            rename = "devicePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IosDeviceFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosDeviceFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosDeviceList {
        #[doc = "Required. A list of iOS devices."]
        #[serde(
            rename = "iosDevices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_devices: ::std::option::Option<Vec<crate::schemas::IosDevice>>,
    }
    impl ::google_field_selector::FieldSelector for IosDeviceList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosDeviceList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosModel {
        #[doc = "Device capabilities. Copied from https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html"]
        #[serde(
            rename = "deviceCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_capabilities: ::std::option::Option<Vec<String>>,
        #[doc = "Whether this device is a phone, tablet, wearable, etc."]
        #[serde(
            rename = "formFactor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_factor: ::std::option::Option<crate::schemas::IosModelFormFactor>,
        #[doc = "The unique opaque id for this model. Use this for invoking the TestExecutionService."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The human-readable name for this device model. Examples: \"iPhone 4s\", \"iPad Mini 2\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Screen density in DPI."]
        #[serde(
            rename = "screenDensity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_density: ::std::option::Option<i32>,
        #[doc = "Screen size in the horizontal (X) dimension measured in pixels."]
        #[serde(
            rename = "screenX",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_x: ::std::option::Option<i32>,
        #[doc = "Screen size in the vertical (Y) dimension measured in pixels."]
        #[serde(
            rename = "screenY",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_y: ::std::option::Option<i32>,
        #[doc = "The set of iOS major software versions this device supports."]
        #[serde(
            rename = "supportedVersionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_version_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for IosModel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosModel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IosModelFormFactor {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormFactorUnspecified,
        #[doc = "This device has the shape of a phone."]
        Phone,
        #[doc = "This device has the shape of a tablet."]
        Tablet,
        #[doc = "This device has the shape of a watch or other wearable."]
        Wearable,
    }
    impl IosModelFormFactor {
        pub fn as_str(self) -> &'static str {
            match self {
                IosModelFormFactor::DeviceFormFactorUnspecified => "DEVICE_FORM_FACTOR_UNSPECIFIED",
                IosModelFormFactor::Phone => "PHONE",
                IosModelFormFactor::Tablet => "TABLET",
                IosModelFormFactor::Wearable => "WEARABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IosModelFormFactor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IosModelFormFactor {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IosModelFormFactor, ()> {
            Ok(match s {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => IosModelFormFactor::DeviceFormFactorUnspecified,
                "PHONE" => IosModelFormFactor::Phone,
                "TABLET" => IosModelFormFactor::Tablet,
                "WEARABLE" => IosModelFormFactor::Wearable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IosModelFormFactor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IosModelFormFactor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IosModelFormFactor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => IosModelFormFactor::DeviceFormFactorUnspecified,
                "PHONE" => IosModelFormFactor::Phone,
                "TABLET" => IosModelFormFactor::Tablet,
                "WEARABLE" => IosModelFormFactor::Wearable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IosModelFormFactor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosModelFormFactor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosRuntimeConfiguration {
        #[doc = "The set of available locales."]
        #[serde(
            rename = "locales",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locales: ::std::option::Option<Vec<crate::schemas::Locale>>,
        #[doc = "The set of available orientations."]
        #[serde(
            rename = "orientations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientations: ::std::option::Option<Vec<crate::schemas::Orientation>>,
    }
    impl ::google_field_selector::FieldSelector for IosRuntimeConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosRuntimeConfiguration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosTestLoop {
        #[doc = "Output only. The bundle id for the application under test."]
        #[serde(
            rename = "appBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_bundle_id: ::std::option::Option<String>,
        #[doc = "Required. The .ipa of the application to test."]
        #[serde(
            rename = "appIpa",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_ipa: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The list of scenarios that should be run during the test. Defaults to the single scenario 0 if unspecified."]
        #[serde(
            rename = "scenarios",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scenarios: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for IosTestLoop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosTestLoop {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosTestSetup {
        #[doc = "iOS apps to install in addition to those being directly tested."]
        #[serde(
            rename = "additionalIpas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_ipas: ::std::option::Option<Vec<crate::schemas::FileReference>>,
        #[doc = "The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
        #[serde(
            rename = "networkProfile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_profile: ::std::option::Option<String>,
        #[doc = "List of directories on the device to upload to Cloud Storage at the end of the test. Directories should either be in a shared directory (such as /private/var/mobile/Media) or within an accessible directory inside the app's filesystem (such as /Documents) by specifying the bundle ID."]
        #[serde(
            rename = "pullDirectories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pull_directories: ::std::option::Option<Vec<crate::schemas::IosDeviceFile>>,
        #[doc = "List of files to push to the device before starting the test."]
        #[serde(
            rename = "pushFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub push_files: ::std::option::Option<Vec<crate::schemas::IosDeviceFile>>,
    }
    impl ::google_field_selector::FieldSelector for IosTestSetup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosTestSetup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosVersion {
        #[doc = "An opaque id for this iOS version. Use this id to invoke the TestExecutionService."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "An integer representing the major iOS version. Examples: \"8\", \"9\"."]
        #[serde(
            rename = "majorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major_version: ::std::option::Option<i32>,
        #[doc = "An integer representing the minor iOS version. Examples: \"1\", \"2\"."]
        #[serde(
            rename = "minorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minor_version: ::std::option::Option<i32>,
        #[doc = "The available Xcode versions for this version."]
        #[serde(
            rename = "supportedXcodeVersionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_xcode_version_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Tags for this dimension. Examples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for IosVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosXcTest {
        #[doc = "Output only. The bundle id for the application under test."]
        #[serde(
            rename = "appBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_bundle_id: ::std::option::Option<String>,
        #[doc = "The option to test special app entitlements. Setting this would re-sign the app having special entitlements with an explicit application-identifier. Currently supports testing aps-environment entitlement."]
        #[serde(
            rename = "testSpecialEntitlements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_special_entitlements: ::std::option::Option<bool>,
        #[doc = "Required. The .zip containing the .xctestrun file and the contents of the DerivedData/Build/Products directory. The .xctestrun file in this zip is ignored if the xctestrun field is specified."]
        #[serde(
            rename = "testsZip",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tests_zip: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The Xcode version that should be used for the test. Use the TestEnvironmentDiscoveryService to get supported options. Defaults to the latest Xcode version Firebase Test Lab supports."]
        #[serde(
            rename = "xcodeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xcode_version: ::std::option::Option<String>,
        #[doc = "An .xctestrun file that will override the .xctestrun file in the tests zip. Because the .xctestrun file contains environment variables along with test methods to run and/or ignore, this can be useful for sharding tests. Default is taken from the tests zip."]
        #[serde(
            rename = "xctestrun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xctestrun: ::std::option::Option<crate::schemas::FileReference>,
    }
    impl ::google_field_selector::FieldSelector for IosXcTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosXcTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct LauncherActivityIntent {}
    impl ::google_field_selector::FieldSelector for LauncherActivityIntent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LauncherActivityIntent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Locale {
        #[doc = "The id for this locale. Example: \"en_US\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A human-friendly name for this language/locale. Example: \"English\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A human-friendly string representing the region for this locale. Example: \"United States\". Not present for every locale."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "Tags for this dimension. Example: \"default\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Locale {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Locale {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ManualSharding {
        #[doc = "Required. Group of packages, classes, and/or test methods to be run for each shard. When any physical devices are selected, the number of test_targets_for_shard must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500."]
        #[serde(
            rename = "testTargetsForShard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_targets_for_shard: ::std::option::Option<Vec<crate::schemas::TestTargetsForShard>>,
    }
    impl ::google_field_selector::FieldSelector for ManualSharding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManualSharding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NetworkConfiguration {
        #[doc = "The emulation rule applying to the download traffic."]
        #[serde(
            rename = "downRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub down_rule: ::std::option::Option<crate::schemas::TrafficRule>,
        #[doc = "The unique opaque id for this network traffic configuration."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The emulation rule applying to the upload traffic."]
        #[serde(
            rename = "upRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub up_rule: ::std::option::Option<crate::schemas::TrafficRule>,
    }
    impl ::google_field_selector::FieldSelector for NetworkConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkConfiguration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NetworkConfigurationCatalog {
        #[serde(
            rename = "configurations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configurations: ::std::option::Option<Vec<crate::schemas::NetworkConfiguration>>,
    }
    impl ::google_field_selector::FieldSelector for NetworkConfigurationCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkConfigurationCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ObbFile {
        #[doc = "Required. Opaque Binary Blob (OBB) file(s) to install on the device."]
        #[serde(
            rename = "obb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub obb: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "Required. OBB file name which must conform to the format as specified by Android e.g. [main|patch].0300110.com.example.android.obb which will be installed into /Android/obb// on the device."]
        #[serde(
            rename = "obbFileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub obb_file_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ObbFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObbFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Orientation {
        #[doc = "The id for this orientation. Example: \"portrait\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A human-friendly name for this orientation. Example: \"portrait\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Tags for this dimension. Example: \"default\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Orientation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Orientation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ProvidedSoftwareCatalog {
        #[doc = "A string representing the current version of AndroidX Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#androidx.test:orchestrator."]
        #[serde(
            rename = "androidxOrchestratorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub androidx_orchestrator_version: ::std::option::Option<String>,
        #[doc = "Deprecated: Use AndroidX Test Orchestrator going forward. A string representing the current version of Android Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#com.android.support.test:orchestrator."]
        #[serde(
            rename = "orchestratorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orchestrator_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProvidedSoftwareCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProvidedSoftwareCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RegularFile {
        #[doc = "Required. The source file."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "Required. Where to put the content on the device. Must be an absolute, allowlisted path. If the file exists, it will be replaced. The following device-side directories and any of their subdirectories are allowlisted: ${EXTERNAL_STORAGE}, /sdcard, or /storage ${ANDROID_DATA}/local/tmp, or /data/local/tmp Specifying a path outside of these directory trees is invalid. The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device and copy the file there. It is strongly advised to use the Environment API in app and test code to access files on the device in a portable way."]
        #[serde(
            rename = "devicePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RegularFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegularFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultStorage {
        #[doc = "Required."]
        #[serde(
            rename = "googleCloudStorage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_cloud_storage: ::std::option::Option<crate::schemas::GoogleCloudStorage>,
        #[doc = "Output only. URL to the results in the Firebase Web Console."]
        #[serde(
            rename = "resultsUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results_url: ::std::option::Option<String>,
        #[doc = "Output only. The tool results execution that results are written to."]
        #[serde(
            rename = "toolResultsExecution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_results_execution: ::std::option::Option<crate::schemas::ToolResultsExecution>,
        #[doc = "The tool results history that contains the tool results execution that results are written to. If not provided, the service will choose an appropriate value."]
        #[serde(
            rename = "toolResultsHistory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_results_history: ::std::option::Option<crate::schemas::ToolResultsHistory>,
    }
    impl ::google_field_selector::FieldSelector for ResultStorage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultStorage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RoboDirective {
        #[doc = "Required. The type of action that Robo should perform on the specified element."]
        #[serde(
            rename = "actionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_type: ::std::option::Option<crate::schemas::RoboDirectiveActionType>,
        #[doc = "The text that Robo is directed to set. If left empty, the directive will be treated as a CLICK on the element matching the resource_name."]
        #[serde(
            rename = "inputText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_text: ::std::option::Option<String>,
        #[doc = "Required. The android resource name of the target UI element. For example, in Java: R.string.foo in xml: @string/foo Only the \"foo\" part is needed. Reference doc: https://developer.android.com/guide/topics/resources/accessing-resources.html"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RoboDirective {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoboDirective {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoboDirectiveActionType {
        #[doc = "DO NOT USE. For proto versioning only."]
        ActionTypeUnspecified,
        #[doc = "Direct Robo to enter text on the specified element. No-op if specified element is not enabled or does not allow text entry."]
        EnterText,
        #[doc = "Direct Robo to ignore interactions with a specific element."]
        Ignore,
        #[doc = "Direct Robo to click on the specified element. No-op if specified element is not clickable."]
        SingleClick,
    }
    impl RoboDirectiveActionType {
        pub fn as_str(self) -> &'static str {
            match self {
                RoboDirectiveActionType::ActionTypeUnspecified => "ACTION_TYPE_UNSPECIFIED",
                RoboDirectiveActionType::EnterText => "ENTER_TEXT",
                RoboDirectiveActionType::Ignore => "IGNORE",
                RoboDirectiveActionType::SingleClick => "SINGLE_CLICK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RoboDirectiveActionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RoboDirectiveActionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RoboDirectiveActionType, ()> {
            Ok(match s {
                "ACTION_TYPE_UNSPECIFIED" => RoboDirectiveActionType::ActionTypeUnspecified,
                "ENTER_TEXT" => RoboDirectiveActionType::EnterText,
                "IGNORE" => RoboDirectiveActionType::Ignore,
                "SINGLE_CLICK" => RoboDirectiveActionType::SingleClick,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RoboDirectiveActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoboDirectiveActionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoboDirectiveActionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TYPE_UNSPECIFIED" => RoboDirectiveActionType::ActionTypeUnspecified,
                "ENTER_TEXT" => RoboDirectiveActionType::EnterText,
                "IGNORE" => RoboDirectiveActionType::Ignore,
                "SINGLE_CLICK" => RoboDirectiveActionType::SingleClick,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RoboDirectiveActionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoboDirectiveActionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RoboStartingIntent {
        #[doc = "An intent that starts the main launcher activity."]
        #[serde(
            rename = "launcherActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launcher_activity: ::std::option::Option<crate::schemas::LauncherActivityIntent>,
        #[doc = "An intent that starts an activity with specific details."]
        #[serde(
            rename = "startActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_activity: ::std::option::Option<crate::schemas::StartActivityIntent>,
        #[doc = "Timeout in seconds for each intent."]
        #[serde(
            rename = "timeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeout: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RoboStartingIntent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoboStartingIntent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Shard {
        #[doc = "Output only. The total number of shards."]
        #[serde(
            rename = "numShards",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_shards: ::std::option::Option<i32>,
        #[doc = "Output only. The index of the shard among all the shards."]
        #[serde(
            rename = "shardIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_index: ::std::option::Option<i32>,
        #[doc = "Output only. Test targets for each shard. Only set for manual sharding."]
        #[serde(
            rename = "testTargetsForShard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_targets_for_shard: ::std::option::Option<crate::schemas::TestTargetsForShard>,
    }
    impl ::google_field_selector::FieldSelector for Shard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Shard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ShardingOption {
        #[doc = "Shards test cases into the specified groups of packages, classes, and/or methods."]
        #[serde(
            rename = "manualSharding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_sharding: ::std::option::Option<crate::schemas::ManualSharding>,
        #[doc = "Uniformly shards test cases given a total number of shards."]
        #[serde(
            rename = "uniformSharding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uniform_sharding: ::std::option::Option<crate::schemas::UniformSharding>,
    }
    impl ::google_field_selector::FieldSelector for ShardingOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShardingOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartActivityIntent {
        #[doc = "Action name. Required for START_ACTIVITY."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[doc = "Intent categories to set on the intent."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "URI for the action."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StartActivityIntent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartActivityIntent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SystraceSetup {
        #[doc = "Systrace duration in seconds. Should be between 1 and 30 seconds. 0 disables systrace."]
        #[serde(
            rename = "durationSeconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration_seconds: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SystraceSetup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystraceSetup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestDetails {
        #[doc = "Output only. If the TestState is ERROR, then this string will contain human-readable details about the error."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Output only. Human-readable, detailed descriptions of the test's progress. For example: \"Provisioning a device\", \"Starting Test\". During the course of execution new data may be appended to the end of progress_messages."]
        #[serde(
            rename = "progressMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_messages: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TestEnvironmentCatalog {
        #[doc = "Supported Android devices."]
        #[serde(
            rename = "androidDeviceCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_device_catalog: ::std::option::Option<crate::schemas::AndroidDeviceCatalog>,
        #[doc = "The IP blocks used by devices in the test environment."]
        #[serde(
            rename = "deviceIpBlockCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_ip_block_catalog: ::std::option::Option<crate::schemas::DeviceIpBlockCatalog>,
        #[doc = "Supported iOS devices."]
        #[serde(
            rename = "iosDeviceCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_device_catalog: ::std::option::Option<crate::schemas::IosDeviceCatalog>,
        #[doc = "Supported network configurations."]
        #[serde(
            rename = "networkConfigurationCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_configuration_catalog:
            ::std::option::Option<crate::schemas::NetworkConfigurationCatalog>,
        #[doc = "The software test environment provided by TestExecutionService."]
        #[serde(
            rename = "softwareCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_catalog: ::std::option::Option<crate::schemas::ProvidedSoftwareCatalog>,
    }
    impl ::google_field_selector::FieldSelector for TestEnvironmentCatalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestEnvironmentCatalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestExecution {
        #[doc = "Output only. How the host machine(s) are configured."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<crate::schemas::Environment>,
        #[doc = "Output only. Unique id set by the service."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Output only. Id of the containing TestMatrix."]
        #[serde(
            rename = "matrixId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matrix_id: ::std::option::Option<String>,
        #[doc = "Output only. The cloud project that owns the test execution."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Output only. Details about the shard."]
        #[serde(
            rename = "shard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard: ::std::option::Option<crate::schemas::Shard>,
        #[doc = "Output only. Indicates the current progress of the test execution (e.g., FINISHED)."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::TestExecutionState>,
        #[doc = "Output only. Additional details about the running test."]
        #[serde(
            rename = "testDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_details: ::std::option::Option<crate::schemas::TestDetails>,
        #[doc = "Output only. How to run the test."]
        #[serde(
            rename = "testSpecification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_specification: ::std::option::Option<crate::schemas::TestSpecification>,
        #[doc = "Output only. The time this test execution was initially created."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Output only. Where the results for this execution are written."]
        #[serde(
            rename = "toolResultsStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_results_step: ::std::option::Option<crate::schemas::ToolResultsStep>,
    }
    impl ::google_field_selector::FieldSelector for TestExecution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestExecution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestExecutionState {
        #[doc = "The user cancelled the execution. Can only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
        Error,
        #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
        Invalid,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed. Can only be set on an execution."]
        Running,
        #[doc = "Do not use. For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution or matrix is being validated."]
        Validating,
    }
    impl TestExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                TestExecutionState::Cancelled => "CANCELLED",
                TestExecutionState::Error => "ERROR",
                TestExecutionState::Finished => "FINISHED",
                TestExecutionState::IncompatibleArchitecture => "INCOMPATIBLE_ARCHITECTURE",
                TestExecutionState::IncompatibleEnvironment => "INCOMPATIBLE_ENVIRONMENT",
                TestExecutionState::Invalid => "INVALID",
                TestExecutionState::Pending => "PENDING",
                TestExecutionState::Running => "RUNNING",
                TestExecutionState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                TestExecutionState::UnsupportedEnvironment => "UNSUPPORTED_ENVIRONMENT",
                TestExecutionState::Validating => "VALIDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestExecutionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestExecutionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestExecutionState, ()> {
            Ok(match s {
                "CANCELLED" => TestExecutionState::Cancelled,
                "ERROR" => TestExecutionState::Error,
                "FINISHED" => TestExecutionState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => TestExecutionState::IncompatibleArchitecture,
                "INCOMPATIBLE_ENVIRONMENT" => TestExecutionState::IncompatibleEnvironment,
                "INVALID" => TestExecutionState::Invalid,
                "PENDING" => TestExecutionState::Pending,
                "RUNNING" => TestExecutionState::Running,
                "TEST_STATE_UNSPECIFIED" => TestExecutionState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => TestExecutionState::UnsupportedEnvironment,
                "VALIDATING" => TestExecutionState::Validating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestExecutionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestExecutionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => TestExecutionState::Cancelled,
                "ERROR" => TestExecutionState::Error,
                "FINISHED" => TestExecutionState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => TestExecutionState::IncompatibleArchitecture,
                "INCOMPATIBLE_ENVIRONMENT" => TestExecutionState::IncompatibleEnvironment,
                "INVALID" => TestExecutionState::Invalid,
                "PENDING" => TestExecutionState::Pending,
                "RUNNING" => TestExecutionState::Running,
                "TEST_STATE_UNSPECIFIED" => TestExecutionState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => TestExecutionState::UnsupportedEnvironment,
                "VALIDATING" => TestExecutionState::Validating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestExecutionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestExecutionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestMatrix {
        #[doc = "Information about the client which invoked the test."]
        #[serde(
            rename = "clientInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_info: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "Required. The devices the tests are being executed on."]
        #[serde(
            rename = "environmentMatrix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment_matrix: ::std::option::Option<crate::schemas::EnvironmentMatrix>,
        #[doc = "If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation."]
        #[serde(
            rename = "failFast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fail_fast: ::std::option::Option<bool>,
        #[doc = "The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns."]
        #[serde(
            rename = "flakyTestAttempts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flaky_test_attempts: ::std::option::Option<i32>,
        #[doc = "Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state."]
        #[serde(
            rename = "invalidMatrixDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_matrix_details:
            ::std::option::Option<crate::schemas::TestMatrixInvalidMatrixDetails>,
        #[doc = "Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED."]
        #[serde(
            rename = "outcomeSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome_summary: ::std::option::Option<crate::schemas::TestMatrixOutcomeSummary>,
        #[doc = "The cloud project that owns the test matrix."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Required. Where the results for the matrix are written."]
        #[serde(
            rename = "resultStorage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result_storage: ::std::option::Option<crate::schemas::ResultStorage>,
        #[doc = "Output only. Indicates the current progress of the test matrix."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::TestMatrixState>,
        #[doc = "Output only. The list of test executions that the service creates for this matrix."]
        #[serde(
            rename = "testExecutions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_executions: ::std::option::Option<Vec<crate::schemas::TestExecution>>,
        #[doc = "Output only. Unique id set by the service."]
        #[serde(
            rename = "testMatrixId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_matrix_id: ::std::option::Option<String>,
        #[doc = "Required. How to run the test."]
        #[serde(
            rename = "testSpecification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_specification: ::std::option::Option<crate::schemas::TestSpecification>,
        #[doc = "Output only. The time this test matrix was initially created."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TestMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestMatrixInvalidMatrixDetails {
        #[doc = "The zipped XCTest was built for the iOS simulator rather than for a physical device."]
        BuiltForIosSimulator,
        #[doc = "The matrix is INVALID, but there are no further details available."]
        DetailsUnavailable,
        #[doc = "Device administrator applications are not allowed."]
        DeviceAdminReceiver,
        #[doc = "The app declares one or more permissions that are not allowed."]
        ForbiddenPermissions,
        #[doc = "The test runner class specified by user or in the test APK's manifest file is not compatible with Android Test Orchestrator. Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher. Orchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR OrchestratorOption."]
        InstrumentationOrchestratorIncompatible,
        #[doc = "APK is built for a preview SDK which is unsupported"]
        InvalidApkPreviewSdk,
        #[doc = "Invalid definition of action in the robo directives (e.g. a click or ignore action includes an input text field)"]
        InvalidDirectiveAction,
        #[doc = "Either the provided input APK path was malformed, the APK file does not exist, or the user does not have permission to access the APK file."]
        InvalidInputApk,
        #[doc = "Do not use. For proto versioning only."]
        InvalidMatrixDetailsUnspecified,
        #[doc = "The APK application ID (aka package name) is invalid. See also https://developer.android.com/studio/build/application-id"]
        InvalidPackageName,
        #[doc = "There is at least one invalid resource name in the provided robo directives"]
        InvalidResourceName,
        #[doc = "There is a conflict in the provided robo_directives."]
        InvalidRoboDirectives,
        #[doc = "The input app APK could not be parsed."]
        MalformedApk,
        #[doc = "The iOS application bundle (.app) couldn't be processed."]
        MalformedAppBundle,
        #[doc = "The input IPA could not be parsed."]
        MalformedIpa,
        #[doc = "The input test APK could not be parsed."]
        MalformedTestApk,
        #[doc = "The zipped XCTest was malformed. The zip did not contain a single .xctestrun file and the contents of the DerivedData/Build/Products directory."]
        MalformedXcTestZip,
        #[doc = "The application doesn't register the game loop URL scheme."]
        MissingUrlScheme,
        #[doc = "APK contains no code. See also https://developer.android.com/guide/topics/manifest/application-element.html#code"]
        NoCodeApk,
        #[doc = "The test apk does not declare an instrumentation."]
        NoInstrumentation,
        #[doc = "A main launcher activity could not be found."]
        NoLauncherActivity,
        #[doc = "The AndroidManifest.xml could not be found."]
        NoManifest,
        #[doc = "The APK manifest does not declare a package name."]
        NoPackageName,
        #[doc = "The input app apk does not have a signature."]
        NoSignature,
        #[doc = "The test APK does not contain the test runner class specified by user or in the manifest file. This can be caused by either of the following reasons: - the user provided a runner class name that's incorrect, or - the test runner isn't built into the test APK (might be in the app APK instead)."]
        NoTestRunnerClass,
        #[doc = "The .xctestrun file did not specify any test targets."]
        NoTestsInXcTestZip,
        #[doc = "An Info.plist file in the XCTest zip could not be parsed."]
        PlistCannotBeParsed,
        #[doc = "There was an error when parsing a label's value."]
        ScenarioLabelMalformed,
        #[doc = "The request contains a scenario label that was not declared in the manifest."]
        ScenarioLabelNotDeclared,
        #[doc = "The request contains a scenario number that was not declared in the manifest."]
        ScenarioNotDeclared,
        #[doc = "There is no test loop intent filter, or the one that is given is not formatted correctly."]
        TestLoopIntentFilterNotFound,
        #[doc = "XC tests which run on physical devices must have \"IsAppHostedTestBundle\" == \"true\" in the xctestrun file."]
        TestNotAppHosted,
        #[doc = "The APK is marked as \"testOnly\". Deprecated and not currently used."]
        TestOnlyApk,
        #[doc = "The test package and app package are the same."]
        TestSameAsApp,
        #[doc = "One or more of the test targets defined in the .xctestrun file specifies \"UseDestinationArtifacts\", which is disallowed."]
        UseDestinationArtifacts,
    }
    impl TestMatrixInvalidMatrixDetails {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixInvalidMatrixDetails::BuiltForIosSimulator => "BUILT_FOR_IOS_SIMULATOR",
                TestMatrixInvalidMatrixDetails::DetailsUnavailable => "DETAILS_UNAVAILABLE",
                TestMatrixInvalidMatrixDetails::DeviceAdminReceiver => "DEVICE_ADMIN_RECEIVER",
                TestMatrixInvalidMatrixDetails::ForbiddenPermissions => "FORBIDDEN_PERMISSIONS",
                TestMatrixInvalidMatrixDetails::InstrumentationOrchestratorIncompatible => {
                    "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE"
                }
                TestMatrixInvalidMatrixDetails::InvalidApkPreviewSdk => "INVALID_APK_PREVIEW_SDK",
                TestMatrixInvalidMatrixDetails::InvalidDirectiveAction => {
                    "INVALID_DIRECTIVE_ACTION"
                }
                TestMatrixInvalidMatrixDetails::InvalidInputApk => "INVALID_INPUT_APK",
                TestMatrixInvalidMatrixDetails::InvalidMatrixDetailsUnspecified => {
                    "INVALID_MATRIX_DETAILS_UNSPECIFIED"
                }
                TestMatrixInvalidMatrixDetails::InvalidPackageName => "INVALID_PACKAGE_NAME",
                TestMatrixInvalidMatrixDetails::InvalidResourceName => "INVALID_RESOURCE_NAME",
                TestMatrixInvalidMatrixDetails::InvalidRoboDirectives => "INVALID_ROBO_DIRECTIVES",
                TestMatrixInvalidMatrixDetails::MalformedApk => "MALFORMED_APK",
                TestMatrixInvalidMatrixDetails::MalformedAppBundle => "MALFORMED_APP_BUNDLE",
                TestMatrixInvalidMatrixDetails::MalformedIpa => "MALFORMED_IPA",
                TestMatrixInvalidMatrixDetails::MalformedTestApk => "MALFORMED_TEST_APK",
                TestMatrixInvalidMatrixDetails::MalformedXcTestZip => "MALFORMED_XC_TEST_ZIP",
                TestMatrixInvalidMatrixDetails::MissingUrlScheme => "MISSING_URL_SCHEME",
                TestMatrixInvalidMatrixDetails::NoCodeApk => "NO_CODE_APK",
                TestMatrixInvalidMatrixDetails::NoInstrumentation => "NO_INSTRUMENTATION",
                TestMatrixInvalidMatrixDetails::NoLauncherActivity => "NO_LAUNCHER_ACTIVITY",
                TestMatrixInvalidMatrixDetails::NoManifest => "NO_MANIFEST",
                TestMatrixInvalidMatrixDetails::NoPackageName => "NO_PACKAGE_NAME",
                TestMatrixInvalidMatrixDetails::NoSignature => "NO_SIGNATURE",
                TestMatrixInvalidMatrixDetails::NoTestRunnerClass => "NO_TEST_RUNNER_CLASS",
                TestMatrixInvalidMatrixDetails::NoTestsInXcTestZip => "NO_TESTS_IN_XC_TEST_ZIP",
                TestMatrixInvalidMatrixDetails::PlistCannotBeParsed => "PLIST_CANNOT_BE_PARSED",
                TestMatrixInvalidMatrixDetails::ScenarioLabelMalformed => {
                    "SCENARIO_LABEL_MALFORMED"
                }
                TestMatrixInvalidMatrixDetails::ScenarioLabelNotDeclared => {
                    "SCENARIO_LABEL_NOT_DECLARED"
                }
                TestMatrixInvalidMatrixDetails::ScenarioNotDeclared => "SCENARIO_NOT_DECLARED",
                TestMatrixInvalidMatrixDetails::TestLoopIntentFilterNotFound => {
                    "TEST_LOOP_INTENT_FILTER_NOT_FOUND"
                }
                TestMatrixInvalidMatrixDetails::TestNotAppHosted => "TEST_NOT_APP_HOSTED",
                TestMatrixInvalidMatrixDetails::TestOnlyApk => "TEST_ONLY_APK",
                TestMatrixInvalidMatrixDetails::TestSameAsApp => "TEST_SAME_AS_APP",
                TestMatrixInvalidMatrixDetails::UseDestinationArtifacts => {
                    "USE_DESTINATION_ARTIFACTS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestMatrixInvalidMatrixDetails {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestMatrixInvalidMatrixDetails {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestMatrixInvalidMatrixDetails, ()> {
            Ok(match s {
                "BUILT_FOR_IOS_SIMULATOR" => TestMatrixInvalidMatrixDetails::BuiltForIosSimulator,
                "DETAILS_UNAVAILABLE" => TestMatrixInvalidMatrixDetails::DetailsUnavailable,
                "DEVICE_ADMIN_RECEIVER" => TestMatrixInvalidMatrixDetails::DeviceAdminReceiver,
                "FORBIDDEN_PERMISSIONS" => TestMatrixInvalidMatrixDetails::ForbiddenPermissions,
                "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE" => {
                    TestMatrixInvalidMatrixDetails::InstrumentationOrchestratorIncompatible
                }
                "INVALID_APK_PREVIEW_SDK" => TestMatrixInvalidMatrixDetails::InvalidApkPreviewSdk,
                "INVALID_DIRECTIVE_ACTION" => {
                    TestMatrixInvalidMatrixDetails::InvalidDirectiveAction
                }
                "INVALID_INPUT_APK" => TestMatrixInvalidMatrixDetails::InvalidInputApk,
                "INVALID_MATRIX_DETAILS_UNSPECIFIED" => {
                    TestMatrixInvalidMatrixDetails::InvalidMatrixDetailsUnspecified
                }
                "INVALID_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::InvalidPackageName,
                "INVALID_RESOURCE_NAME" => TestMatrixInvalidMatrixDetails::InvalidResourceName,
                "INVALID_ROBO_DIRECTIVES" => TestMatrixInvalidMatrixDetails::InvalidRoboDirectives,
                "MALFORMED_APK" => TestMatrixInvalidMatrixDetails::MalformedApk,
                "MALFORMED_APP_BUNDLE" => TestMatrixInvalidMatrixDetails::MalformedAppBundle,
                "MALFORMED_IPA" => TestMatrixInvalidMatrixDetails::MalformedIpa,
                "MALFORMED_TEST_APK" => TestMatrixInvalidMatrixDetails::MalformedTestApk,
                "MALFORMED_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::MalformedXcTestZip,
                "MISSING_URL_SCHEME" => TestMatrixInvalidMatrixDetails::MissingUrlScheme,
                "NO_CODE_APK" => TestMatrixInvalidMatrixDetails::NoCodeApk,
                "NO_INSTRUMENTATION" => TestMatrixInvalidMatrixDetails::NoInstrumentation,
                "NO_LAUNCHER_ACTIVITY" => TestMatrixInvalidMatrixDetails::NoLauncherActivity,
                "NO_MANIFEST" => TestMatrixInvalidMatrixDetails::NoManifest,
                "NO_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::NoPackageName,
                "NO_SIGNATURE" => TestMatrixInvalidMatrixDetails::NoSignature,
                "NO_TEST_RUNNER_CLASS" => TestMatrixInvalidMatrixDetails::NoTestRunnerClass,
                "NO_TESTS_IN_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::NoTestsInXcTestZip,
                "PLIST_CANNOT_BE_PARSED" => TestMatrixInvalidMatrixDetails::PlistCannotBeParsed,
                "SCENARIO_LABEL_MALFORMED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelMalformed
                }
                "SCENARIO_LABEL_NOT_DECLARED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelNotDeclared
                }
                "SCENARIO_NOT_DECLARED" => TestMatrixInvalidMatrixDetails::ScenarioNotDeclared,
                "TEST_LOOP_INTENT_FILTER_NOT_FOUND" => {
                    TestMatrixInvalidMatrixDetails::TestLoopIntentFilterNotFound
                }
                "TEST_NOT_APP_HOSTED" => TestMatrixInvalidMatrixDetails::TestNotAppHosted,
                "TEST_ONLY_APK" => TestMatrixInvalidMatrixDetails::TestOnlyApk,
                "TEST_SAME_AS_APP" => TestMatrixInvalidMatrixDetails::TestSameAsApp,
                "USE_DESTINATION_ARTIFACTS" => {
                    TestMatrixInvalidMatrixDetails::UseDestinationArtifacts
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestMatrixInvalidMatrixDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixInvalidMatrixDetails {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixInvalidMatrixDetails {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUILT_FOR_IOS_SIMULATOR" => TestMatrixInvalidMatrixDetails::BuiltForIosSimulator,
                "DETAILS_UNAVAILABLE" => TestMatrixInvalidMatrixDetails::DetailsUnavailable,
                "DEVICE_ADMIN_RECEIVER" => TestMatrixInvalidMatrixDetails::DeviceAdminReceiver,
                "FORBIDDEN_PERMISSIONS" => TestMatrixInvalidMatrixDetails::ForbiddenPermissions,
                "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE" => {
                    TestMatrixInvalidMatrixDetails::InstrumentationOrchestratorIncompatible
                }
                "INVALID_APK_PREVIEW_SDK" => TestMatrixInvalidMatrixDetails::InvalidApkPreviewSdk,
                "INVALID_DIRECTIVE_ACTION" => {
                    TestMatrixInvalidMatrixDetails::InvalidDirectiveAction
                }
                "INVALID_INPUT_APK" => TestMatrixInvalidMatrixDetails::InvalidInputApk,
                "INVALID_MATRIX_DETAILS_UNSPECIFIED" => {
                    TestMatrixInvalidMatrixDetails::InvalidMatrixDetailsUnspecified
                }
                "INVALID_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::InvalidPackageName,
                "INVALID_RESOURCE_NAME" => TestMatrixInvalidMatrixDetails::InvalidResourceName,
                "INVALID_ROBO_DIRECTIVES" => TestMatrixInvalidMatrixDetails::InvalidRoboDirectives,
                "MALFORMED_APK" => TestMatrixInvalidMatrixDetails::MalformedApk,
                "MALFORMED_APP_BUNDLE" => TestMatrixInvalidMatrixDetails::MalformedAppBundle,
                "MALFORMED_IPA" => TestMatrixInvalidMatrixDetails::MalformedIpa,
                "MALFORMED_TEST_APK" => TestMatrixInvalidMatrixDetails::MalformedTestApk,
                "MALFORMED_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::MalformedXcTestZip,
                "MISSING_URL_SCHEME" => TestMatrixInvalidMatrixDetails::MissingUrlScheme,
                "NO_CODE_APK" => TestMatrixInvalidMatrixDetails::NoCodeApk,
                "NO_INSTRUMENTATION" => TestMatrixInvalidMatrixDetails::NoInstrumentation,
                "NO_LAUNCHER_ACTIVITY" => TestMatrixInvalidMatrixDetails::NoLauncherActivity,
                "NO_MANIFEST" => TestMatrixInvalidMatrixDetails::NoManifest,
                "NO_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::NoPackageName,
                "NO_SIGNATURE" => TestMatrixInvalidMatrixDetails::NoSignature,
                "NO_TEST_RUNNER_CLASS" => TestMatrixInvalidMatrixDetails::NoTestRunnerClass,
                "NO_TESTS_IN_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::NoTestsInXcTestZip,
                "PLIST_CANNOT_BE_PARSED" => TestMatrixInvalidMatrixDetails::PlistCannotBeParsed,
                "SCENARIO_LABEL_MALFORMED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelMalformed
                }
                "SCENARIO_LABEL_NOT_DECLARED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelNotDeclared
                }
                "SCENARIO_NOT_DECLARED" => TestMatrixInvalidMatrixDetails::ScenarioNotDeclared,
                "TEST_LOOP_INTENT_FILTER_NOT_FOUND" => {
                    TestMatrixInvalidMatrixDetails::TestLoopIntentFilterNotFound
                }
                "TEST_NOT_APP_HOSTED" => TestMatrixInvalidMatrixDetails::TestNotAppHosted,
                "TEST_ONLY_APK" => TestMatrixInvalidMatrixDetails::TestOnlyApk,
                "TEST_SAME_AS_APP" => TestMatrixInvalidMatrixDetails::TestSameAsApp,
                "USE_DESTINATION_ARTIFACTS" => {
                    TestMatrixInvalidMatrixDetails::UseDestinationArtifacts
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
    impl ::google_field_selector::FieldSelector for TestMatrixInvalidMatrixDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestMatrixInvalidMatrixDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestMatrixOutcomeSummary {
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[doc = "Do not use. For proto versioning only."]
        OutcomeSummaryUnspecified,
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
    }
    impl TestMatrixOutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixOutcomeSummary::Failure => "FAILURE",
                TestMatrixOutcomeSummary::Inconclusive => "INCONCLUSIVE",
                TestMatrixOutcomeSummary::OutcomeSummaryUnspecified => {
                    "OUTCOME_SUMMARY_UNSPECIFIED"
                }
                TestMatrixOutcomeSummary::Skipped => "SKIPPED",
                TestMatrixOutcomeSummary::Success => "SUCCESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestMatrixOutcomeSummary {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestMatrixOutcomeSummary {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestMatrixOutcomeSummary, ()> {
            Ok(match s {
                "FAILURE" => TestMatrixOutcomeSummary::Failure,
                "INCONCLUSIVE" => TestMatrixOutcomeSummary::Inconclusive,
                "OUTCOME_SUMMARY_UNSPECIFIED" => {
                    TestMatrixOutcomeSummary::OutcomeSummaryUnspecified
                }
                "SKIPPED" => TestMatrixOutcomeSummary::Skipped,
                "SUCCESS" => TestMatrixOutcomeSummary::Success,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestMatrixOutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixOutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixOutcomeSummary {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILURE" => TestMatrixOutcomeSummary::Failure,
                "INCONCLUSIVE" => TestMatrixOutcomeSummary::Inconclusive,
                "OUTCOME_SUMMARY_UNSPECIFIED" => {
                    TestMatrixOutcomeSummary::OutcomeSummaryUnspecified
                }
                "SKIPPED" => TestMatrixOutcomeSummary::Skipped,
                "SUCCESS" => TestMatrixOutcomeSummary::Success,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestMatrixOutcomeSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestMatrixOutcomeSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestMatrixState {
        #[doc = "The user cancelled the execution. Can only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix has stopped because it encountered an infrastructure failure."]
        Error,
        #[doc = "The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware"]
        Invalid,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed. Can only be set on an execution."]
        Running,
        #[doc = "Do not use. For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution or matrix is being validated."]
        Validating,
    }
    impl TestMatrixState {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixState::Cancelled => "CANCELLED",
                TestMatrixState::Error => "ERROR",
                TestMatrixState::Finished => "FINISHED",
                TestMatrixState::IncompatibleArchitecture => "INCOMPATIBLE_ARCHITECTURE",
                TestMatrixState::IncompatibleEnvironment => "INCOMPATIBLE_ENVIRONMENT",
                TestMatrixState::Invalid => "INVALID",
                TestMatrixState::Pending => "PENDING",
                TestMatrixState::Running => "RUNNING",
                TestMatrixState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                TestMatrixState::UnsupportedEnvironment => "UNSUPPORTED_ENVIRONMENT",
                TestMatrixState::Validating => "VALIDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestMatrixState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestMatrixState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestMatrixState, ()> {
            Ok(match s {
                "CANCELLED" => TestMatrixState::Cancelled,
                "ERROR" => TestMatrixState::Error,
                "FINISHED" => TestMatrixState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => TestMatrixState::IncompatibleArchitecture,
                "INCOMPATIBLE_ENVIRONMENT" => TestMatrixState::IncompatibleEnvironment,
                "INVALID" => TestMatrixState::Invalid,
                "PENDING" => TestMatrixState::Pending,
                "RUNNING" => TestMatrixState::Running,
                "TEST_STATE_UNSPECIFIED" => TestMatrixState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => TestMatrixState::UnsupportedEnvironment,
                "VALIDATING" => TestMatrixState::Validating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestMatrixState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => TestMatrixState::Cancelled,
                "ERROR" => TestMatrixState::Error,
                "FINISHED" => TestMatrixState::Finished,
                "INCOMPATIBLE_ARCHITECTURE" => TestMatrixState::IncompatibleArchitecture,
                "INCOMPATIBLE_ENVIRONMENT" => TestMatrixState::IncompatibleEnvironment,
                "INVALID" => TestMatrixState::Invalid,
                "PENDING" => TestMatrixState::Pending,
                "RUNNING" => TestMatrixState::Running,
                "TEST_STATE_UNSPECIFIED" => TestMatrixState::TestStateUnspecified,
                "UNSUPPORTED_ENVIRONMENT" => TestMatrixState::UnsupportedEnvironment,
                "VALIDATING" => TestMatrixState::Validating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestMatrixState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestMatrixState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestSetup {
        #[doc = "The device will be logged in on this account for the duration of the test."]
        #[serde(
            rename = "account",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account: ::std::option::Option<crate::schemas::Account>,
        #[doc = "APKs to install in addition to those being directly tested. Currently capped at 100."]
        #[serde(
            rename = "additionalApks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_apks: ::std::option::Option<Vec<crate::schemas::Apk>>,
        #[doc = "List of directories on the device to upload to GCS at the end of the test; they must be absolute paths under /sdcard, /storage or /data/local/tmp. Path names are restricted to characters a-z A-Z 0-9 _ - . + and / Note: The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device."]
        #[serde(
            rename = "directoriesToPull",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub directories_to_pull: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to prevent all runtime permissions to be granted at app install"]
        #[serde(
            rename = "dontAutograntPermissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dont_autogrant_permissions: ::std::option::Option<bool>,
        #[doc = "Environment variables to set for the test (only applicable for instrumentation tests)."]
        #[serde(
            rename = "environmentVariables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment_variables: ::std::option::Option<Vec<crate::schemas::EnvironmentVariable>>,
        #[doc = "List of files to push to the device before starting the test."]
        #[serde(
            rename = "filesToPush",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files_to_push: ::std::option::Option<Vec<crate::schemas::DeviceFile>>,
        #[doc = "The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
        #[serde(
            rename = "networkProfile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_profile: ::std::option::Option<String>,
        #[doc = "Deprecated: Systrace uses Python 2 which has been sunset 2020-01-01. Support of Systrace may stop at any time, at which point no Systrace file will be provided in the results. Systrace configuration for the run. If set a systrace will be taken, starting on test start and lasting for the configured duration. The systrace file thus obtained is put in the results bucket together with the other artifacts from the run."]
        #[serde(
            rename = "systrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub systrace: ::std::option::Option<crate::schemas::SystraceSetup>,
    }
    impl ::google_field_selector::FieldSelector for TestSetup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestSetup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestSpecification {
        #[doc = "An Android instrumentation test."]
        #[serde(
            rename = "androidInstrumentationTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_instrumentation_test:
            ::std::option::Option<crate::schemas::AndroidInstrumentationTest>,
        #[doc = "An Android robo test."]
        #[serde(
            rename = "androidRoboTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_robo_test: ::std::option::Option<crate::schemas::AndroidRoboTest>,
        #[doc = "An Android Application with a Test Loop."]
        #[serde(
            rename = "androidTestLoop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_test_loop: ::std::option::Option<crate::schemas::AndroidTestLoop>,
        #[doc = "Disables performance metrics recording. May reduce test latency."]
        #[serde(
            rename = "disablePerformanceMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_performance_metrics: ::std::option::Option<bool>,
        #[doc = "Disables video recording. May reduce test latency."]
        #[serde(
            rename = "disableVideoRecording",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_video_recording: ::std::option::Option<bool>,
        #[doc = "An iOS application with a test loop."]
        #[serde(
            rename = "iosTestLoop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_test_loop: ::std::option::Option<crate::schemas::IosTestLoop>,
        #[doc = "Test setup requirements for iOS."]
        #[serde(
            rename = "iosTestSetup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_test_setup: ::std::option::Option<crate::schemas::IosTestSetup>,
        #[doc = "An iOS XCTest, via an .xctestrun file."]
        #[serde(
            rename = "iosXcTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_xc_test: ::std::option::Option<crate::schemas::IosXcTest>,
        #[doc = "Test setup requirements for Android e.g. files to install, bootstrap scripts."]
        #[serde(
            rename = "testSetup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_setup: ::std::option::Option<crate::schemas::TestSetup>,
        #[doc = "Max time a test execution is allowed to run before it is automatically cancelled. The default value is 5 min."]
        #[serde(
            rename = "testTimeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_timeout: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TestSpecification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestSpecification {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestTargetsForShard {
        #[doc = "Group of packages, classes, and/or test methods to be run for each shard. The targets need to be specified in AndroidJUnitRunner argument format. For example, \"package com.my.packages\" \"class com.my.package.MyClass\". The number of test_targets must be greater than 0."]
        #[serde(
            rename = "testTargets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_targets: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestTargetsForShard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestTargetsForShard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolResultsExecution {
        #[doc = "Output only. A tool results execution ID."]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Output only. A tool results history ID."]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Output only. The cloud project that owns the tool results execution."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ToolResultsExecution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolResultsExecution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolResultsHistory {
        #[doc = "Required. A tool results history ID."]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Required. The cloud project that owns the tool results history."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ToolResultsHistory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolResultsHistory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolResultsStep {
        #[doc = "Output only. A tool results execution ID."]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Output only. A tool results history ID."]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Output only. The cloud project that owns the tool results step."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Output only. A tool results step ID."]
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ToolResultsStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolResultsStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrafficRule {
        #[doc = "Bandwidth in kbits/second."]
        #[serde(
            rename = "bandwidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bandwidth: ::std::option::Option<f32>,
        #[doc = "Burst size in kbits."]
        #[serde(
            rename = "burst",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub burst: ::std::option::Option<f32>,
        #[doc = "Packet delay, must be >= 0."]
        #[serde(
            rename = "delay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delay: ::std::option::Option<String>,
        #[doc = "Packet duplication ratio (0.0 - 1.0)."]
        #[serde(
            rename = "packetDuplicationRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packet_duplication_ratio: ::std::option::Option<f32>,
        #[doc = "Packet loss ratio (0.0 - 1.0)."]
        #[serde(
            rename = "packetLossRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packet_loss_ratio: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for TrafficRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrafficRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UniformSharding {
        #[doc = "Required. Total number of shards. When any physical devices are selected, the number must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500."]
        #[serde(
            rename = "numShards",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_shards: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UniformSharding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UniformSharding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct XcodeVersion {
        #[doc = "Tags for this Xcode version. Example: \"default\"."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
        #[doc = "The id for this version. Example: \"9.2\"."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for XcodeVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for XcodeVersion {
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
    #[doc = "Actions that can be performed on the application_detail_service resource"]
    pub fn application_detail_service(
        &self,
    ) -> crate::resources::application_detail_service::ApplicationDetailServiceActions {
        crate::resources::application_detail_service::ApplicationDetailServiceActions {
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
    #[doc = "Actions that can be performed on the test_environment_catalog resource"]
    pub fn test_environment_catalog(
        &self,
    ) -> crate::resources::test_environment_catalog::TestEnvironmentCatalogActions {
        crate::resources::test_environment_catalog::TestEnvironmentCatalogActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod application_detail_service {
        pub mod params {}
        pub struct ApplicationDetailServiceActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ApplicationDetailServiceActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the details of an Android application APK."]
            pub fn get_apk_details(
                &self,
                request: crate::schemas::FileReference,
            ) -> GetApkDetailsRequestBuilder {
                GetApkDetailsRequestBuilder {
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
        }
        #[doc = "Created via [ApplicationDetailServiceActions::get_apk_details()](struct.ApplicationDetailServiceActions.html#method.get_apk_details)"]
        #[derive(Debug, Clone)]
        pub struct GetApkDetailsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::FileReference,
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
        impl<'a> GetApkDetailsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetApkDetailsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetApkDetailsResponse, crate::Error> {
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
                let mut output = "https://testing.googleapis.com/".to_owned();
                output.push_str("v1/applicationDetailService/getApkDetails");
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
            #[doc = "Actions that can be performed on the test_matrices resource"]
            pub fn test_matrices(
                &self,
            ) -> crate::resources::projects::test_matrices::TestMatricesActions {
                crate::resources::projects::test_matrices::TestMatricesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod test_matrices {
            pub mod params {}
            pub struct TestMatricesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> TestMatricesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist"]
                pub fn cancel(
                    &self,
                    project_id: impl Into<String>,
                    test_matrix_id: impl Into<String>,
                ) -> CancelRequestBuilder {
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
                        project_id: project_id.into(),
                        test_matrix_id: test_matrix_id.into(),
                    }
                }
                #[doc = "Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices."]
                pub fn create(
                    &self,
                    request: crate::schemas::TestMatrix,
                    project_id: impl Into<String>,
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
                        project_id: project_id.into(),
                        request_id: None,
                    }
                }
                #[doc = "Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist"]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    test_matrix_id: impl Into<String>,
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
                        project_id: project_id.into(),
                        test_matrix_id: test_matrix_id.into(),
                    }
                }
            }
            #[doc = "Created via [TestMatricesActions::cancel()](struct.TestMatricesActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_id: String,
                test_matrix_id: String,
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
                ) -> Result<crate::schemas::CancelTestMatrixResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::CancelTestMatrixResponse, crate::Error>
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices/");
                    {
                        let var_as_str = &self.test_matrix_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
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
            #[doc = "Created via [TestMatricesActions::create()](struct.TestMatricesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestMatrix,
                project_id: String,
                request_id: ::std::option::Option<String>,
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
                #[doc = "A string id used to detect duplicated requests. Ids are automatically scoped to a project, so users should ensure the ID is unique per-project. A UUID is recommended. Optional, but strongly recommended."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
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
                ) -> Result<crate::schemas::TestMatrix, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestMatrix, crate::Error> {
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("requestId", &self.request_id)]);
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
            #[doc = "Created via [TestMatricesActions::get()](struct.TestMatricesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_id: String,
                test_matrix_id: String,
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
                ) -> Result<crate::schemas::TestMatrix, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestMatrix, crate::Error> {
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices/");
                    {
                        let var_as_str = &self.test_matrix_id;
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
        }
    }
    pub mod test_environment_catalog {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetEnvironmentType {
                #[doc = "A device running a version of the Android OS."]
                Android,
                #[doc = "The IP blocks used by devices in the test environment."]
                DeviceIpBlocks,
                #[doc = "Do not use. For proto versioning only."]
                EnvironmentTypeUnspecified,
                #[doc = "A device running a version of iOS."]
                Ios,
                #[doc = "A network configuration to use when running a test."]
                NetworkConfiguration,
                #[doc = "The software environment provided by TestExecutionService."]
                ProvidedSoftware,
            }
            impl GetEnvironmentType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetEnvironmentType::Android => "ANDROID",
                        GetEnvironmentType::DeviceIpBlocks => "DEVICE_IP_BLOCKS",
                        GetEnvironmentType::EnvironmentTypeUnspecified => {
                            "ENVIRONMENT_TYPE_UNSPECIFIED"
                        }
                        GetEnvironmentType::Ios => "IOS",
                        GetEnvironmentType::NetworkConfiguration => "NETWORK_CONFIGURATION",
                        GetEnvironmentType::ProvidedSoftware => "PROVIDED_SOFTWARE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetEnvironmentType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetEnvironmentType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetEnvironmentType, ()> {
                    Ok(match s {
                        "ANDROID" => GetEnvironmentType::Android,
                        "DEVICE_IP_BLOCKS" => GetEnvironmentType::DeviceIpBlocks,
                        "ENVIRONMENT_TYPE_UNSPECIFIED" => {
                            GetEnvironmentType::EnvironmentTypeUnspecified
                        }
                        "IOS" => GetEnvironmentType::Ios,
                        "NETWORK_CONFIGURATION" => GetEnvironmentType::NetworkConfiguration,
                        "PROVIDED_SOFTWARE" => GetEnvironmentType::ProvidedSoftware,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetEnvironmentType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetEnvironmentType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetEnvironmentType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ANDROID" => GetEnvironmentType::Android,
                        "DEVICE_IP_BLOCKS" => GetEnvironmentType::DeviceIpBlocks,
                        "ENVIRONMENT_TYPE_UNSPECIFIED" => {
                            GetEnvironmentType::EnvironmentTypeUnspecified
                        }
                        "IOS" => GetEnvironmentType::Ios,
                        "NETWORK_CONFIGURATION" => GetEnvironmentType::NetworkConfiguration,
                        "PROVIDED_SOFTWARE" => GetEnvironmentType::ProvidedSoftware,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetEnvironmentType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetEnvironmentType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct TestEnvironmentCatalogActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TestEnvironmentCatalogActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred"]
            pub fn get(
                &self,
                environment_type : crate :: resources :: test_environment_catalog :: params :: GetEnvironmentType,
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
                    environment_type,
                    project_id: None,
                }
            }
        }
        #[doc = "Created via [TestEnvironmentCatalogActions::get()](struct.TestEnvironmentCatalogActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            environment_type:
                crate::resources::test_environment_catalog::params::GetEnvironmentType,
            project_id: ::std::option::Option<String>,
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
            #[doc = "For authorization, the cloud project requesting the TestEnvironmentCatalog."]
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
            ) -> Result<crate::schemas::TestEnvironmentCatalog, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TestEnvironmentCatalog, crate::Error> {
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
                let mut output = "https://testing.googleapis.com/".to_owned();
                output.push_str("v1/testEnvironmentCatalog/");
                {
                    let var_as_string = self.environment_type.to_string();
                    let var_as_str = &var_as_string;
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
