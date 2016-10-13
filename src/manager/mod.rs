use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::thread;
use libc::uid_t;
use libc::getuid;
use container;
use container::Container;
use container::ns_reader;
use container::ns_reader::NS_Group;
use container::process::Process;

pub struct Manager {
    user: uid_t,
    containers: Vec<Container>,
    ext_ns_reader: ns_reader::Reader,
}

impl Manager {
    fn walk_proc(&self, cb: Fn(&DirEntry)) {
        let proc_dir = Path::new("/proc/");
        if proc_dir.is_dir() {
            for entry in try!(fs::read_dir(proc_dir)) {
                let entry = try!(entry);
                cb(&entry);
            }
        }
    }

    fn match_ns(&self, process: &Process, reader: ns_reader::Reader, groups: Vec<NS_Group>) -> Vec<NS_Group> {
        let new_group = vec![
            reader.cpu.read(),
            reader.ipc.read(),
            reader.mnt.read(),
            reader.net.read(),
            reader.pid.read(),
            reader.uts.read(),
        ];
        if groups.capcity() == 0 {
            groups.push(new_group, vec![process]);
            return groups;
        }
        for (group, idx) in groups {
            let (namespaces, procs) = group;
            let mut is_match = true;
            for (ns, i) in namespaces {
                if ns != new_group[i] {
                    is_match = false;
                }
            }
            if is_match {
                procs.push(process);
                groups[idx] = (namespaces, procs);
                break;
            }
        }
        groups
    }

    fn get_process_groups(&self) -> Vec<NS_Group> {
        let mut procs = Vec::<NS_Group>::new();
        self.walk_proc(|entry|{
            let path = entry.strip_prefix("/proc/");
            let path_str = path.to_str();
            let p_proc = Process::new(String::new(path_str));
            let ns = ns_reader::new(p_proc.pid);
            procs = self.match_ns(&p_proc, ns, procs);
        });
        procs;
    }

    fn get_containers(&self) {
        let groups = self.get_process_groups();
        for group in groups {
            self.containers.push(container::new(group));
        }
    }

    fn update(&self) {
        for container in self.containers {
            thread::spawn(|| {
                container.update()
            });
        }
    }
}

pub fn new() -> Manager {
    Manager{
        user: getuid(),
        containers: Vec::<Container>::new(),
        ext_ns_reader: ns_reader::new(),
    }
}
