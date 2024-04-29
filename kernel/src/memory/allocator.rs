use crate::config;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    let page_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let heap_start = VirtAddr::new(config::HEAP_START as u64);
    let heap_end = heap_start + config::HEAP_SIZE as u64;

    const BATCH_SIZE: usize = 32;

    let mut batch_pages: [Option<Page<Size4KiB>>; BATCH_SIZE] = Default::default();
    let mut batch_frames: [Option<PhysFrame>; BATCH_SIZE] = Default::default();

    for page_addr in (heap_start..=heap_end).step_by(Size4KiB::SIZE as usize * BATCH_SIZE as usize)
    {
        let batch_end_addr = page_addr + Size4KiB::SIZE as usize * (BATCH_SIZE as usize - 1);
        let batch_end = heap_end.min(batch_end_addr);

        for (i, page) in Page::range_inclusive(
            Page::containing_address(page_addr),
            Page::containing_address(batch_end),
        )
        .enumerate()
        {
            batch_pages[i] = Some(page);
        }

        for i in 0..BATCH_SIZE {
            if let Some(frame) = frame_allocator.allocate_frame() {
                batch_frames[i] = Some(frame);
            } else {
                return Err("Frame allocation failed");
            }
        }

        for (page_opt, frame_opt) in batch_pages.iter().zip(batch_frames.iter()) {
            if let (Some(page), Some(frame)) = (page_opt, frame_opt) {
                unsafe {
                    let _ = mapper.map_to(*page, *frame, page_flags, frame_allocator);
                };
            }
        }

        batch_pages = Default::default();
        batch_frames = Default::default();
    }

    unsafe {
        ALLOCATOR
            .lock()
            .init(config::HEAP_START as *mut u8, config::HEAP_SIZE);
    }

    Ok(())
}
