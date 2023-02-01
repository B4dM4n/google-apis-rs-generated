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
        let mut app = App::new("dataform1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230121")
            .about("")
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
            .about("sub-resources: locations");
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
        let mut repositories2 = SubCommand::with_name("repositories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, fetch_remote_branches, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Repository in a given project and location.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_remote_branches")
                .about("Fetches a Repository's remote branches.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single Repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Repositories in a given project and location.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a single Repository.");
            repositories2 = repositories2.subcommand(mcmd);
        }
        let mut compilation_results3 = SubCommand::with_name("compilation_results")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and query");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new CompilationResult in a given project and location.");
            compilation_results3 = compilation_results3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single CompilationResult.");
            compilation_results3 = compilation_results3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists CompilationResults in a given Repository.");
            compilation_results3 = compilation_results3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query")
                .about("Returns CompilationResultActions in a given CompilationResult.");
            compilation_results3 = compilation_results3.subcommand(mcmd);
        }
        let mut release_configs3 = SubCommand::with_name("release_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new ReleaseConfig in a given Repository.");
            release_configs3 = release_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single ReleaseConfig.");
            release_configs3 = release_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single ReleaseConfig.");
            release_configs3 = release_configs3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists ReleaseConfigs in a given Repository.");
            release_configs3 = release_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a single ReleaseConfig.");
            release_configs3 = release_configs3.subcommand(mcmd);
        }
        let mut workflow_configs3 = SubCommand::with_name("workflow_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new WorkflowConfig in a given Repository.");
            workflow_configs3 = workflow_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single WorkflowConfig.");
            workflow_configs3 = workflow_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single WorkflowConfig.");
            workflow_configs3 = workflow_configs3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists WorkflowConfigs in a given Repository.");
            workflow_configs3 = workflow_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a single WorkflowConfig.");
            workflow_configs3 = workflow_configs3.subcommand(mcmd);
        }
        let mut workflow_invocations3 = SubCommand::with_name("workflow_invocations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, delete, get, list and query");
        {
            let mcmd = SubCommand::with_name("cancel")
                .about("Requests cancellation of a running WorkflowInvocation.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new WorkflowInvocation in a given Repository.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single WorkflowInvocation.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single WorkflowInvocation.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists WorkflowInvocations in a given Repository.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query")
                .about("Returns WorkflowInvocationActions in a given WorkflowInvocation.");
            workflow_invocations3 = workflow_invocations3.subcommand(mcmd);
        }
        let mut workspaces3 = SubCommand::with_name("workspaces")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: commit, create, delete, fetch_file_diff, fetch_file_git_statuses, fetch_git_ahead_behind, get, install_npm_packages, list, make_directory, move_directory, move_file, pull, push, query_directory_contents, read_file, remove_directory, remove_file, reset and write_file");
        {
            let mcmd = SubCommand::with_name("commit")
                .about("Applies a Git commit for uncommitted files in a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Workspace in a given Repository.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_file_diff")
                .about("Fetches Git diff for an uncommitted file in a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_file_git_statuses")
                .about("Fetches Git statuses for the files in a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_git_ahead_behind")
                .about("Fetches Git ahead/behind against a remote branch.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a single Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("install_npm_packages")
                .about("Installs dependency NPM packages (inside a Workspace).");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists Workspaces in a given Repository.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("make_directory")
                .about("Creates a directory inside a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_directory").about("Moves a directory (inside a Workspace), and all of its contents, to a new location.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_file")
                .about("Moves a file (inside a Workspace) to a new location.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pull")
                .about("Pulls Git commits from the Repository's remote into a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("push")
                .about("Pushes Git commits from a Workspace to the Repository's remote.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query_directory_contents")
                .about("Returns the contents of a given Workspace directory.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("read_file")
                .about("Returns the contents of a file (inside a Workspace).");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_directory")
                .about("Deletes a directory (inside a Workspace) and all of its contents.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("remove_file").about("Deletes a file (inside a Workspace).");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset")
                .about("Performs a Git reset for uncommitted files in a Workspace.");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("write_file").about("Writes to a file (inside a Workspace).");
            workspaces3 = workspaces3.subcommand(mcmd);
        }
        repositories2 = repositories2.subcommand(workspaces3);
        repositories2 = repositories2.subcommand(workflow_invocations3);
        repositories2 = repositories2.subcommand(workflow_configs3);
        repositories2 = repositories2.subcommand(release_configs3);
        repositories2 = repositories2.subcommand(compilation_results3);
        locations1 = locations1.subcommand(repositories2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dataform1_beta1 as api;

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
