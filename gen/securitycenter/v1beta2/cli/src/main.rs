use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("securitycenter1_beta2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230126")
            .about("Security Command Center API provides access to temporal views of assets and findings within an organization.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut folders0 = SubCommand::with_name("folders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_container_threat_detection_settings, get_event_threat_detection_settings, get_onboarding_state, get_rapid_vulnerability_detection_settings, get_security_center_settings, get_security_health_analytics_settings, get_virtual_machine_threat_detection_settings, get_web_security_scanner_settings, update_container_threat_detection_settings, update_event_threat_detection_settings, update_rapid_vulnerability_detection_settings, update_security_health_analytics_settings, update_virtual_machine_threat_detection_settings and update_web_security_scanner_settings");
        {
            let mcmd = SubCommand::with_name("get_container_threat_detection_settings").about("Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_event_threat_detection_settings").about("Get the EventThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetEventThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateEventThreatDetectionSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_onboarding_state")
                .about("Retrieve the OnboardingState of a resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rapid_vulnerability_detection_settings").about("Get the RapidVulnerabilityDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetRapidVulnerabilityDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateRapidVulnerabilityDetectionSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_center_settings")
                .about("Get the SecurityCenterSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_health_analytics_settings").about("Get the SecurityHealthAnalyticsSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetSecurityHealthAnalyticsSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateSecurityHealthAnalyticsSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_virtual_machine_threat_detection_settings").about("Get the VirtualMachineThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetVirtualMachineThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateVirtualMachineThreatDetectionSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_web_security_scanner_settings").about("Get the WebSecurityScannerSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetWebSecurityScannerSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateWebSecurityScannerSettings for this purpose.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_container_threat_detection_settings")
                .about("Update the ContainerThreatDetectionSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_event_threat_detection_settings")
                .about("Update the EventThreatDetectionSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_rapid_vulnerability_detection_settings")
                .about("Update the RapidVulnerabilityDetectionSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_security_health_analytics_settings")
                .about("Update the SecurityHealthAnalyticsSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_virtual_machine_threat_detection_settings")
                .about("Update the VirtualMachineThreatDetectionSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_web_security_scanner_settings")
                .about("Update the WebSecurityScannerSettings resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_container_threat_detection_settings, get_event_threat_detection_settings, get_onboarding_state, get_rapid_vulnerability_detection_settings, get_security_center_settings, get_security_health_analytics_settings, get_subscription, get_virtual_machine_threat_detection_settings, get_web_security_scanner_settings, update_container_threat_detection_settings, update_event_threat_detection_settings, update_rapid_vulnerability_detection_settings, update_security_health_analytics_settings, update_virtual_machine_threat_detection_settings and update_web_security_scanner_settings");
        {
            let mcmd = SubCommand::with_name("get_container_threat_detection_settings").about("Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_event_threat_detection_settings").about("Get the EventThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetEventThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateEventThreatDetectionSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_onboarding_state")
                .about("Retrieve the OnboardingState of a resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rapid_vulnerability_detection_settings").about("Get the RapidVulnerabilityDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetRapidVulnerabilityDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateRapidVulnerabilityDetectionSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_center_settings")
                .about("Get the SecurityCenterSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_health_analytics_settings").about("Get the SecurityHealthAnalyticsSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetSecurityHealthAnalyticsSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateSecurityHealthAnalyticsSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_subscription").about("Get the Subscription resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_virtual_machine_threat_detection_settings").about("Get the VirtualMachineThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetVirtualMachineThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateVirtualMachineThreatDetectionSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_web_security_scanner_settings").about("Get the WebSecurityScannerSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetWebSecurityScannerSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateWebSecurityScannerSettings for this purpose.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_container_threat_detection_settings")
                .about("Update the ContainerThreatDetectionSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_event_threat_detection_settings")
                .about("Update the EventThreatDetectionSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_rapid_vulnerability_detection_settings")
                .about("Update the RapidVulnerabilityDetectionSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_security_health_analytics_settings")
                .about("Update the SecurityHealthAnalyticsSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_virtual_machine_threat_detection_settings")
                .about("Update the VirtualMachineThreatDetectionSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_web_security_scanner_settings")
                .about("Update the WebSecurityScannerSettings resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_container_threat_detection_settings, get_event_threat_detection_settings, get_onboarding_state, get_rapid_vulnerability_detection_settings, get_security_center_settings, get_security_health_analytics_settings, get_virtual_machine_threat_detection_settings, get_web_security_scanner_settings, update_container_threat_detection_settings, update_event_threat_detection_settings, update_rapid_vulnerability_detection_settings, update_security_health_analytics_settings, update_virtual_machine_threat_detection_settings and update_web_security_scanner_settings");
        {
            let mcmd = SubCommand::with_name("get_container_threat_detection_settings").about("Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_event_threat_detection_settings").about("Get the EventThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetEventThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateEventThreatDetectionSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_onboarding_state")
                .about("Retrieve the OnboardingState of a resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rapid_vulnerability_detection_settings").about("Get the RapidVulnerabilityDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetRapidVulnerabilityDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateRapidVulnerabilityDetectionSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_center_settings")
                .about("Get the SecurityCenterSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_security_health_analytics_settings").about("Get the SecurityHealthAnalyticsSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetSecurityHealthAnalyticsSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateSecurityHealthAnalyticsSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_virtual_machine_threat_detection_settings").about("Get the VirtualMachineThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetVirtualMachineThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateVirtualMachineThreatDetectionSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_web_security_scanner_settings").about("Get the WebSecurityScannerSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetWebSecurityScannerSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateWebSecurityScannerSettings for this purpose.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_container_threat_detection_settings")
                .about("Update the ContainerThreatDetectionSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_event_threat_detection_settings")
                .about("Update the EventThreatDetectionSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_rapid_vulnerability_detection_settings")
                .about("Update the RapidVulnerabilityDetectionSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_security_health_analytics_settings")
                .about("Update the SecurityHealthAnalyticsSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_virtual_machine_threat_detection_settings")
                .about("Update the VirtualMachineThreatDetectionSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_web_security_scanner_settings")
                .about("Update the WebSecurityScannerSettings resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut container_threat_detection_settings1 =
            SubCommand::with_name("container_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective ContainerThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            container_threat_detection_settings1 =
                container_threat_detection_settings1.subcommand(mcmd);
        }
        let mut event_threat_detection_settings1 =
            SubCommand::with_name("event_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective EventThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            event_threat_detection_settings1 = event_threat_detection_settings1.subcommand(mcmd);
        }
        let mut rapid_vulnerability_detection_settings1 =
            SubCommand::with_name("rapid_vulnerability_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective RapidVulnerabilityDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            rapid_vulnerability_detection_settings1 =
                rapid_vulnerability_detection_settings1.subcommand(mcmd);
        }
        let mut security_health_analytics_settings1 =
            SubCommand::with_name("security_health_analytics_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective SecurityHealthAnalyticsSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            security_health_analytics_settings1 =
                security_health_analytics_settings1.subcommand(mcmd);
        }
        let mut virtual_machine_threat_detection_settings1 =
            SubCommand::with_name("virtual_machine_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective VirtualMachineThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            virtual_machine_threat_detection_settings1 =
                virtual_machine_threat_detection_settings1.subcommand(mcmd);
        }
        let mut web_security_scanner_settings1 =
            SubCommand::with_name("web_security_scanner_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective WebSecurityScannerSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            web_security_scanner_settings1 = web_security_scanner_settings1.subcommand(mcmd);
        }
        let mut container_threat_detection_settings1 =
            SubCommand::with_name("container_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective ContainerThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            container_threat_detection_settings1 =
                container_threat_detection_settings1.subcommand(mcmd);
        }
        let mut event_threat_detection_settings1 =
            SubCommand::with_name("event_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective EventThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            event_threat_detection_settings1 = event_threat_detection_settings1.subcommand(mcmd);
        }
        let mut rapid_vulnerability_detection_settings1 =
            SubCommand::with_name("rapid_vulnerability_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective RapidVulnerabilityDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            rapid_vulnerability_detection_settings1 =
                rapid_vulnerability_detection_settings1.subcommand(mcmd);
        }
        let mut security_health_analytics_settings1 =
            SubCommand::with_name("security_health_analytics_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective SecurityHealthAnalyticsSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            security_health_analytics_settings1 =
                security_health_analytics_settings1.subcommand(mcmd);
        }
        let mut virtual_machine_threat_detection_settings1 =
            SubCommand::with_name("virtual_machine_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective VirtualMachineThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            virtual_machine_threat_detection_settings1 =
                virtual_machine_threat_detection_settings1.subcommand(mcmd);
        }
        let mut web_security_scanner_settings1 =
            SubCommand::with_name("web_security_scanner_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective WebSecurityScannerSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            web_security_scanner_settings1 = web_security_scanner_settings1.subcommand(mcmd);
        }
        let mut container_threat_detection_settings1 =
            SubCommand::with_name("container_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective ContainerThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            container_threat_detection_settings1 =
                container_threat_detection_settings1.subcommand(mcmd);
        }
        let mut event_threat_detection_settings1 =
            SubCommand::with_name("event_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective EventThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            event_threat_detection_settings1 = event_threat_detection_settings1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: clusters");
        let mut rapid_vulnerability_detection_settings1 =
            SubCommand::with_name("rapid_vulnerability_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective RapidVulnerabilityDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            rapid_vulnerability_detection_settings1 =
                rapid_vulnerability_detection_settings1.subcommand(mcmd);
        }
        let mut security_health_analytics_settings1 =
            SubCommand::with_name("security_health_analytics_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective SecurityHealthAnalyticsSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            security_health_analytics_settings1 =
                security_health_analytics_settings1.subcommand(mcmd);
        }
        let mut virtual_machine_threat_detection_settings1 =
            SubCommand::with_name("virtual_machine_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective VirtualMachineThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            virtual_machine_threat_detection_settings1 =
                virtual_machine_threat_detection_settings1.subcommand(mcmd);
        }
        let mut web_security_scanner_settings1 =
            SubCommand::with_name("web_security_scanner_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective WebSecurityScannerSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            web_security_scanner_settings1 = web_security_scanner_settings1.subcommand(mcmd);
        }
        let mut clusters2 = SubCommand::with_name("clusters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_container_threat_detection_settings and update_container_threat_detection_settings");
        {
            let mcmd = SubCommand::with_name("get_container_threat_detection_settings").about("Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_container_threat_detection_settings")
                .about("Update the ContainerThreatDetectionSettings resource.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        let mut container_threat_detection_settings3 =
            SubCommand::with_name("container_threat_detection_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: calculate");
        {
            let mcmd = SubCommand::with_name("calculate").about("Calculates the effective ContainerThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.");
            container_threat_detection_settings3 =
                container_threat_detection_settings3.subcommand(mcmd);
        }
        clusters2 = clusters2.subcommand(container_threat_detection_settings3);
        locations1 = locations1.subcommand(clusters2);
        projects0 = projects0.subcommand(web_security_scanner_settings1);
        projects0 = projects0.subcommand(virtual_machine_threat_detection_settings1);
        projects0 = projects0.subcommand(security_health_analytics_settings1);
        projects0 = projects0.subcommand(rapid_vulnerability_detection_settings1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(event_threat_detection_settings1);
        projects0 = projects0.subcommand(container_threat_detection_settings1);
        organizations0 = organizations0.subcommand(web_security_scanner_settings1);
        organizations0 = organizations0.subcommand(virtual_machine_threat_detection_settings1);
        organizations0 = organizations0.subcommand(security_health_analytics_settings1);
        organizations0 = organizations0.subcommand(rapid_vulnerability_detection_settings1);
        organizations0 = organizations0.subcommand(event_threat_detection_settings1);
        organizations0 = organizations0.subcommand(container_threat_detection_settings1);
        folders0 = folders0.subcommand(web_security_scanner_settings1);
        folders0 = folders0.subcommand(virtual_machine_threat_detection_settings1);
        folders0 = folders0.subcommand(security_health_analytics_settings1);
        folders0 = folders0.subcommand(rapid_vulnerability_detection_settings1);
        folders0 = folders0.subcommand(event_threat_detection_settings1);
        folders0 = folders0.subcommand(container_threat_detection_settings1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_securitycenter1_beta2 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
