TEST_PATTERNS =
TEST_PATTERNS += $(wildcard ./riscv-tests/isa/rv32ui-p-*.bin)
TEST_PATTERNS += $(wildcard ./riscv-tests/isa/rv32um-p-*.bin)
TEST_PATTERNS += $(wildcard ./riscv-tests/isa/rv32ui-v-*.bin)
TEST_PATTERNS += $(wildcard ./riscv-tests/isa/rv32um-v-*.bin)

RUN_PATTERNS = $(addsuffix _run,$(notdir $(basename $(TEST_PATTERNS))))

.PHONY : all

all: $(RUN_PATTERNS)

%_run:
	@cargo run -q ./riscv-tests/isa/$(subst _run,.bin,$@) > /dev/null

%_debug_run:
	cargo run ./riscv-tests/isa/$(subst _debug_run,.bin,$@)
