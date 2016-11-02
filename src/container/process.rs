pub struct Process {
    pub pid: u32,
}

pub fn new(pid: String) -> Process {
    return Process{
        pid: pid.parse::<u32>().unwrap(),
    };
}
