use super::app_usage_entity::{
    AppBreakdownEntity, AppUsageEntity, ProcessBreakdownEntity, ProcessDetailEntity, ProcessSampleEntity,
};

const INTERPRETERS: &[&str] = &["node", "bun", "deno", "python", "python3", "ruby", "perl", "java"];
const XPC_SERVICE_MARKER: &str = ".xpc/";
const VERSION_DIRECTORY: &str = "versions";

pub fn build_breakdown(
    usage: &AppUsageEntity,
    limit: usize,
    describe: impl Fn(i32) -> ProcessDetailEntity,
) -> AppBreakdownEntity {
    let bundle_path = usage.app.as_ref().and_then(|a| a.bundle_path.as_deref());
    let mut processes: Vec<ProcessBreakdownEntity> = usage
        .members
        .iter()
        .map(|sample| describe_sample(sample, usage.pid, bundle_path, &describe(sample.pid)))
        .collect();
    processes.sort_by(|a, b| b.footprint_bytes.cmp(&a.footprint_bytes));
    let own_bytes = processes.iter().filter(|p| p.is_own).map(|p| p.footprint_bytes).sum();
    processes.truncate(limit);
    AppBreakdownEntity {
        own_bytes,
        launched_bytes: usage.footprint_bytes.saturating_sub(own_bytes),
        processes,
    }
}

fn describe_sample(
    sample: &ProcessSampleEntity,
    owner_pid: i32,
    bundle_path: Option<&str>,
    detail: &ProcessDetailEntity,
) -> ProcessBreakdownEntity {
    ProcessBreakdownEntity {
        pid: sample.pid,
        label: process_label(detail).unwrap_or_else(|| sample.name.clone()),
        footprint_bytes: sample.footprint_bytes,
        is_own: is_own_process(sample.pid, owner_pid, bundle_path, detail.path.as_deref()),
    }
}

fn is_own_process(pid: i32, owner_pid: i32, bundle_path: Option<&str>, path: Option<&str>) -> bool {
    let (Some(bundle), Some(path)) = (bundle_path, path) else {
        return pid == owner_pid || bundle_path.is_none();
    };
    pid == owner_pid || path.starts_with(bundle) || path.contains(XPC_SERVICE_MARKER)
}

fn process_label(detail: &ProcessDetailEntity) -> Option<String> {
    let program = detail.arguments.first().map(|a| file_name(a))?;
    if !INTERPRETERS.contains(&program) {
        return Some(readable_program(program, detail.path.as_deref()));
    }
    let script = detail.arguments.iter().skip(1).find(|a| !a.starts_with('-'));
    Some(match script {
        Some(script) => format!("{program} · {}", file_name(script)),
        None => program.to_string(),
    })
}

// Self-updating CLIs (e.g. ~/.local/share/claude/versions/2.1.311) name their binary after the version.
fn readable_program(program: &str, path: Option<&str>) -> String {
    let looks_like_version = program.starts_with(|c: char| c.is_ascii_digit());
    let owner_dir = path.and_then(|p| {
        let mut parts = p.rsplit('/').skip(1);
        match (parts.next(), parts.next()) {
            (Some(VERSION_DIRECTORY), Some(tool)) => Some(tool.to_string()),
            _ => None,
        }
    });
    match (looks_like_version, owner_dir) {
        (true, Some(tool)) => tool,
        _ => program.to_string(),
    }
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detail(path: &str, arguments: &[&str]) -> ProcessDetailEntity {
        ProcessDetailEntity {
            path: Some(path.into()),
            arguments: arguments.iter().map(|a| a.to_string()).collect(),
        }
    }

    #[test]
    fn labels_interpreters_with_script_and_versioned_binaries_with_tool() {
        let vite = detail("/opt/homebrew/bin/node", &["node", "/p/node_modules/.bin/vite"]);
        let claude = detail("/Users/x/.local/share/claude/versions/2.1.311", &["2.1.311", "--resume"]);
        assert_eq!(process_label(&vite).unwrap(), "node · vite");
        assert_eq!(process_label(&claude).unwrap(), "claude");
    }

    #[test]
    fn separates_own_processes_from_launched_ones() {
        let bundle = Some("/System/Applications/Utilities/Terminal.app");
        let helper = "/System/Library/Frameworks/WebKit.framework/XPCServices/com.apple.WebKit.WebContent.xpc/x";
        assert!(is_own_process(1, 1, bundle, Some("/System/Applications/Utilities/Terminal.app/Contents/MacOS/Terminal")));
        assert!(is_own_process(2, 1, bundle, Some(helper)));
        assert!(!is_own_process(3, 1, bundle, Some("/opt/homebrew/bin/node")));
    }
}
