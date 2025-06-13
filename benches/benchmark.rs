use criterion::{Criterion, criterion_group, criterion_main};
use protolens::bench::*;

fn benches(c: &mut Criterion) {
    new_task(c);
    readline100(c);
    readline100_new_task(c);
    readline100_rev(c);
    readline500(c);
    readline500_new_task(c);
    readline500_new_task_switch(c);
    readline500_rev(c);
    readline1000(c);
    readline1000_new_task(c);
    readline1000_rev(c);
    http(c);
    http_new_task(c);
    smtp(c);
    smtp_new_task(c);
    smtp_rev(c);
    pop3(c);
    pop3_new_task(c);
    imap(c);
    imap_new_task(c);
    sip(c);
    sip_new_task(c);
}

criterion_group!(normal_benches, benches);
criterion_group!(flame_task, new_task_flame);
criterion_group!(flame_readline500_new_task, readline500_new_task_flame);
criterion_group!(flame_http_new_task, http_new_task_flame);
criterion_group!(flame_smtp_new_task, smtp_new_task_flame);
criterion_group!(flame_pop3_new_task, pop3_new_task_flame);
criterion_group!(flame_imap_new_task, imap_new_task_flame);
criterion_main!(
    normal_benches,
    flame_task,
    flame_readline500_new_task,
    flame_http_new_task,
    flame_smtp_new_task,
    flame_pop3_new_task,
    flame_imap_new_task,
);
