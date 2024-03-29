# awesome-cli is a command line tool that orchestrates other command lines

awesome-cli allows you to describe a set of commands as either groups or individually, and run them accordingly.

The configuration file is named `Awesome.toml` and should be located in the same directory as this file.

A `Runner` is a command description that can be executed by awesome-cli.

There are two types of "Runners":

- **Grouped Runners**
	- Described with the `[[runners._group_name_]]` TOML array of tables for each group name (e.g., `[[runners.dev]]`).
	- Group execution: When invoking `awesome _group_name_` (e.g., `awesome dev`), all the runners in this group will be executed in the order they are listed in the file.
	- Individual execution: Dot notation can be used to execute only one runner in a group with `awesome _group_name_._runner_name` (e.g., `awesome dev.cargo_build`).
	- Constraint: Runner names in a group must be unique within that group.

- **Solo Runner**
	- Described with the `[[runner]]` TOML array of tables, where each runner has a `name` or `ref` property.
	- These can be called with `awesome _runner_name` (e.g., `awesome list_files`).
	- Constraint: Solo runner names must be unique and cannot overlap with group names.
	
## Example

```toml
# Solo Runner 
[[runner]]
name = "list_files"
cmd = "ls"
args = ["-ll"]

[[runner]]
name = "list_files_human"
cmd = "ls"
args = ["-llh"]

[[runners.build]]
name = "tauri_icons"
working_dir = "crates/app-desktop/"
when.no_file_at = "icons/32x32.png"
cmd = "cargo"
args = ["tauri", "icon", "icons/app-icon.png"]

[[runners.build]]
name = "cargo_build"
working_dir = "crates/app-desktop/"
cmd = "cargo"
args = ["build"]

[[runners.build]]
name = "pcss"
working_dir = "frontend/"
cmd = "npm"
args = ["run", "pcss"]
concurrent = true 
# if this process exit, end the dev session
end_all_on_exit = true

# Now: runners, can ref other runner (only one hop for now)

[[runners.dev]]
ref = "build.tauri_icons"

[[runners.dev]]
ref = "build.cargo_build"
# The args from the target will be overwritten with the following:
args = ["watch", "build"]

[[runners.dev]]
ref = "build.pcss"
# The args from the target will be extended with the args_add items
args_add = ["--", "-w"]

```

<br />

[This repo on GitHub](https://github.com/jeremychone/awesome-cli)