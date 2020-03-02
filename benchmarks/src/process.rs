use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("process_pids", |b| {
        b.iter(|| {
            let stream = heim::process::pids().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("process_processes", |b| {
        b.iter(|| {
            let stream = heim::process::processes().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    // In this benchmark we are going to try to saturate executor threads
    // as much as possible, so we are going to spawn a lot of tasks.
    //
    // Note: any possible errors are intentionally ignored here,
    // as the both stream and inner futures must be executed for as much as possible
    c.bench_function("process_processes_full", |b| {
        b.iter(|| {
            let stream =
                heim::process::processes().for_each_concurrent(None, |process| async move {
                    let process = match process {
                        Ok(p) => p,
                        Err(..) => return,
                    };

                    let _ = tokio::join!(
                        process.parent_pid(),
                        process.name(),
                        process.exe(),
                        process.command(),
                        process.cwd(),
                        process.status(),
                        process.create_time(),
                        process.cpu_time(),
                        process.cpu_usage(),
                        process.memory(),
                        process.is_running(),
                    );
                });

            rt.block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
