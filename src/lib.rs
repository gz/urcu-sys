#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::os::raw::{c_int, c_ulong};
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "linux")]
mod linux_reexports {
    pub use super::*;
    pub use rcu_flavor_memb as rcu_flavor;

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
}

#[cfg(target_os = "linux")]
pub use linux_reexports::*;

#[cfg(target_os = "macos")]
mod macos_reexports {
    pub use super::*;
    pub use urcu_memb_flavor as rcu_flavor;

    pub use urcu_memb_call_rcu as call_rcu;
    pub use urcu_memb_call_rcu_after_fork_child as call_rcu_after_fork_child;
    pub use urcu_memb_call_rcu_after_fork_parent as call_rcu_after_fork_parent;
    pub use urcu_memb_call_rcu_before_fork as call_rcu_before_fork;
    pub use urcu_memb_call_rcu_data_free as call_rcu_data_free;
    pub use urcu_memb_create_all_cpu_call_rcu_data as create_all_cpu_call_rcu_data;
    pub use urcu_memb_create_call_rcu_data as create_call_rcu_data;
    pub use urcu_memb_defer_rcu as defer_rcu;
    pub use urcu_memb_free_all_cpu_call_rcu_data as free_all_cpu_call_rcu_data;
    pub use urcu_memb_get_call_rcu_data as get_call_rcu_data;
    pub use urcu_memb_get_call_rcu_thread as get_call_rcu_thread;
    pub use urcu_memb_get_cpu_call_rcu_data as get_cpu_call_rcu_data;
    pub use urcu_memb_get_default_call_rcu_data as get_default_call_rcu_data;
    pub use urcu_memb_get_thread_call_rcu_data as get_thread_call_rcu_data;

    pub use urcu_memb_barrier as rcu_barrier;
    pub use urcu_memb_defer_barrier as rcu_defer_barrier;
    pub use urcu_memb_defer_barrier_thread as rcu_defer_barrier_thread;
    pub use urcu_memb_defer_register_thread as rcu_defer_register_thread;
    pub use urcu_memb_defer_unregister_thread as rcu_defer_unregister_thread;
    pub use urcu_memb_init as rcu_init;
    pub use urcu_memb_read_lock as rcu_read_lock;
    pub use urcu_memb_read_ongoing as rcu_read_ongoing;
    pub use urcu_memb_read_unlock as rcu_read_unlock;
    pub use urcu_memb_register_thread as rcu_register_thread;
    pub use urcu_memb_set_cpu_call_rcu_data as set_cpu_call_rcu_data;
    pub use urcu_memb_set_thread_call_rcu_data as set_thread_call_rcu_data;
    pub use urcu_memb_synchronize_rcu as synchronize_rcu;
    pub use urcu_memb_unregister_thread as rcu_unregister_thread;

    pub use urcu_memb_register_rculfhash_atfork as urcu_register_rculfhash_atfork;
    pub use urcu_memb_unregister_rculfhash_atfork as urcu_unregister_rculfhash_atfork;
}

#[cfg(target_os = "macos")]
pub use macos_reexports::*;

pub unsafe fn cds_lfht_new(
    init_size: c_ulong,
    min_nr_alloc_buckets: c_ulong,
    max_nr_buckets: c_ulong,
    flags: c_int,
    attr: *mut pthread_attr_t,
) -> *mut cds_lfht {
    _cds_lfht_new(
        init_size,
        min_nr_alloc_buckets,
        max_nr_buckets,
        flags,
        ptr::null_mut(),
        &rcu_flavor,
        attr,
    )
}

pub unsafe fn cds_lfht_iter_get_node(iter: *mut cds_lfht_iter) -> *mut cds_lfht_node {
    (*iter).node
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::os::raw::c_void;
    use std::ptr;

    #[test]
    fn sanity_check() {
        unsafe {
            rcu_register_thread();
            rcu_read_lock();
            rcu_read_unlock();
            rcu_unregister_thread();
        }
    }

    #[repr(C)]
    struct lfht_test_node {
        node: cds_lfht_node,
        key: u64,
        data: u64,
        /* cache-cold for iteration */
        head: rcu_head,
    }

    unsafe fn to_test_node(node: *mut cds_lfht_node) -> *mut lfht_test_node {
        mem::transmute(node)
    }

    unsafe extern "C" fn test_match(node: *mut cds_lfht_node, key: *const c_void) -> i32 {
        let my_key = key as u64;
        let test_node: *mut lfht_test_node = to_test_node(node);

        (my_key == (*test_node).key) as i32
    }

    #[test]
    fn test_hashtable() {
        unsafe {
            let test_ht: *mut cds_lfht =
                cds_lfht_new(4096, 16, 0, CDS_LFHT_AUTO_RESIZE as i32, ptr::null_mut());
            assert_ne!(test_ht, ptr::null_mut());

            let key_12: u64 = 12;
            let hash_12: u64 = 12;

            rcu_register_thread();
            rcu_read_lock();

            // Insert a key
            let mut node_12 = lfht_test_node {
                node: mem::MaybeUninit::zeroed().assume_init(),
                key: key_12,
                data: 99,
                head: mem::MaybeUninit::zeroed().assume_init(),
            };
            cds_lfht_add(test_ht, hash_12, &mut node_12.node as *mut cds_lfht_node);

            // Try to find non-existent key
            let mut iter: cds_lfht_iter = mem::MaybeUninit::zeroed().assume_init();
            cds_lfht_lookup(
                test_ht,
                0xdead,
                Some(test_match),
                0xdead as *const c_void,
                &mut iter as *mut cds_lfht_iter,
            );
            let found_node: *mut cds_lfht_node = cds_lfht_iter_get_node(&mut iter);
            assert_eq!(found_node, ptr::null_mut());

            // Try to find existing key
            cds_lfht_lookup(
                test_ht,
                hash_12,
                Some(test_match),
                key_12 as *const c_void,
                &mut iter as *mut cds_lfht_iter,
            );
            let found_node: *mut cds_lfht_node = cds_lfht_iter_get_node(&mut iter);
            assert_ne!(found_node, ptr::null_mut());
            assert_eq!((*to_test_node(found_node)).data, 99);

            // Overwrite existing key
            let mut node_12_new = lfht_test_node {
                node: mem::MaybeUninit::zeroed().assume_init(),
                key: key_12,
                data: 101,
                head: mem::MaybeUninit::zeroed().assume_init(),
            };
            let old_node: *mut cds_lfht_node = cds_lfht_add_replace(
                test_ht,
                hash_12,
                Some(test_match),
                key_12 as *const c_void,
                &mut node_12_new.node as *mut cds_lfht_node,
            );
            assert_ne!(old_node, ptr::null_mut());
            assert_eq!((*to_test_node(old_node)).data, 99);

            // Check if it got updated
            cds_lfht_lookup(
                test_ht,
                hash_12,
                Some(test_match),
                key_12 as *const c_void,
                &mut iter as *mut cds_lfht_iter,
            );
            let found_node: *mut cds_lfht_node = cds_lfht_iter_get_node(&mut iter);
            assert_ne!(found_node, ptr::null_mut());
            assert_eq!((*to_test_node(found_node)).data, 101);

            rcu_read_unlock();
            rcu_unregister_thread();
        }
    }
}
