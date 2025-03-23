# DIONE - A Rust based JVM

## Docs

[Current documentation](https://timengbrocks.github.io/dione/doc/dione/)

## Usage

```txt
Usage: dione.exe <COMMAND>

Commands:
  parse-class-file  
  help              Print this message or the help of the given subcommand(s)   

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Build hints

DIONE is being developed on Windows x86-64 with the
latest GNU nightly toolchain from Rust.

## Crates used

- clap: CLI argument parsing
- nom: parsing of bit streams

## Useful Links

- [JVM 19 Spec](https://docs.oracle.com/javase/specs/jvms/se19/html/index.html)
- [Java SE 19](https://download.java.net/java/early_access/panama/docs/api/index.html)
- [OpenJDK](https://github.com/openjdk/jdk)
- [BinaryInternalsViewer - .class Decompiler](https://github.com/amosshi/binaryinternals)

## Implemented Instructions

<label for="status">Status: Finished 0/205 Instructions</label>
<progress id="status" value="0" max="205"></progress>

| Instruction | State |
| --- | --- |
| NOP | not implemented |
| ACONST_NULL | not implemented |
| ICONST_M1 | not implemented |
| ICONST_0 | not implemented |
| ICONST_1 | not implemented |
| ICONST_2 | not implemented |
| ICONST_3 | not implemented |
| ICONST_4 | not implemented |
| ICONST_5 | not implemented |
| LCONST_0 | not implemented |
| LCONST_1 | not implemented |
| FCONST_0 | not implemented |
| FCONST_1 | not implemented |
| FCONST_2 | not implemented |
| DCONST_0 | not implemented |
| DCONST_1 | not implemented |
| BIPUSH | not implemented |
| SIPUSH | not implemented |
| LDC | not implemented |
| LDC_W | not implemented |
| LDC2_W | not implemented |
| ILOAD | not implemented |
| LLOAD | not implemented |
| FLOAD | not implemented |
| DLOAD | not implemented |
| ALOAD | not implemented |
| ILOAD_0 | not implemented |
| ILOAD_1 | not implemented |
| ILOAD_2 | not implemented |
| ILOAD_3 | not implemented |
| LLOAD_0 | not implemented |
| LLOAD_1 | not implemented |
| LLOAD_2 | not implemented |
| LLOAD_3 | not implemented |
| FLOAD_0 | not implemented |
| FLOAD_1 | not implemented |
| FLOAD_2 | not implemented |
| FLOAD_3 | not implemented |
| DLOAD_0 | not implemented |
| DLOAD_1 | not implemented |
| DLOAD_2 | not implemented |
| DLOAD_3 | not implemented |
| ALOAD_0 | not implemented |
| ALOAD_1 | not implemented |
| ALOAD_2 | not implemented |
| ALOAD_3 | not implemented |
| IALOAD | not implemented |
| LALOAD | not implemented |
| FALOAD | not implemented |
| DALOAD | not implemented |
| AALOAD | not implemented |
| BALOAD | not implemented |
| CALOAD | not implemented |
| SALOAD | not implemented |
| ISTORE | not implemented |
| LSTORE | not implemented |
| FSTORE | not implemented |
| DSTORE | not implemented |
| ASTORE | not implemented |
| ISTORE_0 | not implemented |
| ISTORE_1 | not implemented |
| ISTORE_2 | not implemented |
| ISTORE_3 | not implemented |
| LSTORE_0 | not implemented |
| LSTORE_1 | not implemented |
| LSTORE_2 | not implemented |
| LSTORE_3 | not implemented |
| FSTORE_0 | not implemented |
| FSTORE_1 | not implemented |
| FSTORE_2 | not implemented |
| FSTORE_3 | not implemented |
| DSTORE_0 | not implemented |
| DSTORE_1 | not implemented |
| DSTORE_2 | not implemented |
| DSTORE_3 | not implemented |
| ASTORE_0 | not implemented |
| ASTORE_1 | not implemented |
| ASTORE_2 | not implemented |
| ASTORE_3 | not implemented |
| IASTORE | not implemented |
| LASTORE | not implemented |
| FASTORE | not implemented |
| DASTORE | not implemented |
| AASTORE | not implemented |
| BASTORE | not implemented |
| CASTORE | not implemented |
| SASTORE | not implemented |
| POP | not implemented |
| POP2 | not implemented |
| DUP | not implemented |
| DUP_X1 | not implemented |
| DUP_X2 | not implemented |
| DUP2 | not implemented |
| DUP2_X1 | not implemented |
| DUP2_X2 | not implemented |
| SWAP | not implemented |
| IADD | not implemented |
| LADD | not implemented |
| FADD | not implemented |
| DADD | not implemented |
| ISUB | not implemented |
| LSUB | not implemented |
| FSUB | not implemented |
| DSUB | not implemented |
| IMUL | not implemented |
| LMUL | not implemented |
| FMUL | not implemented |
| DMUL | not implemented |
| IDIV | not implemented |
| LDIV | not implemented |
| FDIV | not implemented |
| DDIV | not implemented |
| IREM | not implemented |
| LREM | not implemented |
| FREM | not implemented |
| DREM | not implemented |
| INEG | not implemented |
| LNEG | not implemented |
| FNEG | not implemented |
| DNEG | not implemented |
| ISHL | not implemented |
| LSHL | not implemented |
| ISHR | not implemented |
| LSHR | not implemented |
| IUSHR | not implemented |
| LUSHR | not implemented |
| IAND | not implemented |
| LAND | not implemented |
| IOR | not implemented |
| LOR | not implemented |
| IXOR | not implemented |
| LXOR | not implemented |
| IINC | not implemented |
| I2L | not implemented |
| I2F | not implemented |
| I2D | not implemented |
| L2I | not implemented |
| L2F | not implemented |
| L2D | not implemented |
| F2I | not implemented |
| F2L | not implemented |
| F2D | not implemented |
| D2I | not implemented |
| D2L | not implemented |
| D2F | not implemented |
| I2B | not implemented |
| I2C | not implemented |
| I2S | not implemented |
| LCMP | not implemented |
| FCMPL | not implemented |
| FCMPG | not implemented |
| DCMPL | not implemented |
| DCMPG | not implemented |
| IFEQ | not implemented |
| IFNE | not implemented |
| IFLT | not implemented |
| IFGE | not implemented |
| IFGT | not implemented |
| IFLE | not implemented |
| IF_ICMPEQ | not implemented |
| IF_ICMPNE | not implemented |
| IF_ICMPLT | not implemented |
| IF_ICMPGE | not implemented |
| IF_ICMPGT | not implemented |
| IF_ICMPLE | not implemented |
| IF_ACMPEQ | not implemented |
| IF_ACMPNE | not implemented |
| GOTO | not implemented |
| JSR | not implemented |
| RET | not implemented |
| TABLESWITCH | not implemented |
| LOOKUPSWITCH | not implemented |
| IRETURN | not implemented |
| LRETURN | not implemented |
| FRETURN | not implemented |
| DRETURN | not implemented |
| ARETURN | not implemented |
| RETURN | not implemented |
| GETSTATIC | not implemented |
| PUTSTATIC | not implemented |
| GETFIELD | not implemented |
| PUTFIELD | not implemented |
| INVOKEVIRTUAL | not implemented |
| INVOKESPECIAL | not implemented |
| INVOKESTATIC | not implemented |
| INVOKEINTERFACE | not implemented |
| INVOKEDYNAMIC | not implemented |
| NEW | not implemented |
| NEWARRAY | not implemented |
| ANEWARRAY | not implemented |
| ARRAYLENGTH | not implemented |
| ATHROW | not implemented |
| CHECKCAST | not implemented |
| INSTANCEOF | not implemented |
| MONITORENTER | not implemented |
| MONITOREXIT | not implemented |
| WIDE | not implemented |
| MULTIANEWARRAY | not implemented |
| IFNULL | not implemented |
| IFNONNULL | not implemented |
| GOTO_W | not implemented |
| JSR_W | not implemented |
| BREAKPOINT | not implemented |
| IMPDEP1 | not implemented |
| IMPDEP2 | not implemented |