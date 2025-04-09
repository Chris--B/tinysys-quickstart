#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

extern crate alloc;
// link against panic handler
extern crate panic_halt;
// link against critical section impl
extern crate riscv;

use tinysys_rs::prelude::*;
use tinysys_rs::sys::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// App entry point
fn main() -> ! {
    unsafe {
        let mut video_context = EVideoContext {
            m_vmode: EVideoMode_EVM_320_Wide,
            m_cmode: EColorMode_ECM_8bit_Indexed,
            ..Default::default()
        };
        VPUSetVMode(&mut video_context, EVideoScanoutEnable_EVS_Enable);

        // Note: There is no way to deallocate this buffer currently
        let p_vpu_mem: *mut u8 = VPUAllocateBuffer(WIDTH * HEIGHT);
        let framebuffer: &mut [u8] =
            core::slice::from_raw_parts_mut(p_vpu_mem, (WIDTH * HEIGHT) as usize);

        VPUSetWriteAddress(&mut video_context, p_vpu_mem as u32);
        VPUSetScanoutAddress(&mut video_context, p_vpu_mem as u32);

        // Build a palette that looks a lot like 8-bit RGB332
        for i in 0..256 {
            // Shift out the 2 or 3 bits from `i` into the botton 4 bits of each byte.
            let r = ((i >> 5) & 0b_111) << 1;
            let g = ((i >> 3) & 0b_111) << 1;
            let b = ((i >> 0) & 0b_011) << 2;

            VPUSetPal(i as u8, r, g, b);
        }

        // The palette id (i in above loop) that maps to rgb(0, 0, 0)
        const PAL_BLACK: u8 = 0;

        loop {
            // Note: Each byte is 4 pixels worth of palette indexing
            VPUClear(&mut video_context, u32::from_be_bytes([PAL_BLACK; 4]));

            for frame in 0.. {
                if frame % 30 == 0 {
                    dbg!(frame);
                }

                let mut pixel_idx = 0;
                for _y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        framebuffer[pixel_idx as usize] = x.min(255) as u8;
                        pixel_idx += 1;
                    }
                }
            }

            // Must flush CPU data cache after framebuffer write
            CFLUSH_D_L1();

            VPUWaitVSync();
        }
    }
}

/// The entry point loaded by the system
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Optionally initialize a heap.
    // The quickstart example does this because the tinysys system is generally capable of it.
    {
        use core::mem::MaybeUninit;

        #[global_allocator]
        static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();
        const HEAP_SIZE: usize = 4 * 1024 * 1024;

        unsafe {
            #![allow(static_mut_refs)]

            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
        }
    }

    main()
}
