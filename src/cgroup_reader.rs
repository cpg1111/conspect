use std::fs::File;

pub mod cgroup_reader {
    trait CGroup_Reader {
        fn get_name() -> String;
        fn read(stat: String) -> String;
    }

    fn which_cgroup(pid: u32) -> (Fn(String) -> String) {
        let path = format!("/proc/{}/cgroup");
        let mut cgroup_f = try!(File::open(path));
        let mut buffer = String::new();
        try!(cgroup_f.read_to_string(&mut buffer));
        |resource| {
            let r_cnt = 0;
            let begin = -1;
            let end = -1;
            let colon_cnt = 0;
            for (c, i) in buffer {
                if c == ':' {
                    colon_cnt++;
                } else if begin > 0 && end < 0 && c == '\n'{
                    end = i;
                } else if colon_cnt == 1 {
                    if resource[r_cnt] == c {
                        r_cnt++;
                    } else if r_cnt > 0 && r_cnt + 1 < resource.capacity() {
                        continue;
                    }
                } else {
                    if begin < 0 {
                        begin = i + 1;
                    }
                }
            }
        }
    }

    fn read_cgroup(resource: String, name: String, stat: String) -> String {
        let path = format!("/sys/fs/cgroup/{}/{}/{}", resource, name, stat);
        let mut stat_f = try!(File::open(path))
        let mut buffer = String::new();
        try!(stat_f.read_to_string(&mut buffer));
        return buffer;
    }

    struct CPU_Reader {
        name: String,
        pid: u32,
    }

    impl CGroup_Reader for CPU_Reader {
        fn read(stat: String) -> String {
            return read_cgroup("cpu", self.name, stat);
        }
    }

    struct Mem_Reader {
        name: String,
        pid: u32,
    }

    impl CGroup_Reader for Mem_Reader {
        fn read(stat: String) -> String {
            return read_cgroup("memory", self.name, stat);
        }
    }

    struct BLKIO_Reader {
        name: String,
        pid: u32,
    }

    impl CGroup_Reader for BLKIO_Reader {
        fn read(stat: String) -> String {
            return read_cgroup("blkio", self.name, stat);
        }
    }

    struct Device_Reader {
        name: String,
        pid: u32,
    }

    impl CGroup_Reader for Device_Reader {
        fn read(stat: String) -> String {
            return read_cgroup("devices", self.name, stat);
        }
    }

    struct Net_Reader {
        name: String,
        pid: u32,
    }

    impl CGroup_Reader for Net_Reader {
        fn read(stat: String) -> String {
            return read_cgroup("net_cls", self.name, stat);
        }
    }

    pub struct Reader {
        cpu_stat: CPU_Reader,
    }

    pub fn new(pid: u32) -> Reader {
        let name = which_cgroup(pid);
        return Reader{
            cpu: CPU_Reader{
                name: name("cpu"),
                pid: pid,
            },
            mem: Mem_Reader{
                name: name("memory"),
                pid: pid,
            },
            blkio: BLKIO_Reader{
                name: name("blkio"),
                pid: pid,
            },
            devices: Device_Reader{
                name: name("devices"),
                pid: pid,
            },
            net: Net_Reader{
                name: name("net_cls"),
                pid: pid,
            },
        };
    }
}
