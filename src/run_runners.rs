use crate::config::{find_and_parse_awesome_toml, Runner, ShouldRun};
use crate::{Error, Result};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use sysinfo::{Pid, Process, ProcessRefreshKind, System};
use tokio::process::Child;
use tokio::time::sleep;

const WATCH_CHILD_DELAY: u64 = 3000; // in ms

#[tokio::main]
pub async fn run(run_ref: &str) -> Result<()> {
	// -- Parse the command
	let mut parts = run_ref.splitn(2, '.');
	let part1 = parts.next().ok_or_else(|| Error::RunRefNoParts(run_ref.to_string()))?;
	let part2 = parts.next();

	// -- Parse the "Awesome.toml"
	// TODO: might want to check if "./" works on windows
	let config = find_and_parse_awesome_toml(Path::new("./"))?;

	// -- Compute the Runners
	// If two parts, then, we have a group_name.runner_name
	let runners = if let Some(part2) = part2 {
		config.get_grouped_runner(part1, part2).map(|r| vec![r])
	}
	// otherwise, we just have a group or a solo runner
	else {
		config
			.get_runners(part1)
			.or_else(|| config.get_solo_runner(part1).map(|r| vec![r]))
	};

	// -- Run the runners
	if let Some(runners) = runners {
		run_runners(runners).await?;
	} else {
		println!("No runners found for '{run_ref}'");
	}

	Ok(())
}

async fn run_runners(runners: Vec<&Runner>) -> Result<()> {
	// TODO: needs to get it from the params.
	let root_dir = Path::new(".");

	// Vec to keep track of the concurrent processes.
	struct RunnerConcurrentSpawn {
		name: String,
		child: Child,
		end_all_on_exit: bool,
	}
	let mut children_to_watch: Vec<RunnerConcurrentSpawn> = Vec::new();

	// --- Exec each runner.
	for runner in runners.iter() {
		println!("==== Running runner: {}", runner.name);

		match runner.should_run(root_dir)? {
			ShouldRun::No(reason) => println!("Skip running runner '{}' because {reason}", runner.name),
			ShouldRun::Yes => {
				// exec the runner.
				// returns a child if process is concurrent.
				let child = runner.exec().await?;

				// if concurrent, keep an eye on this child.
				if let Some(child) = child {
					children_to_watch.push(RunnerConcurrentSpawn {
						name: runner.name.to_string(),
						child,
						end_all_on_exit: runner.end_all_on_exit,
					});
				}
			}
		}
	}

	// --- Watch processes when concurrent to end_all_on_exit when flagged.
	// TODO: Probably need to change that to avoid doing polling.
	//       Strategy: Tokio Spawn for the child with mpsc for the end_all event.
	if !children_to_watch.is_empty() {
		let mut end_all = false;

		let mut sys = System::new();

		'main: loop {
			// --- Check if any children is down.
			for RunnerConcurrentSpawn {
				child, end_all_on_exit, ..
			} in children_to_watch.iter_mut()
			{
				let status = child.try_wait()?;
				if status.is_some() && *end_all_on_exit {
					end_all = true;
				}
			}

			// --- If end_all true, then, we terminate all.
			if end_all {
				for RunnerConcurrentSpawn { name, child, .. } in children_to_watch.iter_mut() {
					if (child.try_wait()?).is_none() {
						terminate_process_tree(&mut sys, name, child).await?
					}
				}
				break 'main;
			}

			sleep(Duration::from_millis(WATCH_CHILD_DELAY)).await;
		}
	}

	Ok(())
}

/// Terminate this process and all of its children.
async fn terminate_process_tree(sys: &mut System, name: &str, proc: &mut Child) -> Result<()> {
	if let Some(proc_id) = proc.id() {
		let proc_pid = Pid::from_u32(proc_id);

		// --- Fetch the children
		sys.refresh_processes_specifics(ProcessRefreshKind::everything().without_cpu());
		let sys_processes = sys.processes();
		let children = find_descendant(sys_processes, &proc_pid);

		// --- Terminate the parent
		match proc.kill().await {
			Ok(_) => (),
			Err(ex) => println!("Warning - error while stopping runner {name}. Cause: {ex}"),
		};

		// --- Terminate the children
		for (pid, _) in children {
			if let Some(process) = sys.process(pid) {
				let _ = process.kill();
			}
		}
	}

	Ok(())
}

fn find_descendant(sys_processes: &HashMap<Pid, Process>, root_pid: &Pid) -> Vec<(Pid, String)> {
	let mut children: HashMap<Pid, String> = HashMap::new();

	// NOTE: For now, going a little brute force, but this should be exhaustive
	//       and does not really have significant performance impact for the usecase.
	'main: loop {
		let mut cycle_has = false;
		for (pid, p) in sys_processes.iter() {
			if let Some(parent_pid) = p.parent() {
				if !children.contains_key(pid) && (parent_pid == *root_pid || children.contains_key(&parent_pid)) {
					children.insert(*pid, p.name().to_string());
					cycle_has = true;
				}
			}
		}
		// if this cycle did not find anything, we can break the search.
		if !cycle_has {
			break 'main;
		}
	}

	children.into_iter().collect()
}
