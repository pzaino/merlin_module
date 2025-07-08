#[repr(C)]
pub struct MerlinModuleHeader {
    pub magic: u32,
    pub version: u32,
    pub module_name: [u8; 12],
    pub help_string: *const u8,
    pub init: ModuleFn,
    pub finalise: ModuleFn,
    pub service_call: ServiceFn,
    pub swi_base: u32,
    pub swi_table: *const SWITableEntry,
    pub workspace_size: u32,
    pub flags: u32,
    pub abi_version: u32,
    pub reserved: [u8; 32],
}

pub type ModuleFn = extern "C" fn() -> i32;
pub type ServiceFn = extern "C" fn(u32, *mut u32) -> i32;

#[repr(C)]
pub struct SWITableEntry {
    pub swi_offset: u32,
    pub handler: extern "C" fn(),
}
