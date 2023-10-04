#!/bin/sh

# Ensure svd2rust is installed
if ! command -v svd2rust &> /dev/null
then
    echo "svd2rust must be installed. Make sure it's installed and \$HOME/.cargo/bin is in your \$PATH."
    return
fi

# Ensure form is installed
if ! command -v form &> /dev/null
then
    echo "form must be installed. Make sure it's installed and \$HOME/.cargo/bin is in your \$PATH."
    return
fi

# Ensure xsltproc is installed
if ! command -v xsltproc &> /dev/null
then
    echo "xsltproc must be installed."
    return
fi

# Ensure sd is installed
if ! command -v sd &> /dev/null
then
    echo "sd must be installed. Make sure it's installed and \$HOME/.cargo/bin is in your \$PATH."
    return
fi

function build_pac() {
    chip_upper="$(basename "${1}" ".svd")"
    chip_lower="$(echo "${chip_upper}" | tr '[:upper:]' '[:lower:]')"
    xsl="svd/patches/${chip_lower}.xsl"
    pac_dir="pac/${chip_lower}"

    mkdir -p "${pac_dir}"

    xsltproc -o "${pac_dir}/${chip_upper}.out" "svd/patches/expand-dim.xsl" "${1}"
    xsltproc "${xsl}" "${pac_dir}/${chip_upper}.out" \
        | svd2rust \
            --const_generic \
            --target cortex-m \
            --strict \
            --pascal_enum_values \
            --output-dir "${pac_dir}"
    form -i "${pac_dir}/lib.rs" -o "${pac_dir}/src/"
    rm "${pac_dir}/lib.rs"
    rm "${pac_dir}/${chip_upper}.out"

    pushd "${pac_dir}"
    cargo +nightly fmt
    rustfmt +nightly build.rs
    # Not entirely sure why these `deny`s are still present.
    sd '#!\[deny\(private_in_public\)\]' '' src/lib.rs
    sd '#!\[deny\(private_bounds\)\]' '' src/lib.rs
    sd '#!\[deny\(private_interfaces\)\]' '' src/lib.rs

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
