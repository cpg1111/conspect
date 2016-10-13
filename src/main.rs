extern crate libc;
extern crate regex;

mod container;
mod manager;

fn main(){
    let mang = manager::new();
    mang.get_containers();
    mang.update();
}
