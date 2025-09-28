#[cfg(test)]
mod test {
    use nes_emulator::cpu::{CPU, mem::Mem, processor_status::ProcessorStatus};

    // LDA TESTS
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(&[0xa9, 0x05, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load(&[0xa9, 0x00, 0x00]);
        cpu.reset();
        cpu.run();

        assert!(cpu.is_status_flag_set(&ProcessorStatus::Zero));
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
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

    // == ADC TESTS ==
    #[test]
    fn test_adc_no_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x10, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;

        cpu.run();

        assert_eq!(cpu.register_a, 0x60);
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
    }

    #[test]
    fn test_adc_carry_out_and_overflow() {
        let mut cpu = CPU::new();

        cpu.load(&[0x69, 0x90, 0x00]);
        cpu.reset();

        cpu.register_a = 0xd0;

        cpu.run();

        assert_eq!(cpu.register_a, 0x60);
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Overflow)); // can't overflow if we're adding pos and neg
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Carry));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
    }

    #[test]
    fn test_and_zero_res() {
        let mut cpu = CPU::new();

        cpu.load(&[0x29, 0x1F, 0x00]);
        cpu.reset();

        cpu.register_a = 0x00;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Negative));
    }

    #[test]
    fn test_asl_from_mem() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0xAA);
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Negative));
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
        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Zero));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Overflow));
        assert!(cpu.is_status_flag_set(&ProcessorStatus::Negative));
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

        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Carry));
    }

    #[test]
    fn test_cld_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xD8, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_1000;
        cpu.run();

        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Decimal));
    }

    #[test]
    fn test_cli_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0x58, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0100;
        cpu.run();

        assert!(!cpu.is_status_flag_set(&ProcessorStatus::InterruptDisable));
    }

    #[test]
    fn test_clv_1() {
        let mut cpu = CPU::new();

        cpu.load(&[0xB8, 0x00]);
        cpu.reset();
        cpu.status |= 0b0100_0000;
        cpu.run();

        assert!(!cpu.is_status_flag_set(&ProcessorStatus::Overflow));
    }
    // ===============
}
