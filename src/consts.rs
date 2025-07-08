pub const MERLIN_ABI_VERSION_MAJ: u32 = 1;
pub const MERLIN_ABI_VERSION_MIN: u32 = 0;
pub const MERLIN_ABI_VERSION_REV: u32 = 0;
pub const MERLIN_ABI_VERSION: u32 =
    (MERLIN_ABI_VERSION_MAJ << 16) | (MERLIN_ABI_VERSION_MIN << 8) | MERLIN_ABI_VERSION_REV;
pub const MERLIN_MODULE_MAGIC: u32 = 0x4D4F4455; // "MODU" in ASCII
pub const MERLIN_MODULE_NAME_MAX_LEN: usize = 12;
pub const MERLIN_MODULE_HELP_MAX_LEN: usize = 256;
pub const MERLIN_MODULE_WORKSPACE_SIZE: usize = 1024; // Default workspace size
pub const MERLIN_MODULE_FLAGS: u32 = 0; // Default flags, can be extended later
