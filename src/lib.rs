use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Clone, Debug)]
pub struct LoadAvg {
    pub avg1             : f64,
    pub avg5             : f64,
    pub avg15            : f64,
    pub sched_threads    : String,
    pub lastpid         : i64,
}

impl LoadAvg {
    pub fn new() -> LoadAvg {
        LoadAvg {
            avg1             : 0f64,
            avg5             : 0f64,
            avg15            : 0f64,
            sched_threads    : "0/0".to_string(),
            lastpid          : 0i64,            
        }
    }
    pub fn printsd(&self) -> String{
        format!("{} {} {} {} {}", 
            self.avg1, self.avg5, self.avg15, self.sched_threads, self.lastpid)
    }

    pub fn read_file (filename: &'static str) -> LoadAvg {
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
    
        let mut retval = LoadAvg::new();
    
        for line_result in reader.lines() {
            let line = line_result.expect("error reading line");
            let s = line.as_str(); 
            let mut valiter = s.split_whitespace();
            retval.avg1 = valiter.next().unwrap().parse().unwrap_or(0f64);
            retval.avg5 = valiter.next().unwrap().parse().unwrap_or(0f64);
            retval.avg15 = valiter.next().unwrap().parse().unwrap_or(0f64);
            retval.sched_threads = valiter.next().unwrap().parse().unwrap_or("0/0".to_string());
            retval.lastpid = valiter.next().unwrap().parse().unwrap_or(0i64);
        }
    
        retval
    }
}


#[derive(Clone, Debug)]
pub struct Pressure {
    pub some_avg10       : f64,
    pub some_avg60       : f64,
    pub some_avg300      : f64,
    pub some_total       : u64,
    pub full_avg10       : f64,
    pub full_avg60       : f64,
    pub full_avg300      : f64,
    pub full_total       : u64
}

impl Pressure {
    pub fn new() -> Pressure {
        Pressure {
            some_avg10:    0f64,
            some_avg60:    0f64,
            some_avg300:   0f64,
            some_total:    0u64,
            full_avg10:    0f64,
            full_avg60:    0f64,
            full_avg300:   0f64,
            full_total:    0u64,
        }
    }
    pub fn printsd(&self) -> String{
        format!("{} {} {} {} {} {} {} {}", 
        self.some_avg10, self.some_avg60, self.some_avg300, self.some_total,
        self.full_avg10, self.full_avg60, self.full_avg300, self.full_total)
    }

    pub fn read_file(filename: &'static str) -> Pressure {
        let mut retval = Pressure::new();
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
        
    
        for line_result in reader.lines() {
            let line = line_result.expect("error reading line");
            match line.as_str() {
                s if s.starts_with("some") => {
                    for field in s.split_whitespace() {
                        match field.split_once('=') {
                            Some(("avg10", v)) => {
                                retval.some_avg10 = v.parse().unwrap_or(0f64);
                            }
                            Some(("avg60", v)) => {
                                retval.some_avg60 = v.parse().unwrap_or(0f64);
                            }
                            Some(("avg300", v)) => {
                                retval.some_avg300 = v.parse().unwrap_or(0f64);
                            }
                            Some(("total", v)) => {
                                retval.some_total = v.parse().unwrap_or(0);
                            }
                            _ => {}
                        }
                    }
                }
                s if s.starts_with("full") => {
                    for field in s.split_whitespace() {
                        match field.split_once('=') {
                            Some(("avg10", v)) => {
                                retval.full_avg10 = v.parse().unwrap_or(0f64);
                            }
                            Some(("avg60", v)) => {
                                retval.full_avg60 = v.parse().unwrap_or(0f64);
                            }
                            Some(("avg300", v)) => {
                                retval.full_avg300 = v.parse().unwrap_or(0f64);
                            }
                            Some(("total", v)) => {
                                retval.full_total = v.parse().unwrap_or(0);
                            }                        
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        retval
    }
        
}


#[derive(Clone, Debug)]
pub struct Stat {
    pub ctxt             : u64,
    pub btime            : u64,
    pub processes        : u64,
    pub procs_running    : i64,
    pub procs_blocked    : i64,
}

impl Stat {
    pub fn new() -> Stat {
        Stat {
            ctxt             : 0u64,
            btime            : 0u64,
            processes        : 0u64,
            procs_running    : 0i64,
            procs_blocked    : 0i64            
        }
    }
    pub fn printsd(&self) -> String{
        format!("{} {} {} {} {}", 
            self.ctxt, self.btime, self.processes, self.procs_running, self.procs_blocked)
    }
    pub fn read_file(filename: &'static str) -> Stat {
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
    
        let mut retval = Stat::new();
    
        for line_result in reader.lines() {
            let line = line_result.expect("error reading line");
    
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["ctxt", val] => retval.ctxt = val.parse().unwrap_or_default(),
                ["btime", val] => retval.btime = val.parse().unwrap_or_default(),
                ["processes", val] => retval.processes = val.parse().unwrap_or_default(),
                ["procs_running", val] => retval.procs_running = val.parse().unwrap_or_default(),
                ["procs_blocked", val] => retval.procs_blocked = val.parse().unwrap_or_default(),
                _ => (),
            }
        }
    
        retval
    }    
}

