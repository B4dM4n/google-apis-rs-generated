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
        let mut app = App::new("cloudbuild1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230125")
            .about("Creates and manages builds on Google Cloud Platform.")
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
        let mut github_dot_com_webhook0 = SubCommand::with_name("github_dot_com_webhook")
            .setting(AppSettings::ColoredHelp)
            .about("methods: receive");
        {
            let mcmd = SubCommand::with_name("receive").about(
                "ReceiveGitHubDotComWebhook is called when the API receives a github.com webhook.",
            );
            github_dot_com_webhook0 = github_dot_com_webhook0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: regional_webhook");
        {
            let mcmd = SubCommand::with_name("regional_webhook").about(
                "ReceiveRegionalWebhook is called when the API receives a regional GitHub webhook.",
            );
            locations0 = locations0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: builds, github_enterprise_configs, locations and triggers");
        let mut v_10 = SubCommand::with_name("v_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: webhook");
        {
            let mcmd = SubCommand::with_name("webhook")
                .about("ReceiveWebhook is called when the API receives a GitHub webhook.");
            v_10 = v_10.subcommand(mcmd);
        }
        let mut builds1 = SubCommand::with_name("builds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, cancel, create, get, list and retry");
        {
            let mcmd = SubCommand::with_name("approve").about("Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a build in progress.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retry").about("Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings.");
            builds1 = builds1.subcommand(mcmd);
        }
        let mut github_enterprise_configs1 = SubCommand::with_name("github_enterprise_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs1 = github_enterprise_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Delete an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs1 = github_enterprise_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a GitHubEnterpriseConfig.");
            github_enterprise_configs1 = github_enterprise_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all GitHubEnterpriseConfigs for a given project.");
            github_enterprise_configs1 = github_enterprise_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs1 = github_enterprise_configs1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: bitbucket_server_configs, builds, git_lab_configs, github_enterprise_configs, operations, triggers and worker_pools");
        let mut triggers1 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, run and webhook");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `BuildTrigger`. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a `BuildTrigger`. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists existing `BuildTrigger`s. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhook").about("ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        let mut bitbucket_server_configs2 = SubCommand::with_name("bitbucket_server_configs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, list, patch and remove_bitbucket_server_connected_repository");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `BitbucketServerConfig`. This API is experimental.");
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a `BitbucketServerConfig`. This API is experimental.");
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieve a `BitbucketServerConfig`. This API is experimental.");
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all `BitbucketServerConfigs` for a given project. This API is experimental.",
            );
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing `BitbucketServerConfig`. This API is experimental.");
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_bitbucket_server_connected_repository").about("Remove a Bitbucket Server repository from a given BitbucketServerConfig's connected repositories. This API is experimental.");
            bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(mcmd);
        }
        let mut builds2 = SubCommand::with_name("builds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, cancel, create, get, list and retry");
        {
            let mcmd = SubCommand::with_name("approve").about("Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a build in progress.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retry").about("Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings.");
            builds2 = builds2.subcommand(mcmd);
        }
        let mut git_lab_configs2 = SubCommand::with_name("git_lab_configs")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: create, delete, get, list, patch and remove_git_lab_connected_repository",
            );
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `GitLabConfig`. This API is experimental");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a `GitLabConfig`. This API is experimental");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a `GitLabConfig`. This API is experimental");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all `GitLabConfigs` for a given project. This API is experimental");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing `GitLabConfig`. This API is experimental");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_git_lab_connected_repository").about("Remove a GitLab repository from a given GitLabConfig's connected repositories. This API is experimental.");
            git_lab_configs2 = git_lab_configs2.subcommand(mcmd);
        }
        let mut github_enterprise_configs2 = SubCommand::with_name("github_enterprise_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs2 = github_enterprise_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Delete an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs2 = github_enterprise_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a GitHubEnterpriseConfig.");
            github_enterprise_configs2 = github_enterprise_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all GitHubEnterpriseConfigs for a given project.");
            github_enterprise_configs2 = github_enterprise_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update an association between a GCP project and a GitHub Enterprise server.",
            );
            github_enterprise_configs2 = github_enterprise_configs2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut triggers2 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, run and webhook");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `BuildTrigger`. This API is experimental.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a `BuildTrigger`. This API is experimental.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists existing `BuildTrigger`s. This API is experimental.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhook").about("ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        let mut worker_pools2 = SubCommand::with_name("worker_pools")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a `WorkerPool`.");
            worker_pools2 = worker_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `WorkerPool`.");
            worker_pools2 = worker_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns details of a `WorkerPool`.");
            worker_pools2 = worker_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists `WorkerPool`s.");
            worker_pools2 = worker_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `WorkerPool`.");
            worker_pools2 = worker_pools2.subcommand(mcmd);
        }
        let mut connected_repositories3 = SubCommand::with_name("connected_repositories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create");
        {
            let mcmd = SubCommand::with_name("batch_create")
                .about("Batch connecting Bitbucket Server repositories to Cloud Build.");
            connected_repositories3 = connected_repositories3.subcommand(mcmd);
        }
        let mut repos3 = SubCommand::with_name("repos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all repositories for a given `BitbucketServerConfig`. This API is experimental.");
            repos3 = repos3.subcommand(mcmd);
        }
        let mut connected_repositories3 = SubCommand::with_name("connected_repositories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create");
        {
            let mcmd = SubCommand::with_name("batch_create").about(
                "Batch connecting GitLab repositories to Cloud Build. This API is experimental.",
            );
            connected_repositories3 = connected_repositories3.subcommand(mcmd);
        }
        let mut repos3 = SubCommand::with_name("repos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all repositories for a given `GitLabConfig`. This API is experimental",
            );
            repos3 = repos3.subcommand(mcmd);
        }
        git_lab_configs2 = git_lab_configs2.subcommand(repos3);
        git_lab_configs2 = git_lab_configs2.subcommand(connected_repositories3);
        bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(repos3);
        bitbucket_server_configs2 = bitbucket_server_configs2.subcommand(connected_repositories3);
        locations1 = locations1.subcommand(worker_pools2);
        locations1 = locations1.subcommand(triggers2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(github_enterprise_configs2);
        locations1 = locations1.subcommand(git_lab_configs2);
        locations1 = locations1.subcommand(builds2);
        locations1 = locations1.subcommand(bitbucket_server_configs2);
        projects0 = projects0.subcommand(triggers1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(github_enterprise_configs1);
        projects0 = projects0.subcommand(builds1);
        app = app.subcommand(v_10);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(locations0);
        app = app.subcommand(github_dot_com_webhook0);

        Self { app }
    }
}
use google_cloudbuild1 as api;

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
