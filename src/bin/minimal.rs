#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use test_app as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = stm32l4xx_hal::pac,
    dispatchers = [EXTI0]
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        test_uint: u8,
        test_bool: bool,
    }

    // Local resources go here
    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        task1::spawn().ok();

        (
            Shared {
                test_uint: 254,
                test_bool: false,
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task(priority=1, shared = [test_uint, test_bool])]
    async fn task1(_cx: task1::Context) {
        let uint = _cx.shared.test_uint;
        defmt::info!("Hello from task1!");
    }
}
