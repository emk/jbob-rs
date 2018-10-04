extern crate criterion;
extern crate jbob;

use criterion::{Criterion, criterion_group, criterion_main};
use jbob::JBobContext;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("size", |b| {
        let mut ctx = JBobContext::new();
        ctx.eval("(defun size-bench () (size '(1 2 3 4 5 6)))").unwrap();
        b.iter(|| ctx.eval("(size-bench)").unwrap());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
