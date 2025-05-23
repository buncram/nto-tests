use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

const TARGET: &str = "riscv32imac-unknown-none-elf";

#[derive(Debug)]
#[allow(dead_code)]
enum BuildError {
    #[allow(dead_code)]
    PathConversionError,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BuildError::PathConversionError => write!(f, "could not convert path to UTF-8"),
        }
    }
}

impl std::error::Error for BuildError {}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let hw_pkgs = ["tests"];
    let mut args = env::args();
    let task = args.nth(1);
    let features = get_flag("--feature")?;
    match task.as_deref() {
        Some("boot-image") => build_hw_image(false, features, &hw_pkgs)?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
boot-image     builds a boot image
"
    )
}

fn build_hw_image(debug: bool, features: Vec<String>, packages: &[&str]) -> Result<(), DynError> {
    // make the ELF file
    let mut boot = build(packages, debug, Some(TARGET), Some("tests".into()), features)?;

    boot.push("tests");
    println!("debug: boot path: {}", boot.as_os_str().to_str().unwrap());
    let mut boot_bin = project_root();
    boot_bin.push("rv32.bin");

    // dump the ELF file
    let listing = std::fs::File::create("rv32.map")?;
    let status = Command::new(objdump())
        .current_dir(project_root())
        .args(&["-d", "-S", boot.as_os_str().to_str().unwrap()])
        .stdout(std::process::Stdio::from(listing))
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    // output the binary
    let status = Command::new(objcopy())
        .current_dir(project_root())
        .args(&[
            "-S",
            "-O",
            "binary",
            boot.as_os_str().to_str().unwrap(),
            boot_bin.as_os_str().to_str().unwrap(),
        ])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    // Pad to multiple of 32 bytes length
    let mut file = std::fs::OpenOptions::new().read(true).write(true).open(boot_bin)?;
    use std::io::{Seek, Write};
    // Get the current size of the file
    let current_size = file.seek(std::io::SeekFrom::End(0))?;
    let padding_needed = (32 - (current_size % 32)) % 32;

    // Pad the file with zeros if needed
    if padding_needed > 0 {
        let padding = vec![0u8; padding_needed as usize];
        file.write_all(&padding)?;
    }
    println!();
    println!("Bootloader binary file created at {}", boot.as_os_str().to_str().unwrap());

    Ok(())
}

fn build(
    packages: &[&str],
    debug: bool,
    target: Option<&str>,
    directory: Option<PathBuf>,
    features: Vec<String>,
) -> Result<PathBuf, DynError> {
    let stream = if debug { "debug" } else { "release" };
    let mut args = vec!["build"];
    print!("Building");

    args.push("--no-default-features");
    args.push("--features");
    args.push("aes-zkn");

    for package in packages {
        print!(" {}", package);
        args.push("--package");
        args.push(package);
    }
    println!();
    let mut target_path = "".to_owned();
    if let Some(t) = target {
        args.push("--target");
        args.push(t);
        target_path = format!("{}/", t);
    }

    if !debug {
        args.push("--release");
    }

    if features.len() > 0 {
        for feature in features.iter() {
            args.push("--features");
            args.push(feature);
        }
    }

    let mut dir = project_root();
    if let Some(subdir) = &directory {
        dir.push(subdir);
    }

    // emit debug
    print!("    Command: cargo");
    for &arg in args.iter() {
        print!(" {}", arg);
    }
    println!();

    let status = Command::new(cargo()).current_dir(dir).args(&args).status()?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    Ok(project_root().join(&format!("target/{}{}/", target_path, stream)))
}

fn cargo() -> String { env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()) }

fn objcopy() -> String { env::var("OBJCOPY").unwrap_or_else(|_| "riscv-none-elf-objcopy".to_string()) }
fn objdump() -> String { env::var("OBJDUMP").unwrap_or_else(|_| "riscv-none-elf-objdump".to_string()) }

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

fn get_flag(flag: &str) -> Result<Vec<String>, DynError> {
    let mut list = Vec::<String>::new();
    let args = env::args();
    let mut flag_found = false;
    for arg in args {
        if arg == flag {
            flag_found = true;
            continue;
        }
        if flag_found {
            if arg.starts_with('-') {
                eprintln!("Malformed arguments. Expected argument after flag {}, but found {}", flag, arg);
                return Err("Bad arguments".into());
            }
            list.push(arg);
            flag_found = false;
            continue;
        }
    }
    Ok(list)
}
