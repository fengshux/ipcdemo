use std::mem;
use std::ptr;
use libc::*;
use std::ffi::CString;
use std::{thread, time};

#[repr(C)]
struct ShmData {
    data :[i32;128],
    length: i32,
}

fn main() {
    unsafe {
        // 创建唯一key;
        let cstr = CString::new("/tmp/sharememory/sharememorykey").unwrap();
        let key = ftok(cstr.as_ptr() as *const i8, 1024);
        if key  < 0 {
            println!("{}",key);
            panic!("ftok error");            
        }
        // 创建虚拟内存
        println!("{:?}", key);
        let id = shmget(key, mem::size_of::<ShmData>(), IPC_CREAT|0777);
        // 挂载内存地址
        let ptr = shmat(id, ptr::null(), 0);
        println!("{:?}", ptr);
        let shmdata: *mut ShmData = ptr as *mut ShmData;
        (*shmdata).data[0] = 2;
        (*shmdata).length = 1;

        // let ten_millis = time::Duration::from_secs(60);
        // thread::sleep(ten_millis);
        //shmdt(id, shmdata as *mut libc::c_void);
        shmdt(ptr);
    }    
}
