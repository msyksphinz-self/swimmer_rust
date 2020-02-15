extern crate swimmer_rust;

#[test]fn rv32ui_p_add       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-add.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_addi      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-addi.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_and       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-and.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_andi      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-andi.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_auipc     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-auipc.bin"      .to_string()), 1); }
#[test]fn rv32ui_p_beq       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-beq.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_bge       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-bge.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_bgeu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-bgeu.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_blt       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-blt.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_bltu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-bltu.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_bne       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-bne.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_fence_i   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-fence_i.bin"    .to_string()), 1); }
#[test]fn rv32ui_p_jal       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-jal.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_jalr      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-jalr.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_lb        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lb.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_lbu       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lbu.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_lh        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lh.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_lhu       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lhu.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_lui       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lui.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_lw        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-lw.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_or        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-or.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_ori       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-ori.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_sb        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sb.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_sh        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sh.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_simple    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-simple.bin"     .to_string()), 1); }
#[test]fn rv32ui_p_sll       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sll.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_slli      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-slli.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_slt       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-slt.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_slti      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-slti.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_sltiu     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sltiu.bin"      .to_string()), 1); }
#[test]fn rv32ui_p_sltu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sltu.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_sra       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sra.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_srai      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-srai.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_srl       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-srl.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_srli      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-srli.bin"       .to_string()), 1); }
#[test]fn rv32ui_p_sub       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sub.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_sw        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-sw.bin"         .to_string()), 1); }
#[test]fn rv32ui_p_xor       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-xor.bin"        .to_string()), 1); }
#[test]fn rv32ui_p_xori      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-p-xori.bin"       .to_string()), 1); }

#[test]fn rv32ui_v_add       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-add.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_addi      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-addi.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_and       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-and.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_andi      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-andi.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_auipc     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-auipc.bin"      .to_string()), 1); }
#[test]fn rv32ui_v_beq       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-beq.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_bge       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-bge.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_bgeu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-bgeu.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_blt       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-blt.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_bltu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-bltu.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_bne       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-bne.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_fence_i   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-fence_i.bin"    .to_string()), 1); }
#[test]fn rv32ui_v_jal       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-jal.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_jalr      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-jalr.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_lb        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lb.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_lbu       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lbu.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_lh        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lh.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_lhu       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lhu.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_lui       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lui.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_lw        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-lw.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_or        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-or.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_ori       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-ori.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_sb        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sb.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_sh        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sh.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_simple    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-simple.bin"     .to_string()), 1); }
#[test]fn rv32ui_v_sll       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sll.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_slli      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-slli.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_slt       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-slt.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_slti      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-slti.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_sltiu     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sltiu.bin"      .to_string()), 1); }
#[test]fn rv32ui_v_sltu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sltu.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_sra       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sra.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_srai      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-srai.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_srl       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-srl.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_srli      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-srli.bin"       .to_string()), 1); }
#[test]fn rv32ui_v_sub       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sub.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_sw        ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-sw.bin"         .to_string()), 1); }
#[test]fn rv32ui_v_xor       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-xor.bin"        .to_string()), 1); }
#[test]fn rv32ui_v_xori      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ui-v-xori.bin"       .to_string()), 1); }
#[test]fn rv32um_p_div       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-div.bin"        .to_string()), 1); }
#[test]fn rv32um_p_divu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-divu.bin"       .to_string()), 1); }
#[test]fn rv32um_p_mul       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-mul.bin"        .to_string()), 1); }
#[test]fn rv32um_p_mulh      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-mulh.bin"       .to_string()), 1); }
#[test]fn rv32um_p_mulhsu    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-mulhsu.bin"     .to_string()), 1); }
#[test]fn rv32um_p_mulhu     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-mulhu.bin"      .to_string()), 1); }
#[test]fn rv32um_p_rem       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-rem.bin"        .to_string()), 1); }
#[test]fn rv32um_p_remu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-p-remu.bin"       .to_string()), 1); }
#[test]fn rv32um_v_div       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-div.bin"        .to_string()), 1); }
#[test]fn rv32um_v_divu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-divu.bin"       .to_string()), 1); }
#[test]fn rv32um_v_mul       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-mul.bin"        .to_string()), 1); }
#[test]fn rv32um_v_mulh      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-mulh.bin"       .to_string()), 1); }
#[test]fn rv32um_v_mulhsu    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-mulhsu.bin"     .to_string()), 1); }
#[test]fn rv32um_v_mulhu     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-mulhu.bin"      .to_string()), 1); }
#[test]fn rv32um_v_rem       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-rem.bin"        .to_string()), 1); }
#[test]fn rv32um_v_remu      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32um-v-remu.bin"       .to_string()), 1); }

