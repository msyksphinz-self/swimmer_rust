TEST_PATTERNS = $(wildcard ./riscv-tests/isa/rv32ui-p-*.bin)
RUN_PATTERNS = $(addsuffix _run,$(notdir $(basename $(TEST_PATTERNS))))

.PHONY : all

all: $(RUN_PATTERNS)

%_run:
	@cargo run -q ./riscv-tests/isa/$(subst _run,.bin,$@) > /dev/null
