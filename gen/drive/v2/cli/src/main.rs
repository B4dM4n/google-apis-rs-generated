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
        let mut app = App::new("drive2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230122")
            .about("Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.")
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
        let mut about0 = SubCommand::with_name("about")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the information about the current user along with Drive API settings");
            about0 = about0.subcommand(mcmd);
        }
        let mut apps0 = SubCommand::with_name("apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific app.");
            apps0 = apps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a user's installed apps.");
            apps0 = apps0.subcommand(mcmd);
        }
        let mut changes0 = SubCommand::with_name("changes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_start_page_token, list and watch");
        {
            let mcmd = SubCommand::with_name("get").about("Deprecated - Use changes.getStartPageToken and changes.list to retrieve recent changes.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_start_page_token")
                .about("Gets the starting pageToken for listing future changes.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the changes for a user or shared drive.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Subscribe to changes for a user.");
            changes0 = changes0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut children0 = SubCommand::with_name("children")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a child from a folder.");
            children0 = children0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific child reference.");
            children0 = children0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a file into a folder.");
            children0 = children0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a folder's children.");
            children0 = children0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a comment by ID.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new comment on the given file.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file's comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut drives0 = SubCommand::with_name("drives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, hide, insert, list, unhide and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a shared drive's metadata by ID.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("hide").about("Hides a shared drive from the default view.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new shared drive.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the user's shared drives.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unhide")
                .about("Restores a shared drive to the default view.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the metadata for a shared drive.");
            drives0 = drives0.subcommand(mcmd);
        }
        let mut files0 = SubCommand::with_name("files")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: copy, delete, empty_trash, export, generate_ids, get, insert, list, list_labels, modify_labels, patch, touch, trash, untrash, update and watch");
        {
            let mcmd = SubCommand::with_name("copy")
                .about("Creates a copy of the specified file. Folders cannot be copied.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a file by ID. Skips the trash. The currently authenticated user must own the file or be an organizer on the parent for shared drive files.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("empty_trash")
                .about("Permanently deletes all of the user's trashed files.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_ids").about(
                "Generates a set of file IDs which can be provided in insert or copy requests.",
            );
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a file's metadata or content by ID.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Insert a new file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the user's files.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_labels").about("Lists the labels on a file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_labels")
                .about("Modifies the set of labels on a file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("touch")
                .about("Set the file's updated time to the current server time.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("trash").about("Moves a file to the trash. The currently authenticated user must own the file or be at least a fileOrganizer on the parent for shared drive files. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("untrash").about("Restores a file from the trash. The currently authenticated user must own the file or be at least a fileOrganizer on the parent for shared drive files. Only the owner may untrash a file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as modifiedDate. This method supports patch semantics.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Subscribes to changes to a file. While you can establish a channel for changes to a file on a shared drive, a change to a shared drive file won't create a notification.");
            files0 = files0.subcommand(mcmd);
        }
        let mut parents0 = SubCommand::with_name("parents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a parent from a file.");
            parents0 = parents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific parent reference.");
            parents0 = parents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a parent folder for a file.");
            parents0 = parents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file's parents.");
            parents0 = parents0.subcommand(mcmd);
        }
        let mut permissions0 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, get_id_for_email, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a permission from a file or shared drive.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a permission by ID.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_id_for_email")
                .about("Returns the permission ID for an email address.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a permission for a file or shared drive.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists a file's or shared drive's permissions.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a permission using patch semantics.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a permission.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        let mut properties0 = SubCommand::with_name("properties")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a property by its key.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Adds a property to a file, or updates it if it already exists.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file's properties.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a property.");
            properties0 = properties0.subcommand(mcmd);
        }
        let mut replies0 = SubCommand::with_name("replies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a reply.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a reply.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new reply to the given comment.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all of the replies to a comment.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing reply.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing reply.");
            replies0 = replies0.subcommand(mcmd);
        }
        let mut revisions0 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a file version. You can only delete revisions for files with binary content, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific revision.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file's revisions.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a revision.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a revision.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        let mut teamdrives0 = SubCommand::with_name("teamdrives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deprecated use drives.delete instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Deprecated use drives.get instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Deprecated use drives.insert instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Deprecated use drives.list instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Deprecated use drives.update instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        app = app.subcommand(teamdrives0);
        app = app.subcommand(revisions0);
        app = app.subcommand(replies0);
        app = app.subcommand(properties0);
        app = app.subcommand(permissions0);
        app = app.subcommand(parents0);
        app = app.subcommand(files0);
        app = app.subcommand(drives0);
        app = app.subcommand(comments0);
        app = app.subcommand(children0);
        app = app.subcommand(channels0);
        app = app.subcommand(changes0);
        app = app.subcommand(apps0);
        app = app.subcommand(about0);

        Self { app }
    }
}
use google_drive2 as api;

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
