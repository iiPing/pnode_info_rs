use axum::{routing::get, Router};
use std::net::SocketAddr;

use std::path::Path;

use lazy_static::lazy_static;
use prometheus::{self, IntCounter, Gauge, IntGauge, register_int_counter, register_gauge, register_int_gauge};
use tokio::time::{sleep, Duration};
use humantime::format_rfc3339;
use std::time::{SystemTime};
use pnode_info_rs::*;



lazy_static! {
    static ref SCRAPE_COUNT : IntCounter  =
        register_int_counter!("SCRAPE_COUNT", "Number of scrape requests").unwrap();
    
    static ref LOADAVG_AVG1 : Gauge  =
        register_gauge!("LOADAVG_AVG1", "Load Average in 1 min").unwrap();

    static ref LOADAVG_AVG5: Gauge = 
        register_gauge!("LOADAVG_AVG5", "Load Average in 5 mins").unwrap();

    static ref LOADAVG_AVG15: Gauge = 
        register_gauge!("LOADAVG_AVG15", "Load Average in 15 mins").unwrap();

    static ref PSI_CPU_SOME_AVG10: Gauge = 
        register_gauge!("PSI_CPU_SOME_AVG10", "PSI Some 10 secs").unwrap();

    static ref PSI_CPU_SOME_AVG60: Gauge = 
        register_gauge!("PSI_CPU_SOME_AVG60", "PSI Some 60 secs").unwrap();

    static ref PSI_CPU_SOME_AVG300: Gauge = 
        register_gauge!("PSI_CPU_SOME_AVG300", "PSI Some 300 secs").unwrap();

    static ref PSI_MEM_SOME_AVG10: Gauge = 
        register_gauge!("PSI_MEM_SOME_AVG10", "PSI Some 10 secs").unwrap();

    static ref PSI_MEM_SOME_AVG60: Gauge = 
        register_gauge!("PSI_MEM_SOME_AVG60", "PSI Some 60 secs").unwrap();

    static ref PSI_MEM_SOME_AVG300: Gauge = 
        register_gauge!("PSI_MEM_SOME_AVG300", "PSI Some 300 secs").unwrap();

    static ref PSI_IO_SOME_AVG10: Gauge = 
        register_gauge!("PSI_IO_SOME_AVG10", "PSI Some 10 secs").unwrap();

    static ref PSI_IO_SOME_AVG60: Gauge = 
        register_gauge!("PSI_IO_SOME_AVG60", "PSI Some 60 secs").unwrap();

    static ref PSI_IO_SOME_AVG300: Gauge = 
        register_gauge!("PSI_IO_SOME_AVG300", "PSI Some 300 secs").unwrap();


    static ref PSI_CPU_FULL_AVG10: Gauge = 
        register_gauge!("PSI_CPU_FULL_AVG10", "PSI FULL 10 secs").unwrap();

    static ref PSI_CPU_FULL_AVG60: Gauge = 
        register_gauge!("PSI_CPU_FULL_AVG60", "PSI FULL 60 secs").unwrap();

    static ref PSI_CPU_FULL_AVG300: Gauge = 
        register_gauge!("PSI_CPU_FULL_AVG300", "PSI FULL 300 secs").unwrap();


    static ref PSI_MEM_FULL_AVG10: Gauge = 
        register_gauge!("PSI_MEM_FULL_AVG10", "PSI FULL 10 secs").unwrap();

    static ref PSI_MEM_FULL_AVG60: Gauge = 
        register_gauge!("PSI_MEM_FULL_AVG60", "PSI FULL 60 secs").unwrap();

    static ref PSI_MEM_FULL_AVG300: Gauge = 
        register_gauge!("PSI_MEM_FULL_AVG300", "PSI FULL 300 secs").unwrap();


    static ref PSI_IO_FULL_AVG10: Gauge = 
        register_gauge!("PSI_IO_FULL_AVG10", "PSI FULL 10 secs").unwrap();

    static ref PSI_IO_FULL_AVG60: Gauge = 
        register_gauge!("PSI_IO_FULL_AVG60", "PSI FULL 60 secs").unwrap();

    static ref PSI_IO_FULL_AVG300: Gauge = 
        register_gauge!("PSI_IO_FULL_AVG300", "PSI FULL 300 secs").unwrap();



    static ref PROC_RUN_QUEUE: IntGauge = 
        register_int_gauge!("PROC_RUN_QUEUE", "process run queue size ").unwrap();

    static ref PROC_BLOCKED_QUEUE: IntGauge = 
        register_int_gauge!("PROC_BLOCKED_QUEUE", "process blocked queue size").unwrap();
    
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/metrics", get(metrics));

    let addr = SocketAddr::from(([0, 0, 0, 0], 15000));

    let mut handles = vec![];

    handles.push(tokio::spawn({
            let addr = addr.clone();
            async move {
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await.unwrap()}
            }
    ));

    handles.push(tokio::spawn({
            async move {
                loop {

                    let file = "/proc/loadavg";
                    let loadavg = if Path::new(file).is_file() { 
                        LoadAvg::read_file(file)
                    }
                    else {
                        LoadAvg::new()
                    };

                    LOADAVG_AVG1.set(loadavg.avg1);
                    LOADAVG_AVG5.set(loadavg.avg5);
                    LOADAVG_AVG15.set(loadavg.avg15);
                    

                    let file = "/proc/pressure/cpu";
                    let psi_cpu = if Path::new(file).is_file() { 
                        Pressure::read_file(file) 
                    }
                    else {
                        Pressure::new()
                    };

                    PSI_CPU_FULL_AVG10.set(psi_cpu.full_avg10);
                    PSI_CPU_FULL_AVG60.set(psi_cpu.full_avg60);
                    PSI_CPU_FULL_AVG300.set(psi_cpu.full_avg300);
                    PSI_CPU_SOME_AVG10.set(psi_cpu.some_avg10);
                    PSI_CPU_SOME_AVG60.set(psi_cpu.some_avg60);
                    PSI_CPU_FULL_AVG300.set(psi_cpu.some_avg300);

                    
                    let file = "/proc/pressure/memory";
                    let psi_mem = if Path::new(file).is_file() {
                        Pressure::read_file(file)
                    }
                    else {
                        Pressure::new()
                    };

                    PSI_MEM_FULL_AVG10.set(psi_mem.full_avg10);
                    PSI_MEM_FULL_AVG60.set(psi_mem.full_avg60);
                    PSI_MEM_FULL_AVG300.set(psi_mem.full_avg300);
                    PSI_MEM_SOME_AVG10.set(psi_mem.some_avg10);
                    PSI_MEM_SOME_AVG60.set(psi_mem.some_avg60);
                    PSI_MEM_SOME_AVG300.set(psi_mem.some_avg300);


                    let file = "/proc/pressure/io";
                    let psi_io = if Path::new(file).is_file() {
                        Pressure::read_file(file)
                    }
                    else{
                        Pressure::new()
                    };

                    PSI_IO_FULL_AVG10.set(psi_io.full_avg10);
                    PSI_IO_FULL_AVG60.set(psi_io.full_avg60);
                    PSI_IO_FULL_AVG300.set(psi_io.full_avg300);
                    PSI_IO_SOME_AVG10.set(psi_io.some_avg10);
                    PSI_IO_SOME_AVG60.set(psi_io.some_avg60);
                    PSI_IO_SOME_AVG300.set(psi_io.some_avg300);


                    let file = "/proc/stat";
                    let stat = if Path::new(file).is_file(){
                        Stat::read_file(file)
                    }
                    else {
                        Stat::new()
                    };

                    PROC_RUN_QUEUE.set(stat.procs_running);
                    PROC_BLOCKED_QUEUE.set(stat.procs_blocked);

                    let now = format_rfc3339(SystemTime::now());
                    let output   = format!("{} {} {} {} {} {}", now,  psi_cpu.printsd(), psi_io.printsd(), psi_mem.printsd(), loadavg.printsd(), stat.printsd());
                    println!("{}", output);
                    sleep(Duration::from_secs(5)).await;
                }
            }
    }));

   futures::future::join_all(handles).await;


}


async fn home() -> &'static str {
    "Node Information"
}


async fn metrics() -> Result<String, String> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    SCRAPE_COUNT.inc();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::default_registry().gather(), &mut buffer) {
        return Err(format!("could not encode custom metrics: {e}"));
    };
    String::from_utf8(buffer.clone())
        .map_err(|e| format!("custom metrics could not be from_utf8'd: {e}"))
}
