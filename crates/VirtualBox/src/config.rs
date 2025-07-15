//! Configuration management for VirtualBox plugin

#[derive(Clone)]
pub struct VirtualBoxConfig {
    pub vboxmanage_path: String,
    pub default_vm_folder: Option<String>,
    pub max_concurrent_vms: u32,
}

impl VirtualBoxConfig {
    pub fn new() -> Self {
        Self {
            vboxmanage_path: Self::find_vboxmanage(),
            default_vm_folder: None,
            max_concurrent_vms: 10,
        }
    }

    /// Find VBoxManage executable path
    fn find_vboxmanage() -> String {
        let common_paths = if cfg!(windows) {
            vec![
                "C:\\Program Files\\Oracle\\VirtualBox\\VBoxManage.exe",
                "C:\\Program Files (x86)\\Oracle\\VirtualBox\\VBoxManage.exe",
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                "/Applications/VirtualBox.app/Contents/MacOS/VBoxManage",
                "/usr/local/bin/VBoxManage",
            ]
        } else {
            vec!["/usr/bin/VBoxManage", "/usr/local/bin/VBoxManage"]
        };

        for path in common_paths {
            if std::path::Path::new(path).exists() {
                return path.to_string();
            }
        }

        "VBoxManage".to_string()
    }

    pub fn get_vboxmanage_path(&self) -> &str {
        &self.vboxmanage_path
    }

    pub fn get_max_concurrent_vms(&self) -> u32 {
        self.max_concurrent_vms
    }

    pub fn get_default_vm_folder(&self) -> Option<&String> {
        self.default_vm_folder.as_ref()
    }

    pub fn set_default_vm_folder(&mut self, folder: Option<String>) {
        self.default_vm_folder = folder;
    }

    pub fn set_max_concurrent_vms(&mut self, max: u32) {
        self.max_concurrent_vms = max;
    }
}