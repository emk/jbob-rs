extern crate jbob;

use jbob::JBobContext;

fn main() {
    let mut ctx = JBobContext::new();
    ctx.require_jbob().unwrap();
    ctx.require_little_prover().unwrap();
    ctx.eval("(dethm.align/align)").unwrap();
}
