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
        let mut app = App::new("artifactregistry1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220401")
            .about("Store and manage build artifacts in a scalable and integrated service built on Google infrastructure.")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_project_settings and update_project_settings");
        {
            let mcmd = SubCommand::with_name("get_project_settings")
                .about("Retrieves the Settings for the Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_project_settings")
                .about("Updates the Settings for the Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut repositories2 = SubCommand::with_name("repositories")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a repository. The returned Operation will finish once the repository has been created. Its response will be the created Repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a repository and all of its contents. The returned Operation will finish once the repository has been deleted. It will not have any Operation metadata and will return a google.protobuf.Empty response.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the IAM policy for a given resource.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists repositories.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Updates the IAM policy for a given resource.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Tests if the caller has a list of permissions on a resource.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        let mut apt_artifacts3 = SubCommand::with_name("apt_artifacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: import and upload");
        {
            let mcmd = SubCommand::with_name("import").about("Imports Apt artifacts. The returned Operation will complete once the resources are imported. Package, Version, and File resources are created based on the imported artifacts. Imported artifacts that conflict with existing resources are ignored.");
            apt_artifacts3 = apt_artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Directly uploads an Apt artifact. The returned Operation will complete once the resources are uploaded. Package, Version, and File resources are created based on the imported artifact. Imported artifacts that conflict with existing resources are ignored.");
            apt_artifacts3 = apt_artifacts3.subcommand(mcmd);
        }
        let mut docker_images3 = SubCommand::with_name("docker_images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a docker image.");
            docker_images3 = docker_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists docker images.");
            docker_images3 = docker_images3.subcommand(mcmd);
        }
        let mut files3 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a file.");
            files3 = files3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists files.");
            files3 = files3.subcommand(mcmd);
        }
        let mut packages3 = SubCommand::with_name("packages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a package and all of its versions and tags. The returned operation will complete once the package has been deleted.");
            packages3 = packages3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a package.");
            packages3 = packages3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists packages.");
            packages3 = packages3.subcommand(mcmd);
        }
        let mut yum_artifacts3 = SubCommand::with_name("yum_artifacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: import and upload");
        {
            let mcmd = SubCommand::with_name("import").about("Imports Yum (RPM) artifacts. The returned Operation will complete once the resources are imported. Package, Version, and File resources are created based on the imported artifacts. Imported artifacts that conflict with existing resources are ignored.");
            yum_artifacts3 = yum_artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Directly uploads a Yum artifact. The returned Operation will complete once the resources are uploaded. Package, Version, and File resources are created based on the imported artifact. Imported artifacts that conflict with existing resources are ignored.");
            yum_artifacts3 = yum_artifacts3.subcommand(mcmd);
        }
        let mut tags4 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists tags.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        let mut versions4 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a version and all of its content. The returned operation will complete once the version has been deleted.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a version");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists versions.");
            versions4 = versions4.subcommand(mcmd);
        }
        packages3 = packages3.subcommand(versions4);
        packages3 = packages3.subcommand(tags4);
        repositories2 = repositories2.subcommand(yum_artifacts3);
        repositories2 = repositories2.subcommand(packages3);
        repositories2 = repositories2.subcommand(files3);
        repositories2 = repositories2.subcommand(docker_images3);
        repositories2 = repositories2.subcommand(apt_artifacts3);
        locations1 = locations1.subcommand(repositories2);
        locations1 = locations1.subcommand(operations2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_artifactregistry1 as api;

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
