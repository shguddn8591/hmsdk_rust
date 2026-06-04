use clap::Parser;
use libc::{c_char, c_ulong};
use std::ffi::CString;

#[repr(C)]
struct Bitmask {
    size: c_ulong,
    maskp: *mut c_ulong,
}

extern "C" {
    fn numa_parse_nodestring(s: *const c_char) -> *mut Bitmask;
    fn numa_bitmask_free(bm: *mut Bitmask);
}

#[derive(Parser)]
#[command(
    name = "hmctl",
    about = "Control heterogeneous memory allocation policy for hmalloc"
)]
struct Cli {
    /// Bind memory to specified NUMA nodes (MPOL_BIND)
    #[arg(short = 'm', long, value_name = "nodes", conflicts_with_all = &["preferred", "preferred_many", "interleave", "weighted_interleave"])]
    membind: Option<String>,

    /// Prefer a single NUMA node (MPOL_PREFERRED)
    #[arg(short = 'p', long, value_name = "node", conflicts_with_all = &["membind", "preferred_many", "interleave", "weighted_interleave"])]
    preferred: Option<i32>,

    /// Prefer multiple NUMA nodes (MPOL_PREFERRED_MANY)
    #[arg(short = 'P', long, value_name = "nodes", conflicts_with_all = &["membind", "preferred", "interleave", "weighted_interleave"])]
    preferred_many: Option<String>,

    /// Interleave memory across NUMA nodes (MPOL_INTERLEAVE)
    #[arg(short = 'i', long, value_name = "nodes", conflicts_with_all = &["membind", "preferred", "preferred_many", "weighted_interleave"])]
    interleave: Option<String>,

    /// Weighted interleave across NUMA nodes (MPOL_WEIGHTED_INTERLEAVE)
    #[arg(short = 'w', long, value_name = "nodes", conflicts_with_all = &["membind", "preferred", "preferred_many", "interleave"])]
    weighted_interleave: Option<String>,

    /// Program to execute with the specified memory policy
    #[arg(required = true, last = true)]
    program: Vec<String>,
}

// MPOL mode constants matching Linux kernel values
const MPOL_PREFERRED: i32 = 1;
const MPOL_BIND: i32 = 2;
const MPOL_INTERLEAVE: i32 = 3;
const MPOL_PREFERRED_MANY: i32 = 5;
const MPOL_WEIGHTED_INTERLEAVE: i32 = 6;

unsafe fn parse_nodestring(s: &str) -> Option<u64> {
    let c_str = CString::new(s).ok()?;
    let bm = numa_parse_nodestring(c_str.as_ptr());
    if bm.is_null() {
        return None;
    }
    let mask = *(*bm).maskp;
    numa_bitmask_free(bm);
    Some(mask)
}

fn setup_environ(cli: &Cli) {
    use std::env;

    // Priority order mirrors the C hmctl setup_child_environ():
    // membind > preferred_many > preferred(>=0) > weighted_interleave > interleave
    if let Some(ref nodes) = cli.membind {
        env::set_var("HMALLOC_MPOL_MODE", MPOL_BIND.to_string());
        if let Some(mask) = unsafe { parse_nodestring(nodes) } {
            env::set_var("HMALLOC_NODEMASK", mask.to_string());
        }
    } else if let Some(ref nodes) = cli.preferred_many {
        env::set_var("HMALLOC_MPOL_MODE", MPOL_PREFERRED_MANY.to_string());
        if let Some(mask) = unsafe { parse_nodestring(nodes) } {
            env::set_var("HMALLOC_NODEMASK", mask.to_string());
        }
    } else if let Some(node) = cli.preferred.filter(|&n| n >= 0) {
        env::set_var("HMALLOC_MPOL_MODE", MPOL_PREFERRED.to_string());
        let mask: u64 = 1u64 << node;
        env::set_var("HMALLOC_NODEMASK", mask.to_string());
    } else if let Some(ref nodes) = cli.weighted_interleave {
        env::set_var("HMALLOC_MPOL_MODE", MPOL_WEIGHTED_INTERLEAVE.to_string());
        if let Some(mask) = unsafe { parse_nodestring(nodes) } {
            env::set_var("HMALLOC_NODEMASK", mask.to_string());
        }
    } else if let Some(ref nodes) = cli.interleave {
        env::set_var("HMALLOC_MPOL_MODE", MPOL_INTERLEAVE.to_string());
        if let Some(mask) = unsafe { parse_nodestring(nodes) } {
            env::set_var("HMALLOC_NODEMASK", mask.to_string());
        }
    }

    env::set_var("HMALLOC_JEMALLOC", "1");
}

fn main() {
    let cli = Cli::parse();

    setup_environ(&cli);

    let program = &cli.program[0];
    let c_program = match CString::new(program.as_str()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("hmctl: invalid program name: {}", e);
            std::process::exit(1);
        }
    };

    let c_args: Vec<CString> = cli
        .program
        .iter()
        .map(|s| CString::new(s.as_str()).expect("argument contains null byte"))
        .collect();

    let mut argv: Vec<*const c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    argv.push(std::ptr::null());

    unsafe {
        libc::execvp(c_program.as_ptr(), argv.as_ptr());
        let err = std::io::Error::last_os_error();
        eprintln!("hmctl: {}: {}", program, err);
        std::process::exit(127);
    }
}
