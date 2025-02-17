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
        let mut app = App::new("searchconsole1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
            .about("The Search Console API provides access to both Search Console data (verified users only) and to public information on an URL basis (anyone)")
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
        let mut searchanalytics0 = SubCommand::with_name("searchanalytics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query");
        {
            let mcmd = SubCommand::with_name("query").about("Query your data with filters and parameters that you define. Returns zero or more rows grouped by the row keys that you define. You must define a date range of one or more days. When date is one of the group by values, any days without data are omitted from the result list. If you need to know which days have data, issue a broad date range query grouped by date for any metric, and see which day rows are returned.");
            searchanalytics0 = searchanalytics0.subcommand(mcmd);
        }
        let mut sitemaps0 = SubCommand::with_name("sitemaps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and submit");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sitemap from the Sitemaps report. Does not stop Google from crawling this sitemap or the URLs that were previously crawled in the deleted sitemap.");
            sitemaps0 = sitemaps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves information about a specific sitemap.");
            sitemaps0 = sitemaps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(" Lists the [sitemaps-entries](/webmaster-tools/v3/sitemaps) submitted for this site, or included in the sitemap index file (if `sitemapIndex` is specified in the request).");
            sitemaps0 = sitemaps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit").about("Submits a sitemap for a site.");
            sitemaps0 = sitemaps0.subcommand(mcmd);
        }
        let mut sites0 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add, delete, get and list");
        {
            let mcmd = SubCommand::with_name("add")
                .about(" Adds a site to the set of the user's sites in Search Console.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about(" Removes a site from the set of the user's Search Console sites.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about(" Retrieves information about specific site.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about(" Lists the user's Search Console sites.");
            sites0 = sites0.subcommand(mcmd);
        }
        let mut url_inspection0 = SubCommand::with_name("url_inspection")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: index");
        let mut url_testing_tools0 = SubCommand::with_name("url_testing_tools")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: mobile_friendly_test");
        let mut index1 = SubCommand::with_name("index")
            .setting(AppSettings::ColoredHelp)
            .about("methods: inspect");
        {
            let mcmd = SubCommand::with_name("inspect").about("Index inspection.");
            index1 = index1.subcommand(mcmd);
        }
        let mut mobile_friendly_test1 = SubCommand::with_name("mobile_friendly_test")
            .setting(AppSettings::ColoredHelp)
            .about("methods: run");
        {
            let mcmd =
                SubCommand::with_name("run").about("Runs Mobile-Friendly Test for a given URL.");
            mobile_friendly_test1 = mobile_friendly_test1.subcommand(mcmd);
        }
        url_testing_tools0 = url_testing_tools0.subcommand(mobile_friendly_test1);
        url_inspection0 = url_inspection0.subcommand(index1);
        app = app.subcommand(url_testing_tools0);
        app = app.subcommand(url_inspection0);
        app = app.subcommand(sites0);
        app = app.subcommand(sitemaps0);
        app = app.subcommand(searchanalytics0);

        Self { app }
    }
}
use google_searchconsole1 as api;

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
