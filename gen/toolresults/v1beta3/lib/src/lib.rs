#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [*getSettings*](resources/projects/struct.GetSettingsRequestBuilder.html), [*initializeSettings*](resources/projects/struct.InitializeSettingsRequestBuilder.html)\n  * [histories](resources/projects/histories/struct.HistoriesActions.html)\n    * [*create*](resources/projects/histories/struct.CreateRequestBuilder.html), [*get*](resources/projects/histories/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/struct.ListRequestBuilder.html)\n    * [executions](resources/projects/histories/executions/struct.ExecutionsActions.html)\n      * [*create*](resources/projects/histories/executions/struct.CreateRequestBuilder.html), [*get*](resources/projects/histories/executions/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/executions/struct.ListRequestBuilder.html), [*patch*](resources/projects/histories/executions/struct.PatchRequestBuilder.html)\n      * [clusters](resources/projects/histories/executions/clusters/struct.ClustersActions.html)\n        * [*get*](resources/projects/histories/executions/clusters/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/executions/clusters/struct.ListRequestBuilder.html)\n      * [environments](resources/projects/histories/executions/environments/struct.EnvironmentsActions.html)\n        * [*get*](resources/projects/histories/executions/environments/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/executions/environments/struct.ListRequestBuilder.html)\n      * [steps](resources/projects/histories/executions/steps/struct.StepsActions.html)\n        * [*accessibilityClusters*](resources/projects/histories/executions/steps/struct.AccessibilityClustersRequestBuilder.html), [*create*](resources/projects/histories/executions/steps/struct.CreateRequestBuilder.html), [*get*](resources/projects/histories/executions/steps/struct.GetRequestBuilder.html), [*getPerfMetricsSummary*](resources/projects/histories/executions/steps/struct.GetPerfMetricsSummaryRequestBuilder.html), [*list*](resources/projects/histories/executions/steps/struct.ListRequestBuilder.html), [*patch*](resources/projects/histories/executions/steps/struct.PatchRequestBuilder.html), [*publishXunitXmlFiles*](resources/projects/histories/executions/steps/struct.PublishXunitXmlFilesRequestBuilder.html)\n        * [perf_metrics_summary](resources/projects/histories/executions/steps/perf_metrics_summary/struct.PerfMetricsSummaryActions.html)\n          * [*create*](resources/projects/histories/executions/steps/perf_metrics_summary/struct.CreateRequestBuilder.html)\n        * [perf_sample_series](resources/projects/histories/executions/steps/perf_sample_series/struct.PerfSampleSeriesActions.html)\n          * [*create*](resources/projects/histories/executions/steps/perf_sample_series/struct.CreateRequestBuilder.html), [*get*](resources/projects/histories/executions/steps/perf_sample_series/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/executions/steps/perf_sample_series/struct.ListRequestBuilder.html)\n          * [samples](resources/projects/histories/executions/steps/perf_sample_series/samples/struct.SamplesActions.html)\n            * [*batchCreate*](resources/projects/histories/executions/steps/perf_sample_series/samples/struct.BatchCreateRequestBuilder.html), [*list*](resources/projects/histories/executions/steps/perf_sample_series/samples/struct.ListRequestBuilder.html)\n        * [test_cases](resources/projects/histories/executions/steps/test_cases/struct.TestCasesActions.html)\n          * [*get*](resources/projects/histories/executions/steps/test_cases/struct.GetRequestBuilder.html), [*list*](resources/projects/histories/executions/steps/test_cases/struct.ListRequestBuilder.html)\n        * [thumbnails](resources/projects/histories/executions/steps/thumbnails/struct.ThumbnailsActions.html)\n          * [*list*](resources/projects/histories/executions/steps/thumbnails/struct.ListRequestBuilder.html)\n"]
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
    pub struct AndroidAppInfo {
        #[doc = "The name of the app. Optional"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The package name of the app. Required."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The internal version code of the app. Optional."]
        #[serde(
            rename = "versionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_code: ::std::option::Option<String>,
        #[doc = "The version name of the app. Optional."]
        #[serde(
            rename = "versionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidAppInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The java package for the test to be executed. Required"]
        #[serde(
            rename = "testPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_package_id: ::std::option::Option<String>,
        #[doc = "The InstrumentationTestRunner class. Required"]
        #[serde(
            rename = "testRunnerClass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_runner_class: ::std::option::Option<String>,
        #[doc = "Each target must be fully qualified with the package name or class name, in one of these formats: - “package package_name” - “class package_name.class_name” - “class package_name.class_name#method_name” If empty, all targets in the module will be run."]
        #[serde(
            rename = "testTargets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_targets: ::std::option::Option<Vec<String>>,
        #[doc = "The flag indicates whether Android Test Orchestrator will be used to run test or not."]
        #[serde(
            rename = "useOrchestrator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_orchestrator: ::std::option::Option<bool>,
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
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The initial activity that should be used to start the app. Optional"]
        #[serde(
            rename = "appInitialActivity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_initial_activity: ::std::option::Option<String>,
        #[doc = "The java package for the bootstrap. Optional"]
        #[serde(
            rename = "bootstrapPackageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bootstrap_package_id: ::std::option::Option<String>,
        #[doc = "The runner class for the bootstrap. Optional"]
        #[serde(
            rename = "bootstrapRunnerClass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bootstrap_runner_class: ::std::option::Option<String>,
        #[doc = "The max depth of the traversal stack Robo can explore. Optional"]
        #[serde(
            rename = "maxDepth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_depth: ::std::option::Option<i32>,
        #[doc = "The max number of steps/actions Robo can execute. Default is no limit (0). Optional"]
        #[serde(
            rename = "maxSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_steps: ::std::option::Option<i32>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidTest {
        #[doc = "Information about the application under test."]
        #[serde(
            rename = "androidAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_app_info: ::std::option::Option<crate::schemas::AndroidAppInfo>,
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
        #[doc = "An Android test loop."]
        #[serde(
            rename = "androidTestLoop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_test_loop: ::std::option::Option<crate::schemas::AndroidTestLoop>,
        #[doc = "Max time a test is allowed to run before it is automatically cancelled."]
        #[serde(
            rename = "testTimeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_timeout: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::google_field_selector::FieldSelector for AndroidTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct AndroidTestLoop {}
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Anr {
        #[doc = "The stack trace of the ANR crash. Optional."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
    }
    impl ::google_field_selector::FieldSelector for Anr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Anr {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Any {
        #[doc = "A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one “/” character. The last segment of the URL’s path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading “.” is not accepted). In practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows: * If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a google.protobuf.Type value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the URL, or have them precompiled into a binary to avoid any lookup. Therefore, binary compatibility needs to be preserved on changes to types. (Use versioned type names to manage breaking changes.) Note: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com. Schemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics."]
        #[serde(
            rename = "typeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_url: ::std::option::Option<String>,
        #[doc = "Must be a valid serialized protocol buffer of the above specified type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Any {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Any {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppStartTime {
        #[doc = "Optional. The time from app start to reaching the developer-reported “fully drawn” time. This is only stored if the app includes a call to Activity.reportFullyDrawn(). See https://developer.android.com/topic/performance/launch-time.html#time-full"]
        #[serde(
            rename = "fullyDrawnTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_drawn_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The time from app start to the first displayed activity being drawn, as reported in Logcat. See https://developer.android.com/topic/performance/launch-time.html#time-initial"]
        #[serde(
            rename = "initialDisplayTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_display_time: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::google_field_selector::FieldSelector for AppStartTime {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppStartTime {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct AvailableDeepLinks {}
    impl ::google_field_selector::FieldSelector for AvailableDeepLinks {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AvailableDeepLinks {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BasicPerfSampleSeries {
        #[serde(
            rename = "perfMetricType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_metric_type:
            ::std::option::Option<crate::schemas::BasicPerfSampleSeriesPerfMetricType>,
        #[serde(
            rename = "perfUnit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_unit: ::std::option::Option<crate::schemas::BasicPerfSampleSeriesPerfUnit>,
        #[serde(
            rename = "sampleSeriesLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_series_label:
            ::std::option::Option<crate::schemas::BasicPerfSampleSeriesSampleSeriesLabel>,
    }
    impl ::google_field_selector::FieldSelector for BasicPerfSampleSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicPerfSampleSeries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesPerfMetricType {
        Cpu,
        Graphics,
        Memory,
        Network,
        PerfMetricTypeUnspecified,
    }
    impl BasicPerfSampleSeriesPerfMetricType {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesPerfMetricType::Cpu => "cpu",
                BasicPerfSampleSeriesPerfMetricType::Graphics => "graphics",
                BasicPerfSampleSeriesPerfMetricType::Memory => "memory",
                BasicPerfSampleSeriesPerfMetricType::Network => "network",
                BasicPerfSampleSeriesPerfMetricType::PerfMetricTypeUnspecified => {
                    "perfMetricTypeUnspecified"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicPerfSampleSeriesPerfMetricType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicPerfSampleSeriesPerfMetricType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicPerfSampleSeriesPerfMetricType, ()> {
            Ok(match s {
                "cpu" => BasicPerfSampleSeriesPerfMetricType::Cpu,
                "graphics" => BasicPerfSampleSeriesPerfMetricType::Graphics,
                "memory" => BasicPerfSampleSeriesPerfMetricType::Memory,
                "network" => BasicPerfSampleSeriesPerfMetricType::Network,
                "perfMetricTypeUnspecified" => {
                    BasicPerfSampleSeriesPerfMetricType::PerfMetricTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesPerfMetricType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesPerfMetricType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesPerfMetricType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpu" => BasicPerfSampleSeriesPerfMetricType::Cpu,
                "graphics" => BasicPerfSampleSeriesPerfMetricType::Graphics,
                "memory" => BasicPerfSampleSeriesPerfMetricType::Memory,
                "network" => BasicPerfSampleSeriesPerfMetricType::Network,
                "perfMetricTypeUnspecified" => {
                    BasicPerfSampleSeriesPerfMetricType::PerfMetricTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for BasicPerfSampleSeriesPerfMetricType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicPerfSampleSeriesPerfMetricType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesPerfUnit {
        Byte,
        BytesPerSecond,
        FramesPerSecond,
        Kibibyte,
        Percent,
        PerfUnitUnspecified,
    }
    impl BasicPerfSampleSeriesPerfUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesPerfUnit::Byte => "byte",
                BasicPerfSampleSeriesPerfUnit::BytesPerSecond => "bytesPerSecond",
                BasicPerfSampleSeriesPerfUnit::FramesPerSecond => "framesPerSecond",
                BasicPerfSampleSeriesPerfUnit::Kibibyte => "kibibyte",
                BasicPerfSampleSeriesPerfUnit::Percent => "percent",
                BasicPerfSampleSeriesPerfUnit::PerfUnitUnspecified => "perfUnitUnspecified",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicPerfSampleSeriesPerfUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicPerfSampleSeriesPerfUnit {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicPerfSampleSeriesPerfUnit, ()> {
            Ok(match s {
                "byte" => BasicPerfSampleSeriesPerfUnit::Byte,
                "bytesPerSecond" => BasicPerfSampleSeriesPerfUnit::BytesPerSecond,
                "framesPerSecond" => BasicPerfSampleSeriesPerfUnit::FramesPerSecond,
                "kibibyte" => BasicPerfSampleSeriesPerfUnit::Kibibyte,
                "percent" => BasicPerfSampleSeriesPerfUnit::Percent,
                "perfUnitUnspecified" => BasicPerfSampleSeriesPerfUnit::PerfUnitUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesPerfUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesPerfUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesPerfUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "byte" => BasicPerfSampleSeriesPerfUnit::Byte,
                "bytesPerSecond" => BasicPerfSampleSeriesPerfUnit::BytesPerSecond,
                "framesPerSecond" => BasicPerfSampleSeriesPerfUnit::FramesPerSecond,
                "kibibyte" => BasicPerfSampleSeriesPerfUnit::Kibibyte,
                "percent" => BasicPerfSampleSeriesPerfUnit::Percent,
                "perfUnitUnspecified" => BasicPerfSampleSeriesPerfUnit::PerfUnitUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicPerfSampleSeriesPerfUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicPerfSampleSeriesPerfUnit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesSampleSeriesLabel {
        CpuKernel,
        CpuTotal,
        #[doc = "CPU sample series"]
        CpuUser,
        #[doc = "Graphics sample series"]
        GraphicsFrameRate,
        #[doc = "Memory sample series"]
        MemoryRssPrivate,
        MemoryRssShared,
        MemoryRssTotal,
        MemoryTotal,
        NetworkReceived,
        NetworkSent,
        NtBytesReceived,
        #[doc = "Network sample series"]
        NtBytesTransferred,
        SampleSeriesTypeUnspecified,
    }
    impl BasicPerfSampleSeriesSampleSeriesLabel {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesSampleSeriesLabel::CpuKernel => "cpuKernel",
                BasicPerfSampleSeriesSampleSeriesLabel::CpuTotal => "cpuTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::CpuUser => "cpuUser",
                BasicPerfSampleSeriesSampleSeriesLabel::GraphicsFrameRate => "graphicsFrameRate",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssPrivate => "memoryRssPrivate",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssShared => "memoryRssShared",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssTotal => "memoryRssTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryTotal => "memoryTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::NetworkReceived => "networkReceived",
                BasicPerfSampleSeriesSampleSeriesLabel::NetworkSent => "networkSent",
                BasicPerfSampleSeriesSampleSeriesLabel::NtBytesReceived => "ntBytesReceived",
                BasicPerfSampleSeriesSampleSeriesLabel::NtBytesTransferred => "ntBytesTransferred",
                BasicPerfSampleSeriesSampleSeriesLabel::SampleSeriesTypeUnspecified => {
                    "sampleSeriesTypeUnspecified"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicPerfSampleSeriesSampleSeriesLabel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicPerfSampleSeriesSampleSeriesLabel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicPerfSampleSeriesSampleSeriesLabel, ()> {
            Ok(match s {
                "cpuKernel" => BasicPerfSampleSeriesSampleSeriesLabel::CpuKernel,
                "cpuTotal" => BasicPerfSampleSeriesSampleSeriesLabel::CpuTotal,
                "cpuUser" => BasicPerfSampleSeriesSampleSeriesLabel::CpuUser,
                "graphicsFrameRate" => BasicPerfSampleSeriesSampleSeriesLabel::GraphicsFrameRate,
                "memoryRssPrivate" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssPrivate,
                "memoryRssShared" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssShared,
                "memoryRssTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssTotal,
                "memoryTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryTotal,
                "networkReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkReceived,
                "networkSent" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkSent,
                "ntBytesReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesReceived,
                "ntBytesTransferred" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesTransferred,
                "sampleSeriesTypeUnspecified" => {
                    BasicPerfSampleSeriesSampleSeriesLabel::SampleSeriesTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesSampleSeriesLabel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesSampleSeriesLabel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesSampleSeriesLabel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpuKernel" => BasicPerfSampleSeriesSampleSeriesLabel::CpuKernel,
                "cpuTotal" => BasicPerfSampleSeriesSampleSeriesLabel::CpuTotal,
                "cpuUser" => BasicPerfSampleSeriesSampleSeriesLabel::CpuUser,
                "graphicsFrameRate" => BasicPerfSampleSeriesSampleSeriesLabel::GraphicsFrameRate,
                "memoryRssPrivate" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssPrivate,
                "memoryRssShared" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssShared,
                "memoryRssTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssTotal,
                "memoryTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryTotal,
                "networkReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkReceived,
                "networkSent" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkSent,
                "ntBytesReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesReceived,
                "ntBytesTransferred" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesTransferred,
                "sampleSeriesTypeUnspecified" => {
                    BasicPerfSampleSeriesSampleSeriesLabel::SampleSeriesTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for BasicPerfSampleSeriesSampleSeriesLabel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicPerfSampleSeriesSampleSeriesLabel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchCreatePerfSamplesRequest {
        #[doc = "The set of PerfSamples to create should not include existing timestamps"]
        #[serde(
            rename = "perfSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreatePerfSamplesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreatePerfSamplesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchCreatePerfSamplesResponse {
        #[serde(
            rename = "perfSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreatePerfSamplesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreatePerfSamplesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BlankScreen {
        #[doc = "The screen id of the element"]
        #[serde(
            rename = "screenId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BlankScreen {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BlankScreen {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cpuinfo {
        #[doc = "description of the device processor ie ‘1.8 GHz hexa core 64-bit ARMv8-A’"]
        #[serde(
            rename = "cpuProcessor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_processor: ::std::option::Option<String>,
        #[doc = "the CPU clock speed in GHz"]
        #[serde(
            rename = "cpuSpeedInGhz",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_speed_in_ghz: ::std::option::Option<f32>,
        #[doc = "the number of CPU cores"]
        #[serde(
            rename = "numberOfCores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_of_cores: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Cpuinfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cpuinfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CrashDialogError {
        #[doc = "The name of the package that caused the dialog."]
        #[serde(
            rename = "crashPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crash_package: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CrashDialogError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CrashDialogError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct DetectedAppSplashScreen {}
    impl ::google_field_selector::FieldSelector for DetectedAppSplashScreen {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DetectedAppSplashScreen {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct DeviceOutOfMemory {}
    impl ::google_field_selector::FieldSelector for DeviceOutOfMemory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceOutOfMemory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Duration {
        #[doc = "Signed fractions of a second at nanosecond resolution of the span of time. Durations less than one second are represented with a 0 `seconds` field and a positive or negative `nanos` field. For durations of one second or more, a non-zero value for the `nanos` field must be of the same sign as the `seconds` field. Must be from -999,999,999 to +999,999,999 inclusive."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Signed seconds of the span of time. Must be from -315,576,000,000 to +315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub seconds: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Duration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Duration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EncounteredLoginScreen {
        #[doc = "Number of encountered distinct login screens."]
        #[serde(
            rename = "distinctScreens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distinct_screens: ::std::option::Option<i32>,
        #[doc = "Subset of login screens."]
        #[serde(
            rename = "screenIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for EncounteredLoginScreen {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncounteredLoginScreen {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EncounteredNonAndroidUiWidgetScreen {
        #[doc = "Number of encountered distinct screens with non Android UI widgets."]
        #[serde(
            rename = "distinctScreens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distinct_screens: ::std::option::Option<i32>,
        #[doc = "Subset of screens which contain non Android UI widgets."]
        #[serde(
            rename = "screenIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for EncounteredNonAndroidUiWidgetScreen {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncounteredNonAndroidUiWidgetScreen {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Output only. The time when the Environment status was set to complete. This value will be set automatically when state transitions to COMPLETE."]
        #[serde(
            rename = "completionTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completion_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Output only. The time when the Environment was created."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Dimension values describing the environment. Dimension values always consist of “Model”, “Version”, “Locale”, and “Orientation”. - In response: always set - In create request: always set - In update request: never set"]
        #[serde(
            rename = "dimensionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_value:
            ::std::option::Option<Vec<crate::schemas::EnvironmentDimensionValueEntry>>,
        #[doc = "A short human-readable name to display in the UI. Maximum of 100 characters. For example: Nexus 5, API 27."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. An Environment id."]
        #[serde(
            rename = "environmentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment_id: ::std::option::Option<String>,
        #[doc = "Merged result of the environment."]
        #[serde(
            rename = "environmentResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment_result: ::std::option::Option<crate::schemas::MergedResult>,
        #[doc = "Output only. An Execution id."]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Output only. A History id."]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Output only. A Project id."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The location where output files are stored in the user bucket."]
        #[serde(
            rename = "resultsStorage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results_storage: ::std::option::Option<crate::schemas::ResultsStorage>,
        #[doc = "Output only. Summaries of shards. Only one shard will present unless sharding feature is enabled in TestExecutionService."]
        #[serde(
            rename = "shardSummaries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_summaries: ::std::option::Option<Vec<crate::schemas::ShardSummary>>,
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
    pub struct EnvironmentDimensionValueEntry {
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnvironmentDimensionValueEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvironmentDimensionValueEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Execution {
        #[doc = "The time when the Execution status transitioned to COMPLETE. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        #[serde(
            rename = "completionTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completion_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The time when the Execution was created. This value will be set automatically when CreateExecution is called. - In response: always set - In create/update request: never set"]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The dimensions along which different steps in this execution may vary. This must remain fixed over the life of the execution. Returns INVALID_ARGUMENT if this field is set in an update request. Returns INVALID_ARGUMENT if the same name occurs in more than one dimension_definition. Returns INVALID_ARGUMENT if the size of the list is over 100. - In response: present if set by create - In create request: optional - In update request: never set"]
        #[serde(
            rename = "dimensionDefinitions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_definitions:
            ::std::option::Option<Vec<crate::schemas::MatrixDimensionDefinition>>,
        #[doc = "A unique identifier within a History for this Execution. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create/update request: never set"]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Classify the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "outcome",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome: ::std::option::Option<crate::schemas::Outcome>,
        #[doc = "Lightweight information about execution request. - In response: present if set by create - In create: optional - In update: optional"]
        #[serde(
            rename = "specification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub specification: ::std::option::Option<crate::schemas::Specification>,
        #[doc = "The initial state is IN_PROGRESS. The only legal state transitions is from IN_PROGRESS to COMPLETE. A PRECONDITION_FAILED will be returned if an invalid transition is requested. The state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times. If the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE. - In response always set - In create/update request: optional"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ExecutionState>,
        #[doc = "TestExecution Matrix ID that the TestExecutionService uses. - In response: present if set by create - In create: optional - In update: never set"]
        #[serde(
            rename = "testExecutionMatrixId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_execution_matrix_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Execution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Execution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExecutionState {
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
    }
    impl ExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionState::Complete => "complete",
                ExecutionState::InProgress => "inProgress",
                ExecutionState::Pending => "pending",
                ExecutionState::UnknownState => "unknownState",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExecutionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExecutionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExecutionState, ()> {
            Ok(match s {
                "complete" => ExecutionState::Complete,
                "inProgress" => ExecutionState::InProgress,
                "pending" => ExecutionState::Pending,
                "unknownState" => ExecutionState::UnknownState,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "complete" => ExecutionState::Complete,
                "inProgress" => ExecutionState::InProgress,
                "pending" => ExecutionState::Pending,
                "unknownState" => ExecutionState::UnknownState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExecutionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecutionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FailedToInstall {}
    impl ::google_field_selector::FieldSelector for FailedToInstall {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FailedToInstall {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FailureDetail {
        #[doc = "If the failure was severe because the system (app) under test crashed."]
        #[serde(
            rename = "crashed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crashed: ::std::option::Option<bool>,
        #[doc = "If the device ran out of memory during a test, causing the test to crash."]
        #[serde(
            rename = "deviceOutOfMemory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_out_of_memory: ::std::option::Option<bool>,
        #[doc = "If the Roboscript failed to complete successfully, e.g., because a Roboscript action or assertion failed or a Roboscript action could not be matched during the entire crawl."]
        #[serde(
            rename = "failedRoboscript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failed_roboscript: ::std::option::Option<bool>,
        #[doc = "If an app is not installed and thus no test can be run with the app. This might be caused by trying to run a test on an unsupported platform."]
        #[serde(
            rename = "notInstalled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_installed: ::std::option::Option<bool>,
        #[doc = "If a native process (including any other than the app) crashed."]
        #[serde(
            rename = "otherNativeCrash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub other_native_crash: ::std::option::Option<bool>,
        #[doc = "If the test overran some time limit, and that is why it failed."]
        #[serde(
            rename = "timedOut",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timed_out: ::std::option::Option<bool>,
        #[doc = "If the robo was unable to crawl the app; perhaps because the app did not start."]
        #[serde(
            rename = "unableToCrawl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unable_to_crawl: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for FailureDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FailureDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FatalException {
        #[doc = "The stack trace of the fatal exception. Optional."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
    }
    impl ::google_field_selector::FieldSelector for FatalException {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FatalException {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The URI of a file stored in Google Cloud Storage. For example: http://storage.googleapis.com/mybucket/path/to/test.xml or in gsutil format: gs://mybucket/path/to/test.xml with version-specific info, gs://mybucket/path/to/test.xml#1360383693690000 An INVALID_ARGUMENT error will be returned if the URI format is not supported. - In response: always set - In create/update request: always set"]
        #[serde(
            rename = "fileUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_uri: ::std::option::Option<String>,
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
    pub struct GraphicsStats {
        #[doc = "Histogram of frame render times. There should be 154 buckets ranging from \\[5ms, 6ms) to \\[4950ms, infinity)"]
        #[serde(
            rename = "buckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buckets: ::std::option::Option<Vec<crate::schemas::GraphicsStatsBucket>>,
        #[doc = "Total “high input latency” events."]
        #[serde(
            rename = "highInputLatencyCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub high_input_latency_count: ::std::option::Option<i64>,
        #[doc = "Total frames with slow render time. Should be \\<= total_frames."]
        #[serde(
            rename = "jankyFrames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub janky_frames: ::std::option::Option<i64>,
        #[doc = "Total “missed vsync” events."]
        #[serde(
            rename = "missedVsyncCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub missed_vsync_count: ::std::option::Option<i64>,
        #[doc = "50th percentile frame render time in milliseconds."]
        #[serde(
            rename = "p50Millis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub p_50_millis: ::std::option::Option<i64>,
        #[doc = "90th percentile frame render time in milliseconds."]
        #[serde(
            rename = "p90Millis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub p_90_millis: ::std::option::Option<i64>,
        #[doc = "95th percentile frame render time in milliseconds."]
        #[serde(
            rename = "p95Millis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub p_95_millis: ::std::option::Option<i64>,
        #[doc = "99th percentile frame render time in milliseconds."]
        #[serde(
            rename = "p99Millis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub p_99_millis: ::std::option::Option<i64>,
        #[doc = "Total “slow bitmap upload” events."]
        #[serde(
            rename = "slowBitmapUploadCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slow_bitmap_upload_count: ::std::option::Option<i64>,
        #[doc = "Total “slow draw” events."]
        #[serde(
            rename = "slowDrawCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slow_draw_count: ::std::option::Option<i64>,
        #[doc = "Total “slow UI thread” events."]
        #[serde(
            rename = "slowUiThreadCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slow_ui_thread_count: ::std::option::Option<i64>,
        #[doc = "Total frames rendered by package."]
        #[serde(
            rename = "totalFrames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_frames: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GraphicsStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GraphicsStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GraphicsStatsBucket {
        #[doc = "Number of frames in the bucket."]
        #[serde(
            rename = "frameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub frame_count: ::std::option::Option<i64>,
        #[doc = "Lower bound of render time in milliseconds."]
        #[serde(
            rename = "renderMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub render_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GraphicsStatsBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GraphicsStatsBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct History {
        #[doc = "A short human-readable (plain text) name to display in the UI. Maximum of 100 characters. - In response: present if set during create. - In create request: optional"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A unique identifier within a project for this History. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create request: never set"]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "A name to uniquely identify a history within a project. Maximum of 200 characters. - In response always set - In create request: always set"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The platform of the test history. - In response: always set. Returns the platform of the last execution if unknown."]
        #[serde(
            rename = "testPlatform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_platform: ::std::option::Option<crate::schemas::HistoryTestPlatform>,
    }
    impl ::google_field_selector::FieldSelector for History {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for History {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HistoryTestPlatform {
        Android,
        Ios,
        UnknownPlatform,
    }
    impl HistoryTestPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                HistoryTestPlatform::Android => "android",
                HistoryTestPlatform::Ios => "ios",
                HistoryTestPlatform::UnknownPlatform => "unknownPlatform",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HistoryTestPlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HistoryTestPlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<HistoryTestPlatform, ()> {
            Ok(match s {
                "android" => HistoryTestPlatform::Android,
                "ios" => HistoryTestPlatform::Ios,
                "unknownPlatform" => HistoryTestPlatform::UnknownPlatform,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HistoryTestPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HistoryTestPlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HistoryTestPlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "android" => HistoryTestPlatform::Android,
                "ios" => HistoryTestPlatform::Ios,
                "unknownPlatform" => HistoryTestPlatform::UnknownPlatform,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HistoryTestPlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistoryTestPlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Image {
        #[doc = "An error explaining why the thumbnail could not be rendered."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "A reference to the full-size, original image. This is the same as the tool_outputs entry for the image under its Step. Always set."]
        #[serde(
            rename = "sourceImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_image: ::std::option::Option<crate::schemas::ToolOutputReference>,
        #[doc = "The step to which the image is attached. Always set."]
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
        #[doc = "The thumbnail."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<crate::schemas::Thumbnail>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InAppPurchasesFound {
        #[doc = "The total number of in-app purchases flows explored: how many times the robo tries to buy a SKU."]
        #[serde(
            rename = "inAppPurchasesFlowsExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub in_app_purchases_flows_explored: ::std::option::Option<i32>,
        #[doc = "The total number of in-app purchases flows started."]
        #[serde(
            rename = "inAppPurchasesFlowsStarted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub in_app_purchases_flows_started: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for InAppPurchasesFound {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InAppPurchasesFound {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InconclusiveDetail {
        #[doc = "If the end user aborted the test execution before a pass or fail could be determined. For example, the user pressed ctrl-c which sent a kill signal to the test runner while the test was running."]
        #[serde(
            rename = "abortedByUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aborted_by_user: ::std::option::Option<bool>,
        #[doc = "If results are being provided to the user in certain cases of infrastructure failures"]
        #[serde(
            rename = "hasErrorLogs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_error_logs: ::std::option::Option<bool>,
        #[doc = "If the test runner could not determine success or failure because the test depends on a component other than the system under test which failed. For example, a mobile test requires provisioning a device where the test executes, and that provisioning can fail."]
        #[serde(
            rename = "infrastructureFailure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub infrastructure_failure: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for InconclusiveDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InconclusiveDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IndividualOutcome {
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        #[serde(
            rename = "multistepNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multistep_number: ::std::option::Option<i32>,
        #[serde(
            rename = "outcomeSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome_summary: ::std::option::Option<crate::schemas::IndividualOutcomeOutcomeSummary>,
        #[doc = "How long it took for this step to run."]
        #[serde(
            rename = "runDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub run_duration: ::std::option::Option<crate::schemas::Duration>,
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IndividualOutcome {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndividualOutcome {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IndividualOutcomeOutcomeSummary {
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[doc = "Do not use. For proto versioning only."]
        Unset,
    }
    impl IndividualOutcomeOutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                IndividualOutcomeOutcomeSummary::Failure => "failure",
                IndividualOutcomeOutcomeSummary::Flaky => "flaky",
                IndividualOutcomeOutcomeSummary::Inconclusive => "inconclusive",
                IndividualOutcomeOutcomeSummary::Skipped => "skipped",
                IndividualOutcomeOutcomeSummary::Success => "success",
                IndividualOutcomeOutcomeSummary::Unset => "unset",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IndividualOutcomeOutcomeSummary {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IndividualOutcomeOutcomeSummary {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IndividualOutcomeOutcomeSummary, ()> {
            Ok(match s {
                "failure" => IndividualOutcomeOutcomeSummary::Failure,
                "flaky" => IndividualOutcomeOutcomeSummary::Flaky,
                "inconclusive" => IndividualOutcomeOutcomeSummary::Inconclusive,
                "skipped" => IndividualOutcomeOutcomeSummary::Skipped,
                "success" => IndividualOutcomeOutcomeSummary::Success,
                "unset" => IndividualOutcomeOutcomeSummary::Unset,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IndividualOutcomeOutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IndividualOutcomeOutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IndividualOutcomeOutcomeSummary {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => IndividualOutcomeOutcomeSummary::Failure,
                "flaky" => IndividualOutcomeOutcomeSummary::Flaky,
                "inconclusive" => IndividualOutcomeOutcomeSummary::Inconclusive,
                "skipped" => IndividualOutcomeOutcomeSummary::Skipped,
                "success" => IndividualOutcomeOutcomeSummary::Success,
                "unset" => IndividualOutcomeOutcomeSummary::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IndividualOutcomeOutcomeSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndividualOutcomeOutcomeSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct InsufficientCoverage {}
    impl ::google_field_selector::FieldSelector for InsufficientCoverage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsufficientCoverage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosAppCrashed {
        #[doc = "The stack trace, if one is available. Optional."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
    }
    impl ::google_field_selector::FieldSelector for IosAppCrashed {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosAppCrashed {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosAppInfo {
        #[doc = "The name of the app. Required"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IosAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosAppInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct IosRoboTest {}
    impl ::google_field_selector::FieldSelector for IosRoboTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosRoboTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosTest {
        #[doc = "Information about the application under test."]
        #[serde(
            rename = "iosAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_app_info: ::std::option::Option<crate::schemas::IosAppInfo>,
        #[doc = "An iOS Robo test."]
        #[serde(
            rename = "iosRoboTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_robo_test: ::std::option::Option<crate::schemas::IosRoboTest>,
        #[doc = "An iOS test loop."]
        #[serde(
            rename = "iosTestLoop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_test_loop: ::std::option::Option<crate::schemas::IosTestLoop>,
        #[doc = "An iOS XCTest."]
        #[serde(
            rename = "iosXcTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_xc_test: ::std::option::Option<crate::schemas::IosXcTest>,
        #[doc = "Max time a test is allowed to run before it is automatically cancelled."]
        #[serde(
            rename = "testTimeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_timeout: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::google_field_selector::FieldSelector for IosTest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosTest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Bundle ID of the app."]
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<String>,
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
    pub struct IosXcTest {
        #[doc = "Bundle ID of the app."]
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<String>,
        #[doc = "Xcode version that the test was run with."]
        #[serde(
            rename = "xcodeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xcode_version: ::std::option::Option<String>,
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
    pub struct LauncherActivityNotFound {}
    impl ::google_field_selector::FieldSelector for LauncherActivityNotFound {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LauncherActivityNotFound {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListEnvironmentsResponse {
        #[doc = "Environments. Always set."]
        #[serde(
            rename = "environments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environments: ::std::option::Option<Vec<crate::schemas::Environment>>,
        #[doc = "A Execution id Always set."]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "A History id. Always set."]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more Environments to fetch."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A Project id. Always set."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListEnvironmentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEnvironmentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListEnvironmentsResponse {
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
    pub struct ListExecutionsResponse {
        #[doc = "Executions. Always set."]
        #[serde(
            rename = "executions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub executions: ::std::option::Option<Vec<crate::schemas::Execution>>,
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more Executions to fetch."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListExecutionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListExecutionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListExecutionsResponse {
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
    pub struct ListHistoriesResponse {
        #[doc = "Histories."]
        #[serde(
            rename = "histories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histories: ::std::option::Option<Vec<crate::schemas::History>>,
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more histories to fetch. Tokens are valid for up to one hour from the time of the first list request. For instance, if you make a list request at 1PM and use the token from this first request 10 minutes later, the token from this second response will only be valid for 50 minutes."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListHistoriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListHistoriesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListHistoriesResponse {
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
    pub struct ListPerfSampleSeriesResponse {
        #[doc = "The resulting PerfSampleSeries sorted by id"]
        #[serde(
            rename = "perfSampleSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_sample_series: ::std::option::Option<Vec<crate::schemas::PerfSampleSeries>>,
    }
    impl ::google_field_selector::FieldSelector for ListPerfSampleSeriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPerfSampleSeriesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListPerfSamplesResponse {
        #[doc = "Optional, returned if result size exceeds the page size specified in the request (or the default page size, 500, if unspecified). It indicates the last sample timestamp to be used as page_token in subsequent request"]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "perfSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::google_field_selector::FieldSelector for ListPerfSamplesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPerfSamplesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListPerfSamplesResponse {
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
    pub struct ListScreenshotClustersResponse {
        #[doc = "The set of clusters associated with an execution Always set"]
        #[serde(
            rename = "clusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clusters: ::std::option::Option<Vec<crate::schemas::ScreenshotCluster>>,
    }
    impl ::google_field_selector::FieldSelector for ListScreenshotClustersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListScreenshotClustersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListStepAccessibilityClustersResponse {
        #[doc = "A sequence of accessibility suggestions, grouped into clusters. Within the sequence, clusters that belong to the same SuggestionCategory should be adjacent. Within each category, clusters should be ordered by their SuggestionPriority (ERRORs first). The categories should be ordered by their highest priority cluster."]
        #[serde(
            rename = "clusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clusters: ::std::option::Option<Vec<crate::schemas::SuggestionClusterProto>>,
        #[doc = "A full resource name of the step. For example, projects/my-project/histories/bh.1234567890abcdef/executions/ 1234567890123456789/steps/bs.1234567890abcdef Always presents."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListStepAccessibilityClustersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListStepAccessibilityClustersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListStepThumbnailsResponse {
        #[doc = "A continuation token to resume the query at the next item. If set, indicates that there are more thumbnails to read, by calling list again with this value in the page_token field."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of image data. Images are returned in a deterministic order; they are ordered by these factors, in order of importance: * First, by their associated test case. Images without a test case are considered greater than images with one. * Second, by their creation time. Images without a creation time are greater than images with one. * Third, by the order in which they were added to the step (by calls to CreateStep or UpdateStep)."]
        #[serde(
            rename = "thumbnails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnails: ::std::option::Option<Vec<crate::schemas::Image>>,
    }
    impl ::google_field_selector::FieldSelector for ListStepThumbnailsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListStepThumbnailsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListStepThumbnailsResponse {
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
    pub struct ListStepsResponse {
        #[doc = "A continuation token to resume the query at the next item. If set, indicates that there are more steps to read, by calling list again with this value in the page_token field."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Steps."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<Vec<crate::schemas::Step>>,
    }
    impl ::google_field_selector::FieldSelector for ListStepsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListStepsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListStepsResponse {
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
    pub struct ListTestCasesResponse {
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of test cases."]
        #[serde(
            rename = "testCases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_cases: ::std::option::Option<Vec<crate::schemas::TestCase>>,
    }
    impl ::google_field_selector::FieldSelector for ListTestCasesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListTestCasesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListTestCasesResponse {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LogcatCollectionError {}
    impl ::google_field_selector::FieldSelector for LogcatCollectionError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LogcatCollectionError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct MatrixDimensionDefinition {}
    impl ::google_field_selector::FieldSelector for MatrixDimensionDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatrixDimensionDefinition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Maximum memory that can be allocated to the process in KiB"]
        #[serde(
            rename = "memoryCapInKibibyte",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub memory_cap_in_kibibyte: ::std::option::Option<i64>,
        #[doc = "Total memory available on the device in KiB"]
        #[serde(
            rename = "memoryTotalInKibibyte",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub memory_total_in_kibibyte: ::std::option::Option<i64>,
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
    pub struct MergedResult {
        #[doc = "Outcome of the resource"]
        #[serde(
            rename = "outcome",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome: ::std::option::Option<crate::schemas::Outcome>,
        #[doc = "State of the resource"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::MergedResultState>,
        #[doc = "The combined and rolled-up result of each test suite that was run as part of this environment. Combining: When the test cases from a suite are run in different steps (sharding), the results are added back together in one overview. (e.g., if shard1 has 2 failures and shard2 has 1 failure than the overview failure_count = 3). Rollup: When test cases from the same suite are run multiple times (flaky), the results are combined (e.g., if testcase1.run1 fails, testcase1.run2 passes, and both testcase2.run1 and testcase2.run2 fail then the overview flaky_count = 1 and failure_count = 1)."]
        #[serde(
            rename = "testSuiteOverviews",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_suite_overviews: ::std::option::Option<Vec<crate::schemas::TestSuiteOverview>>,
    }
    impl ::google_field_selector::FieldSelector for MergedResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergedResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MergedResultState {
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
    }
    impl MergedResultState {
        pub fn as_str(self) -> &'static str {
            match self {
                MergedResultState::Complete => "complete",
                MergedResultState::InProgress => "inProgress",
                MergedResultState::Pending => "pending",
                MergedResultState::UnknownState => "unknownState",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MergedResultState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MergedResultState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MergedResultState, ()> {
            Ok(match s {
                "complete" => MergedResultState::Complete,
                "inProgress" => MergedResultState::InProgress,
                "pending" => MergedResultState::Pending,
                "unknownState" => MergedResultState::UnknownState,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MergedResultState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MergedResultState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MergedResultState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "complete" => MergedResultState::Complete,
                "inProgress" => MergedResultState::InProgress,
                "pending" => MergedResultState::Pending,
                "unknownState" => MergedResultState::UnknownState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MergedResultState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergedResultState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MultiStep {
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        #[serde(
            rename = "multistepNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multistep_number: ::std::option::Option<i32>,
        #[doc = "Present if it is a primary (original) step."]
        #[serde(
            rename = "primaryStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_step: ::std::option::Option<crate::schemas::PrimaryStep>,
        #[doc = "Step Id of the primary (original) step, which might be this step."]
        #[serde(
            rename = "primaryStepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MultiStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MultiStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NativeCrash {
        #[doc = "The stack trace of the native crash. Optional."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
    }
    impl ::google_field_selector::FieldSelector for NativeCrash {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NativeCrash {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonSdkApi {
        #[doc = "The signature of the Non-SDK API"]
        #[serde(
            rename = "apiSignature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_signature: ::std::option::Option<String>,
        #[doc = "Example stack traces of this API being called."]
        #[serde(
            rename = "exampleStackTraces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub example_stack_traces: ::std::option::Option<Vec<String>>,
        #[doc = "Optional debugging insights for non-SDK API violations."]
        #[serde(
            rename = "insights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insights: ::std::option::Option<Vec<crate::schemas::NonSdkApiInsight>>,
        #[doc = "The total number of times this API was observed to have been called."]
        #[serde(
            rename = "invocationCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invocation_count: ::std::option::Option<i32>,
        #[doc = "Which list this API appears on"]
        #[serde(
            rename = "list",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list: ::std::option::Option<crate::schemas::NonSdkApiList>,
    }
    impl ::google_field_selector::FieldSelector for NonSdkApi {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonSdkApi {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonSdkApiList {
        Black,
        Grey,
        GreyMaxO,
        GreyMaxP,
        GreyMaxQ,
        GreyMaxR,
        None,
        White,
    }
    impl NonSdkApiList {
        pub fn as_str(self) -> &'static str {
            match self {
                NonSdkApiList::Black => "BLACK",
                NonSdkApiList::Grey => "GREY",
                NonSdkApiList::GreyMaxO => "GREY_MAX_O",
                NonSdkApiList::GreyMaxP => "GREY_MAX_P",
                NonSdkApiList::GreyMaxQ => "GREY_MAX_Q",
                NonSdkApiList::GreyMaxR => "GREY_MAX_R",
                NonSdkApiList::None => "NONE",
                NonSdkApiList::White => "WHITE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NonSdkApiList {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NonSdkApiList {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NonSdkApiList, ()> {
            Ok(match s {
                "BLACK" => NonSdkApiList::Black,
                "GREY" => NonSdkApiList::Grey,
                "GREY_MAX_O" => NonSdkApiList::GreyMaxO,
                "GREY_MAX_P" => NonSdkApiList::GreyMaxP,
                "GREY_MAX_Q" => NonSdkApiList::GreyMaxQ,
                "GREY_MAX_R" => NonSdkApiList::GreyMaxR,
                "NONE" => NonSdkApiList::None,
                "WHITE" => NonSdkApiList::White,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NonSdkApiList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonSdkApiList {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonSdkApiList {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLACK" => NonSdkApiList::Black,
                "GREY" => NonSdkApiList::Grey,
                "GREY_MAX_O" => NonSdkApiList::GreyMaxO,
                "GREY_MAX_P" => NonSdkApiList::GreyMaxP,
                "GREY_MAX_Q" => NonSdkApiList::GreyMaxQ,
                "GREY_MAX_R" => NonSdkApiList::GreyMaxR,
                "NONE" => NonSdkApiList::None,
                "WHITE" => NonSdkApiList::White,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NonSdkApiList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonSdkApiList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonSdkApiInsight {
        #[doc = "Optional sample stack traces, for which this insight applies (there should be at least one)."]
        #[serde(
            rename = "exampleTraceMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub example_trace_messages: ::std::option::Option<Vec<String>>,
        #[doc = "A unique ID, to be used for determining the effectiveness of this particular insight in the context of a matcher. (required)"]
        #[serde(
            rename = "matcherId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matcher_id: ::std::option::Option<String>,
        #[doc = "An insight indicating that the hidden API usage originates from a Google-provided library."]
        #[serde(
            rename = "pendingGoogleUpdateInsight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_google_update_insight:
            ::std::option::Option<crate::schemas::PendingGoogleUpdateInsight>,
        #[doc = "An insight indicating that the hidden API usage originates from the use of a library that needs to be upgraded."]
        #[serde(
            rename = "upgradeInsight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade_insight: ::std::option::Option<crate::schemas::UpgradeInsight>,
    }
    impl ::google_field_selector::FieldSelector for NonSdkApiInsight {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonSdkApiInsight {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonSdkApiUsageViolation {
        #[doc = "Signatures of a subset of those hidden API’s."]
        #[serde(
            rename = "apiSignatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_signatures: ::std::option::Option<Vec<String>>,
        #[doc = "Total number of unique hidden API’s accessed."]
        #[serde(
            rename = "uniqueApis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_apis: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for NonSdkApiUsageViolation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonSdkApiUsageViolation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonSdkApiUsageViolationReport {
        #[doc = "Examples of the detected API usages."]
        #[serde(
            rename = "exampleApis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub example_apis: ::std::option::Option<Vec<crate::schemas::NonSdkApi>>,
        #[doc = "Minimum API level required for the application to run."]
        #[serde(
            rename = "minSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_sdk_version: ::std::option::Option<i32>,
        #[doc = "Specifies the API Level on which the application is designed to run."]
        #[serde(
            rename = "targetSdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_sdk_version: ::std::option::Option<i32>,
        #[doc = "Total number of unique Non-SDK API’s accessed."]
        #[serde(
            rename = "uniqueApis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_apis: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for NonSdkApiUsageViolationReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonSdkApiUsageViolationReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Outcome {
        #[doc = "More information about a FAILURE outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not FAILURE. Optional"]
        #[serde(
            rename = "failureDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_detail: ::std::option::Option<crate::schemas::FailureDetail>,
        #[doc = "More information about an INCONCLUSIVE outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not INCONCLUSIVE. Optional"]
        #[serde(
            rename = "inconclusiveDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inconclusive_detail: ::std::option::Option<crate::schemas::InconclusiveDetail>,
        #[doc = "More information about a SKIPPED outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not SKIPPED. Optional"]
        #[serde(
            rename = "skippedDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skipped_detail: ::std::option::Option<crate::schemas::SkippedDetail>,
        #[doc = "More information about a SUCCESS outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not SUCCESS. Optional"]
        #[serde(
            rename = "successDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub success_detail: ::std::option::Option<crate::schemas::SuccessDetail>,
        #[doc = "The simplest way to interpret a result. Required"]
        #[serde(
            rename = "summary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary: ::std::option::Option<crate::schemas::OutcomeSummary>,
    }
    impl ::google_field_selector::FieldSelector for Outcome {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Outcome {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutcomeSummary {
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[doc = "Do not use. For proto versioning only."]
        Unset,
    }
    impl OutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                OutcomeSummary::Failure => "failure",
                OutcomeSummary::Flaky => "flaky",
                OutcomeSummary::Inconclusive => "inconclusive",
                OutcomeSummary::Skipped => "skipped",
                OutcomeSummary::Success => "success",
                OutcomeSummary::Unset => "unset",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OutcomeSummary {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OutcomeSummary {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OutcomeSummary, ()> {
            Ok(match s {
                "failure" => OutcomeSummary::Failure,
                "flaky" => OutcomeSummary::Flaky,
                "inconclusive" => OutcomeSummary::Inconclusive,
                "skipped" => OutcomeSummary::Skipped,
                "success" => OutcomeSummary::Success,
                "unset" => OutcomeSummary::Unset,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutcomeSummary {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => OutcomeSummary::Failure,
                "flaky" => OutcomeSummary::Flaky,
                "inconclusive" => OutcomeSummary::Inconclusive,
                "skipped" => OutcomeSummary::Skipped,
                "success" => OutcomeSummary::Success,
                "unset" => OutcomeSummary::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OutcomeSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutcomeSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OverlappingUIElements {
        #[doc = "Resource names of the overlapping screen elements"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<Vec<String>>,
        #[doc = "The screen id of the elements"]
        #[serde(
            rename = "screenId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OverlappingUIElements {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OverlappingUIElements {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PendingGoogleUpdateInsight {
        #[doc = "The name of the Google-provided library with the non-SDK API dependency."]
        #[serde(
            rename = "nameOfGoogleLibrary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_of_google_library: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PendingGoogleUpdateInsight {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PendingGoogleUpdateInsight {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PerfEnvironment {
        #[doc = "CPU related environment info"]
        #[serde(
            rename = "cpuInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_info: ::std::option::Option<crate::schemas::Cpuinfo>,
        #[doc = "Memory related environment info"]
        #[serde(
            rename = "memoryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_info: ::std::option::Option<crate::schemas::MemoryInfo>,
    }
    impl ::google_field_selector::FieldSelector for PerfEnvironment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerfEnvironment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PerfMetricsSummary {
        #[serde(
            rename = "appStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_start_time: ::std::option::Option<crate::schemas::AppStartTime>,
        #[doc = "A tool results execution ID. @OutputOnly"]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Graphics statistics for the entire run. Statistics are reset at the beginning of the run and collected at the end of the run."]
        #[serde(
            rename = "graphicsStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub graphics_stats: ::std::option::Option<crate::schemas::GraphicsStats>,
        #[doc = "A tool results history ID. @OutputOnly"]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Describes the environment in which the performance metrics were collected"]
        #[serde(
            rename = "perfEnvironment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_environment: ::std::option::Option<crate::schemas::PerfEnvironment>,
        #[doc = "Set of resource collected"]
        #[serde(
            rename = "perfMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perf_metrics:
            ::std::option::Option<Vec<crate::schemas::PerfMetricsSummaryPerfMetricsItems>>,
        #[doc = "The cloud project @OutputOnly"]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "A tool results step ID. @OutputOnly"]
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PerfMetricsSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerfMetricsSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PerfMetricsSummaryPerfMetricsItems {
        Cpu,
        Graphics,
        Memory,
        Network,
        PerfMetricTypeUnspecified,
    }
    impl PerfMetricsSummaryPerfMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PerfMetricsSummaryPerfMetricsItems::Cpu => "cpu",
                PerfMetricsSummaryPerfMetricsItems::Graphics => "graphics",
                PerfMetricsSummaryPerfMetricsItems::Memory => "memory",
                PerfMetricsSummaryPerfMetricsItems::Network => "network",
                PerfMetricsSummaryPerfMetricsItems::PerfMetricTypeUnspecified => {
                    "perfMetricTypeUnspecified"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PerfMetricsSummaryPerfMetricsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PerfMetricsSummaryPerfMetricsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PerfMetricsSummaryPerfMetricsItems, ()> {
            Ok(match s {
                "cpu" => PerfMetricsSummaryPerfMetricsItems::Cpu,
                "graphics" => PerfMetricsSummaryPerfMetricsItems::Graphics,
                "memory" => PerfMetricsSummaryPerfMetricsItems::Memory,
                "network" => PerfMetricsSummaryPerfMetricsItems::Network,
                "perfMetricTypeUnspecified" => {
                    PerfMetricsSummaryPerfMetricsItems::PerfMetricTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PerfMetricsSummaryPerfMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PerfMetricsSummaryPerfMetricsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PerfMetricsSummaryPerfMetricsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpu" => PerfMetricsSummaryPerfMetricsItems::Cpu,
                "graphics" => PerfMetricsSummaryPerfMetricsItems::Graphics,
                "memory" => PerfMetricsSummaryPerfMetricsItems::Memory,
                "network" => PerfMetricsSummaryPerfMetricsItems::Network,
                "perfMetricTypeUnspecified" => {
                    PerfMetricsSummaryPerfMetricsItems::PerfMetricTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for PerfMetricsSummaryPerfMetricsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerfMetricsSummaryPerfMetricsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PerfSample {
        #[doc = "Timestamp of collection."]
        #[serde(
            rename = "sampleTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Value observed"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for PerfSample {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerfSample {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PerfSampleSeries {
        #[doc = "Basic series represented by a line chart"]
        #[serde(
            rename = "basicPerfSampleSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basic_perf_sample_series: ::std::option::Option<crate::schemas::BasicPerfSampleSeries>,
        #[doc = "A tool results execution ID. @OutputOnly"]
        #[serde(
            rename = "executionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "A tool results history ID. @OutputOnly"]
        #[serde(
            rename = "historyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub history_id: ::std::option::Option<String>,
        #[doc = "The cloud project @OutputOnly"]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "A sample series id @OutputOnly"]
        #[serde(
            rename = "sampleSeriesId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_series_id: ::std::option::Option<String>,
        #[doc = "A tool results step ID. @OutputOnly"]
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PerfSampleSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerfSampleSeries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct PerformedGoogleLogin {}
    impl ::google_field_selector::FieldSelector for PerformedGoogleLogin {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerformedGoogleLogin {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PerformedMonkeyActions {
        #[doc = "The total number of monkey actions performed during the crawl."]
        #[serde(
            rename = "totalActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_actions: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PerformedMonkeyActions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerformedMonkeyActions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PrimaryStep {
        #[doc = "Step Id and outcome of each individual step."]
        #[serde(
            rename = "individualOutcome",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub individual_outcome: ::std::option::Option<Vec<crate::schemas::IndividualOutcome>>,
        #[doc = "Rollup test status of multiple steps that were run with the same configuration as a group."]
        #[serde(
            rename = "rollUp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roll_up: ::std::option::Option<crate::schemas::PrimaryStepRollUp>,
    }
    impl ::google_field_selector::FieldSelector for PrimaryStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrimaryStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PrimaryStepRollUp {
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[doc = "Do not use. For proto versioning only."]
        Unset,
    }
    impl PrimaryStepRollUp {
        pub fn as_str(self) -> &'static str {
            match self {
                PrimaryStepRollUp::Failure => "failure",
                PrimaryStepRollUp::Flaky => "flaky",
                PrimaryStepRollUp::Inconclusive => "inconclusive",
                PrimaryStepRollUp::Skipped => "skipped",
                PrimaryStepRollUp::Success => "success",
                PrimaryStepRollUp::Unset => "unset",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PrimaryStepRollUp {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PrimaryStepRollUp {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PrimaryStepRollUp, ()> {
            Ok(match s {
                "failure" => PrimaryStepRollUp::Failure,
                "flaky" => PrimaryStepRollUp::Flaky,
                "inconclusive" => PrimaryStepRollUp::Inconclusive,
                "skipped" => PrimaryStepRollUp::Skipped,
                "success" => PrimaryStepRollUp::Success,
                "unset" => PrimaryStepRollUp::Unset,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PrimaryStepRollUp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PrimaryStepRollUp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryStepRollUp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => PrimaryStepRollUp::Failure,
                "flaky" => PrimaryStepRollUp::Flaky,
                "inconclusive" => PrimaryStepRollUp::Inconclusive,
                "skipped" => PrimaryStepRollUp::Skipped,
                "success" => PrimaryStepRollUp::Success,
                "unset" => PrimaryStepRollUp::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PrimaryStepRollUp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrimaryStepRollUp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ProjectSettings {
        #[doc = "The name of the Google Cloud Storage bucket to which results are written. By default, this is unset. In update request: optional In response: optional"]
        #[serde(
            rename = "defaultBucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_bucket: ::std::option::Option<String>,
        #[doc = "The name of the project’s settings. Always of the form: projects/{project-id}/settings In update request: never set In response: always set"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProjectSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PublishXunitXmlFilesRequest {
        #[doc = "URI of the Xunit XML files to publish. The maximum size of the file this reference is pointing to is 50MB. Required."]
        #[serde(
            rename = "xunitXmlFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xunit_xml_files: ::std::option::Option<Vec<crate::schemas::FileReference>>,
    }
    impl ::google_field_selector::FieldSelector for PublishXunitXmlFilesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublishXunitXmlFilesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RegionProto {
        #[doc = "The height, in pixels. Always set."]
        #[serde(
            rename = "heightPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_px: ::std::option::Option<i32>,
        #[doc = "The left side of the rectangle, in pixels. Always set."]
        #[serde(
            rename = "leftPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_px: ::std::option::Option<i32>,
        #[doc = "The top of the rectangle, in pixels. Always set."]
        #[serde(
            rename = "topPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_px: ::std::option::Option<i32>,
        #[doc = "The width, in pixels. Always set."]
        #[serde(
            rename = "widthPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_px: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for RegionProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegionProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultsStorage {
        #[doc = "The root directory for test results."]
        #[serde(
            rename = "resultsStoragePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results_storage_path: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The path to the Xunit XML file."]
        #[serde(
            rename = "xunitXmlFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xunit_xml_file: ::std::option::Option<crate::schemas::FileReference>,
    }
    impl ::google_field_selector::FieldSelector for ResultsStorage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultsStorage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RoboScriptExecution {
        #[doc = "The number of Robo script actions executed successfully."]
        #[serde(
            rename = "successfulActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub successful_actions: ::std::option::Option<i32>,
        #[doc = "The total number of actions in the Robo script."]
        #[serde(
            rename = "totalActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_actions: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for RoboScriptExecution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoboScriptExecution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SafeHtmlProto {
        #[doc = "IMPORTANT: Never set or read this field, even from tests, it is private. See documentation at the top of .proto file for programming language packages with which to create or read this message."]
        #[serde(
            rename = "privateDoNotAccessOrElseSafeHtmlWrappedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_do_not_access_or_else_safe_html_wrapped_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SafeHtmlProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SafeHtmlProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Screen {
        #[doc = "File reference of the png file. Required."]
        #[serde(
            rename = "fileReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_reference: ::std::option::Option<String>,
        #[doc = "Locale of the device that the screenshot was taken on. Required."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Model of the device that the screenshot was taken on. Required."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
        #[doc = "OS version of the device that the screenshot was taken on. Required."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Screen {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Screen {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ScreenshotCluster {
        #[doc = "A string that describes the activity of every screen in the cluster."]
        #[serde(
            rename = "activity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity: ::std::option::Option<String>,
        #[doc = "A unique identifier for the cluster. @OutputOnly"]
        #[serde(
            rename = "clusterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_id: ::std::option::Option<String>,
        #[doc = "A singular screen that represents the cluster as a whole. This screen will act as the “cover” of the entire cluster. When users look at the clusters, only the key screen from each cluster will be shown. Which screen is the key screen is determined by the ClusteringAlgorithm"]
        #[serde(
            rename = "keyScreen",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_screen: ::std::option::Option<crate::schemas::Screen>,
        #[doc = "Full list of screens."]
        #[serde(
            rename = "screens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screens: ::std::option::Option<Vec<crate::schemas::Screen>>,
    }
    impl ::google_field_selector::FieldSelector for ScreenshotCluster {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScreenshotCluster {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ShardSummary {
        #[doc = "Summaries of the steps belonging to the shard. With flaky_test_attempts enabled from TestExecutionService, more than one run (Step) can present. And the runs will be sorted by multistep_number."]
        #[serde(
            rename = "runs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runs: ::std::option::Option<Vec<crate::schemas::StepSummary>>,
        #[doc = "Merged result of the shard."]
        #[serde(
            rename = "shardResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_result: ::std::option::Option<crate::schemas::MergedResult>,
    }
    impl ::google_field_selector::FieldSelector for ShardSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShardSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SkippedDetail {
        #[doc = "If the App doesn’t support the specific API level."]
        #[serde(
            rename = "incompatibleAppVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incompatible_app_version: ::std::option::Option<bool>,
        #[doc = "If the App doesn’t run on the specific architecture, for example, x86."]
        #[serde(
            rename = "incompatibleArchitecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incompatible_architecture: ::std::option::Option<bool>,
        #[doc = "If the requested OS version doesn’t run on the specific device model."]
        #[serde(
            rename = "incompatibleDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incompatible_device: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SkippedDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SkippedDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Specification {
        #[doc = "An Android mobile test execution specification."]
        #[serde(
            rename = "androidTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_test: ::std::option::Option<crate::schemas::AndroidTest>,
        #[doc = "An iOS mobile test execution specification."]
        #[serde(
            rename = "iosTest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_test: ::std::option::Option<crate::schemas::IosTest>,
    }
    impl ::google_field_selector::FieldSelector for Specification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Specification {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StackTrace {
        #[doc = "The stack trace message. Required"]
        #[serde(
            rename = "exception",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exception: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StackTrace {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackTrace {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartActivityNotFound {
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StartActivityNotFound {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartActivityNotFound {
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
        #[doc = "The time when the step status was set to complete. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        #[serde(
            rename = "completionTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completion_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The time when the step was created. - In response: always set - In create/update request: never set"]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "A description of this tool For example: mvn clean package -D skipTests=true - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "How much the device resource is used to perform the test. This is the device usage used for billing purpose, which is different from the run_duration, for example, infrastructure failure won’t be charged for device usage. PRECONDITION_FAILED will be returned if one attempts to set a device_usage on a step which already has this field set. - In response: present if previously set. - In create request: optional - In update request: optional"]
        #[serde(
            rename = "deviceUsageDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_usage_duration: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "If the execution containing this step has any dimension_definition set, then this field allows the child to specify the values of the dimensions. The keys must exactly match the dimension_definition of the execution. For example, if the execution has `dimension_definition = ['attempt', 'device']` then a step must define values for those dimensions, eg. `dimension_value = ['attempt': '1', 'device': 'Nexus 6']` If a step does not participate in one dimension of the matrix, the value for that dimension should be empty string. For example, if one of the tests is executed by a runner which does not support retries, the step could have `dimension_value = ['attempt': '', 'device': 'Nexus 6']` If the step does not participate in any dimensions of the matrix, it may leave dimension_value unset. A PRECONDITION_FAILED will be returned if any of the keys do not exist in the dimension_definition of the execution. A PRECONDITION_FAILED will be returned if another step in this execution already has the same name and dimension_value, but differs on other data fields, for example, step field is different. A PRECONDITION_FAILED will be returned if dimension_value is set, and there is a dimension_definition in the execution which is not specified as one of the keys. - In response: present if set by create - In create request: optional - In update request: never set"]
        #[serde(
            rename = "dimensionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_value: ::std::option::Option<Vec<crate::schemas::StepDimensionValueEntry>>,
        #[doc = "Whether any of the outputs of this step are images whose thumbnails can be fetched with ListThumbnails. - In response: always set - In create/update request: never set"]
        #[serde(
            rename = "hasImages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_images: ::std::option::Option<bool>,
        #[doc = "Arbitrary user-supplied key/value pairs that are associated with the step. Users are responsible for managing the key namespace such that keys don’t accidentally collide. An INVALID_ARGUMENT will be returned if the number of labels exceeds 100 or if the length of any of the keys or values exceeds 100 characters. - In response: always set - In create request: optional - In update request: optional; any new key/value pair will be added to the map, and any new value for an existing key will update that key’s value"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::StepLabelsEntry>>,
        #[doc = "Details when multiple steps are run with the same configuration as a group. These details can be used identify which group this step is part of. It also identifies the groups ‘primary step’ which indexes all the group members. - In response: present if previously set. - In create request: optional, set iff this step was performed more than once. - In update request: optional"]
        #[serde(
            rename = "multiStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_step: ::std::option::Option<crate::schemas::MultiStep>,
        #[doc = "A short human-readable name to display in the UI. Maximum of 100 characters. For example: Clean build A PRECONDITION_FAILED will be returned upon creating a new step if it shares its name and dimension_value with an existing step. If two steps represent a similar action, but have different dimension values, they should share the same name. For instance, if the same set of tests is run on two different platforms, the two steps should have the same name. - In response: always set - In create request: always set - In update request: never set"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Classification of the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "outcome",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome: ::std::option::Option<crate::schemas::Outcome>,
        #[doc = "How long it took for this step to run. If unset, this is set to the difference between creation_time and completion_time when the step is set to the COMPLETE state. In some cases, it is appropriate to set this value separately: For instance, if a step is created, but the operation it represents is queued for a few minutes before it executes, it would be appropriate not to include the time spent queued in its run_duration. PRECONDITION_FAILED will be returned if one attempts to set a run_duration on a step which already has this field set. - In response: present if previously set; always present on COMPLETE step - In create request: optional - In update request: optional"]
        #[serde(
            rename = "runDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub run_duration: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE A PRECONDITION_FAILED will be returned if an invalid transition is requested. It is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times. - In response: always set - In create/update request: optional"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::StepState>,
        #[doc = "A unique identifier within a Execution for this Step. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response: always set - In create/update request: never set"]
        #[serde(
            rename = "stepId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step_id: ::std::option::Option<String>,
        #[doc = "An execution of a test runner."]
        #[serde(
            rename = "testExecutionStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_execution_step: ::std::option::Option<crate::schemas::TestExecutionStep>,
        #[doc = "An execution of a tool (used for steps we don’t explicitly support)."]
        #[serde(
            rename = "toolExecutionStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_execution_step: ::std::option::Option<crate::schemas::ToolExecutionStep>,
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
    pub enum StepState {
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
    }
    impl StepState {
        pub fn as_str(self) -> &'static str {
            match self {
                StepState::Complete => "complete",
                StepState::InProgress => "inProgress",
                StepState::Pending => "pending",
                StepState::UnknownState => "unknownState",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StepState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StepState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StepState, ()> {
            Ok(match s {
                "complete" => StepState::Complete,
                "inProgress" => StepState::InProgress,
                "pending" => StepState::Pending,
                "unknownState" => StepState::UnknownState,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StepState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StepState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StepState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "complete" => StepState::Complete,
                "inProgress" => StepState::InProgress,
                "pending" => StepState::Pending,
                "unknownState" => StepState::UnknownState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StepState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StepState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StepDimensionValueEntry {
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StepDimensionValueEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StepDimensionValueEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StepLabelsEntry {
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StepLabelsEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StepLabelsEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct StepSummary {}
    impl ::google_field_selector::FieldSelector for StepSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StepSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SuccessDetail {
        #[doc = "If a native process other than the app crashed."]
        #[serde(
            rename = "otherNativeCrash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub other_native_crash: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SuccessDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuccessDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestionClusterProto {
        #[doc = "Category in which these types of suggestions should appear. Always set."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<crate::schemas::SuggestionClusterProtoCategory>,
        #[doc = "A sequence of suggestions. All of the suggestions within a cluster must have the same SuggestionPriority and belong to the same SuggestionCategory. Suggestions with the same screenshot URL should be adjacent."]
        #[serde(
            rename = "suggestions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestions: ::std::option::Option<Vec<crate::schemas::SuggestionProto>>,
    }
    impl ::google_field_selector::FieldSelector for SuggestionClusterProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestionClusterProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SuggestionClusterProtoCategory {
        ContentLabeling,
        Implementation,
        LowContrast,
        TouchTargetSize,
        UnknownCategory,
    }
    impl SuggestionClusterProtoCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                SuggestionClusterProtoCategory::ContentLabeling => "contentLabeling",
                SuggestionClusterProtoCategory::Implementation => "implementation",
                SuggestionClusterProtoCategory::LowContrast => "lowContrast",
                SuggestionClusterProtoCategory::TouchTargetSize => "touchTargetSize",
                SuggestionClusterProtoCategory::UnknownCategory => "unknownCategory",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SuggestionClusterProtoCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SuggestionClusterProtoCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SuggestionClusterProtoCategory, ()> {
            Ok(match s {
                "contentLabeling" => SuggestionClusterProtoCategory::ContentLabeling,
                "implementation" => SuggestionClusterProtoCategory::Implementation,
                "lowContrast" => SuggestionClusterProtoCategory::LowContrast,
                "touchTargetSize" => SuggestionClusterProtoCategory::TouchTargetSize,
                "unknownCategory" => SuggestionClusterProtoCategory::UnknownCategory,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SuggestionClusterProtoCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuggestionClusterProtoCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuggestionClusterProtoCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "contentLabeling" => SuggestionClusterProtoCategory::ContentLabeling,
                "implementation" => SuggestionClusterProtoCategory::Implementation,
                "lowContrast" => SuggestionClusterProtoCategory::LowContrast,
                "touchTargetSize" => SuggestionClusterProtoCategory::TouchTargetSize,
                "unknownCategory" => SuggestionClusterProtoCategory::UnknownCategory,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SuggestionClusterProtoCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestionClusterProtoCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestionProto {
        #[doc = "Reference to a help center article concerning this type of suggestion. Always set."]
        #[serde(
            rename = "helpUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub help_url: ::std::option::Option<String>,
        #[doc = "Message, in the user’s language, explaining the suggestion, which may contain markup. Always set."]
        #[serde(
            rename = "longMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_message: ::std::option::Option<crate::schemas::SafeHtmlProto>,
        #[doc = "Relative importance of a suggestion. Always set."]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority: ::std::option::Option<crate::schemas::SuggestionProtoPriority>,
        #[doc = "A somewhat human readable identifier of the source view, if it does not have a resource_name. This is a path within the accessibility hierarchy, an element with resource name; similar to an XPath."]
        #[serde(
            rename = "pseudoResourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pseudo_resource_id: ::std::option::Option<String>,
        #[doc = "Region within the screenshot that is relevant to this suggestion. Optional."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<crate::schemas::RegionProto>,
        #[doc = "Reference to a view element, identified by its resource name, if it has one."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "ID of the screen for the suggestion. It is used for getting the corresponding screenshot path. For example, screen_id “1” corresponds to “1.png” file in GCS. Always set."]
        #[serde(
            rename = "screenId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_id: ::std::option::Option<String>,
        #[doc = "Relative importance of a suggestion as compared with other suggestions that have the same priority and category. This is a meaningless value that can be used to order suggestions that are in the same category and have the same priority. The larger values have higher priority (i.e., are more important). Optional."]
        #[serde(
            rename = "secondaryPriority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary_priority: ::std::option::Option<f64>,
        #[doc = "Concise message, in the user’s language, representing the suggestion, which may contain markup. Always set."]
        #[serde(
            rename = "shortMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_message: ::std::option::Option<crate::schemas::SafeHtmlProto>,
        #[doc = "General title for the suggestion, in the user’s language, without markup. Always set."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SuggestionProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestionProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SuggestionProtoPriority {
        Error,
        Info,
        UnknownPriority,
        Warning,
    }
    impl SuggestionProtoPriority {
        pub fn as_str(self) -> &'static str {
            match self {
                SuggestionProtoPriority::Error => "error",
                SuggestionProtoPriority::Info => "info",
                SuggestionProtoPriority::UnknownPriority => "unknownPriority",
                SuggestionProtoPriority::Warning => "warning",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SuggestionProtoPriority {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SuggestionProtoPriority {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SuggestionProtoPriority, ()> {
            Ok(match s {
                "error" => SuggestionProtoPriority::Error,
                "info" => SuggestionProtoPriority::Info,
                "unknownPriority" => SuggestionProtoPriority::UnknownPriority,
                "warning" => SuggestionProtoPriority::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SuggestionProtoPriority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuggestionProtoPriority {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuggestionProtoPriority {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "error" => SuggestionProtoPriority::Error,
                "info" => SuggestionProtoPriority::Info,
                "unknownPriority" => SuggestionProtoPriority::UnknownPriority,
                "warning" => SuggestionProtoPriority::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SuggestionProtoPriority {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestionProtoPriority {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestCase {
        #[doc = "The elapsed run time of the test case. Required."]
        #[serde(
            rename = "elapsedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elapsed_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The end time of the test case."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Why the test case was skipped. Present only for skipped test case"]
        #[serde(
            rename = "skippedMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skipped_message: ::std::option::Option<String>,
        #[doc = "The stack trace details if the test case failed or encountered an error. The maximum size of the stack traces is 100KiB, beyond which the stack track will be truncated. Zero if the test case passed."]
        #[serde(
            rename = "stackTraces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_traces: ::std::option::Option<Vec<crate::schemas::StackTrace>>,
        #[doc = "The start time of the test case."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The status of the test case. Required."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::TestCaseStatus>,
        #[doc = "A unique identifier within a Step for this Test Case."]
        #[serde(
            rename = "testCaseId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_case_id: ::std::option::Option<String>,
        #[doc = "Test case reference, e.g. name, class name and test suite name. Required."]
        #[serde(
            rename = "testCaseReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_case_reference: ::std::option::Option<crate::schemas::TestCaseReference>,
        #[doc = "References to opaque files of any format output by the tool execution. @OutputOnly"]
        #[serde(
            rename = "toolOutputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_outputs: ::std::option::Option<Vec<crate::schemas::ToolOutputReference>>,
    }
    impl ::google_field_selector::FieldSelector for TestCase {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestCase {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestCaseStatus {
        #[doc = "Test encountered an error"]
        Error,
        #[doc = "Test failed."]
        Failed,
        #[doc = "Test flaked. Present only for rollup test cases; test cases from steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
        #[doc = "Test passed."]
        Passed,
        #[doc = "Test skipped"]
        Skipped,
    }
    impl TestCaseStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                TestCaseStatus::Error => "error",
                TestCaseStatus::Failed => "failed",
                TestCaseStatus::Flaky => "flaky",
                TestCaseStatus::Passed => "passed",
                TestCaseStatus::Skipped => "skipped",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestCaseStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestCaseStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestCaseStatus, ()> {
            Ok(match s {
                "error" => TestCaseStatus::Error,
                "failed" => TestCaseStatus::Failed,
                "flaky" => TestCaseStatus::Flaky,
                "passed" => TestCaseStatus::Passed,
                "skipped" => TestCaseStatus::Skipped,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestCaseStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestCaseStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestCaseStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "error" => TestCaseStatus::Error,
                "failed" => TestCaseStatus::Failed,
                "flaky" => TestCaseStatus::Flaky,
                "passed" => TestCaseStatus::Passed,
                "skipped" => TestCaseStatus::Skipped,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestCaseStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestCaseStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestCaseReference {
        #[doc = "The name of the class."]
        #[serde(
            rename = "className",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub class_name: ::std::option::Option<String>,
        #[doc = "The name of the test case. Required."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The name of the test suite to which this test case belongs."]
        #[serde(
            rename = "testSuiteName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_suite_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TestCaseReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestCaseReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestExecutionStep {
        #[doc = "Issues observed during the test execution. For example, if the mobile app under test crashed during the test, the error message and the stack trace content can be recorded here to assist debugging. - In response: present if set by create or update - In create/update request: optional"]
        #[serde(
            rename = "testIssues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_issues: ::std::option::Option<Vec<crate::schemas::TestIssue>>,
        #[doc = "List of test suite overview contents. This could be parsed from xUnit XML log by server, or uploaded directly by user. This references should only be called when test suites are fully parsed or uploaded. The maximum allowed number of test suite overviews per step is 1000. - In response: always set - In create request: optional - In update request: never (use publishXunitXmlFiles custom method instead)"]
        #[serde(
            rename = "testSuiteOverviews",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_suite_overviews: ::std::option::Option<Vec<crate::schemas::TestSuiteOverview>>,
        #[doc = "The timing break down of the test execution. - In response: present if set by create or update - In create/update request: optional"]
        #[serde(
            rename = "testTiming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_timing: ::std::option::Option<crate::schemas::TestTiming>,
        #[doc = "Represents the execution of the test runner. The exit code of this tool will be used to determine if the test passed. - In response: always set - In create/update request: optional"]
        #[serde(
            rename = "toolExecution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_execution: ::std::option::Option<crate::schemas::ToolExecution>,
    }
    impl ::google_field_selector::FieldSelector for TestExecutionStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestExecutionStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIssue {
        #[doc = "Category of issue. Required."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<crate::schemas::TestIssueCategory>,
        #[doc = "A brief human-readable message describing the issue. Required."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Type of issue. Required."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::TestIssueType>,
        #[doc = "Severity of issue. Required."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::TestIssueSeverity>,
        #[doc = "Deprecated in favor of stack trace fields inside specific warnings."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
        #[doc = "Warning message with additional details of the issue. Should always be a message from com.google.devtools.toolresults.v1.warnings"]
        #[serde(
            rename = "warning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning: ::std::option::Option<crate::schemas::Any>,
    }
    impl ::google_field_selector::FieldSelector for TestIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIssue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueCategory {
        #[doc = "Issue is not specific to a particular test kind (e.g., a native crash)."]
        Common,
        #[doc = "Issue is specific to Robo run."]
        Robo,
        #[doc = "Default unspecified category. Do not use. For versioning only."]
        UnspecifiedCategory,
    }
    impl TestIssueCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueCategory::Common => "common",
                TestIssueCategory::Robo => "robo",
                TestIssueCategory::UnspecifiedCategory => "unspecifiedCategory",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestIssueCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestIssueCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestIssueCategory, ()> {
            Ok(match s {
                "common" => TestIssueCategory::Common,
                "robo" => TestIssueCategory::Robo,
                "unspecifiedCategory" => TestIssueCategory::UnspecifiedCategory,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestIssueCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "common" => TestIssueCategory::Common,
                "robo" => TestIssueCategory::Robo,
                "unspecifiedCategory" => TestIssueCategory::UnspecifiedCategory,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestIssueCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIssueCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueType {
        #[doc = "Issue is an ANR crash."]
        Anr,
        #[doc = "The app-under-test has deep links, but none were provided to Robo."]
        AvailableDeepLinks,
        #[doc = "Blank screen is found in the Robo crawl"]
        BlankScreen,
        #[doc = "Issue is a suggestion to use orchestrator."]
        CompatibleWithOrchestrator,
        #[doc = "A Robo script was fully and successfully executed."]
        CompleteRoboScriptExecution,
        #[doc = "Crash dialog was detected during the test execution"]
        CrashDialogError,
        #[doc = "Robo detected a splash screen provided by app (vs. Android OS splash screen)."]
        DetectedAppSplashScreen,
        #[doc = "Device running out of memory was detected"]
        DeviceOutOfMemory,
        #[doc = "Robo crawl encountered at least one probable login screen."]
        EncounteredLoginScreen,
        #[doc = "Robo crawl encountered at least one screen with elements that are not Android UI widgets."]
        EncounteredNonAndroidUiWidgetScreen,
        #[doc = "The APK failed to install."]
        FailedToInstall,
        #[doc = "Issue is a fatal exception."]
        FatalException,
        #[doc = "Robo crawl involved some in-app purchases."]
        InAppPurchases,
        #[doc = "A Robo script was not fully executed."]
        IncompleteRoboScriptExecution,
        #[doc = "Robo did not crawl some potentially important parts of the app."]
        InsufficientCoverage,
        #[doc = "iOS App crashed without an exception (e.g. killed)."]
        IosCrash,
        #[doc = "iOS App crashed with an exception."]
        IosException,
        #[doc = "Issue with finding a launcher activity"]
        LauncherActivityNotFound,
        #[doc = "Problems detected while collecting logcat"]
        LogcatCollectionError,
        #[doc = "Issue is a native crash."]
        NativeCrash,
        #[doc = "App accessed a non-sdk Api (new detailed report)"]
        NonSdkApiUsageReport,
        #[doc = "App accessed a non-sdk Api."]
        NonSdkApiUsageViolation,
        #[doc = "Overlapping UI elements are found in the Robo crawl"]
        OverlappingUiElements,
        #[doc = "Robo signed in with Google."]
        PerformedGoogleLogin,
        #[doc = "Robo crawl involved performing some monkey actions."]
        PerformedMonkeyActions,
        #[doc = "Issue with resolving a user-provided intent to start an activity"]
        StartActivityNotFound,
        #[doc = "UI element depth is greater than the threshold"]
        UiElementsTooDeep,
        #[doc = "An uncaught Unity exception was detected (these don’t crash apps)."]
        UnityException,
        #[doc = "Default unspecified type. Do not use. For versioning only."]
        UnspecifiedType,
        #[doc = "Issue is an unused robo directive."]
        UnusedRoboDirective,
        #[doc = "Robo crawl used a Robo directive."]
        UsedRoboDirective,
        #[doc = "Robo crawl used a Robo directive to ignore an UI element."]
        UsedRoboIgnoreDirective,
    }
    impl TestIssueType {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueType::Anr => "anr",
                TestIssueType::AvailableDeepLinks => "availableDeepLinks",
                TestIssueType::BlankScreen => "blankScreen",
                TestIssueType::CompatibleWithOrchestrator => "compatibleWithOrchestrator",
                TestIssueType::CompleteRoboScriptExecution => "completeRoboScriptExecution",
                TestIssueType::CrashDialogError => "crashDialogError",
                TestIssueType::DetectedAppSplashScreen => "detectedAppSplashScreen",
                TestIssueType::DeviceOutOfMemory => "deviceOutOfMemory",
                TestIssueType::EncounteredLoginScreen => "encounteredLoginScreen",
                TestIssueType::EncounteredNonAndroidUiWidgetScreen => {
                    "encounteredNonAndroidUiWidgetScreen"
                }
                TestIssueType::FailedToInstall => "failedToInstall",
                TestIssueType::FatalException => "fatalException",
                TestIssueType::InAppPurchases => "inAppPurchases",
                TestIssueType::IncompleteRoboScriptExecution => "incompleteRoboScriptExecution",
                TestIssueType::InsufficientCoverage => "insufficientCoverage",
                TestIssueType::IosCrash => "iosCrash",
                TestIssueType::IosException => "iosException",
                TestIssueType::LauncherActivityNotFound => "launcherActivityNotFound",
                TestIssueType::LogcatCollectionError => "logcatCollectionError",
                TestIssueType::NativeCrash => "nativeCrash",
                TestIssueType::NonSdkApiUsageReport => "nonSdkApiUsageReport",
                TestIssueType::NonSdkApiUsageViolation => "nonSdkApiUsageViolation",
                TestIssueType::OverlappingUiElements => "overlappingUiElements",
                TestIssueType::PerformedGoogleLogin => "performedGoogleLogin",
                TestIssueType::PerformedMonkeyActions => "performedMonkeyActions",
                TestIssueType::StartActivityNotFound => "startActivityNotFound",
                TestIssueType::UiElementsTooDeep => "uiElementsTooDeep",
                TestIssueType::UnityException => "unityException",
                TestIssueType::UnspecifiedType => "unspecifiedType",
                TestIssueType::UnusedRoboDirective => "unusedRoboDirective",
                TestIssueType::UsedRoboDirective => "usedRoboDirective",
                TestIssueType::UsedRoboIgnoreDirective => "usedRoboIgnoreDirective",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestIssueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestIssueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestIssueType, ()> {
            Ok(match s {
                "anr" => TestIssueType::Anr,
                "availableDeepLinks" => TestIssueType::AvailableDeepLinks,
                "blankScreen" => TestIssueType::BlankScreen,
                "compatibleWithOrchestrator" => TestIssueType::CompatibleWithOrchestrator,
                "completeRoboScriptExecution" => TestIssueType::CompleteRoboScriptExecution,
                "crashDialogError" => TestIssueType::CrashDialogError,
                "detectedAppSplashScreen" => TestIssueType::DetectedAppSplashScreen,
                "deviceOutOfMemory" => TestIssueType::DeviceOutOfMemory,
                "encounteredLoginScreen" => TestIssueType::EncounteredLoginScreen,
                "encounteredNonAndroidUiWidgetScreen" => {
                    TestIssueType::EncounteredNonAndroidUiWidgetScreen
                }
                "failedToInstall" => TestIssueType::FailedToInstall,
                "fatalException" => TestIssueType::FatalException,
                "inAppPurchases" => TestIssueType::InAppPurchases,
                "incompleteRoboScriptExecution" => TestIssueType::IncompleteRoboScriptExecution,
                "insufficientCoverage" => TestIssueType::InsufficientCoverage,
                "iosCrash" => TestIssueType::IosCrash,
                "iosException" => TestIssueType::IosException,
                "launcherActivityNotFound" => TestIssueType::LauncherActivityNotFound,
                "logcatCollectionError" => TestIssueType::LogcatCollectionError,
                "nativeCrash" => TestIssueType::NativeCrash,
                "nonSdkApiUsageReport" => TestIssueType::NonSdkApiUsageReport,
                "nonSdkApiUsageViolation" => TestIssueType::NonSdkApiUsageViolation,
                "overlappingUiElements" => TestIssueType::OverlappingUiElements,
                "performedGoogleLogin" => TestIssueType::PerformedGoogleLogin,
                "performedMonkeyActions" => TestIssueType::PerformedMonkeyActions,
                "startActivityNotFound" => TestIssueType::StartActivityNotFound,
                "uiElementsTooDeep" => TestIssueType::UiElementsTooDeep,
                "unityException" => TestIssueType::UnityException,
                "unspecifiedType" => TestIssueType::UnspecifiedType,
                "unusedRoboDirective" => TestIssueType::UnusedRoboDirective,
                "usedRoboDirective" => TestIssueType::UsedRoboDirective,
                "usedRoboIgnoreDirective" => TestIssueType::UsedRoboIgnoreDirective,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestIssueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "anr" => TestIssueType::Anr,
                "availableDeepLinks" => TestIssueType::AvailableDeepLinks,
                "blankScreen" => TestIssueType::BlankScreen,
                "compatibleWithOrchestrator" => TestIssueType::CompatibleWithOrchestrator,
                "completeRoboScriptExecution" => TestIssueType::CompleteRoboScriptExecution,
                "crashDialogError" => TestIssueType::CrashDialogError,
                "detectedAppSplashScreen" => TestIssueType::DetectedAppSplashScreen,
                "deviceOutOfMemory" => TestIssueType::DeviceOutOfMemory,
                "encounteredLoginScreen" => TestIssueType::EncounteredLoginScreen,
                "encounteredNonAndroidUiWidgetScreen" => {
                    TestIssueType::EncounteredNonAndroidUiWidgetScreen
                }
                "failedToInstall" => TestIssueType::FailedToInstall,
                "fatalException" => TestIssueType::FatalException,
                "inAppPurchases" => TestIssueType::InAppPurchases,
                "incompleteRoboScriptExecution" => TestIssueType::IncompleteRoboScriptExecution,
                "insufficientCoverage" => TestIssueType::InsufficientCoverage,
                "iosCrash" => TestIssueType::IosCrash,
                "iosException" => TestIssueType::IosException,
                "launcherActivityNotFound" => TestIssueType::LauncherActivityNotFound,
                "logcatCollectionError" => TestIssueType::LogcatCollectionError,
                "nativeCrash" => TestIssueType::NativeCrash,
                "nonSdkApiUsageReport" => TestIssueType::NonSdkApiUsageReport,
                "nonSdkApiUsageViolation" => TestIssueType::NonSdkApiUsageViolation,
                "overlappingUiElements" => TestIssueType::OverlappingUiElements,
                "performedGoogleLogin" => TestIssueType::PerformedGoogleLogin,
                "performedMonkeyActions" => TestIssueType::PerformedMonkeyActions,
                "startActivityNotFound" => TestIssueType::StartActivityNotFound,
                "uiElementsTooDeep" => TestIssueType::UiElementsTooDeep,
                "unityException" => TestIssueType::UnityException,
                "unspecifiedType" => TestIssueType::UnspecifiedType,
                "unusedRoboDirective" => TestIssueType::UnusedRoboDirective,
                "usedRoboDirective" => TestIssueType::UsedRoboDirective,
                "usedRoboIgnoreDirective" => TestIssueType::UsedRoboIgnoreDirective,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestIssueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIssueType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueSeverity {
        #[doc = "Non critical issue, providing users with some info about the test run."]
        Info,
        #[doc = "Critical issue."]
        Severe,
        #[doc = "Non critical issue, providing users with some hints on improving their testing experience, e.g., suggesting to use Game Loops."]
        Suggestion,
        #[doc = "Default unspecified severity. Do not use. For versioning only."]
        UnspecifiedSeverity,
        #[doc = "Potentially critical issue."]
        Warning,
    }
    impl TestIssueSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueSeverity::Info => "info",
                TestIssueSeverity::Severe => "severe",
                TestIssueSeverity::Suggestion => "suggestion",
                TestIssueSeverity::UnspecifiedSeverity => "unspecifiedSeverity",
                TestIssueSeverity::Warning => "warning",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestIssueSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestIssueSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestIssueSeverity, ()> {
            Ok(match s {
                "info" => TestIssueSeverity::Info,
                "severe" => TestIssueSeverity::Severe,
                "suggestion" => TestIssueSeverity::Suggestion,
                "unspecifiedSeverity" => TestIssueSeverity::UnspecifiedSeverity,
                "warning" => TestIssueSeverity::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestIssueSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "info" => TestIssueSeverity::Info,
                "severe" => TestIssueSeverity::Severe,
                "suggestion" => TestIssueSeverity::Suggestion,
                "unspecifiedSeverity" => TestIssueSeverity::UnspecifiedSeverity,
                "warning" => TestIssueSeverity::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestIssueSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIssueSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestSuiteOverview {
        #[doc = "Elapsed time of test suite."]
        #[serde(
            rename = "elapsedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elapsed_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "Number of test cases in error, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        #[serde(
            rename = "errorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_count: ::std::option::Option<i32>,
        #[doc = "Number of failed test cases, typically set by the service by parsing the xml_source. May also be set by the user. - In create/response: always set - In update request: never"]
        #[serde(
            rename = "failureCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_count: ::std::option::Option<i32>,
        #[doc = "Number of flaky test cases, set by the service by rolling up flaky test attempts. Present only for rollup test suite overview at environment level. A step cannot have flaky test cases."]
        #[serde(
            rename = "flakyCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flaky_count: ::std::option::Option<i32>,
        #[doc = "The name of the test suite. - In create/response: always set - In update request: never"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Number of test cases not run, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        #[serde(
            rename = "skippedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skipped_count: ::std::option::Option<i32>,
        #[doc = "Number of test cases, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        #[serde(
            rename = "totalCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_count: ::std::option::Option<i32>,
        #[doc = "If this test suite was parsed from XML, this is the URI where the original XML file is stored. Note: Multiple test suites can share the same xml_source Returns INVALID_ARGUMENT if the uri format is not supported. - In create/response: optional - In update request: never"]
        #[serde(
            rename = "xmlSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xml_source: ::std::option::Option<crate::schemas::FileReference>,
    }
    impl ::google_field_selector::FieldSelector for TestSuiteOverview {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestSuiteOverview {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestTiming {
        #[doc = "How long it took to run the test process. - In response: present if previously set. - In create/update request: optional"]
        #[serde(
            rename = "testProcessDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_process_duration: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::google_field_selector::FieldSelector for TestTiming {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestTiming {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Thumbnail {
        #[doc = "The thumbnail’s content type, i.e. “image/png”. Always set."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The thumbnail file itself. That is, the bytes here are precisely the bytes that make up the thumbnail file; they can be served as an image as-is (with the appropriate content type.) Always set."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The height of the thumbnail, in pixels. Always set."]
        #[serde(
            rename = "heightPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_px: ::std::option::Option<i32>,
        #[doc = "The width of the thumbnail, in pixels. Always set."]
        #[serde(
            rename = "widthPx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_px: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Thumbnail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Thumbnail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Timestamp {
        #[doc = "Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive."]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub seconds: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Timestamp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Timestamp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolExecution {
        #[doc = "The full tokenized command line including the program name (equivalent to argv in a C program). - In response: present if set by create request - In create request: optional - In update request: never set"]
        #[serde(
            rename = "commandLineArguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub command_line_arguments: ::std::option::Option<Vec<String>>,
        #[doc = "Tool execution exit code. This field will be set once the tool has exited. - In response: present if set by create/update request - In create request: optional - In update request: optional, a FAILED_PRECONDITION error will be returned if an exit_code is already set."]
        #[serde(
            rename = "exitCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exit_code: ::std::option::Option<crate::schemas::ToolExitCode>,
        #[doc = "References to any plain text logs output the tool execution. This field can be set before the tool has exited in order to be able to have access to a live view of the logs while the tool is running. The maximum allowed number of tool logs per step is 1000. - In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        #[serde(
            rename = "toolLogs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_logs: ::std::option::Option<Vec<crate::schemas::FileReference>>,
        #[doc = "References to opaque files of any format output by the tool execution. The maximum allowed number of tool outputs per step is 1000. - In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        #[serde(
            rename = "toolOutputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_outputs: ::std::option::Option<Vec<crate::schemas::ToolOutputReference>>,
    }
    impl ::google_field_selector::FieldSelector for ToolExecution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolExecution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolExecutionStep {
        #[doc = "A Tool execution. - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "toolExecution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tool_execution: ::std::option::Option<crate::schemas::ToolExecution>,
    }
    impl ::google_field_selector::FieldSelector for ToolExecutionStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolExecutionStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolExitCode {
        #[doc = "Tool execution exit code. A value of 0 means that the execution was successful. - In response: always set - In create/update request: always set"]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ToolExitCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolExitCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ToolOutputReference {
        #[doc = "The creation time of the file. - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "A FileReference to an output file. - In response: always set - In create/update request: always set"]
        #[serde(
            rename = "output",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The test case to which this output file belongs. - In response: present if set by create/update request - In create/update request: optional"]
        #[serde(
            rename = "testCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_case: ::std::option::Option<crate::schemas::TestCaseReference>,
    }
    impl ::google_field_selector::FieldSelector for ToolOutputReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ToolOutputReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UielementTooDeep {
        #[doc = "The depth of the screen element"]
        #[serde(
            rename = "depth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub depth: ::std::option::Option<i32>,
        #[doc = "The screen id of the element"]
        #[serde(
            rename = "screenId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_id: ::std::option::Option<String>,
        #[doc = "The screen state id of the element"]
        #[serde(
            rename = "screenStateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_state_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UielementTooDeep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UielementTooDeep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct UnspecifiedWarning {}
    impl ::google_field_selector::FieldSelector for UnspecifiedWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnspecifiedWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UnusedRoboDirective {
        #[doc = "The name of the resource that was unused."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UnusedRoboDirective {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnusedRoboDirective {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpgradeInsight {
        #[doc = "The name of the package to be upgraded."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The suggested version to upgrade to. Optional: In case we are not sure which version solves this problem"]
        #[serde(
            rename = "upgradeToVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade_to_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeInsight {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeInsight {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UsedRoboDirective {
        #[doc = "The name of the resource that was used."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsedRoboDirective {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsedRoboDirective {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UsedRoboIgnoreDirective {
        #[doc = "The name of the resource that was ignored."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsedRoboIgnoreDirective {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsedRoboIgnoreDirective {
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
            #[doc = "Gets the Tool Results settings for a project. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read from project"]
            pub fn get_settings(&self, project_id: impl Into<String>) -> GetSettingsRequestBuilder {
                GetSettingsRequestBuilder {
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
                }
            }
            #[doc = "Creates resources for settings which have not yet been set. Currently, this creates a single resource: a Google Cloud Storage bucket, to be used as the default bucket for this project. The bucket is created in an FTL-own storage project. Except for in rare cases, calling this method in parallel from multiple clients will only create a single bucket. In order to avoid unnecessary storage charges, the bucket is configured to automatically delete objects older than 90 days. The bucket is created with the following permissions: - Owner access for owners of central storage project (FTL-owned) - Writer access for owners/editors of customer project - Reader access for viewers of customer project The default ACL on objects created in the bucket is: - Owner access for owners of central storage project - Reader access for owners/editors/viewers of customer project See Google Cloud Storage documentation for more details. If there is already a default bucket set and the project can access the bucket, this call does nothing. However, if the project doesn’t have the permission to access the bucket or the bucket is deleted, a new bucket will be created. May return any canonical error codes, including the following: - PERMISSION_DENIED - if the user is not authorized to write to project - Any error code raised by Google Cloud Storage"]
            pub fn initialize_settings(
                &self,
                project_id: impl Into<String>,
            ) -> InitializeSettingsRequestBuilder {
                InitializeSettingsRequestBuilder {
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
                }
            }
            #[doc = "Actions that can be performed on the histories resource"]
            pub fn histories(&self) -> crate::resources::projects::histories::HistoriesActions {
                crate::resources::projects::histories::HistoriesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::get_settings()](struct.ProjectsActions.html#method.get_settings)"]
        #[derive(Debug, Clone)]
        pub struct GetSettingsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
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
        impl<'a> GetSettingsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ProjectSettings, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ProjectSettings, crate::Error> {
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
                let mut output = "https://toolresults.googleapis.com/".to_owned();
                output.push_str("toolresults/v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/settings");
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
        #[doc = "Created via [ProjectsActions::initialize_settings()](struct.ProjectsActions.html#method.initialize_settings)"]
        #[derive(Debug, Clone)]
        pub struct InitializeSettingsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
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
        impl<'a> InitializeSettingsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ProjectSettings, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ProjectSettings, crate::Error> {
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
                let mut output = "https://toolresults.googleapis.com/".to_owned();
                output.push_str("toolresults/v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":initializeSettings");
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
        pub mod histories {
            pub mod params {}
            pub struct HistoriesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> HistoriesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a History. The returned History will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing project does not exist"]
                pub fn create(
                    &self,
                    request: crate::schemas::History,
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
                #[doc = "Gets a History. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    history_id: impl Into<String>,
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
                        history_id: history_id.into(),
                    }
                }
                #[doc = "Lists Histories for a given Project. The histories are sorted by modification time in descending order. The history_id key will be used to order the history with the same modification time. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"]
                pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder {
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
                        project_id: project_id.into(),
                        filter_by_name: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the executions resource"]
                pub fn executions(
                    &self,
                ) -> crate::resources::projects::histories::executions::ExecutionsActions
                {
                    crate::resources::projects::histories::executions::ExecutionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [HistoriesActions::create()](struct.HistoriesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::History,
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
                #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
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
                ) -> Result<crate::schemas::History, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::History, crate::Error> {
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
                    let mut output = "https://toolresults.googleapis.com/".to_owned();
                    output.push_str("toolresults/v1beta3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories");
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
            #[doc = "Created via [HistoriesActions::get()](struct.HistoriesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_id: String,
                history_id: String,
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
                ) -> Result<crate::schemas::History, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::History, crate::Error> {
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
                    let mut output = "https://toolresults.googleapis.com/".to_owned();
                    output.push_str("toolresults/v1beta3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories/");
                    {
                        let var_as_str = &self.history_id;
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
            #[doc = "Created via [HistoriesActions::list()](struct.HistoriesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_id: String,
                filter_by_name: ::std::option::Option<String>,
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
                #[doc = "If set, only return histories with the given name. Optional."]
                pub fn filter_by_name(mut self, value: impl Into<String>) -> Self {
                    self.filter_by_name = Some(value.into());
                    self
                }
                #[doc = "The maximum number of Histories to fetch. Default value: 20. The server will use this default if the field is not set or has a value of 0. Any value greater than 100 will be treated as 100. Optional."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                #[doc = "\nExecute the request and yield each item in the `histories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_histories<T>(
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
                    self.stream_histories_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `histories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_histories_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::History, crate::Error>> + 'a
                {
                    self.stream_histories_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `histories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_histories_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::History, crate::Error>> + 'a
                {
                    self.stream_histories_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `histories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_histories_with_fields<T, F>(
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
                        #[serde(rename = "histories")]
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
                        let mut selector = concat!("nextPageToken,", "histories").to_owned();
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
                    Item = Result<crate::schemas::ListHistoriesResponse, crate::Error>,
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
                    Item = Result<crate::schemas::ListHistoriesResponse, crate::Error>,
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
                ) -> Result<crate::schemas::ListHistoriesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListHistoriesResponse, crate::Error> {
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
                    let mut output = "https://toolresults.googleapis.com/".to_owned();
                    output.push_str("toolresults/v1beta3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filterByName", &self.filter_by_name)]);
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
            pub mod executions {
                pub mod params {}
                pub struct ExecutionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ExecutionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates an Execution. The returned Execution will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"]
                    pub fn create(
                        &self,
                        request: crate::schemas::Execution,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
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
                            history_id: history_id.into(),
                            request_id: None,
                        }
                    }
                    #[doc = "Gets an Execution. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Execution does not exist"]
                    pub fn get(
                        &self,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                        execution_id: impl Into<String>,
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
                            history_id: history_id.into(),
                            execution_id: execution_id.into(),
                        }
                    }
                    #[doc = "Lists Executions for a given History. The executions are sorted by creation_time in descending order. The execution_id key will be used to order the executions with the same creation_time. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                    ) -> ListRequestBuilder {
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
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates an existing Execution with the supplied partial entity. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal - NOT_FOUND - if the containing History does not exist"]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Execution,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                        execution_id: impl Into<String>,
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
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            execution_id: execution_id.into(),
                            request_id: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the clusters resource"]
                    pub fn clusters(
                        &self,
                    ) -> crate::resources::projects::histories::executions::clusters::ClustersActions
                    {
                        crate :: resources :: projects :: histories :: executions :: clusters :: ClustersActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the environments resource"]                    pub fn environments (& self) -> crate :: resources :: projects :: histories :: executions :: environments :: EnvironmentsActions{
                        crate :: resources :: projects :: histories :: executions :: environments :: EnvironmentsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the steps resource"]
                    pub fn steps(
                        &self,
                    ) -> crate::resources::projects::histories::executions::steps::StepsActions
                    {
                        crate::resources::projects::histories::executions::steps::StepsActions {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                        }
                    }
                }
                #[doc = "Created via [ExecutionsActions::create()](struct.ExecutionsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Execution,
                    project_id: String,
                    history_id: String,
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
                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
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
                    ) -> Result<crate::schemas::Execution, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Execution, crate::Error> {
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
                        let mut output = "https://toolresults.googleapis.com/".to_owned();
                        output.push_str("toolresults/v1beta3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions");
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
                #[doc = "Created via [ExecutionsActions::get()](struct.ExecutionsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    project_id: String,
                    history_id: String,
                    execution_id: String,
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
                    ) -> Result<crate::schemas::Execution, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Execution, crate::Error> {
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
                        let mut output = "https://toolresults.googleapis.com/".to_owned();
                        output.push_str("toolresults/v1beta3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions/");
                        {
                            let var_as_str = &self.execution_id;
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
                #[doc = "Created via [ExecutionsActions::list()](struct.ExecutionsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    project_id: String,
                    history_id: String,
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
                    #[doc = "The maximum number of Executions to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0. Optional."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                    #[doc = "\nExecute the request and yield each item in the `executions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_executions<T>(
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
                        self.stream_executions_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `executions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_executions_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Execution, crate::Error>>
                           + 'a {
                        self.stream_executions_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `executions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_executions_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Execution, crate::Error>>
                           + 'a {
                        self.stream_executions_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `executions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_executions_with_fields<T, F>(
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
                            #[serde(rename = "executions")]
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
                            let mut selector = concat!("nextPageToken,", "executions").to_owned();
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
                        Item = Result<crate::schemas::ListExecutionsResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListExecutionsResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListExecutionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListExecutionsResponse, crate::Error>
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
                        let mut output = "https://toolresults.googleapis.com/".to_owned();
                        output.push_str("toolresults/v1beta3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions");
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
                #[doc = "Created via [ExecutionsActions::patch()](struct.ExecutionsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Execution,
                    project_id: String,
                    history_id: String,
                    execution_id: String,
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
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
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
                    ) -> Result<crate::schemas::Execution, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Execution, crate::Error> {
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
                        let mut output = "https://toolresults.googleapis.com/".to_owned();
                        output.push_str("toolresults/v1beta3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions/");
                        {
                            let var_as_str = &self.execution_id;
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
                        let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
                pub mod clusters {
                    pub mod params {}
                    pub struct ClustersActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ClustersActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Retrieves a single screenshot cluster by its ID"]
                        pub fn get(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            cluster_id: impl Into<String>,
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                cluster_id: cluster_id.into(),
                            }
                        }
                        #[doc = "Lists Screenshot Clusters Returns the list of screenshot clusters corresponding to an execution. Screenshot clusters are created after the execution is finished. Clusters are created from a set of screenshots. Between any two screenshots, a matching score is calculated based off their metadata that determines how similar they are. Screenshots are placed in the cluster that has screens which have the highest matching scores."]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> ListRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                            }
                        }
                    }
                    #[doc = "Created via [ClustersActions::get()](struct.ClustersActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        cluster_id: String,
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
                        ) -> Result<crate::schemas::ScreenshotCluster, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ScreenshotCluster, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/clusters/");
                            {
                                let var_as_str = &self.cluster_id;
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
                    #[doc = "Created via [ClustersActions::list()](struct.ClustersActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
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
                        ) -> Result<crate::schemas::ListScreenshotClustersResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListScreenshotClustersResponse, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/clusters");
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
                pub mod environments {
                    pub mod params {}
                    pub struct EnvironmentsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> EnvironmentsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Gets an Environment. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Environment does not exist"]
                        pub fn get(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            environment_id: impl Into<String>,
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                environment_id: environment_id.into(),
                            }
                        }
                        #[doc = "Lists Environments for a given Execution. The Environments are sorted by display name. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> ListRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [EnvironmentsActions::get()](struct.EnvironmentsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        environment_id: String,
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
                        ) -> Result<crate::schemas::Environment, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Environment, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/environments/");
                            {
                                let var_as_str = &self.environment_id;
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
                    #[doc = "Created via [EnvironmentsActions::list()](struct.EnvironmentsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
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
                        #[doc = "The maximum number of Environments to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A continuation token to resume the query at the next item."]
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
                        #[doc = "\nExecute the request and yield each item in the `environments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_environments<T>(
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
                            self.stream_environments_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `environments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_environments_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Environment, crate::Error>,
                        > + 'a {
                            self.stream_environments_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `environments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_environments_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Environment, crate::Error>,
                        > + 'a {
                            self.stream_environments_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `environments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_environments_with_fields<T, F>(
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
                                #[serde(rename = "environments")]
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
                                    concat!("nextPageToken,", "environments").to_owned();
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
                            Item = Result<crate::schemas::ListEnvironmentsResponse, crate::Error>,
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
                            Item = Result<crate::schemas::ListEnvironmentsResponse, crate::Error>,
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
                        ) -> Result<crate::schemas::ListEnvironmentsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListEnvironmentsResponse, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/environments");
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
                pub mod steps {
                    pub mod params {}
                    pub struct StepsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> StepsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Lists accessibility clusters for a given Step May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if an argument in the request happens to be invalid; e.g. if the locale format is incorrect - NOT_FOUND - if the containing Step does not exist"]
                        pub fn accessibility_clusters(
                            &self,
                            name: impl Into<String>,
                        ) -> AccessibilityClustersRequestBuilder {
                            AccessibilityClustersRequestBuilder {
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
                                locale: None,
                            }
                        }
                        #[doc = "Creates a Step. The returned Step will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn create(
                            &self,
                            request: crate::schemas::Step,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                request_id: None,
                            }
                        }
                        #[doc = "Gets a Step. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Step does not exist"]
                        pub fn get(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Retrieves a PerfMetricsSummary. May return any of the following error code(s): - NOT_FOUND - The specified PerfMetricsSummary does not exist"]
                        pub fn get_perf_metrics_summary(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> GetPerfMetricsSummaryRequestBuilder {
                            GetPerfMetricsSummaryRequestBuilder {
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Lists Steps for a given Execution. The steps are sorted by creation_time in descending order. The step_id key will be used to order the steps with the same creation_time. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if an argument in the request happens to be invalid; e.g. if an attempt is made to list the children of a nonexistent Step - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> ListRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Updates an existing Step with the supplied partial entity. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal (e.g try to upload a duplicate xml file), if the updated step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn patch(
                            &self,
                            request: crate::schemas::Step,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                                request_id: None,
                            }
                        }
                        #[doc = "Publish xml files to an existing Step. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal, e.g try to upload a duplicate xml file or a file too large. - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn publish_xunit_xml_files(
                            &self,
                            request: crate::schemas::PublishXunitXmlFilesRequest,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> PublishXunitXmlFilesRequestBuilder {
                            PublishXunitXmlFilesRequestBuilder {
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
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Actions that can be performed on the perf_metrics_summary resource"]                        pub fn perf_metrics_summary (& self) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_metrics_summary :: PerfMetricsSummaryActions{
                            crate :: resources :: projects :: histories :: executions :: steps :: perf_metrics_summary :: PerfMetricsSummaryActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the perf_sample_series resource"]                        pub fn perf_sample_series (& self) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: PerfSampleSeriesActions{
                            crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: PerfSampleSeriesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the test_cases resource"]                        pub fn test_cases (& self) -> crate :: resources :: projects :: histories :: executions :: steps :: test_cases :: TestCasesActions{
                            crate :: resources :: projects :: histories :: executions :: steps :: test_cases :: TestCasesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the thumbnails resource"]                        pub fn thumbnails (& self) -> crate :: resources :: projects :: histories :: executions :: steps :: thumbnails :: ThumbnailsActions{
                            crate :: resources :: projects :: histories :: executions :: steps :: thumbnails :: ThumbnailsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                    }
                    #[doc = "Created via [StepsActions::accessibility_clusters()](struct.StepsActions.html#method.accessibility_clusters)"]
                    #[derive(Debug, Clone)]
                    pub struct AccessibilityClustersRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        locale: ::std::option::Option<String>,
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
                    impl<'a> AccessibilityClustersRequestBuilder<'a> {
                        #[doc = "The accepted format is the canonical Unicode format with hyphen as a delimiter. Language must be lowercase, Language Script - Capitalized, Region - UPPERCASE. See http://www.unicode.org/reports/tr35/#Unicode_locale_identifier for details. Required."]
                        pub fn locale(mut self, value: impl Into<String>) -> Self {
                            self.locale = Some(value.into());
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
                            crate::schemas::ListStepAccessibilityClustersResponse,
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
                            crate::schemas::ListStepAccessibilityClustersResponse,
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":accessibilityClusters");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("locale", &self.locale)]);
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
                    #[doc = "Created via [StepsActions::create()](struct.StepsActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::Step,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
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
                        #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
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
                        ) -> Result<crate::schemas::Step, crate::Error> {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Step, crate::Error> {
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [StepsActions::get()](struct.StepsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
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
                        ) -> Result<crate::schemas::Step, crate::Error> {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Step, crate::Error> {
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
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
                    #[doc = "Created via [StepsActions::get_perf_metrics_summary()](struct.StepsActions.html#method.get_perf_metrics_summary)"]
                    #[derive(Debug, Clone)]
                    pub struct GetPerfMetricsSummaryRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
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
                    impl<'a> GetPerfMetricsSummaryRequestBuilder<'a> {
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
                        ) -> Result<crate::schemas::PerfMetricsSummary, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::PerfMetricsSummary, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/perfMetricsSummary");
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
                    #[doc = "Created via [StepsActions::list()](struct.StepsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
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
                        #[doc = "The maximum number of Steps to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0. Optional."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                        #[doc = "\nExecute the request and yield each item in the `steps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_steps<T>(
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
                            self.stream_steps_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `steps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_steps_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<Item = Result<crate::schemas::Step, crate::Error>> + 'a
                        {
                            self.stream_steps_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `steps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_steps_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<Item = Result<crate::schemas::Step, crate::Error>> + 'a
                        {
                            self.stream_steps_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `steps` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_steps_with_fields<T, F>(
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
                                #[serde(rename = "steps")]
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
                                let mut selector = concat!("nextPageToken,", "steps").to_owned();
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
                            Item = Result<crate::schemas::ListStepsResponse, crate::Error>,
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
                            Item = Result<crate::schemas::ListStepsResponse, crate::Error>,
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
                        ) -> Result<crate::schemas::ListStepsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListStepsResponse, crate::Error>
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps");
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
                    #[doc = "Created via [StepsActions::patch()](struct.StepsActions.html#method.patch)"]
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::Step,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
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
                    impl<'a> PatchRequestBuilder<'a> {
                        #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
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
                        ) -> Result<crate::schemas::Step, crate::Error> {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Step, crate::Error> {
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
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
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
                    #[doc = "Created via [StepsActions::publish_xunit_xml_files()](struct.StepsActions.html#method.publish_xunit_xml_files)"]
                    #[derive(Debug, Clone)]
                    pub struct PublishXunitXmlFilesRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::PublishXunitXmlFilesRequest,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
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
                    impl<'a> PublishXunitXmlFilesRequestBuilder<'a> {
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
                        ) -> Result<crate::schemas::Step, crate::Error> {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Step, crate::Error> {
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
                            let mut output = "https://toolresults.googleapis.com/".to_owned();
                            output.push_str("toolresults/v1beta3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str(":publishXunitXmlFiles");
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
                    pub mod perf_metrics_summary {
                        pub mod params {}
                        pub struct PerfMetricsSummaryActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> PerfMetricsSummaryActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Creates a PerfMetricsSummary resource. Returns the existing one if it has already been created. May return any of the following error code(s): - NOT_FOUND - The containing Step does not exist"]
                            pub fn create(
                                &self,
                                request: crate::schemas::PerfMetricsSummary,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
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
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                }
                            }
                        }
                        #[doc = "Created via [PerfMetricsSummaryActions::create()](struct.PerfMetricsSummaryActions.html#method.create)"]
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::PerfMetricsSummary,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
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
                            ) -> Result<crate::schemas::PerfMetricsSummary, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::PerfMetricsSummary, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfMetricsSummary");
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
                    pub mod perf_sample_series {
                        pub mod params {
                            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                            pub enum ListFilterItems {
                                Cpu,
                                Graphics,
                                Memory,
                                Network,
                                PerfMetricTypeUnspecified,
                            }
                            impl ListFilterItems {
                                pub fn as_str(self) -> &'static str {
                                    match self {
                                        ListFilterItems::Cpu => "cpu",
                                        ListFilterItems::Graphics => "graphics",
                                        ListFilterItems::Memory => "memory",
                                        ListFilterItems::Network => "network",
                                        ListFilterItems::PerfMetricTypeUnspecified => {
                                            "perfMetricTypeUnspecified"
                                        }
                                    }
                                }
                            }
                            impl ::std::convert::AsRef<str> for ListFilterItems {
                                fn as_ref(&self) -> &str {
                                    self.as_str()
                                }
                            }
                            impl ::std::str::FromStr for ListFilterItems {
                                type Err = ();
                                fn from_str(s: &str) -> ::std::result::Result<ListFilterItems, ()> {
                                    Ok(match s {
                                        "cpu" => ListFilterItems::Cpu,
                                        "graphics" => ListFilterItems::Graphics,
                                        "memory" => ListFilterItems::Memory,
                                        "network" => ListFilterItems::Network,
                                        "perfMetricTypeUnspecified" => {
                                            ListFilterItems::PerfMetricTypeUnspecified
                                        }
                                        _ => return Err(()),
                                    })
                                }
                            }
                            impl ::std::fmt::Display for ListFilterItems {
                                fn fmt(
                                    &self,
                                    f: &mut std::fmt::Formatter<'_>,
                                ) -> ::std::fmt::Result {
                                    f.write_str(self.as_str())
                                }
                            }
                            impl ::serde::Serialize for ListFilterItems {
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
                            impl<'de> ::serde::Deserialize<'de> for ListFilterItems {
                                fn deserialize<D>(
                                    deserializer: D,
                                ) -> ::std::result::Result<Self, D::Error>
                                where
                                    D: ::serde::de::Deserializer<'de>,
                                {
                                    let value: &'de str = <&str>::deserialize(deserializer)?;
                                    Ok(match value {
                                        "cpu" => ListFilterItems::Cpu,
                                        "graphics" => ListFilterItems::Graphics,
                                        "memory" => ListFilterItems::Memory,
                                        "network" => ListFilterItems::Network,
                                        "perfMetricTypeUnspecified" => {
                                            ListFilterItems::PerfMetricTypeUnspecified
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
                            impl ::google_field_selector::FieldSelector for ListFilterItems {
                                fn fields() -> Vec<::google_field_selector::Field> {
                                    Vec::new()
                                }
                            }
                            impl ::google_field_selector::ToFieldType for ListFilterItems {
                                fn field_type() -> ::google_field_selector::FieldType {
                                    ::google_field_selector::FieldType::Leaf
                                }
                            }
                        }
                        pub struct PerfSampleSeriesActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> PerfSampleSeriesActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Creates a PerfSampleSeries. May return any of the following error code(s): - ALREADY_EXISTS - PerfMetricSummary already exists for the given Step - NOT_FOUND - The containing Step does not exist"]
                            pub fn create(
                                &self,
                                request: crate::schemas::PerfSampleSeries,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
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
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                }
                            }
                            #[doc = "Gets a PerfSampleSeries. May return any of the following error code(s): - NOT_FOUND - The specified PerfSampleSeries does not exist"]
                            pub fn get(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                                sample_series_id: impl Into<String>,
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
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    sample_series_id: sample_series_id.into(),
                                }
                            }
                            #[doc = "Lists PerfSampleSeries for a given Step. The request provides an optional filter which specifies one or more PerfMetricsType to include in the result; if none returns all. The resulting PerfSampleSeries are sorted by ids. May return any of the following canonical error codes: - NOT_FOUND - The containing Step does not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder {
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
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    filter: None,
                                }
                            }
                            #[doc = "Actions that can be performed on the samples resource"]                            pub fn samples (& self) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: samples :: SamplesActions{
                                crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: samples :: SamplesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                            }
                        }
                        #[doc = "Created via [PerfSampleSeriesActions::create()](struct.PerfSampleSeriesActions.html#method.create)"]
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request: crate::schemas::PerfSampleSeries,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
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
                            ) -> Result<crate::schemas::PerfSampleSeries, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::PerfSampleSeries, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries");
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
                        #[doc = "Created via [PerfSampleSeriesActions::get()](struct.PerfSampleSeriesActions.html#method.get)"]
                        #[derive(Debug, Clone)]
                        pub struct GetRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            sample_series_id: String,
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
                            ) -> Result<crate::schemas::PerfSampleSeries, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::PerfSampleSeries, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries/");
                                {
                                    let var_as_str = &self.sample_series_id;
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
                        #[doc = "Created via [PerfSampleSeriesActions::list()](struct.PerfSampleSeriesActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , project_id : String , history_id : String , execution_id : String , step_id : String , filter : :: std :: option :: Option < Vec < crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: params :: ListFilterItems > > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
                        impl<'a> ListRequestBuilder<'a> {
                            #[doc = "Specify one or more PerfMetricType values such as CPU to filter the result"]
                            pub fn filter(
                                mut self,
                                value : impl Into < Vec < crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: params :: ListFilterItems > >,
                            ) -> Self {
                                self.filter = Some(value.into());
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
                            ) -> Result<crate::schemas::ListPerfSampleSeriesResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::ListPerfSampleSeriesResponse, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                for value in self.filter.iter().flatten() {
                                    req = req.query(&[("filter", value)]);
                                }
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
                        pub mod samples {
                            pub mod params {}
                            pub struct SamplesActions<'a> {
                                pub(crate) reqwest: &'a reqwest::Client,
                                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            }
                            impl<'a> SamplesActions<'a> {
                                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                    self.auth
                                }
                                #[doc = "Creates a batch of PerfSamples - a client can submit multiple batches of Perf Samples through repeated calls to this method in order to split up a large request payload - duplicates and existing timestamp entries will be ignored. - the batch operation may partially succeed - the set of elements successfully inserted is returned in the response (omits items which already existed in the database). May return any of the following canonical error codes: - NOT_FOUND - The containing PerfSampleSeries does not exist"]
                                pub fn batch_create(
                                    &self,
                                    request: crate::schemas::BatchCreatePerfSamplesRequest,
                                    project_id: impl Into<String>,
                                    history_id: impl Into<String>,
                                    execution_id: impl Into<String>,
                                    step_id: impl Into<String>,
                                    sample_series_id: impl Into<String>,
                                ) -> BatchCreateRequestBuilder {
                                    BatchCreateRequestBuilder {
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
                                        history_id: history_id.into(),
                                        execution_id: execution_id.into(),
                                        step_id: step_id.into(),
                                        sample_series_id: sample_series_id.into(),
                                    }
                                }
                                #[doc = "Lists the Performance Samples of a given Sample Series - The list results are sorted by timestamps ascending - The default page size is 500 samples; and maximum size allowed 5000 - The response token indicates the last returned PerfSample timestamp - When the results size exceeds the page size, submit a subsequent request including the page token to return the rest of the samples up to the page limit May return any of the following canonical error codes: - OUT_OF_RANGE - The specified request page_token is out of valid range - NOT_FOUND - The containing PerfSampleSeries does not exist"]
                                pub fn list(
                                    &self,
                                    project_id: impl Into<String>,
                                    history_id: impl Into<String>,
                                    execution_id: impl Into<String>,
                                    step_id: impl Into<String>,
                                    sample_series_id: impl Into<String>,
                                ) -> ListRequestBuilder {
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
                                        project_id: project_id.into(),
                                        history_id: history_id.into(),
                                        execution_id: execution_id.into(),
                                        step_id: step_id.into(),
                                        sample_series_id: sample_series_id.into(),
                                        page_size: None,
                                        page_token: None,
                                    }
                                }
                            }
                            #[doc = "Created via [SamplesActions::batch_create()](struct.SamplesActions.html#method.batch_create)"]
                            #[derive(Debug, Clone)]
                            pub struct BatchCreateRequestBuilder<'a> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                                request: crate::schemas::BatchCreatePerfSamplesRequest,
                                project_id: String,
                                history_id: String,
                                execution_id: String,
                                step_id: String,
                                sample_series_id: String,
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
                            impl<'a> BatchCreateRequestBuilder<'a> {
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
                                    let fields: ::std::option::Option<String> = if fields.is_empty()
                                    {
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
                                    crate::schemas::BatchCreatePerfSamplesResponse,
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
                                    crate::schemas::BatchCreatePerfSamplesResponse,
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
                                    let req = req.json(&self.request);
                                    Ok(req.send().await?.error_for_status()?.json().await?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://toolresults.googleapis.com/".to_owned();
                                    output.push_str("toolresults/v1beta3/projects/");
                                    {
                                        let var_as_str = &self.project_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/histories/");
                                    {
                                        let var_as_str = &self.history_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/executions/");
                                    {
                                        let var_as_str = &self.execution_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/steps/");
                                    {
                                        let var_as_str = &self.step_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/perfSampleSeries/");
                                    {
                                        let var_as_str = &self.sample_series_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/samples:batchCreate");
                                    output
                                }
                                async fn _request(
                                    &self,
                                    path: &str,
                                ) -> Result<::reqwest::RequestBuilder, crate::Error>
                                {
                                    let mut req =
                                        self.reqwest.request(::reqwest::Method::POST, path);
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
                            #[doc = "Created via [SamplesActions::list()](struct.SamplesActions.html#method.list)"]
                            #[derive(Debug, Clone)]
                            pub struct ListRequestBuilder<'a> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                                project_id: String,
                                history_id: String,
                                execution_id: String,
                                step_id: String,
                                sample_series_id: String,
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
                                #[doc = "The default page size is 500 samples, and the maximum size is 5000. If the page_size is greater than 5000, the effective page size will be 5000"]
                                pub fn page_size(mut self, value: i32) -> Self {
                                    self.page_size = Some(value);
                                    self
                                }
                                #[doc = "Optional, the next_page_token returned in the previous response"]
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
                                #[doc = "\nExecute the request and yield each item in the `perfSamples` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                                pub fn stream_perf_samples<T>(
                                    self,
                                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::google_field_selector::FieldSelector
                                        + 'a,
                                {
                                    let fields = ::google_field_selector::to_string::<T>();
                                    let fields: ::std::option::Option<String> = if fields.is_empty()
                                    {
                                        None
                                    } else {
                                        Some(fields)
                                    };
                                    self.stream_perf_samples_with_fields(fields)
                                }
                                #[doc = "\nExecute the request and yield each item in the `perfSamples` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                                pub fn stream_perf_samples_with_default_fields(
                                    self,
                                ) -> impl ::futures::Stream<
                                    Item = Result<crate::schemas::PerfSample, crate::Error>,
                                > + 'a {
                                    self.stream_perf_samples_with_fields(None::<String>)
                                }
                                #[doc = "\nExecute the request and yield each item in the `perfSamples` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                                pub fn stream_perf_samples_with_all_fields(
                                    self,
                                ) -> impl ::futures::Stream<
                                    Item = Result<crate::schemas::PerfSample, crate::Error>,
                                > + 'a {
                                    self.stream_perf_samples_with_fields(Some("*"))
                                }
                                #[doc = "\nExecute the request and yield each item in the `perfSamples` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                                pub fn stream_perf_samples_with_fields<T, F>(
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
                                        #[serde(rename = "perfSamples")]
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
                                            concat!("nextPageToken,", "perfSamples").to_owned();
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
                                    T: crate::GetNextPageToken<String>
                                        + ::serde::de::DeserializeOwned
                                        + ::google_field_selector::FieldSelector
                                        + 'a,
                                {
                                    let fields = ::google_field_selector::to_string::<T>();
                                    let fields: ::std::option::Option<String> = if fields.is_empty()
                                    {
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
                                        crate::schemas::ListPerfSamplesResponse,
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
                                        crate::schemas::ListPerfSamplesResponse,
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
                                    T: crate::GetNextPageToken<String>
                                        + ::serde::de::DeserializeOwned
                                        + 'a,
                                    F: AsRef<str>,
                                {
                                    let mut fields = fields
                                        .as_ref()
                                        .map(|x| x.as_ref())
                                        .unwrap_or("")
                                        .to_owned();
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
                                    let fields: ::std::option::Option<String> = if fields.is_empty()
                                    {
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
                                ) -> Result<crate::schemas::ListPerfSamplesResponse, crate::Error>
                                {
                                    self.execute_with_fields(None::<&str>).await
                                }
                                #[doc = r" Execute the given operation. This will provide a `fields`"]
                                #[doc = r" selector of `*`. This will include every attribute of the"]
                                #[doc = r" response resource and should be limited to use during"]
                                #[doc = r" development or debugging."]
                                pub async fn execute_with_all_fields(
                                    self,
                                ) -> Result<crate::schemas::ListPerfSamplesResponse, crate::Error>
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
                                    let mut output =
                                        "https://toolresults.googleapis.com/".to_owned();
                                    output.push_str("toolresults/v1beta3/projects/");
                                    {
                                        let var_as_str = &self.project_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/histories/");
                                    {
                                        let var_as_str = &self.history_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/executions/");
                                    {
                                        let var_as_str = &self.execution_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/steps/");
                                    {
                                        let var_as_str = &self.step_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/perfSampleSeries/");
                                    {
                                        let var_as_str = &self.sample_series_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/samples");
                                    output
                                }
                                async fn _request(
                                    &self,
                                    path: &str,
                                ) -> Result<::reqwest::RequestBuilder, crate::Error>
                                {
                                    let mut req =
                                        self.reqwest.request(::reqwest::Method::GET, path);
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
                                    T: crate::GetNextPageToken<String>
                                        + ::serde::de::DeserializeOwned,
                                {
                                    self._execute().await
                                }
                            }
                        }
                    }
                    pub mod test_cases {
                        pub mod params {}
                        pub struct TestCasesActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> TestCasesActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Gets details of a Test Case for a Step. Experimental test cases API. Still in active development. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Test Case does not exist"]
                            pub fn get(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                                test_case_id: impl Into<String>,
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
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    test_case_id: test_case_id.into(),
                                }
                            }
                            #[doc = "Lists Test Cases attached to a Step. Experimental test cases API. Still in active development. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Step does not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder {
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
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[doc = "Created via [TestCasesActions::get()](struct.TestCasesActions.html#method.get)"]
                        #[derive(Debug, Clone)]
                        pub struct GetRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            test_case_id: String,
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
                            ) -> Result<crate::schemas::TestCase, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::TestCase, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/testCases/");
                                {
                                    let var_as_str = &self.test_case_id;
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
                        #[doc = "Created via [TestCasesActions::list()](struct.TestCasesActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
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
                            #[doc = "The maximum number of TestCases to fetch. Default value: 100. The server will use this default if the field is not set or has a value of 0. Optional."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                            #[doc = "\nExecute the request and yield each item in the `testCases` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                            pub fn stream_test_cases<T>(
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
                                self.stream_test_cases_with_fields(fields)
                            }
                            #[doc = "\nExecute the request and yield each item in the `testCases` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                            pub fn stream_test_cases_with_default_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::TestCase, crate::Error>,
                            > + 'a {
                                self.stream_test_cases_with_fields(None::<String>)
                            }
                            #[doc = "\nExecute the request and yield each item in the `testCases` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                            pub fn stream_test_cases_with_all_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::TestCase, crate::Error>,
                            > + 'a {
                                self.stream_test_cases_with_fields(Some("*"))
                            }
                            #[doc = "\nExecute the request and yield each item in the `testCases` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                            pub fn stream_test_cases_with_fields<T, F>(
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
                                    #[serde(rename = "testCases")]
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
                                        concat!("nextPageToken,", "testCases").to_owned();
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
                                Item = Result<crate::schemas::ListTestCasesResponse, crate::Error>,
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
                                Item = Result<crate::schemas::ListTestCasesResponse, crate::Error>,
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
                                T: crate::GetNextPageToken<String>
                                    + ::serde::de::DeserializeOwned
                                    + 'a,
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
                            ) -> Result<crate::schemas::ListTestCasesResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::ListTestCasesResponse, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/testCases");
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
                    pub mod thumbnails {
                        pub mod params {}
                        pub struct ThumbnailsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> ThumbnailsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Lists thumbnails of images attached to a step. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read from the project, or from any of the images - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the step does not exist, or if any of the images do not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder {
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
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[doc = "Created via [ThumbnailsActions::list()](struct.ThumbnailsActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
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
                            #[doc = "The maximum number of thumbnails to fetch. Default value: 50. The server will use this default if the field is not set or has a value of 0. Optional."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                            #[doc = "\nExecute the request and yield each item in the `thumbnails` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                            pub fn stream_thumbnails<T>(
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
                                self.stream_thumbnails_with_fields(fields)
                            }
                            #[doc = "\nExecute the request and yield each item in the `thumbnails` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                            pub fn stream_thumbnails_with_default_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::Image, crate::Error>,
                            > + 'a {
                                self.stream_thumbnails_with_fields(None::<String>)
                            }
                            #[doc = "\nExecute the request and yield each item in the `thumbnails` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                            pub fn stream_thumbnails_with_all_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<crate::schemas::Image, crate::Error>,
                            > + 'a {
                                self.stream_thumbnails_with_fields(Some("*"))
                            }
                            #[doc = "\nExecute the request and yield each item in the `thumbnails` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                            pub fn stream_thumbnails_with_fields<T, F>(
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
                                    #[serde(rename = "thumbnails")]
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
                                        concat!("nextPageToken,", "thumbnails").to_owned();
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
                                    crate::schemas::ListStepThumbnailsResponse,
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
                                    crate::schemas::ListStepThumbnailsResponse,
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
                                T: crate::GetNextPageToken<String>
                                    + ::serde::de::DeserializeOwned
                                    + 'a,
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
                            ) -> Result<crate::schemas::ListStepThumbnailsResponse, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::ListStepThumbnailsResponse, crate::Error>
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
                                let mut output = "https://toolresults.googleapis.com/".to_owned();
                                output.push_str("toolresults/v1beta3/projects/");
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/thumbnails");
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
