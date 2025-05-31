use colored::Colorize;
use dialoguer::{Select, Input};
use crate::models::{DeviceType, GpuType, UserChoices};
use crate::utils::{sleep, create_progress_bar, show_header, show_success, show_warning};
use crate::input_handler::read_input_with_history;
use std::io::Write;

/// Runs full Arch Linux installation simulation
pub fn run_simulation(user_choices: &UserChoices) {
    show_header("Starting Arch Linux Installation Simulator...");
    sleep(3);
    show_header("Welcome to Arch Linux!");
    sleep(2);

    // Network setup
    if let DeviceType::Pc = user_choices.device_type {
        println!("\n{}", "Connected via LAN.".bright_green());
        sleep(1);
    } else if let DeviceType::Laptop = user_choices.device_type {
        simulate_wifi_setup();
    }

    // Disk preparation
    simulate_partitioning();
    simulate_mount();

    // Base system installation
    simulate_pacstrap(&user_choices.gpu_type);

    // Final configuration
    simulate_final_steps();
}

/// Simulates Wi-Fi setup for laptops
fn simulate_wifi_setup() {
    show_header("Setting up Wi-Fi...");
    show_warning("Note: You need to manually connect to Wi-Fi.");

    simulate_command("ip a", "Show network interfaces", false, || {
        println!("1: wlan0: <NO-CARRIER,BROADCAST,MULTICAST,UP>");
    });

    simulate_command("iwctl station wlan0 get-networks", "Scan for Wi-Fi", false, || {
        println!("Available networks: archtry_wifi");
    });

    simulate_command("iwctl station wlan0 connect archtry_wifi", "Connect to Wi-Fi", false, || {
        println!("Connected to archtry_wifi.");
    });

    sleep(1);
}

/// Simulates disk partitioning with educational guidance
fn simulate_partitioning() {
    show_header("Partitioning disk...");
    show_warning("Note: You need to manually partition the disk.");

    simulate_command("lsblk", "Show disk layout", false, || {
        println!("NAME        SIZE\nsdz         50.0G");
    });

    // محاكاة موسعة لأمر fdisk مع تعليمات مفصلة
    simulate_command("fdisk /dev/sdz", "Create partitions", false, || {
        println!("\n{}", "Starting fdisk simulation...".bright_blue());
        sleep(1);
        
        // Step 1: Create GPT partition table
        println!("\n{}", "Step 1: Create a new GPT partition table".bright_yellow());
        println!("Command: {}", "g".bright_green());
        println!("Created a new GPT disklabel (GUID: 12345678-1234-1234-1234-1234567890AB).");
        sleep(1);
        
        // Step 2: Create EFI System partition (1GB)
        println!("\n{}", "Step 2: Create EFI System partition (1GB)".bright_yellow());
        println!("Command: {}", "n".bright_green());
        println!("Partition number (1-128, default 1): ");
        println!("First sector (2048-104857566, default 2048): ");
        println!("Last sector, +/-sectors or +/-size{{K,M,G,T,P}} (2048-104857566, default 104857566): {}", "+1G".bright_green());
        println!("Created a new partition 1 of type 'Linux filesystem' and of size 1 GiB.");
        sleep(1);
        
        // Step 3: Change partition type to EFI System
        println!("\n{}", "Step 3: Change partition type to EFI System".bright_yellow());
        println!("Command: {}", "t".bright_green());
        println!("Partition number (1,2, default 2): {}", "1".bright_green());
        println!("Partition type or alias (type L to list all): {}", "1".bright_green());
        println!("Changed type of partition 'Linux filesystem' to 'EFI System'.");
        sleep(1);
        
        // Step 4: Create root partition (using remaining space)
        println!("\n{}", "Step 4: Create root partition (using remaining space)".bright_yellow());
        println!("Command: {}", "n".bright_green());
        println!("Partition number (2-128, default 2): ");
        println!("First sector (1128448-104857566, default 1128448): ");
        println!("Last sector, +/-sectors or +/-size{{K,M,G,T,P}} (1128448-104857566, default 104857566): ");
        println!("Created a new partition 2 of type 'Linux filesystem' and of size 49 GiB.");
        sleep(1);
        
        // Step 5: Write changes to disk
        println!("\n{}", "Step 5: Write changes to disk".bright_yellow());
        println!("Command: {}", "w".bright_green());
        println!("The partition table has been altered.");
        println!("Calling ioctl() to re-read partition table.");
        println!("Syncing disks.");
        
        sleep(1);
        println!("\n{}", "Partitioning completed successfully.".bright_green());
    });
}

