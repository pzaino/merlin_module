#[macro_export]
macro_rules! define_module {
    (
        name = $name:expr,
        help = $help:expr,
        version = $version:expr,
        swi_base = $swi_base:expr,
        swi_table = $swi_table:expr,
        workspace_size = $workspace_size:expr,
        flags = $flags:expr
    ) => {
        #[link_section = ".header"]
        #[no_mangle]
        pub static __MERLIN_MODULE_HEADER: $crate::MerlinModuleHeader =
            $crate::MerlinModuleHeader {
                magic: 0x4D4F4455,
                version: $version,
                module_name: {
                    let mut name = [0u8; 12];
                    let src = $name;
                    let len = core::cmp::min(src.len(), 12);
                    name[..len].copy_from_slice(&src[..len]);
                    name
                },
                help_string: $help.as_ptr(),
                init,
                finalise,
                service_call,
                swi_base: $swi_base,
                swi_table: $swi_table,
                workspace_size: $workspace_size,
                flags: $flags,
                abi_version: $crate::consts::MERLIN_ABI_VERSION,
                reserved: [0; 32],
            };
    };
}