#[test]fn rv64ui_p_add    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-add.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_addi   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-addi.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_addiw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-addiw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_addw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-addw.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_and    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-and.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_andi   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-andi.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_auipc  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-auipc.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_beq    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-beq.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_bge    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-bge.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_bgeu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-bgeu.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_blt    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-blt.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_bltu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-bltu.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_bne    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-bne.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_fence_i() { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-fence_i.bin" .to_string()), 1); }
#[test]fn rv64ui_p_jal    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-jal.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_jalr   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-jalr.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_lb     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lb.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_lbu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lbu.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_ld     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-ld.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_lh     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lh.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_lhu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lhu.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_lui    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lui.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_lw     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lw.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_lwu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-lwu.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_or     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-or.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_ori    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-ori.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_sb     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sb.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_sd     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sd.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_sh     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sh.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_simple () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-simple.bin"  .to_string()), 1); }
#[test]fn rv64ui_p_sll    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sll.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_slli   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-slli.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_slliw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-slliw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_sllw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sllw.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_slt    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-slt.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_slti   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-slti.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_sltiu  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sltiu.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_sltu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sltu.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_sra    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sra.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_srai   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-srai.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_sraiw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sraiw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_sraw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sraw.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_srl    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-srl.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_srli   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-srli.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_srliw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-srliw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_srlw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-srlw.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_sub    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sub.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_subw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-subw.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_sw     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-sw.bin"      .to_string()), 1); }
#[test]fn rv64ui_p_xor    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-xor.bin"     .to_string()), 1); }
#[test]fn rv64ui_p_xori   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-p-xori.bin"    .to_string()), 1); }

#[test]fn rv64ui_p_div    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-div.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_divu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-divu.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_divuw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-divuw.bin"  .to_string()), 1); }
#[test]fn rv64ui_p_divw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-divw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_mul    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-mul.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_mulh   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-mulh.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_mulhsu () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-mulhsu.bin" .to_string()), 1); }
#[test]fn rv64ui_p_mulhu  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-mulhu.bin"  .to_string()), 1); }
#[test]fn rv64ui_p_mulw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-mulw.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_rem    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-rem.bin"    .to_string()), 1); }
#[test]fn rv64ui_p_remu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-remu.bin"   .to_string()), 1); }
#[test]fn rv64ui_p_remuw  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-remuw.bin"  .to_string()), 1); }
#[test]fn rv64ui_p_remw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-p-remw.bin"   .to_string()), 1); }


#[test]fn rv64ui_v_add     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-add.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_addi    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-addi.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_addiw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-addiw.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_addw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-addw.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_and     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-and.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_andi    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-andi.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_auipc   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-auipc.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_beq     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-beq.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_bge     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-bge.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_bgeu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-bgeu.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_blt     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-blt.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_bltu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-bltu.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_bne     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-bne.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_fence_i () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-fence_i.bin" .to_string()), 1); }
#[test]fn rv64ui_v_jal     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-jal.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_jalr    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-jalr.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_lb      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lb.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_lbu     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lbu.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_ld      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-ld.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_lh      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lh.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_lhu     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lhu.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_lui     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lui.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_lw      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lw.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_lwu     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-lwu.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_or      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-or.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_ori     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-ori.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_sb      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sb.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_sd      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sd.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_sh      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sh.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_simple  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-simple.bin"  .to_string()), 1); }
#[test]fn rv64ui_v_sll     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sll.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_slli    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-slli.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_slliw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-slliw.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_sllw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sllw.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_slt     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-slt.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_slti    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-slti.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_sltiu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sltiu.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_sltu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sltu.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_sra     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sra.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_srai    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-srai.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_sraiw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sraiw.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_sraw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sraw.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_srl     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-srl.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_srli    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-srli.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_srliw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-srliw.bin"   .to_string()), 1); }
#[test]fn rv64ui_v_srlw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-srlw.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_sub     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sub.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_subw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-subw.bin"    .to_string()), 1); }
#[test]fn rv64ui_v_sw      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-sw.bin"      .to_string()), 1); }
#[test]fn rv64ui_v_xor     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-xor.bin"     .to_string()), 1); }
#[test]fn rv64ui_v_xori    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ui-v-xori.bin"    .to_string()), 1); }

#[test]fn rv64um_v_div     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-div.bin"     .to_string()), 1); }
#[test]fn rv64um_v_divu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-divu.bin"    .to_string()), 1); }
#[test]fn rv64um_v_divuw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-divuw.bin"   .to_string()), 1); }
#[test]fn rv64um_v_divw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-divw.bin"    .to_string()), 1); }
#[test]fn rv64um_v_mul     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-mul.bin"     .to_string()), 1); }
#[test]fn rv64um_v_mulh    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-mulh.bin"    .to_string()), 1); }
#[test]fn rv64um_v_mulhsu  () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-mulhsu.bin"  .to_string()), 1); }
#[test]fn rv64um_v_mulhu   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-mulhu.bin"   .to_string()), 1); }
#[test]fn rv64um_v_mulw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-mulw.bin"    .to_string()), 1); }
#[test]fn rv64um_v_rem     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-rem.bin"     .to_string()), 1); }
#[test]fn rv64um_v_remu    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-remu.bin"    .to_string()), 1); }
#[test]fn rv64um_v_remuw   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-remuw.bin"   .to_string()), 1); }
#[test]fn rv64um_v_remw    () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64um-v-remw.bin"    .to_string()), 1); }

