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
        let mut app = App::new("firebaseappcheck1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220415")
            .about("Firebase App Check works alongside other Firebase services to help protect your backend resources from abuse, such as billing fraud or phishing.")
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
        let mut jwks0 = SubCommand::with_name("jwks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a public JWK set as specified by [RFC 7517](https://tools.ietf.org/html/rfc7517) that can be used to verify App Check tokens. Exactly one of the public keys in the returned set will successfully validate any App Check token that is currently valid.");
            jwks0 = jwks0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: apps and services");
        let mut apps1 = SubCommand::with_name("apps")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: exchange_app_attest_assertion, exchange_app_attest_attestation, exchange_custom_token, exchange_debug_token, exchange_device_check_token, exchange_recaptcha_enterprise_token, exchange_recaptcha_token, exchange_recaptcha_v3_token, exchange_safety_net_token and generate_app_attest_challenge");
        {
            let mcmd = SubCommand::with_name("exchange_app_attest_assertion").about("Accepts an App Attest assertion and an artifact previously obtained from ExchangeAppAttestAttestation and verifies those with Apple. If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_app_attest_attestation").about("Accepts an App Attest CBOR attestation and verifies it with Apple using your preconfigured team and bundle IDs. If valid, returns an attestation artifact that can later be exchanged for an AppCheckToken using ExchangeAppAttestAssertion. For convenience and performance, this method's response object will also contain an AppCheckToken (if the verification is successful).");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_custom_token").about("Validates a custom token signed using your project's Admin SDK service account credentials. If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_debug_token").about("Validates a debug token secret that you have previously created using CreateDebugToken. If valid, returns an AppCheckToken. Note that a restrictive quota is enforced on this method to prevent accidental exposure of the app to abuse.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_device_check_token").about("Accepts a [`device_token`](https://developer.apple.com/documentation/devicecheck/dcdevice) issued by DeviceCheck, and attempts to validate it with Apple. If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_recaptcha_enterprise_token").about("Validates a [reCAPTCHA Enterprise response token](https://cloud.google.com/recaptcha-enterprise/docs/create-assessment#retrieve_token). If valid, returns an App Check token AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_recaptcha_token").about("Validates a [reCAPTCHA v3 response token](https://developers.google.com/recaptcha/docs/v3). If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_recaptcha_v3_token").about("Validates a [reCAPTCHA v3 response token](https://developers.google.com/recaptcha/docs/v3). If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("exchange_safety_net_token").about("Validates a [SafetyNet token](https://developer.android.com/training/safetynet/attestation#request-attestation-step). If valid, returns an AppCheckToken.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_app_attest_challenge").about("Generates a challenge that protects the integrity of an immediately following call to ExchangeAppAttestAttestation or ExchangeAppAttestAssertion. A challenge should not be reused for multiple calls.");
            apps1 = apps1.subcommand(mcmd);
        }
        let mut services1 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_update")
                .about("Atomically updates the specified Service configurations.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the Service configuration for the specified service name.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all Service configurations for the specified project. Only Services which were explicitly configured using UpdateService or BatchUpdateServices will be returned.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the specified Service configuration.");
            services1 = services1.subcommand(mcmd);
        }
        let mut app_attest_config2 = SubCommand::with_name("app_attest_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get")
                .about("Atomically gets the AppAttestConfigs for the specified list of apps.");
            app_attest_config2 = app_attest_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the AppAttestConfig for the specified app.");
            app_attest_config2 = app_attest_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the AppAttestConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange AppAttest tokens for App Check tokens.");
            app_attest_config2 = app_attest_config2.subcommand(mcmd);
        }
        let mut debug_tokens2 = SubCommand::with_name("debug_tokens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new DebugToken for the specified app. For security reasons, after the creation operation completes, the `token` field cannot be updated or retrieved, but you can revoke the debug token using DeleteDebugToken. Each app can have a maximum of 20 debug tokens.");
            debug_tokens2 = debug_tokens2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified DebugToken. A deleted debug token cannot be used to exchange for an App Check token. Use this method when you suspect the secret `token` has been compromised or when you no longer need the debug token.");
            debug_tokens2 = debug_tokens2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified DebugToken. For security reasons, the `token` field is never populated in the response.");
            debug_tokens2 = debug_tokens2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all DebugTokens for the specified app. For security reasons, the `token` field is never populated in the response.");
            debug_tokens2 = debug_tokens2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified DebugToken. For security reasons, the `token` field cannot be updated, nor will it be populated in the response, but you can revoke the debug token using DeleteDebugToken.");
            debug_tokens2 = debug_tokens2.subcommand(mcmd);
        }
        let mut device_check_config2 = SubCommand::with_name("device_check_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Atomically gets the DeviceCheckConfigs for the specified list of apps. For security reasons, the `private_key` field is never populated in the response.");
            device_check_config2 = device_check_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the DeviceCheckConfig for the specified app. For security reasons, the `private_key` field is never populated in the response.");
            device_check_config2 = device_check_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeviceCheckConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange DeviceCheck tokens for App Check tokens. For security reasons, the `private_key` field is never populated in the response.");
            device_check_config2 = device_check_config2.subcommand(mcmd);
        }
        let mut recaptcha_config2 = SubCommand::with_name("recaptcha_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Atomically gets the RecaptchaConfigs for the specified list of apps. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_config2 = recaptcha_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the RecaptchaConfig for the specified app. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_config2 = recaptcha_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the RecaptchaConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA tokens for App Check tokens. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_config2 = recaptcha_config2.subcommand(mcmd);
        }
        let mut recaptcha_enterprise_config2 = SubCommand::with_name("recaptcha_enterprise_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get").about(
                "Atomically gets the RecaptchaEnterpriseConfigs for the specified list of apps.",
            );
            recaptcha_enterprise_config2 = recaptcha_enterprise_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the RecaptchaEnterpriseConfig for the specified app.");
            recaptcha_enterprise_config2 = recaptcha_enterprise_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the RecaptchaEnterpriseConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA Enterprise tokens for App Check tokens.");
            recaptcha_enterprise_config2 = recaptcha_enterprise_config2.subcommand(mcmd);
        }
        let mut recaptcha_v3_config2 = SubCommand::with_name("recaptcha_v3_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Atomically gets the RecaptchaV3Configs for the specified list of apps. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_v3_config2 = recaptcha_v3_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the RecaptchaV3Config for the specified app. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_v3_config2 = recaptcha_v3_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the RecaptchaV3Config for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA V3 tokens for App Check tokens. For security reasons, the `site_secret` field is never populated in the response.");
            recaptcha_v3_config2 = recaptcha_v3_config2.subcommand(mcmd);
        }
        let mut safety_net_config2 = SubCommand::with_name("safety_net_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, get and patch");
        {
            let mcmd = SubCommand::with_name("batch_get")
                .about("Atomically gets the SafetyNetConfigs for the specified list of apps.");
            safety_net_config2 = safety_net_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the SafetyNetConfig for the specified app.");
            safety_net_config2 = safety_net_config2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the SafetyNetConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange SafetyNet tokens for App Check tokens.");
            safety_net_config2 = safety_net_config2.subcommand(mcmd);
        }
        apps1 = apps1.subcommand(safety_net_config2);
        apps1 = apps1.subcommand(recaptcha_v3_config2);
        apps1 = apps1.subcommand(recaptcha_enterprise_config2);
        apps1 = apps1.subcommand(recaptcha_config2);
        apps1 = apps1.subcommand(device_check_config2);
        apps1 = apps1.subcommand(debug_tokens2);
        apps1 = apps1.subcommand(app_attest_config2);
        projects0 = projects0.subcommand(services1);
        projects0 = projects0.subcommand(apps1);
        app = app.subcommand(projects0);
        app = app.subcommand(jwks0);

        Self { app }
    }
}
use google_firebaseappcheck1_beta as api;

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
