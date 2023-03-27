set -eu

# make sure it compiles & lints first
cargo doc
# cargo build
# cargo clippy

export PROFDATA=.coverage/all.profdata
mkdir -p .coverage
RUSTFLAGS='-Cinstrument-coverage' \
    LLVM_PROFILE_FILE='.coverage/%m_%p.profraw' \
    cargo test --tests
llvm-profdata merge -sparse .coverage/*.profraw -o ${PROFDATA}
rm .coverage/*.profraw
export TESTEXEC=$(for f in target/debug/deps/*; do test -x ${f} && echo ${f}; done)
export TESTFLAG=$(echo "${TESTEXEC}" | tr ' ' '\n' | sed 's/^/--object /g' | tr '\n' ' ')
llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=${PROFDATA} ${TESTFLAG}
# llvm-cov show   --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=${PROFDATA} ${TESTFLAG} --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt #| less -R