#[test]fn rv64ud_p_fadd       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fadd.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_fclass     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fclass.bin"     .to_string()), 1); }
#[test]fn rv64ud_p_fcmp       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fcmp.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_fcvt       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fcvt.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_fcvt_w     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fcvt_w.bin"     .to_string()), 1); }
#[test]fn rv64ud_p_fdiv       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fdiv.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_fmadd      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fmadd.bin"      .to_string()), 1); }
#[test]fn rv64ud_p_fmin       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-fmin.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_ldst       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-ldst.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_move       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-move.bin"       .to_string()), 1); }
#[test]fn rv64ud_p_recoding   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-recoding.bin"   .to_string()), 1); }
#[test]fn rv64ud_p_structural () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-p-structural.bin" .to_string()), 1); }
#[test]fn rv64uf_p_fadd       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fadd.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_fclass     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fclass.bin"     .to_string()), 1); }
#[test]fn rv64uf_p_fcmp       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fcmp.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_fcvt       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fcvt.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_fcvt_w     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fcvt_w.bin"     .to_string()), 1); }
#[test]fn rv64uf_p_fdiv       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fdiv.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_fmadd      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fmadd.bin"      .to_string()), 1); }
#[test]fn rv64uf_p_fmin       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-fmin.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_ldst       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-ldst.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_move       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-move.bin"       .to_string()), 1); }
#[test]fn rv64uf_p_recoding   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-p-recoding.bin"   .to_string()), 1); }

#[test]fn rv64ud_v_fadd       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fadd.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_fclass     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fclass.bin"     .to_string()), 1); }
#[test]fn rv64ud_v_fcmp       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fcmp.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_fcvt       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fcvt.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_fcvt_w     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fcvt_w.bin"     .to_string()), 1); }
#[test]fn rv64ud_v_fdiv       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fdiv.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_fmadd      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fmadd.bin"      .to_string()), 1); }
#[test]fn rv64ud_v_fmin       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-fmin.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_ldst       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-ldst.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_move       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-move.bin"       .to_string()), 1); }
#[test]fn rv64ud_v_recoding   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-recoding.bin"   .to_string()), 1); }
#[test]fn rv64ud_v_structural () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64ud-v-structural.bin" .to_string()), 1); }
#[test]fn rv64uf_v_fadd       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fadd.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_fclass     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fclass.bin"     .to_string()), 1); }
#[test]fn rv64uf_v_fcmp       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fcmp.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_fcvt       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fcvt.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_fcvt_w     () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fcvt_w.bin"     .to_string()), 1); }
#[test]fn rv64uf_v_fdiv       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fdiv.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_fmadd      () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fmadd.bin"      .to_string()), 1); }
#[test]fn rv64uf_v_fmin       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-fmin.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_ldst       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-ldst.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_move       () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-move.bin"       .to_string()), 1); }
#[test]fn rv64uf_v_recoding   () { assert_eq!(swimmer_rust::swimmer_rust_exec(64, "riscv-tests/isa/rv64uf-v-recoding.bin"   .to_string()), 1); }

