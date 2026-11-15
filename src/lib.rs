#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// A HAMMER2 volume descriptor.
#[derive(Debug, Clone)]
pub struct Hammer2Volume {
    pub label: String,
    pub size_mb: u64,
    pub blocksize: u32,
}

impl Hammer2Volume {
    /// Create a new `Hammer2Volume` with the default blocksize of 65536.
    pub fn new(label: &str, size_mb: u64) -> Self {
        Self {
            label: label.to_string(),
            size_mb,
            blocksize: 65536,
        }
    }

    /// Return the `newfs_hammer2` command string.
    pub fn newfs_hammer2_command(&self) -> String {
        format!("newfs_hammer2 -L {} /dev/vn0", self.label)
    }

    /// Return a human-readable volume description.
    pub fn volume_description(&self) -> String {
        format!(
            "HAMMER2 volume '{}' {}MB bs={}",
            self.label, self.size_mb, self.blocksize
        )
    }
}

/// Configuration for a DragonFly BSD vkernel instance.
#[derive(Debug, Clone)]
pub struct VkernelConfig {
    pub memory_mb: u64,
    pub ncpus: u8,
    pub root_image: String,
}

impl VkernelConfig {
    /// Create a new `VkernelConfig`, clamping ncpus to a minimum of 1.
    pub fn new(memory_mb: u64, ncpus: u8, root_image: &str) -> Self {
        Self {
            memory_mb,
            ncpus: ncpus.max(1),
            root_image: root_image.to_string(),
        }
    }

    /// Return the `vkernel` launch command string.
    pub fn vkernel_launch_command(&self) -> String {
        format!(
            "vkernel -m {}m -c {} -r {}",
            self.memory_mb, self.ncpus, self.root_image
        )
    }
}

/// A HAMMER2 PFS (pseudo-filesystem) entry.
#[derive(Debug, Clone)]
pub struct HammerPfs {
    pub name: String,
    pub pfs_type: String,
}

impl HammerPfs {
    /// Create a new `HammerPfs`.
    pub fn new(name: &str, pfs_type: &str) -> Self {
        Self {
            name: name.to_string(),
            pfs_type: pfs_type.to_string(),
        }
    }

    /// Return the `hammer2 pfs-create` command string.
    pub fn pfs_create_command(&self) -> String {
        format!("hammer2 pfs-create {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_volume() -> Hammer2Volume {
        Hammer2Volume::new("DATA", 10240)
    }

    fn make_vkernel() -> VkernelConfig {
        VkernelConfig::new(2048, 4, "/var/vkernel/root.img")
    }

    fn make_pfs() -> HammerPfs {
        HammerPfs::new("pfs-data", "Master")
    }

    // Arrange + Act + Assert: newfs_hammer2_command
    #[test]
    fn test_newfs_command() {
        let vol = make_volume();
        let cmd = vol.newfs_hammer2_command();
        assert_eq!(cmd, "newfs_hammer2 -L DATA /dev/vn0");
    }

    // Arrange + Act + Assert: volume_description
    #[test]
    fn test_volume_description() {
        let vol = make_volume();
        let desc = vol.volume_description();
        assert_eq!(desc, "HAMMER2 volume 'DATA' 10240MB bs=65536");
    }

    // Arrange + Act + Assert: vkernel_launch_command
    #[test]
    fn test_vkernel_launch() {
        let vk = make_vkernel();
        let cmd = vk.vkernel_launch_command();
        assert_eq!(cmd, "vkernel -m 2048m -c 4 -r /var/vkernel/root.img");
    }

    // Arrange + Act + Assert: pfs_create_command
    #[test]
    fn test_pfs_create() {
        let pfs = make_pfs();
        let cmd = pfs.pfs_create_command();
        assert_eq!(cmd, "hammer2 pfs-create pfs-data");
    }

    // Arrange + Act + Assert: default blocksize is 65536
    #[test]
    fn test_blocksize_default() {
        let vol = Hammer2Volume::new("BOOT", 512);
        assert_eq!(vol.blocksize, 65536);
    }

    // Arrange + Act + Assert: memory_mb is stored correctly (rounding handled by caller)
    #[test]
    fn test_memory_stored() {
        let vk = VkernelConfig::new(4096, 2, "/img/root.img");
        assert_eq!(vk.memory_mb, 4096);
        let cmd = vk.vkernel_launch_command();
        assert!(cmd.contains("4096m"));
    }

    // Arrange + Act + Assert: ncpus clamped to minimum of 1
    #[test]
    fn test_ncpus_bounds() {
        let vk = VkernelConfig::new(1024, 0, "/img/root.img");
        assert_eq!(vk.ncpus, 1);
        let cmd = vk.vkernel_launch_command();
        assert!(cmd.contains("-c 1"));
    }
}
