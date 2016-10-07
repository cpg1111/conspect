use std::fs;
use std::fs::File;
use std::sync::mpsc;
use std::thread;
use regex::Regex;

const STATS: Vec<String> = vec![
    String::from("cpu.shares"),
    String::from("cpu.cfs_period_us"),
    String::from("cpu.rt_period_us"),
    String::from("cpu.rt_runtime_us"),
    String::from("cpuacct.usage"),
    String::from("cpuacct.usage_percpu"),
    String::from("memory.failcnt"),
    String::from("memory.limit_in_bytes"),
    String::from("memory.max_usage_in_bytes"),
    String::from("memory.move_charge_at_immigrate"),
    String::from("memory.pressure_level"),
    String::from("memory.swappiness"),
    String::from("memory.usage_in_bytes"),
    String::from("blkio.io_merged"),
    String::from("blkio.io_queued"),
    String::from("blkio.io_service_bytes"),
    String::from("blkio.io_service_time"),
    String::from("blkio.io_wait_time"),
    String::from("blkio.io_time"),
    String::from("net_cls.classid"),
    String::from("net_prio.ifpriomap"),
    String::from("net_prio.prioidx"),
];

trait CGroup_Reader {
    fn read(&self, stat: String) -> String;
}

fn which_cgroup(pid: u32) -> Box<Fn(String) -> String> {
    use std::fs;
    use std::fs::File;
    let path = format!("/proc/{}/cgroup", pid);
    let mut cgroup_f = File::open(path).unwrap();
    let mut buffer = String::new();
    cgroup_f.read_to_string(&mut buffer);
    Box::new(move |resource| {
        let mut r_cnt = 0;
        let mut begin = -1;
        let mut end = -1;
        let mut colon_cnt = 0;
        for (c, i) in buffer {
            if c == ':' {
                colon_cnt += 1;
            } else if begin > 0 && end < 0 && c == '\n'{
                end = i;
            } else if colon_cnt == 1 {
                if resource[r_cnt] == c {
                    r_cnt += 1;
                } else if r_cnt > 0 && r_cnt + 1 < resource.capacity() {
                    continue;
                }
            } else {
                if begin < 0 {
                    begin = i + 1;
                }
            }
        }
        buffer[begin..end]
    })
}

fn read_cgroup(resource: String, name: String, stat: String) -> String {
    let path = format!("/sys/fs/cgroup/{}/{}/{}", resource, name, stat);
    let mut stat_f = try!(fs::File::open(path));
    let mut buffer = String::new();
    try!(stat_f.read_to_string(&mut buffer));
    return buffer;
}

struct CPU_Reader {
    name: String,
    pid: u32,
}

impl CGroup_Reader for CPU_Reader {
    fn read(&self, stat: String) -> String {
        return read_cgroup("cpu,cpuacct", self.name, stat);
    }
}

struct Mem_Reader {
    name: String,
    pid: u32,
}

impl CGroup_Reader for Mem_Reader {
    fn read(&self, stat: String) -> String {
        return read_cgroup("memory", self.name, stat);
    }
}

struct BLKIO_Reader {
    name: String,
    pid: u32,
}

impl CGroup_Reader for BLKIO_Reader {
    fn read(&self, stat: String) -> String {
        return read_cgroup("blkio", self.name, stat);
    }
}

struct Net_Reader {
    name: String,
    pid: u32,
}

impl CGroup_Reader for Net_Reader {
    fn read(&self, stat: String) -> String {
        return read_cgroup("net_cls,net_prio", self.name, stat);
    }
}

pub struct Reader {
    cpu: CPU_Reader,
    mem: Mem_Reader,
    blkio: BLKIO_Reader,
    net: Net_Reader,
}

pub type CGroup_Stat = (String, String);

impl Reader {
    fn read_res(&self, res: String, tx: mpsc::Sender<String>) {
        let ctx = self;
        thread::spawn(|| {
            let shares;
            let cpu_re = Regex::new(r"^cpu(.*)").unwrap();
            let mem_re = Regex::new(r"^mem(.*)").unwrap();
            let blkio_re = Regex::new(r"^mem(.*)").unwrap();
            let net_re = Regex::new(r"^net(.*)").unwrap();
            if cpu_re.is_match(res) {
                shares = ctx.cpu.read(res);
            } else if mem_re.is_match(res) {
                shares = ctx.mem.read(res);
            } else if blkio_re.is_match(res) {
                shares = ctx.blkio.read(res);
            } else {
                shares = ctx.net.read(res);
            }
            tx.send(shares).unwrap();
        })
    }
    fn read(&self) -> Vec<CGroup_Stat> {
        let stats = Vec::<CGroup_Stat>::new();
        let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        for stat in STATS {
            self.read_res(stat, tx);
        }
        for _ in STATS {
            stats.push(rx.recv().unwrap());
        }
        return stats;
    }
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
        net: Net_Reader{
            name: name("net_cls"),
            pid: pid,
        },
    };
}