//
// Unsupported RV32 Instruction Tests
//
// #[test]fn rv32mi_p_breakpoint()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-breakpoint.bin" .to_string()), 1); }
// #[test]fn rv32mi_p_csr       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-csr.bin"        .to_string()), 1); }
// #[test]fn rv32mi_p_illegal   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-illegal.bin"    .to_string()), 1); }
// #[test]fn rv32mi_p_ma_addr   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-ma_addr.bin"    .to_string()), 1); }
// #[test]fn rv32mi_p_ma_fetch  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-ma_fetch.bin"   .to_string()), 1); }
// #[test]fn rv32mi_p_mcsr      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-mcsr.bin"       .to_string()), 1); }
// #[test]fn rv32mi_p_sbreak    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-sbreak.bin"     .to_string()), 1); }
// #[test]fn rv32mi_p_scall     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-scall.bin"      .to_string()), 1); }
// #[test]fn rv32mi_p_shamt     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32mi-p-shamt.bin"      .to_string()), 1); }
//
// #[test]fn rv32si_p_csr       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-csr.bin"        .to_string()), 1); }
// #[test]fn rv32si_p_dirty     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-dirty.bin"      .to_string()), 1); }
// #[test]fn rv32si_p_ma_fetch  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-ma_fetch.bin"   .to_string()), 1); }
// #[test]fn rv32si_p_sbreak    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-sbreak.bin"     .to_string()), 1); }
// #[test]fn rv32si_p_scall     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-scall.bin"      .to_string()), 1); }
// #[test]fn rv32si_p_wfi       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32si-p-wfi.bin"        .to_string()), 1); }
//
// #[test]fn rv32ua_p_amoadd_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amoadd_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_p_amoand_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amoand_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_p_amomax_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amomax_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_p_amomaxu_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amomaxu_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_p_amomin_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amomin_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_p_amominu_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amominu_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_p_amoor_w   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amoor_w.bin"    .to_string()), 1); }
// #[test]fn rv32ua_p_amoswap_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amoswap_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_p_amoxor_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-amoxor_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_p_lrsc      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-p-lrsc.bin"       .to_string()), 1); }
//
// #[test]fn rv32ua_v_amoadd_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amoadd_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_v_amoand_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amoand_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_v_amomax_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amomax_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_v_amomaxu_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amomaxu_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_v_amomin_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amomin_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_v_amominu_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amominu_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_v_amoor_w   ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amoor_w.bin"    .to_string()), 1); }
// #[test]fn rv32ua_v_amoswap_w ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amoswap_w.bin"  .to_string()), 1); }
// #[test]fn rv32ua_v_amoxor_w  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-amoxor_w.bin"   .to_string()), 1); }
// #[test]fn rv32ua_v_lrsc      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ua-v-lrsc.bin"       .to_string()), 1); }
//
// #[test]fn rv32uc_p_rvc       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uc-p-rvc.bin"        .to_string()), 1); }
// #[test]fn rv32uc_v_rvc       ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uc-v-rvc.bin"        .to_string()), 1); }
//
// #[test]fn rv32ud_p_fadd      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fadd.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_fclass    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fclass.bin"     .to_string()), 1); }
// #[test]fn rv32ud_p_fcmp      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fcmp.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_fcvt      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fcvt.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_fcvt_w    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fcvt_w.bin"     .to_string()), 1); }
// #[test]fn rv32ud_p_fdiv      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fdiv.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_fmadd     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fmadd.bin"      .to_string()), 1); }
// #[test]fn rv32ud_p_fmin      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-fmin.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_ldst      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-ldst.bin"       .to_string()), 1); }
// #[test]fn rv32ud_p_recoding  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-p-recoding.bin"   .to_string()), 1); }
//
// #[test]fn rv32ud_v_fadd      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fadd.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_fclass    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fclass.bin"     .to_string()), 1); }
// #[test]fn rv32ud_v_fcmp      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fcmp.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_fcvt      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fcvt.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_fcvt_w    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fcvt_w.bin"     .to_string()), 1); }
// #[test]fn rv32ud_v_fdiv      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fdiv.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_fmadd     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fmadd.bin"      .to_string()), 1); }
// #[test]fn rv32ud_v_fmin      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-fmin.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_ldst      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-ldst.bin"       .to_string()), 1); }
// #[test]fn rv32ud_v_recoding  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32ud-v-recoding.bin"   .to_string()), 1); }
//
// #[test]fn rv32uf_p_fadd      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fadd.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_fclass    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fclass.bin"     .to_string()), 1); }
// #[test]fn rv32uf_p_fcmp      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fcmp.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_fcvt      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fcvt.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_fcvt_w    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fcvt_w.bin"     .to_string()), 1); }
// #[test]fn rv32uf_p_fdiv      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fdiv.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_fmadd     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fmadd.bin"      .to_string()), 1); }
// #[test]fn rv32uf_p_fmin      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-fmin.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_ldst      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-ldst.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_move      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-move.bin"       .to_string()), 1); }
// #[test]fn rv32uf_p_recoding  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-p-recoding.bin"   .to_string()), 1); }
//
// #[test]fn rv32uf_v_fadd      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fadd.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_fclass    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fclass.bin"     .to_string()), 1); }
// #[test]fn rv32uf_v_fcmp      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fcmp.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_fcvt      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fcvt.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_fcvt_w    ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fcvt_w.bin"     .to_string()), 1); }
// #[test]fn rv32uf_v_fdiv      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fdiv.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_fmadd     ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fmadd.bin"      .to_string()), 1); }
// #[test]fn rv32uf_v_fmin      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-fmin.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_ldst      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-ldst.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_move      ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-move.bin"       .to_string()), 1); }
// #[test]fn rv32uf_v_recoding  ()  { assert_eq!(swimmer_rust::swimmer_rust_exec(32, "riscv-tests/isa/rv32uf-v-recoding.bin"   .to_string()), 1); }