/// Simulates mounting partitions
fn simulate_mount() {
    show_header("Mounting partitions...");

    simulate_command("mount /dev/sdz2 /mnt", "Mount root partition", false, || {
        println!("Root partition mounted.");
    });

    simulate_command("mount --mkdir /dev/sdz1 /mnt/boot/efi", "Mount EFI partition", false, || {
        println!("EFI partition mounted.");
    });

    show_success("Partitions mounted.");
}

/// Simulates base system installation
fn simulate_pacstrap(gpu_type: &GpuType) {
    show_header("Installing base system...");

    let base_packages = match gpu_type {
        GpuType::Amd => "amd-ucode",
        GpuType::Intel => "intel-ucode",
        GpuType::Nvidia => "nvidia",
    };

    simulate_command(
        &format!("pacstrap -K /mnt base base-devel linux linux-firmware {} grub efibootmgr", base_packages),
        "Install base system and GRUB",
        false,
        || {
            let pb = create_progress_bar(10);
            for i in 0..10 {
                pb.set_position(i);
                pb.set_message("Installing packages...");
                sleep(1);
            }
            pb.finish_with_message("Installation complete.");
        },
    );

    show_success("Base system installed.");
}

/// Simulates final installation steps
fn simulate_final_steps() {
    show_header("Finishing installation...");

    // Generate fstab
    simulate_command("genfstab -U /mnt >> /mnt/etc/fstab", "Generate fstab", false, || {
        println!("fstab generated.");
    });

    // Enter chroot
    simulate_command("arch-chroot /mnt", "Enter chroot environment", false, || {
        println!("Now in chroot environment.");
    });

    // Configure system
    configure_hostname();
    configure_timezone();
    configure_locale();
    configure_users();
    configure_bootloader();
    install_desktop();

    // Exit and reboot
    simulate_command("exit", "Exit chroot", true, || {
        println!("Exited chroot.");
    });

    simulate_command("umount -R /mnt", "Unmount partitions", false, || {
        println!("Partitions unmounted.");
    });

    simulate_command("reboot", "Reboot system", false, || {
        println!("System rebooting...");
    });

    show_success("Installation complete!");
    println!("{}", "You've learned the basics of Arch Linux installation.".bright_green());
    println!("{}", "For complete guide: https://wiki.archlinux.org/title/Installation_guide".bright_blue());
}

/// Configures system hostname
fn configure_hostname() {
    show_header("Configuring system hostname...");
    
    let hostname = Input::new()
        .with_prompt("Enter hostname")
        .default("archlinux".to_string())
        .interact()
        .unwrap();
    
    simulate_command(
        &format!("echo \"{}\" > /etc/hostname", hostname),
        "Set hostname",
        true,
        || println!("Hostname set to {}", hostname.bright_green()),
    );
    
    // Simplified hosts file configuration
    simulate_command(
        &format!("echo '127.0.1.1 {}' >> /etc/hosts", hostname),
        "Configure hosts file (simplified)",
        true,
        || println!("/etc/hosts configured with hostname"),
    );
}

/// Configures system timezone
fn configure_timezone() {
    show_header("Configuring timezone...");
    
    let regions = &["Africa", "America", "Asia", "Europe", "Australia"];
    let region = regions[Select::new()
        .with_prompt("Select continent")
        .items(regions)
        .default(2)
        .interact()
        .unwrap()];
    
    let cities = match region {
        "Africa" => vec!["Cairo", "Johannesburg", "Nairobi"],
        "America" => vec!["New_York", "Chicago", "Los_Angeles"],
        "Asia" => vec!["Riyadh", "Dubai", "Tokyo"],
        "Europe" => vec!["London", "Paris", "Berlin"],
        "Australia" => vec!["Sydney", "Melbourne"],
        _ => vec!["UTC"],
    };
    
    let city = cities[Select::new()
        .with_prompt("Select city")
        .items(&cities)
        .interact()
        .unwrap()];
    
    let timezone = format!("{}/{}", region, city);
    
    simulate_command(
        &format!("ln -sf /usr/share/zoneinfo/{} /etc/localtime", timezone),
        "Set timezone",
        true,
        || println!("Timezone set to {}", timezone.bright_green()),
    );

    simulate_command("hwclock --systohc", "Sync hardware clock", true, || {
        println!("Hardware clock synced.");
    });
}

