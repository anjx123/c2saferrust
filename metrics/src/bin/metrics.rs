#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::Queries;
use rustc_interface::interface::Compiler;

use metrics::{analyze, compile_time_sysroot};

pub static METRICS_DEFAULT_ARGS: &[&str] =
    &["-Zmir-emit-retag", "-Zalways-encode-mir", "-Zmir-opt-level=0"];

struct CallgraphCallbacks;

impl Callbacks for CallgraphCallbacks {

    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, queries: &'tcx Queries<'tcx>) -> Compilation {
        
        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            analyze(&tcx);
        });

        Compilation::Stop
    }
}

fn main() {
    let mut args: Vec<_> = std::env::args().collect();

    // Make sure we use the right default sysroot. The default sysroot is wrong,
    // because `get_or_default_sysroot` in `librustc_session` bases that on `current_exe`.
    //
    // Make sure we always call `compile_time_sysroot` as that also does some sanity-checks
    // of the environment we were built in.
    // FIXME: Ideally we'd turn a bad build env into a compile-time error via CTFE or so.
    if let Some(sysroot) = compile_time_sysroot() {
        let sysroot_flag = "--sysroot";
        if !args.iter().any(|e| e == sysroot_flag) {
            // We need to overwrite the default that librustc_session would compute.
            args.push(sysroot_flag.to_owned());
            args.push(sysroot);
        }
    }

    args.splice(
        1..1,
        METRICS_DEFAULT_ARGS.iter().map(ToString::to_string),
    );

    let mut calls = CallgraphCallbacks;

    let run_compiler = rustc_driver::RunCompiler::new(&args, &mut calls);
    run_compiler.run();
}