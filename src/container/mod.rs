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
                println!("{}", cgroup.read());
            }
        }
    }
}

pub fn new(groups: Vec<ns_reader::NS_Group>) -> Container {
    let (namespaces, processes) = groups;
    return Container{
        processes: processes,
        namespaces: namespaces,
        cgroups: load_cgroups(processes),
    }
}
