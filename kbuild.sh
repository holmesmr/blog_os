#!/usr/bin/env bash

read -r -d '' INTEGRATION_TESTS <<-'END_TESTS'
    basic-boot
    panic
END_TESTS


banner_text_start() {
    echo ""
}

banner_text() {
    message=$1
    length=$(expr length "${message}")
    stripe=$(printf "%${length}s\n" '' | tr '[:blank:]' '[=]')

    echo ${stripe}
    echo ${message}
    echo ${stripe}

    return ${length}
}

banner_text_continue() {
    message=$1
    length=$2
    stripe=$(printf "%${length}s\n" '' | tr '[:blank:]' '[=]')

    echo ${message}
    echo ${stripe}
    return ${length}
}

banner_text_end() {
    echo ""
}

bootimage_cmd() {
    cmd=$1
    shift

    mode=$1

    if [[ ${mode} != "" ]]; then
        shift

        if [[ "$mode" = "release" ]]; then
            bootimg_opts="--release"
        elif [[ "$mode" = "integration" ]]; then
            bootimg_opts="--features integration-test"
        fi
    fi

    kimg=$1

    if [[ ${kimg} != "" ]]; then
        shift
    fi

    opts=$*

    if [[ ${kimg} != "" ]]; then
        bootimage ${cmd} ${bootimg_opts} --bin ${kimg} -- ${opts}
    else
        bootimage ${cmd} ${bootimg_opts} -- ${opts}
    fi
}

run_integration_test() {
    test=$1
    PASS_PATTERN="[integration-test-result:pass]"
    FAIL_PATTERN="[integration-test-result:fail]"

    banner_text_start
    banner_text "Running integration test ${test}..."
    banner_text_end

    output=$(bootimage_cmd run integration integration-test-${test} \
        -serial mon:stdio -display none \
        -device isa-debug-exit,iobase=0xf4,iosize=0x04)

    if [[ ${output} == *"${PASS_PATTERN}"* ]]; then
        banner_text_start
        banner_text "Integration test ${test}: PASS"
        banner_text_end
        return 0
    elif [[ ${output} == *"${FAIL_PATTERN}"* ]]; then
        banner_text_start
        banner_text "Integration test ${test}: FAIL"
        banner_text_continue "Test output" $?
        banner_text_end
        echo "${output}"
        return 1
    else
        banner_text_start
        banner_text "Integration test ${test}: ERROR"
        banner_text_continue "Test output" $?
        banner_text_end
        echo "${output}"
        return 1
    fi
}

run_integration_test_suite() {
    has_fail=0
    for TEST in ${INTEGRATION_TESTS}; do
        run_integration_test ${TEST}

        if [[ $? -ne 0 ]]; then
            has_fail=1
        fi
    done

    if [[ ${has_fail} -ne 0 ]]; then
        banner_text_start
        banner_text "TEST SUITE FAILED"
        banner_text_end
        exit 1
    fi
}

cmd=$1

if [[ ${cmd} = "build" ]]; then
    mode=$2

    if [[ ${mode} != "debug" ]] && [[ ${mode} != "release" ]]; then
        echo "usage: $0 build (debug | release)"
        exit 255
    fi

    bootimage_cmd build ${mode}
elif [[ ${cmd} = "clean" ]]; then
    cargo clean
elif [[ ${cmd} = "integration-test" ]]; then
    run_integration_test_suite
elif [[ ${cmd} = "test" ]]; then
    cargo test --lib
elif [[ ${cmd} = "run" ]]; then
    mode=$2

    if [[ ${mode} != "debug" ]] && [[ ${mode} != "release" ]]; then
        echo "usage: $0 run (debug | release) [QEMU_OPTS...]"
        exit 255
    fi

    shift 2

    bootimage_cmd run ${mode} "" $*
else
    echo "usage: $0 (build | clean | integration-test | test | run)"
    exit 255
fi