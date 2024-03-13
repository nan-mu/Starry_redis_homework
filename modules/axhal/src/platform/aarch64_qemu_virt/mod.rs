pub mod misc {
    pub use crate::platform::aarch64_common::psci::system_off as terminate;
}

// pub mod console {
//     use arm_pl011::pl011::Pl011Uart;
//     use memory_addr::PhysAddr;
//     use spinlock::SpinNoIrq;

//     use crate::mem::phys_to_virt;

//     const UART_BASE: PhysAddr = PhysAddr::from(axconfig::UART_PADDR);

//     static UART: SpinNoIrq<Pl011Uart> =
//         SpinNoIrq::new(Pl011Uart::new(phys_to_virt(UART_BASE).as_mut_ptr()));

//     /// Writes a byte to the console.
//     pub fn putchar(c: u8) {
//         let mut uart = UART.lock();
//         match c {
//             b'\n' => {
//                 uart.putchar(b'\r');
//                 uart.putchar(b'\n');
//             }
//             c => uart.putchar(c),
//         }
//     }

//     /// Reads a byte from the console, or returns [`None`] if no input is available.
//     pub fn getchar() -> Option<u8> {
//         UART.lock().getchar()
//     }
// }
