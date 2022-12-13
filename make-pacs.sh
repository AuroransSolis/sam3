#!/bin/sh

# Ensure svd2rust is installed
if ! command -v svd2rust &> /dev/null
then
    echo "svd2rust must be installed. Make sure it's installed and $HOME/.cargo/bin is in your \$PATH."
    return
fi

# Ensure form is installed
if ! command -v form &> /dev/null
then
    echo "form must be installed. Make sure it's installed and $HOME/.cargo/bin is in your \$PATH."
    return
fi

function build_pac() {
    chip_upper=$(basename "${1}" ".svd")
    chip_lower=$(echo "${chip_upper}" | tr '[:upper:]' '[:lower:]')
    xsl=svd/patches/${chip_lower}.xsl
    pac_dir=pac/${chip_lower}

    mkdir -p ${pac_dir}

    xsltproc ${xsl} ${1} \
        | svd2rust \
            --nightly \
            --const_generic \
            --target cortex-m \
            --strict \
            --pascal_enum_values \
            --derive_more \
            --output-dir ${pac_dir}
    form -i ${pac_dir}/lib.rs -o ${pac_dir}/src/
    rm ${pac_dir}/lib.rs

    pushd ${pac_dir}
    cargo +nightly fmt
    rustfmt +nightly build.rs
    
    popd
}

if [[ -z "$@" ]]
then
    echo "No target device(s) provided. Building all PACs."
    for svd in svd/*.svd
    do
        set -xe
        build_pac "$svd"
        set +xe
    done
else
    echo "Targets provided: '$@'"
    for target_arg in "$@"
    do
        if [ -f "svd/$1.svd" ]
        then
            echo "Building PAC for device: $1"
            set -xe
            build_pac "svd/$1.svd"
            set +xe
        else
            echo "Unknown device: '$1'"
        fi
    done
fi
