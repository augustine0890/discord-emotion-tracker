use sysinfo::{System, SystemExt};

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

pub fn display_memory_stats() {
    let mut system = System::new_all();
    system.refresh_all();

    let total_memory = system.total_memory() * 1024;
    let used_memory = system.used_memory() * 1024;
    let free_memory = system.free_memory() * 1024;
    // "free" memory refers to unallocated memory whereas "available" memory refers to memory that is available for (re)use.
    let available_memory = system.available_memory() * 1024;
    let used_memory_percent = used_memory as f64 / total_memory as f64 * 100.0;

    println!(
        "Total memory: {} bytes ({:.2} GB)",
        total_memory,
        bytes_to_gb(total_memory)
    );
    println!(
        "Used memory: {} bytes ({:.2} GB)",
        used_memory,
        bytes_to_gb(used_memory)
    );
    println!(
        "Free memory: {} bytes ({:.2} GB)",
        free_memory,
        bytes_to_gb(free_memory)
    );
    println!(
        "Available memory: {} bytes ({:.2} GB)",
        available_memory,
        bytes_to_gb(available_memory)
    );
    println!("Used memory percentage: {:.2}%\n", used_memory_percent);
    println!("-----------------------------------------------------------");
}
