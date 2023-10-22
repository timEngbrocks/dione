#[macro_export]
macro_rules! opcodes {
	($instruction:path) => {
		match stringify!($instruction) {
			"Instructions::NOP" => 0x00,
			"Instructions::ACONST_NULL" => 0x01,
			"Instructions::ICONST_M1" => 0x02,
			"Instructions::ICONST_0" => 0x03,
			"Instructions::ICONST_1" => 0x04,
			"Instructions::ICONST_2" => 0x05,
			"Instructions::ICONST_3" => 0x06,
			"Instructions::ICONST_4" => 0x07,
			"Instructions::ICONST_5" => 0x08,
			"Instructions::LCONST_0" => 0x09,
			"Instructions::LCONST_1" => 0x0a,
			"Instructions::FCONST_0" => 0x0b,
			"Instructions::FCONST_1" => 0x0c,
			"Instructions::FCONST_2" => 0x0d,
			"Instructions::DCONST_0" => 0x0e,
			"Instructions::DCONST_1" => 0x0f,
			"Instructions::BIPUSH" => 0x10,
			"Instructions::SIPUSH" => 0x11,
			"Instructions::LDC" => 0x12,
			"Instructions::LDC_W" => 0x13,
			"Instructions::LDC2_W" => 0x14,
			"Instructions::ILOAD" => 0x15,
			"Instructions::LLOAD" => 0x16,
			"Instructions::FLOAD" => 0x17,
			"Instructions::DLOAD" => 0x18,
			"Instructions::ALOAD" => 0x19,
			"Instructions::ILOAD_0" => 0x1a,
			"Instructions::ILOAD_1" => 0x1b,
			"Instructions::ILOAD_2" => 0x1c,
			"Instructions::ILOAD_3" => 0x1d,
			"Instructions::LLOAD_0" => 0x1e,
			"Instructions::LLOAD_1" => 0x1f,
			"Instructions::LLOAD_2" => 0x20,
			"Instructions::LLOAD_3" => 0x21,
			"Instructions::FLOAD_0" => 0x22,
			"Instructions::FLOAD_1" => 0x23,
			"Instructions::FLOAD_2" => 0x24,
			"Instructions::FLOAD_3" => 0x25,
			"Instructions::DLOAD_0" => 0x26,
			"Instructions::DLOAD_1" => 0x27,
			"Instructions::DLOAD_2" => 0x28,
			"Instructions::DLOAD_3" => 0x29,
			"Instructions::ALOAD_0" => 0x2a,
			"Instructions::ALOAD_1" => 0x2b,
			"Instructions::ALOAD_2" => 0x2c,
			"Instructions::ALOAD_3" => 0x2d,
			"Instructions::IALOAD" => 0x2e,
			"Instructions::LALOAD" => 0x2f,
			"Instructions::FALOAD" => 0x30,
			"Instructions::DALOAD" => 0x31,
			"Instructions::AALOAD" => 0x32,
			"Instructions::BALOAD" => 0x33,
			"Instructions::CALOAD" => 0x34,
			"Instructions::SALOAD" => 0x35,
			"Instructions::ISTORE" => 0x36,
			"Instructions::LSTORE" => 0x37,
			"Instructions::FSTORE" => 0x38,
			"Instructions::DSTORE" => 0x39,
			"Instructions::ASTORE" => 0x3a,
			"Instructions::ISTORE_0" => 0x3b,
			"Instructions::ISTORE_1" => 0x3c,
			"Instructions::ISTORE_2" => 0x3d,
			"Instructions::ISTORE_3" => 0x3e,
			"Instructions::LSTORE_0" => 0x3f,
			"Instructions::LSTORE_1" => 0x40,
			"Instructions::LSTORE_2" => 0x41,
			"Instructions::LSTORE_3" => 0x42,
			"Instructions::FSTORE_0" => 0x43,
			"Instructions::FSTORE_1" => 0x44,
			"Instructions::FSTORE_2" => 0x45,
			"Instructions::FSTORE_3" => 0x46,
			"Instructions::DSTORE_0" => 0x47,
			"Instructions::DSTORE_1" => 0x48,
			"Instructions::DSTORE_2" => 0x49,
			"Instructions::DSTORE_3" => 0x4a,
			"Instructions::ASTORE_0" => 0x4b,
			"Instructions::ASTORE_1" => 0x4c,
			"Instructions::ASTORE_2" => 0x4d,
			"Instructions::ASTORE_3" => 0x4e,
			"Instructions::IASTORE" => 0x4f,
			"Instructions::LASTORE" => 0x50,
			"Instructions::FASTORE" => 0x51,
			"Instructions::DASTORE" => 0x52,
			"Instructions::AASTORE" => 0x53,
			"Instructions::BASTORE" => 0x54,
			"Instructions::CASTORE" => 0x55,
			"Instructions::SASTORE" => 0x56,
			"Instructions::POP" => 0x57,
			"Instructions::POP2" => 0x58,
			"Instructions::DUP" => 0x59,
			"Instructions::DUP_X1" => 0x5a,
			"Instructions::DUP_X2" => 0x5b,
			"Instructions::DUP2" => 0x5c,
			"Instructions::DUP2_X1" => 0x5d,
			"Instructions::DUP2_X2" => 0x5e,
			"Instructions::SWAP" => 0x5f,
			"Instructions::IADD" => 0x60,
			"Instructions::LADD" => 0x61,
			"Instructions::FADD" => 0x62,
			"Instructions::DADD" => 0x63,
			"Instructions::ISUB" => 0x64,
			"Instructions::LSUB" => 0x65,
			"Instructions::FSUB" => 0x66,
			"Instructions::DSUB" => 0x67,
			"Instructions::IMUL" => 0x68,
			"Instructions::LMUL" => 0x69,
			"Instructions::FMUL" => 0x6a,
			"Instructions::DMUL" => 0x6b,
			"Instructions::IDIV" => 0x6c,
			"Instructions::LDIV" => 0x6d,
			"Instructions::FDIV" => 0x6e,
			"Instructions::DDIV" => 0x6f,
			"Instructions::IREM" => 0x70,
			"Instructions::LREM" => 0x71,
			"Instructions::FREM" => 0x72,
			"Instructions::DREM" => 0x73,
			"Instructions::INEG" => 0x74,
			"Instructions::LNEG" => 0x75,
			"Instructions::FNEG" => 0x76,
			"Instructions::DNEG" => 0x77,
			"Instructions::ISHL" => 0x78,
			"Instructions::LSHL" => 0x79,
			"Instructions::ISHR" => 0x7a,
			"Instructions::LSHR" => 0x7b,
			"Instructions::IUSHR" => 0x7c,
			"Instructions::LUSHR" => 0x7d,
			"Instructions::IAND" => 0x7e,
			"Instructions::LAND" => 0x7f,
			"Instructions::IOR" => 0x80,
			"Instructions::LOR" => 0x81,
			"Instructions::IXOR" => 0x82,
			"Instructions::LXOR" => 0x83,
			"Instructions::IINC" => 0x84,
			"Instructions::I2L" => 0x85,
			"Instructions::I2F" => 0x86,
			"Instructions::I2D" => 0x87,
			"Instructions::L2I" => 0x88,
			"Instructions::L2F" => 0x89,
			"Instructions::L2D" => 0x8a,
			"Instructions::F2I" => 0x8b,
			"Instructions::F2L" => 0x8c,
			"Instructions::F2D" => 0x8d,
			"Instructions::D2I" => 0x8e,
			"Instructions::D2L" => 0x8f,
			"Instructions::D2F" => 0x90,
			"Instructions::I2B" => 0x91,
			"Instructions::I2C" => 0x92,
			"Instructions::I2S" => 0x93,
			"Instructions::LCMP" => 0x94,
			"Instructions::FCMPL" => 0x95,
			"Instructions::FCMPG" => 0x96,
			"Instructions::DCMPL" => 0x97,
			"Instructions::DCMPG" => 0x98,
			"Instructions::IFEQ" => 0x99,
			"Instructions::IFNE" => 0x9a,
			"Instructions::IFLT" => 0x9b,
			"Instructions::IFGE" => 0x9c,
			"Instructions::IFGT" => 0x9d,
			"Instructions::IFLE" => 0x9e,
			"Instructions::IF_ICMPEQ" => 0x9f,
			"Instructions::IF_ICMPNE" => 0xa0,
			"Instructions::IF_ICMPLT" => 0xa1,
			"Instructions::IF_ICMPGE" => 0xa2,
			"Instructions::IF_ICMPGT" => 0xa3,
			"Instructions::IF_ICMPLE" => 0xa4,
			"Instructions::IF_ACMPEQ" => 0xa5,
			"Instructions::IF_ACMPNE" => 0xa6,
			"Instructions::GOTO" => 0xa7,
			"Instructions::JSR" => 0xa8,
			"Instructions::RET" => 0xa9,
			"Instructions::TABLESWITCH" => 0xaa,
			"Instructions::LOOKUPSWITCH" => 0xab,
			"Instructions::IRETURN" => 0xac,
			"Instructions::LRETURN" => 0xad,
			"Instructions::FRETURN" => 0xae,
			"Instructions::DRETURN" => 0xaf,
			"Instructions::ARETURN" => 0xb0,
			"Instructions::RETURN" => 0xb1,
			"Instructions::GETSTATIC" => 0xb2,	
			"Instructions::PUTSTATIC" => 0xb3,	
			"Instructions::GETFIELD" => 0xb4,	
			"Instructions::PUTFIELD" => 0xb5,	
			"Instructions::INVOKEVIRTUAL" => 0xb6,	
			"Instructions::INVOKESPECIAL" => 0xb7,	
			"Instructions::INVOKESTATIC" => 0xb8,	
			"Instructions::INVOKEINTERFACE" => 0xb9,	
			"Instructions::INVOKEDYNAMIC" => 0xba,	
			"Instructions::NEW" => 0xbb,	
			"Instructions::NEWARRAY" => 0xbc,	
			"Instructions::ANEWARRAY" => 0xbd,	
			"Instructions::ARRAYLENGTH" => 0xbe,	
			"Instructions::ATHROW" => 0xbf,	
			"Instructions::CHECKCAST" => 0xc0,	
			"Instructions::INSTANCEOF" => 0xc1,	
			"Instructions::MONITORENTER" => 0xc2,	
			"Instructions::MONITOREXIT" => 0xc3,	
			"Instructions::WIDE" => 0xc4,
			"Instructions::MULTIANEWARRAY" => 0xc5,
			"Instructions::IFNULL" => 0xc6,
			"Instructions::IFNONNULL" => 0xc7,
			"Instructions::GOTO_W" => 0xc8,
			"Instructions::JSR_W" => 0xc9,
			"Instructions::BREAKPOINT" => 0xca,
			"Instructions::IMPDEP1" => 0xfe,
			"Instructions::IMPDEP2" => 0xff,
			_ => panic!("Unknown opcode: {}", stringify!($instruction)),
		}
	};
}