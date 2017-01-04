use std::fs;
use std::io;

pub mod cgroup_reader;
pub mod ns_reader;
pub mod process;

fn load_cgroups(procs: Vec<process::Process>) -> Vec<cgroup_reader::Reader> {
    let cgroups = Vec::<cgroup_reader::Reader>::new();
    for p in procs {
        cgroups.push(cgroup_reader::new(p.pid));
    }
    cgroups
}

pub struct Container {
    processes: Vec<process::Process>,
    namespaces: Vec<String>,
    cgroups: Vec<cgroup_reader::Reader>,
    update_intv: i32
}

impl Container {
    fn update(&self) {
        loop {
            for cgroup in self.cgroups {
                let kvs = cgroup.read();
                for kv in kvs {
                    let (key, val) = kv;
                    println!("{} : {}", key, val);
                }
            }
        }
    }
}

pub fn new(group: ns_reader::NS_Group) -> Container {
    let (namespaces, process) = group;
    return Container{
        processes: process,
        namespaces: namespaces,
        cgroups: load_cgroups(process),
        update_intv: 1,
    }
}
