use winreg::enums::*;

// HKEY_CLASSES_ROOT: 0x80000000
// HKEY_CURRENT_USER: 0x80000001
// HKEY_LOCAL_MACHINE: 0x80000002
// HKEY_USERS: 0x80000003
// HKEY_CURRENT_CONFIG: 0x80000005

static _HKCU: isize = 0x80000001;
static HKLM: isize = 0x80000002;

fn main() -> Result<(), std::io::Error> {
    //  let args: Vec<String> = env::args().collect();
    //  let hkcu = RegKey::predef(HKCU);
    let hklm = winreg::RegKey::predef(HKLM);
    let path = r"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\BlackDesert64.exe\\PerfOptions";
    let key = hklm.open_subkey_with_flags(path, KEY_WRITE)?;
    let dword_val: u32 = 00000003;
    key.set_value("CpuPriorityClass", &dword_val)?;
    Ok(())
}




// https://docs.rs/winreg/latest/winreg/enums/enum.RegType.html
// https://docs.rs/winreg/latest/winreg/enums/enum.RegType.html
// https://docs.rs/winreg/latest/winreg/enums/			
