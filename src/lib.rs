#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use call_rcu_after_fork_child_memb as call_rcu_after_fork_child;
pub use call_rcu_after_fork_parent_memb as call_rcu_after_fork_parent;
pub use call_rcu_before_fork_memb as call_rcu_before_fork;
pub use call_rcu_data_free_memb as call_rcu_data_free;
pub use call_rcu_memb as call_rcu;
pub use create_all_cpu_call_rcu_data_memb as create_all_cpu_call_rcu_data;
pub use create_call_rcu_data_memb as create_call_rcu_data;
pub use defer_rcu_memb as defer_rcu;
pub use free_all_cpu_call_rcu_data_memb as free_all_cpu_call_rcu_data;
pub use get_call_rcu_data_memb as get_call_rcu_data;
pub use get_call_rcu_thread_memb as get_call_rcu_thread;
pub use get_cpu_call_rcu_data_memb as get_cpu_call_rcu_data;
pub use get_default_call_rcu_data_memb as get_default_call_rcu_data;
pub use get_thread_call_rcu_data_memb as get_thread_call_rcu_data;
pub use rcu_barrier_memb as rcu_barrier;
pub use rcu_defer_barrier_memb as rcu_defer_barrier;
pub use rcu_defer_barrier_thread_memb as rcu_defer_barrier_thread;
pub use rcu_defer_register_thread_memb as rcu_defer_register_thread;
pub use rcu_defer_unregister_thread_memb as rcu_defer_unregister_thread;
pub use rcu_init_memb as rcu_init;
pub use rcu_read_lock_memb as rcu_read_lock;
pub use rcu_read_ongoing_memb as rcu_read_ongoing;
pub use rcu_read_unlock_memb as rcu_read_unlock;
pub use rcu_register_thread_memb as rcu_register_thread;
pub use rcu_unregister_thread_memb as rcu_unregister_thread;
pub use set_cpu_call_rcu_data_memb as set_cpu_call_rcu_data;
pub use set_thread_call_rcu_data_memb as set_thread_call_rcu_data;
pub use synchronize_rcu_memb as synchronize_rcu;
pub use urcu_register_rculfhash_atfork_memb as urcu_register_rculfhash_atfork;
pub use urcu_unregister_rculfhash_atfork_memb as urcu_unregister_rculfhash_atfork;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        unsafe {
            rcu_register_thread();
            rcu_read_lock();
            rcu_read_unlock();
            rcu_unregister_thread();
        }
    }
}