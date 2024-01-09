# awesome-cli is a command line to orchestrates command lines

The goal of this CLI is to be able to run multi commands from a `Awesome.toml` file. 

Setup a `Awesome.toml` with the format like: 

```toml
[[dev.runners]]
name = "tauri_icons"
working_dir = "crates/app-desktop/"
when.no_file_at = "icons/32x32.png"
cmd = "cargo"
args = ["tauri", "icon", "icons/app-icon.png"]

[[dev.runners]]
name = "cargo_build"
working_dir = "crates/app-desktop/"
cmd = "cargo"
args = ["build"]

[[dev.runners]]
name = "pcss"
working_dir = "frontend/"
cmd = "npm"
args = ["run", "pcss", "--", "-w"]
concurrent = true 
# if this process exit, end the dev session
end_all_on_exit = true

[[dev.runners]]
name = "rollup"
working_dir = "frontend/"
cmd = "npm"
args = ["run", "rollup", "--", "-w"]
concurrent = true
end_all_on_exit = true

[[dev.runners]]
name = "tauri_dev"
working_dir = "crates/app-desktop"
wait_before = 2000 # wait in ms, before getting called
cmd = "cargo"
args = ["tauri", "dev"]
concurrent = true
end_all_on_exit = true
```

<br />

[This repo on GitHub](https://github.com/jeremychone/awesome-cli)