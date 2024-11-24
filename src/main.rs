use sysinfo::System;

fn main() {
    // create system obj
    let mut sys = System::new_all();

    // refresh system information
    sys.refresh_all();

    // display general information
    println!("System name : {:?}", sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("Kernel version : {:?}", sysinfo::System::kernel_version().unwrap_or_else(|| "Unknow".to_string()));
    println!("Os version : {:?}", sysinfo::System::os_version().unwrap_or_else(|| "Unknow".to_string()));
    println!("Total memory : {} KB", sys.total_memory());
    println!("Used memory : {} KB", sys.used_memory());
    println!("Uptime : {} seconds", sysinfo::System::uptime());

    println!("\nProcesses :");
    println!("{:<10} {:<20} {:<10} {:<10}", "PID", "Name", "Memory", "Status");

    // iterate over all processes
    for (pid, process) in sys.processes() {
        println!(
            "{:<10} {:<10} {:<10}",
            //"{:<10} {:<20} {:<10} {:<10}",
            pid,
            //process.name(),
            process.memory(),
            process.status().to_string()
        );
    }
}

