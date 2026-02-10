use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

struct WorldStatus {
    name: String,
    mass: u64,
    stability: f32,
}

fn scan_system(root: &str) -> Vec<WorldStatus> {
    let mut system = Vec::new();
    let now = SystemTime::now();
    let decay_period = Duration::from_secs(60 * 60 * 24 * 7);

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let (count, last_mod) = analyze_world(&path);

                let elapsed = now.duration_since(last_mod).unwrap_or(Duration::ZERO);
                let stability =
                    (1.0 - (elapsed.as_secs_f32() / decay_period.as_secs_f32())).max(0.0);

                system.push(WorldStatus {
                    name,
                    mass: count,
                    stability,
                });
            }
        }
    }
    system
}

fn analyze_world(path: &Path) -> (u64, SystemTime) {
    let mut count = 0;
    let mut latest = SystemTime::UNIX_EPOCH;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            count += 1;

            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    if modified > latest {
                        latest = modified;
                    }
                }
            }
        }
    }
    (count, latest)
}

fn main() {
    let worlds = scan_system("./src");
    println!("--- ORBITAL STABILITY REPORT ---");
    for world in worlds {
        let status = if world.stability > 0.7 {
            "STABLE"
        } else if world.stability > 0.3 {
            "FALLING"
        } else {
            "COLLISION WARNING"
        };
        println!(
            "{:<25} | Mass: {:>3} | Health: {:.2} [{}]",
            world.name, world.mass, world.stability, status
        );
    }
}
