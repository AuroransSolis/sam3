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

set -xe

for svd in svd/*.svd
do
    chip_upper=$(basename "${svd}" ".svd")
    chip_lower=$(echo "${chip_upper}" | tr '[:upper:]' '[:lower:]')
    xsl=svd/patches/${chip_lower}.xsl
    pac_dir=pac/${chip_lower}

    mkdir -p ${pac_dir}

    xsltproc ${xsl} ${svd} \
        | svd2rust --nightly --const_generic --target cortex-m --output-dir ${pac_dir}
    form -i ${pac_dir}/lib.rs -o ${pac_dir}/src/
    rm ${pac_dir}/lib.rs

    pushd ${pac_dir}
    cargo +nightly fmt
    rustfmt +nightly build.rs
    popd
done
