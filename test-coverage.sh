#!/bin/sh

set -eu

echo ''
echo '%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%'

export PROFDATA=.coverage/all.profdata
export REPORT=.coverage/report.out
mkdir -p .coverage
set +u
if [ -z "${RUSTFLAGS}" ]; then echo '\033[0;1;31mWarning: ${RUSTFLAGS} is empty!\033[0m'; export RUSTFLAGS=''; else echo 'Note: ${RUSTFLAGS}='"'${RUSTFLAGS}'"; fi
if [ -z "${CARGOFLAGS}" ]; then echo '\033[0;1;31mWarning: ${CARGOFLAGS} is empty!\033[0m'; export CARGOFLAGS=''; else echo 'Note: ${CARGOFLAGS}='"'${CARGOFLAGS}'"; fi
set -u
export RUST_BACKTRACE=full
export RUSTFLAGS="${RUSTFLAGS} -Zmacro-backtrace -Cinstrument-coverage"
export LLVM_PROFILE_FILE='.coverage/%m_%p.profraw'
cargo rustc ${CARGOFLAGS} # rustc first: backtrace doesn't apply for whatever reason with doc
cargo doc ${CARGOFLAGS}
cargo clippy ${CARGOFLAGS}
cargo test ${CARGOFLAGS}
llvm-profdata merge -sparse .coverage/*.profraw -o ${PROFDATA}
rm .coverage/*.profraw
export TESTEXEC=$(for f in target/debug/deps/*; do test -x ${f} && echo ${f}; done)
export TESTFLAG=$(echo "${TESTEXEC}" | tr ' ' '\n' | sed 's/^/--object /g' | tr '\n' ' ')
# https://llvm.org/docs/CommandGuide/llvm-cov.html#report-command
llvm-cov report \
    --use-color \
    --Xdemangler=rustfilt \
    --ignore-filename-regex='/.cargo/registry' \
    --ignore-filename-regex='test' \
    --instr-profile=${PROFDATA} \
    ${TESTEXEC} \
    ${TESTFLAG} \
    > ${REPORT}
cat ${REPORT} # for the user
export PERCENT=$(cat ${REPORT} | grep '%' | tail -n 1 | cut -d '%' -f 1 | rev | cut -d ' ' -f 1 | rev)
echo "Tests cover ${PERCENT}% of all source code"
if [ "${PERCENT}" = "100.00" ]
then
    echo "Good to go!"
else
    # https://llvm.org/docs/CommandGuide/llvm-cov.html#show-command
    llvm-cov show \
        --use-color \
        --Xdemangler=rustfilt \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='test' \
        --instr-profile=${PROFDATA} \
        ${TESTFLAG} \
        --show-line-counts-or-regions \
        --show-branches=count \
        #| less -R
    # exit 1
fi
