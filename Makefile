TEST32_PATTERNS =
TEST32_PATTERNS += $(wildcard ./riscv-tests/isa/rv32ui-p-*.bin)
TEST32_PATTERNS += $(wildcard ./riscv-tests/isa/rv32um-p-*.bin)
TEST32_PATTERNS += $(wildcard ./riscv-tests/isa/rv32ui-v-*.bin)
TEST32_PATTERNS += $(wildcard ./riscv-tests/isa/rv32um-v-*.bin)

TEST64_PATTERNS =
TEST64_PATTERNS += $(wildcard ./riscv-tests/isa/rv64ui-p-*.bin)

RUN32_PATTERNS = $(addsuffix _run,$(notdir $(basename $(TEST32_PATTERNS))))
RUN64_PATTERNS = $(addsuffix _run,$(notdir $(basename $(TEST64_PATTERNS))))

.PHONY : all

all: $(RUN32_PATTERNS) $(RUN64_PATTERNS)
run64 : $(RUN64_PATTERNS)

%_run:
	@cargo run -q ./riscv-tests/isa/$(subst _run,.bin,$@) > /dev/null

%_debug_run:
	cargo run ./riscv-tests/isa/$(subst _debug_run,.bin,$@)
