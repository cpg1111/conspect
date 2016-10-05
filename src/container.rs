use std::fs;
use cgroup_reader;
use cgroup_reader::CGroup_Reader
use ns_reader;
use ns_reader::NS_Group;
use process;
use process::Process;

pub mod container {
    fn load_cgroups(procs Vec<Process>) -> Vec<CGroup_Reader> {
        let cgroups = Vec<CGroup_Reader>::new()
        for p in procs {
            cgroups.push(cgroup_reader::new(p.pid));
        }
        cgroup
    }

    pub struct Container {
        processes: Vec<Process>,
        namespaces: Vec<String>,
        cgroups: Vec<CGroup_Reader>,
        update_intv: i32
    }

    impl Container {
        fn update(&self) {
            loop {
                
            }
        }
    }

    pub fn new(groups Vec<NS_Group>) -> Container {
        (namespaces, processes) = groups;
        return Container{
            processes: processes,
            namespaces: namespaces,
            cgroups: load_cgroups(processes),
        }
    }

}
