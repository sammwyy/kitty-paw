pub mod mem_active_level_4_table;
pub mod mem_boot_info_frame_allocator;
pub mod mem_example_mapping;

use self::mem_active_level_4_table::active_level_4_table;
use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
