//! All CPU instruction tests reside here
//!
//! These tests are not exhaustive though, so any contributions
//! or additions here would be appreciated

#[cfg(test)]
mod test {
    use nes_emulator::cpu::{CPU, mem::Mem, processor_status::ProcessorStatus};

    // == ADC TESTS ==
    #[test]
    fn test_adc_no_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x10, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;

        cpu.run();

        assert_eq!(cpu.register_a, 0x60);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_out_and_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x90, 0x00]);
        cpu.reset();

        cpu.register_a = 0xd0;

        cpu.run();

        assert_eq!(cpu.register_a, 0x60);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0xd0, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;
        cpu.status |= 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0x21);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow)); // can't overflow if we're adding pos and neg
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_no_carry_out_and_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x50, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;
        cpu.status |= 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0xa1);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_in_sets_carry_out() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x9f, 0x00]);
        cpu.reset();

        cpu.register_a = 0x60;
        cpu.status |= 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0x0);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_in_sets_carry_out_2() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x00, 0x00]);
        cpu.reset();

        cpu.register_a = 0xFF;
        cpu.status |= 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0x0);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_in_sets_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x39, 0x00]);
        cpu.reset();

        cpu.register_a = 0x46;
        cpu.status |= 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0x80);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == AND TESTS ==
    #[test]
    fn test_and_non_zero_res() {
        let mut cpu = CPU::new();

        cpu.load(&[0x29, 0x11, 0x00]);
        cpu.reset();

        cpu.register_a = 0x01;

        cpu.run();

        assert_eq!(cpu.register_a, 0x01);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_and_zero_res() {
        let mut cpu = CPU::new();

        cpu.load(&[0x29, 0x1F, 0x00]);
        cpu.reset();

        cpu.register_a = 0x00;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == ASL TESTS ==
    #[test]
    fn test_asl_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x0A, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;

        cpu.run();

        assert_eq!(cpu.register_a, 0x0A);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_asl_from_mem() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0xAA);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == BRANCH TESTS ==
    #[test]
    fn test_bcc_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x90, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bcs_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xB0, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();

        cpu.status |= 0x01;

        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_beq_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xF0, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.status |= 0b10;
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bmi_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x30, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.status |= 0b1000_0000;
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bne_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xD0, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bpl_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x10, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bvc_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x50, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }

    #[test]
    fn test_bvs_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x90, 0x50]);
        cpu.mem_write(0x8052, 0x00);
        cpu.reset();
        cpu.status |= 0b0100_0000;
        cpu.run();

        // this should be the last address read before it
        // returns
        assert_eq!(cpu.program_counter, 0x8052);
    }
    // ===============

    // == BIT TESTS ==
    #[test]
    fn test_bit_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x24, 0x80, 0x00]);
        cpu.reset();
        cpu.mem_write(0x80, 0xC0);
        cpu.register_a = 0x40;
        cpu.run();

        assert_eq!(cpu.register_a, 0x40);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == CLR TESTS ==
    #[test]
    fn test_clc_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x18, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001;
        cpu.run();

        assert!(!cpu.is_status_flag_set(ProcessorStatus::Carry));
    }

    #[test]
    fn test_cld_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xD8, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_1000;
        cpu.run();

        assert!(!cpu.is_status_flag_set(ProcessorStatus::Decimal));
    }

    #[test]
    fn test_cli_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x58, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0100;
        cpu.run();

        assert!(!cpu.is_status_flag_set(ProcessorStatus::InterruptDisable));
    }

    #[test]
    fn test_clv_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xB8, 0x00]);
        cpu.reset();
        cpu.status |= 0b0100_0000;
        cpu.run();

        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow));
    }
    // ===============

    // == CMP TESTS ==
    #[test]
    fn test_cmp_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xC9, 0x80, 0x00]);
        cpu.reset();
        cpu.register_a = 0x80;
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == CPX TESTS ==
    #[test]
    fn test_cpx_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xE0, 0x80, 0x00]);
        cpu.reset();
        cpu.register_x = 0x80;
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == CPY TESTS ==
    #[test]
    fn test_cpy_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xC0, 0x80, 0x00]);
        cpu.reset();
        cpu.register_y = 0x80;
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == DEC TESTS ==
    #[test]
    fn test_dec_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xC6, 0x80, 0x00]);
        cpu.reset();
        cpu.mem_write(0x80, 0x8C);
        cpu.run();

        assert_eq!(cpu.mem_read(0x80), 0x8B);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == DEX TESTS ==
    #[test]
    fn test_dex_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xCA, 0x00]);
        cpu.reset();
        cpu.register_x = 0x8C;
        cpu.run();

        assert_eq!(cpu.register_x, 0x8B);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == DEY TESTS ==
    #[test]
    fn test_dey_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x88, 0x00]);
        cpu.reset();
        cpu.register_y = 0x8C;
        cpu.run();

        assert_eq!(cpu.register_y, 0x8B);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == EOR TESTS ==
    #[test]
    fn test_eor_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x49, 0x45, 0x00]);
        cpu.reset();
        cpu.register_a = 0x77;
        cpu.run();

        assert_eq!(cpu.register_a, 0x32);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == INC TESTS ==
    #[test]
    fn test_inc_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xE6, 0x80, 0x00]);
        cpu.reset();
        cpu.mem_write(0x80, 0x8C);
        cpu.run();

        assert_eq!(cpu.mem_read(0x80), 0x8D);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // INX TESTS
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0xe8, 0xe8, 0x00]);
        cpu.reset();

        cpu.register_x = 0xff;

        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }
    // ===============

    // == INY TESTS ==
    #[test]
    fn test_iny_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xC8, 0x00]);
        cpu.reset();
        cpu.register_y = 0x8C;
        cpu.run();

        assert_eq!(cpu.register_y, 0x8D);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == JMP TESTS ==
    #[test]
    fn test_jmp_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x4C, 0x21, 0x20]);
        cpu.reset();
        cpu.mem_write(0x2021, 0x00);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x2021);
    }
    // ===============

    // == JSR TESTS ==
    #[test]
    fn test_jsr_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x20, 0x21, 0x20]);
        cpu.reset();
        cpu.mem_write(0x2021, 0x00);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x2021);
        assert_eq!(cpu.stack_pointer, 0xFB);
        assert_eq!(cpu.mem_read(0x1FD), 0x80);
        assert_eq!(cpu.mem_read(0x1FC), 0x02);
    }
    // ===============

    // LDA TESTS
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(&[0xa9, 0x05, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load(&[0xa9, 0x00, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
    // ===============

    // LDX TESTS
    #[test]
    fn test_0xa9_ldx_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(&[0xa2, 0x05, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_0xa9_ldx_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load(&[0xa2, 0x00, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
    }

    #[test]
    fn test_ldx_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa6, 0x10, 0x00]);

        assert_eq!(cpu.register_x, 0x55);
    }
    // ===============

    // LDY TESTS
    #[test]
    fn test_0xa9_ldy_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(&[0xa0, 0x05, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_0xa9_ldy_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load(&[0xa0, 0x00, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Zero));
    }

    #[test]
    fn test_ldy_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa4, 0x10, 0x00]);

        assert_eq!(cpu.register_y, 0x55);
    }
    // ===============

    // == ASL TESTS ==
    #[test]
    fn test_lsr_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x4A, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;

        cpu.run();

        assert_eq!(cpu.register_a, 0x02);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_lsr_from_mem() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0x46, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x2A);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == ORA TESTS ==
    #[test]
    fn test_ora_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x09, 0x45, 0x00]);
        cpu.reset();
        cpu.register_a = 0x76;
        cpu.run();

        assert_eq!(cpu.register_a, 0x77);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == PHA TESTS ==
    #[test]
    fn test_pha_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x48, 0x00]);
        cpu.reset();
        cpu.register_a = 0x56;
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFC);
        assert_eq!(cpu.register_a, 0x56);
        assert_eq!(cpu.mem_read(0x1FD), 0x56);
    }
    // ===============

    // == PHP TESTS ==
    #[test]
    fn test_php_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x08, 0x00]);
        cpu.reset();
        cpu.status = 0b1000_1100;
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFC);
        assert_eq!(cpu.status, 0b1000_1100);
        assert_eq!(cpu.mem_read(0x1FD), 0b1000_1100);
    }
    // ===============

    // == PLA TESTS ==
    #[test]
    fn test_pla_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x68, 0x00]);
        cpu.reset();
        cpu.mem_write(0x1FE, 0x59);
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFE);
        assert_eq!(cpu.register_a, 0x59);
        assert_eq!(cpu.mem_read(0x1FE), 0x59);
    }
    // ===============

    // == PLP TESTS ==
    #[test]
    fn test_plp_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x28, 0x00]);
        cpu.reset();
        cpu.mem_write(0x1FE, 0b1100_0101);
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFE);
        assert_eq!(cpu.status, 0b1100_0101);
        assert_eq!(cpu.mem_read(0x1FE), 0b1100_0101);
    }
    // ===============

    // == ROL TESTS ==
    #[test]
    fn test_rol_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x2A, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;

        cpu.run();

        assert_eq!(cpu.register_a, 0x0A);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }

    #[test]
    fn test_rol_from_mem() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0x26, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0xAA);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == ROR TESTS ==
    #[test]
    fn test_ror_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x6A, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;
        cpu.status = 0b0000_0001;

        cpu.run();

        assert_eq!(cpu.register_a, 0x82);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Negative));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
    }

    #[test]
    fn test_ror_from_mem() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0x66, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x2A);
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
    }
    // ===============

    // == RTI TESTS ==
    #[test]
    fn test_rti_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x40, 0x00]);
        cpu.reset();
        cpu.mem_write(0x2021, 0x00);
        cpu.mem_write(0x1FE, 0b1100_0011);
        cpu.mem_write(0x1FF, 0x21);
        cpu.mem_write(0x100, 0x20);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x2021);
        assert_eq!(cpu.stack_pointer, 0x00);
        assert_eq!(cpu.status, 0b1100_0011);
    }
    // ===============

    // == RTS TESTS ==
    #[test]
    fn test_rts_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x20, 0x21, 0x20, 0x00]);
        cpu.reset();
        cpu.mem_write(0x2021, 0x60);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x8003);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }
    // ===============

    // == SBC TESTS ==
    #[test]
    fn test_sbc_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0xE9, 0x20, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;
        cpu.status = 0b0000_0001;

        cpu.run();

        assert_eq!(cpu.register_a, 0x30);
        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(ProcessorStatus::Negative));
    }
    // ===============

    // == SET TESTS ==
    #[test]
    fn test_sec_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x38, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Carry));
    }

    #[test]
    fn test_sed_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xF8, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::Decimal));
    }

    #[test]
    fn test_sei_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x78, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(ProcessorStatus::InterruptDisable));
    }
    // ===============

    // LDA, TAX, INX, TESTS
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.load(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_x, 0xc1)
    }
    // ===============
}
