use axhal::{arch::TrapFrame, mem::VirtAddr, paging::MappingFlags};
// use axlog::info;

use crate::syscall::syscall;

struct TrapHandlerImpl {
    // count: usize,
    // port: [(u16, u16); 999],
}

#[crate_interface::impl_interface]
impl axhal::trap::TrapHandler for TrapHandlerImpl {
    fn handle_irq(_irq_num: usize) {
        #[cfg(feature = "irq")]
        {
            let guard = kernel_guard::NoPreempt::new();
            // trap进来，统计时间信息
            // 只有当trap是来自用户态才进行统计
            axprocess::time_stat_from_user_to_kernel();
            axhal::irq::dispatch_irq(_irq_num);
            axprocess::time_stat_from_kernel_to_user();
            drop(guard); // rescheduling may occur when preemption is re-enabled.
        }
    }

    fn handle_syscall(syscall_id: usize, args: [usize; 6]) -> isize {
        axprocess::time_stat_from_user_to_kernel();
        let ans = syscall(syscall_id, args);
        // if syscall_id == 203 {
        //     info!("!@# catch connect: {}", unsafe {
        //         syscall_net::socket::socket_address_from(args[1] as *const u8)
        //     });
        // }
        // if syscall_id == 208 {
        //     info!("!@# catch SETSOCKOPT: {:?}", unsafe {
        //         from_raw_parts(args[3] as *const u8, args[4] as u32 as usize)
        //     });
        // }
        axprocess::time_stat_from_kernel_to_user();
        // current().update_timer();
        ans
    }

    #[cfg(feature = "paging")]
    fn handle_page_fault(addr: VirtAddr, flags: MappingFlags, tf: &mut TrapFrame) {
        use axprocess::handle_page_fault;
        use syscall_utils::deal_result;

        axprocess::time_stat_from_user_to_kernel();

        #[cfg(feature = "signal")]
        if addr.as_usize() == axsignal::SIGNAL_RETURN_TRAP {
            use syscall_task::syscall_sigreturn;
            // 说明是信号执行完毕，此时应当执行sig return
            tf.regs.a0 = deal_result(syscall_sigreturn()) as usize;
            return;
        }
        handle_page_fault(addr, flags);
        axprocess::time_stat_from_kernel_to_user();
    }

    #[cfg(feature = "signal")]
    fn handle_signal() {
        axprocess::signal::handle_signals();
    }
}
