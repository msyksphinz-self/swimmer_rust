#!/bin/sh

git submodule update --init --recursive riscv-tests
module switch riscv/riscv64

PWD=${pwd}
cd riscv-tests/isa
make -j8

for f in `ls -1 | grep -e rv32..- -e rv64..- | grep -v "\.dump$" | grep -v "\.hex$" | grep -v "\.bin$"`
do
    echo "Compile ${f} ..."
    riscv64-unknown-elf-objcopy -O binary ${f} ${f}.bin
    # hexdump -v -e ' 1/4 "%08x " "\n"' ${f}.bin > ${f}.hex
done

cd ${PWD}
