use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    c.bench_function("process_pids", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::process::pids().await?.for_each(|_| async {}).await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("process_processes", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::process::processes()
                    .await?
                    .for_each(|_| async {})
                    .await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    // In this benchmark we are going to try to saturate executor threads
    // as much as possible, so we are going to spawn a lot of tasks.
    //
    // Note: any possible errors are intentionally ignored here,
    // as the both stream and inner futures must be executed for as much as possible
    c.bench_function("process_processes_full", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::process::processes()
                    .await?
                    .for_each_concurrent(None, |process| async move {
                        let process = match process {
                            Ok(p) => p,
                            Err(..) => return,
                        };

                        let _ = futures::join!(
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
                    })
                    .await;

                Ok::<(), heim::Error>(())
            })
        })
    });
}

criterion_group!(bench, inner);
