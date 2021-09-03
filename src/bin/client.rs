use std::mem;
use std::ptr;
use libc::*;
use std::ffi::CString;

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
        println!("{:?}", key);
        // 创建虚拟内存
        let id = shmget(key, mem::size_of::<ShmData>(), IPC_CREAT|0777);
        // 挂载内存地址
        let ptr = shmat(id, ptr::null(), 0);
        let shmdata: *const ShmData = ptr as *const ShmData;
        let data = (*shmdata).data[0];
        let length = (*shmdata).length;
        println!("data:{}, length:{}", data, length);
        shmdt(shmdata as *mut libc::c_void);
    }    
}
