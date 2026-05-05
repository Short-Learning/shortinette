use std::{fs, io::Write as _, path};

use crate::{cargo::Cargo, repository_path, result::TestResult, testable::Testable};

#[derive(Debug, PartialEq, Eq)]
pub struct Exercise00;

impl Testable for Exercise00 {
    fn path(&self) -> path::PathBuf {
        repository_path().join("ex00")
    }

    fn cargo_test_mod(&self) -> &'static str {
        include_str!("./shortinette_tests_success_compile.rs")
    }

    fn run_test(&self) -> TestResult {
        if !self.check_clippy() {
            eprint!("`cargo clippy -- -D warnings` failed");

            return TestResult::CompilationError;
        }

        if self.compile().is_err() {
            eprintln!("Failed to compile");

            return TestResult::CompilationError;
        }

        let fail_compile_result = self.run_test_fail_compile();
        if !fail_compile_result.is_success() {
            return fail_compile_result;
        }

        if let Err(test_output) = self.run_cargo_tests() {
            eprintln!("{test_output}");

            return TestResult::Failed;
        }

        TestResult::Passed
    }
}

impl Exercise00 {
    fn run_test_fail_compile(&self) -> TestResult {
        const TEST_MOD: &'static [u8] = include_bytes!("./shortinette_tests_fail_compile.rs");

        let cargo = Cargo::new("shortinette-test-module", true);
        cargo
            .add_local_dependency(
                self.path()
                    .to_str()
                    .expect("Path did not contain valid unicode"),
            )
            .expect("Failed to add exercise as dependency to test project");

        let mut lib_file = fs::File::create(cargo.path().join("src/lib.rs"))
            .expect("Failed to open src/lib.rs of test module");

        lib_file
            .write_all(TEST_MOD)
            .expect("Failed to write test module into src/lib.rs of test module");

        if cargo.compile().is_err() {
            eprintln!("Failed to compile");

            return TestResult::Failed;
        }

        if let Ok(_) = cargo.run_test(["shortinette_tests_0200f::free_complex_struct"]) {
            eprintln!(
                "An instance of `ComplexStruct` can still be used even after calling `.free()` on it."
            );

            return TestResult::Failed;
        }

        TestResult::Passed
    }

    // fn run_test_success_compile(&self) -> TestResult {
    //     const TEST_MOD: &'static [u8] = include_bytes!("./shortinette_test_success_compile.rs");

    //     let cargo = Cargo::new("shortinette-test-module", true);
    //     cargo
    //         .add_local_dependency(
    //             self.path()
    //                 .to_str()
    //                 .expect("Path did not contain valid unicode"),
    //         )
    //         .expect("Failed to add exercise as dependency to test project");

    //     let mut lib_file = fs::File::create(cargo.path().join("src/lib.rs"))
    //         .expect("Failed to open src/lib.rs of test module");

    //     lib_file
    //         .write_all(TEST_MOD)
    //         .expect("Failed to write test module into src/lib.rs of test module");

    //     if cargo.compile().is_err() {
    //         eprintln!("Failed to compile");

    //         return TestResult::CompilationError;
    //     }

    //     if let Err(test_output) = cargo.run_test(["shortinette_tests_0200s::free_complex_struct"]) {
    //         eprintln!("{test_output}");

    //         return TestResult::Failed;
    //     }

    //     TestResult::Passed
    // }
}
