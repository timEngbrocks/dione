# DIONE - A Rust based JVM

## Usage

### run.sh

Runs the Main.class file located at the project root
with a DEBUG build of DIONE.

### debug.sh

Starts the Main.class file located at the project root
with a DEBUG build of DIONE in GDB for thougher debugging
sessions :^) .

## Build hints

DIONE is being developed on Windows x86-64 with the
latest GNU nightly toolchain from Rust.

## Crates used

- args: CLI argument parsing
- getopts: CLI argument parsing
- bitutils: i32 sign extension
- itertools: Join Vec to String
- nom: parsing of bit streams
- spin: Mutex for Heap implementation

## Useful Links

- [JVM 19 Spec](https://docs.oracle.com/javase/specs/jvms/se19/html/index.html)
- [Java SE 19](https://download.java.net/java/early_access/panama/docs/api/index.html)
- [OpenJDK](https://github.com/openjdk/jdk)
- [BinaryInternalsViewer - .class Decompiler](https://github.com/amosshi/binaryinternals)

## Implemented Instructions


<label for="status">Status: Finished 80/205 Instructions</label>
<progress id="status" value="80" max="205"></progress>

| Instruction | State |
| --- | --- |
| <span style="color:green">NOP</span> | implemented |
| <span style="color:green">ACONST_NULL</span> | implemented |
| <span style="color:green">ICONST_M1</span> | implemented |
| <span style="color:green">ICONST_0</span> | implemented |
| <span style="color:green">ICONST_1</span> | implemented |
| <span style="color:green">ICONST_2</span> | implemented |
| <span style="color:green">ICONST_3</span> | implemented |
| <span style="color:green">ICONST_4</span> | implemented |
| <span style="color:green">ICONST_5</span> | implemented |
| <span style="color:green">LCONST_0</span> | implemented |
| <span style="color:green">LCONST_1</span> | implemented |
| <span style="color:green">FCONST_0</span> | implemented |
| <span style="color:green">FCONST_1</span> | implemented |
| <span style="color:green">FCONST_2</span> | implemented |
| <span style="color:green">DCONST_0</span> | implemented |
| <span style="color:green">DCONST_1</span> | implemented |
| <span style="color:green">BIPUSH</span> | implemented |
| <span style="color:green">SIPUSH</span> | implemented |
| <span style="color:yellow">LDC</span> | partially implemented |
| <span style="color:yellow">LDC_W</span> | partially implemented |
| <span style="color:yellow">LDC2_W</span> | partially implemented |
| <span style="color:green">ILOAD</span> | implemented |
| <span style="color:green">LLOAD</span> | implemented |
| <span style="color:green">FLOAD</span> | implemented |
| <span style="color:green">DLOAD</span> | implemented |
| <span style="color:green">ALOAD</span> | implemented |
| <span style="color:green">ILOAD_0</span> | implemented |
| <span style="color:green">ILOAD_1</span> | implemented |
| <span style="color:green">ILOAD_2</span> | implemented |
| <span style="color:green">ILOAD_3</span> | implemented |
| <span style="color:green">LLOAD_0</span> | implemented |
| <span style="color:green">LLOAD_1</span> | implemented |
| <span style="color:green">LLOAD_2</span> | implemented |
| <span style="color:green">LLOAD_3</span> | implemented |
| <span style="color:green">FLOAD_0</span> | implemented |
| <span style="color:green">FLOAD_1</span> | implemented |
| <span style="color:green">FLOAD_2</span> | implemented |
| <span style="color:green">FLOAD_3</span> | implemented |
| <span style="color:green">DLOAD_0</span> | implemented |
| <span style="color:green">DLOAD_1</span> | implemented |
| <span style="color:green">DLOAD_2</span> | implemented |
| <span style="color:green">DLOAD_3</span> | implemented |
| <span style="color:green">ALOAD_0</span> | implemented |
| <span style="color:green">ALOAD_1</span> | implemented |
| <span style="color:green">ALOAD_2</span> | implemented |
| <span style="color:green">ALOAD_3</span> | implemented |
| IALOAD | not implemented |
| LALOAD | not implemented |
| FALOAD | not implemented |
| DALOAD | not implemented |
| AALOAD | not implemented |
| BALOAD | not implemented |
| CALOAD | not implemented |
| SALOAD | not implemented |
| <span style="color:green">ISTORE</span> | implemented |
| <span style="color:green">LSTORE</span> | implemented |
| <span style="color:green">FSTORE</span> | implemented |
| <span style="color:green">DSTORE</span> | implemented |
| <span style="color:green">ASTORE</span> | implemented |
| <span style="color:green">ISTORE_0</span> | implemented |
| <span style="color:green">ISTORE_1</span> | implemented |
| <span style="color:green">ISTORE_2</span> | implemented |
| <span style="color:green">ISTORE_3</span> | implemented |
| <span style="color:green">LSTORE_0</span> | implemented |
| <span style="color:green">LSTORE_1</span> | implemented |
| <span style="color:green">LSTORE_2</span> | implemented |
| <span style="color:green">LSTORE_3</span> | implemented |
| <span style="color:green">FSTORE_0</span> | implemented |
| <span style="color:green">FSTORE_1</span> | implemented |
| <span style="color:green">FSTORE_2</span> | implemented |
| <span style="color:green">FSTORE_3</span> | implemented |
| <span style="color:green">DSTORE_0</span> | implemented |
| <span style="color:green">DSTORE_1</span> | implemented |
| <span style="color:green">DSTORE_2</span> | implemented |
| <span style="color:green">DSTORE_3</span> | implemented |
| <span style="color:green">ASTORE_0</span> | implemented |
| <span style="color:green">ASTORE_1</span> | implemented |
| <span style="color:green">ASTORE_2</span> | implemented |
| <span style="color:green">ASTORE_3</span> | implemented |
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
| <span style="color:green">DUP</span> | implemented |
| <span style="color:green">DUP_X1</span> | implemented |
| <span style="color:green">DUP_X2</span> | implemented |
| <span style="color:green">DUP2</span> | implemented |
| <span style="color:green">DUP2_X1</span> | implemented |
| <span style="color:green">DUP2_X2</span> | implemented |
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
| <span style="color:green">IFEQ</span> | not implemented |
| <span style="color:green">IFNE</span> | not implemented |
| <span style="color:green">IFLT</span> | not implemented |
| <span style="color:green">IFGE</span> | not implemented |
| <span style="color:green">IFGT</span> | not implemented |
| <span style="color:green">IFLE</span> | not implemented |
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
| <span style="color:yellow">INVOKEVIRTUAL</span> | partially implemented |
| <span style="color:yellow">INVOKESPECIAL</span> | partially implemented |
| <span style="color:yellow">INVOKESTATIC</span> | partially implemented |
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