/// Configures system locale
fn configure_locale() {
    show_header("Configuring system locale...");
    
    // Use fixed English locale only
    let locale = "en_US.UTF-8 UTF-8";
    
    simulate_command(
        &format!("sed -i 's/^#{}//' /etc/locale.gen", locale),
        "Uncomment locale",
        true,
        || println!("Uncommented {}", locale.bright_green()),
    );
    
    simulate_command("locale-gen", "Generate locales", true, || {
        println!("Locales generated.");
    });
    
    let lang = locale.split_whitespace().next().unwrap();
    simulate_command(
        &format!("echo \"LANG={}\" > /etc/locale.conf", lang),
        "Set system language",
        true,
        || println!("Language set to {}", lang.bright_green()),
    );
}

/// Configures system users
fn configure_users() {
    show_header("Configuring users...");
    
    set_password("root");

    let username: String = Input::new()
        .with_prompt("Enter username for new user")
        .interact()
        .unwrap();
    
    simulate_command(&format!("useradd -mG wheel {}", username), "Create user", true, || {
        println!("User {} created.", username);
    });
    
    set_password(&username);
}

/// Configures bootloader
fn configure_bootloader() {
    show_header("Configuring bootloader...");
    
    simulate_command(
        "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=GRUB",
        "Install GRUB",
        true,
        || println!("GRUB installed."),
    );

    simulate_command(
        "grub-mkconfig -o /boot/grub/grub.cfg",
        "Generate GRUB config",
        true,
        || println!("GRUB config generated."),
    );
}

/// Installs desktop environment
fn install_desktop() {
    show_header("Installing desktop environment...");
    
    let options = &["GNOME", "Plasma", "Hyprland", "None"];
    let selection = Select::new()
        .with_prompt("Choose desktop environment")
        .items(options)
        .interact()
        .unwrap();

    if selection == 3 {
        println!("\n{}", "Skipping desktop installation.".bright_yellow());
        return;
    }

    let package = match selection {
        0 => "gnome",
        1 => "plasma",
        2 => "hyprland",
        _ => return,
    };

    simulate_command(&format!("pacman -S {}", package), &format!("Install {}", options[selection]), true, || {
        let pb = create_progress_bar(7);
        for i in 0..7 {
            pb.set_position(i);
            pb.set_message(format!("Installing {}...", options[selection]));
            sleep(1);
        }
        pb.finish_with_message("Installation complete.");
    });
}

/// Helper: Sets password for user
fn set_password(username: &str) {
    println!("\n{}", format!("Set password for {}:", username).bright_blue());
    
    // Skip command simulation for password setting
    let pwd1 = read_password_with_prompt("New password: ");
    let pwd2 = read_password_with_prompt("Retype password: ");
    
    if pwd1 == pwd2 && !pwd1.is_empty() {
        println!("{}", "Password updated.".bright_green());
    } else {
        println!("{}", "Passwords don't match or empty.".red());
        set_password(username); // Retry if passwords don't match
    }
}

/// Helper: Reads password with prompt
fn read_password_with_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap_or_default()
}

/// Helper: Simulates command execution
fn simulate_command<F>(command: &str, description: &str, in_chroot: bool, action: F)
where
    F: FnOnce(),
{
    println!("\n# {}", description.bright_blue());
    println!("[hint] type: {}", command.bright_cyan());
    
    let prompt = if in_chroot {
        format!("{}:{} # ", "root".bright_red(), "/".bright_blue())
    } else {
        format!("{}@archiso {} # ", "root".bright_red(), "~".bright_green())
    };

    let input = read_input_with_history(&prompt).unwrap_or_default();

    if input.trim() == command {
        action();
    } else {
        println!("{}", "Error: Invalid command. Try again.".red());
        simulate_command(command, description, in_chroot, action);
    }
}
