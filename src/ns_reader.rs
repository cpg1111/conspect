use std::fs;
use std::fs::File;

pub mod ns_reader {
    trait NS_Reader {
        fn read(&self) -> i32;
    }

    fn parse_ns_f(content: String) -> u32 {
        let mut begin;
        let mut end;
        for (c, i) in content.chars() {
            match c {
                '[' => begin = i,
                ']' => end = i,
                _ => continue,
            }
        }
        let ns = format!("{}", &content[begin..end]);
        return ns.parse::<u32>();
    }

    fn read_ns(pid: u32, ns: String) -> String {
        let l_path = format!("/proc/{}/ns/{}", pid, ns);
        let ns = try!(fs::read_link(l_path));
        return parse_ns_f(ns);
    }

    struct User_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for User_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "user");
        }
    }

    struct IPC_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for IPC_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "ipc");
        }
    }

    struct MNT_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for MNT_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "mnt");
        }
    }

    struct Net_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for Net_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "net");
        }
    }

    struct PID_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for PID_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "pid");
        }
    }

    struct UTS_NS_Reader {
        pid: u32,
    }

    impl NS_Reader for UTS_NS_Reader {
        fn read(&self) -> u32 {
            return read_ns(self.pid, "uts");
        }
    }

    pub struct Reader {
        user: User_NS_Reader,
        ipc: IPC_NS_Reader,
        mnt: MNT_NS_Reader,
        net: Net_NS_Reader,
        pid: PID_NS_Reader,
        uts: UTS_NS_Reader,
    }

    pub new(pid: u32) -> Reader {
        return Reader{
            user: User_NS_Reader{pid: pid},
            ipc: IPC_NS_Reader{pid: pid},
            mnt: MNT_NS_Reader{pid: pid},
            net: Net_NS_Reader{pid: pid},
            pid: PID_NS_Reader{pid: pid},
            uts: UTS_NS_Reader{pid: pid},
        }
    }
}
