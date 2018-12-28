extern crate nvptxglue;

use nvptxglue::prelude::*;

use std::io::{Write, Result as IoResult};

struct PtxOnlyTestGlue;

impl Glue for PtxOnlyTestGlue {
    fn write_bindings(&self, spec: &GlueSpec, writer: &mut dyn Write) -> IoResult<()> {
        let ptx_path = match &spec.output {
            &CompilerOutput::Ptx(ref path) => path.to_str().unwrap(),
            _ => panic!(),
        };
        writeln!(writer, "static _REF_PTX: &'static str = include_str!(\"../assets/reference.ptx\");")?;
        writeln!(writer, "static _TEST_PTX: &'static str = include_str!(\"{}\");", ptx_path)?;
        writeln!(writer, "")?;
        writeln!(writer, "#[test]")?;
        writeln!(writer, "fn test_ptx_equal_to_reference_ptx() {{")?;
        writeln!(writer, "    assert_eq!(_TEST_PTX, _REF_PTX);")?;
        writeln!(writer, "}}")?;
        Ok(())
    }
}

fn main() {
    nvptxglue::Builder::default()
        .crate_dir("../stdsimd-nvptx-kernels")
        .gencode(Gencode::Ptx(Cc_3_5))
        .whitelist_kernel("_stdsimd_nvptx_test_syncthreads_kernel")
        .compile(Phase::Ptx)
        .expect("nvptxglue failure: compile to ptx")
        .write_bindings_to_file(PtxOnlyTestGlue, "src/tests.rs")
        .expect("nvptxglue failure: generate testglue bindings");
}
