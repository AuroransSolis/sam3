#!/bin/sh

passpacs=()
failcount=0

pushd pac >/dev/null
for pac in ./*
do
    pacname="$(echo $pac | sd '\./' '')"
    pushd $pac >/dev/null
    if [ "$(cargo check 2>&1 | rg 'error')" == "" ]
    then
        passpacs+=("$pacname")
    else
        echo "fail: $pacname"
        failcount=$(($failcount + 1))
    fi
    popd >/dev/null
done
popd >/dev/null

echo "pacs failing to build: $failcount"
echo

passcount=0
failcount=0

pushd hal >/dev/null
for name in ${passpacs[@]}
do
    shortname="$(echo $name | sd '.+(sam3.+)' '$1')"

    for suffix in {"","-rt"}
    do
        feature="$shortname$suffix"
        for other in {"","unproven"}
        do
            featurelist="$feature,$other"
            checkresult=$(cargo check --features="$featurelist" 2>&1)

            if [ "$(echo $checkresult | rg 'error')" != "" ]
            then
                echo "FAILURE: $featurelist"
                failcount=$(($failcount + 1))
            else
                passcount=$(($passcount + 1))
            fi
        done
    done
done
popd >/dev/null

echo "fails: $failcount"
echo "passes: $passcount"
