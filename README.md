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


<label for="status">Status: Finished 125/205 Instructions</label>
<progress id="status" value="125" max="205"></progress>

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
| <span style="color:green">POP</span> | implemented |
| <span style="color:green">POP2</span> | implemented |
| <span style="color:green">DUP</span> | implemented |
| <span style="color:green">DUP_X1</span> | implemented |
| <span style="color:green">DUP_X2</span> | implemented |
| <span style="color:green">DUP2</span> | implemented |
| <span style="color:green">DUP2_X1</span> | implemented |
| <span style="color:green">DUP2_X2</span> | implemented |
| <span style="color:green">SWAP</span> | implemented |
| <span style="color:yellow">IADD</span> | partially implemented |
| <span style="color:yellow">LADD</span> | partially implemented |
| <span style="color:yellow">FADD</span> | partially implemented |
| <span style="color:yellow">DADD</span> | partially implemented |
| <span style="color:yellow">ISUB</span> | partially implemented |
| <span style="color:yellow">LSUB</span> | partially implemented |
| <span style="color:yellow">FSUB</span> | partially implemented |
| <span style="color:yellow">DSUB</span> | partially implemented |
| <span style="color:yellow">IMUL</span> | partially implemented |
| <span style="color:yellow">LMUL</span> | partially implemented |
| <span style="color:yellow">FMUL</span> | partially implemented |
| <span style="color:yellow">DMUL</span> | partially implemented |
| <span style="color:yellow">IDIV</span> | partially implemented |
| <span style="color:yellow">LDIV</span> | partially implemented |
| <span style="color:yellow">FDIV</span> | partially implemented |
| <span style="color:yellow">DDIV</span> | partially implemented |
| <span style="color:yellow">IREM</span> | partially implemented |
| <span style="color:yellow">LREM</span> | partially implemented |
| <span style="color:yellow">FREM</span> | partially implemented |
| <span style="color:yellow">DREM</span> | partially implemented |
| <span style="color:yellow">INEG</span> | partially implemented |
| <span style="color:yellow">LNEG</span> | partially implemented |
| <span style="color:yellow">FNEG</span> | partially implemented |
| <span style="color:yellow">DNEG</span> | partially implemented |
| <span style="color:green">ISHL</span> | implemented |
| <span style="color:green">LSHL</span> | implemented |
| <span style="color:green">ISHR</span> | implemented |
| <span style="color:green">LSHR</span> | implemented |
| <span style="color:green">IUSHR</span> | implemented |
| <span style="color:green">LUSHR</span> | implemented |
| <span style="color:green">IAND</span> | implemented |
| <span style="color:green">LAND</span> | implemented |
| <span style="color:green">IOR</span> | implemented |
| <span style="color:green">LOR</span> | implemented |
| <span style="color:green">IXOR</span> | implemented |
| <span style="color:green">LXOR</span> | implemented |
| <span style="color:green">IINC</span> | implemented |
| <span style="color:green">I2L</span> | implemented |
| <span style="color:green">I2F</span> | implemented |
| <span style="color:green">I2D</span> | implemented |
| <span style="color:green">L2I</span> | implemented |
| <span style="color:green">L2F</span> | implemented |
| <span style="color:green">L2D</span> | implemented |
| <span style="color:green">F2I</span> | implemented |
| <span style="color:green">F2L</span> | implemented |
| <span style="color:green">F2D</span> | implemented |
| <span style="color:green">D2I</span> | implemented |
| <span style="color:green">D2L</span> | implemented |
| <span style="color:green">D2F</span> | implemented |
| <span style="color:green">I2B</span> | implemented |
| <span style="color:green">I2C</span> | implemented |
| <span style="color:green">I2S</span> | implemented |
| <span style="color:green">LCMP</span> | implemented |
| <span style="color:green">FCMPL</span> | implemented |
| <span style="color:green">FCMPG</span> | implemented |
| <span style="color:green">DCMPL</span> | implemented |
| <span style="color:green">DCMPG</span> | implemented |
| <span style="color:green">IFEQ</span> | implemented |
| <span style="color:green">IFNE</span> | implemented |
| <span style="color:green">IFLT</span> | implemented |
| <span style="color:green">IFGE</span> | implemented |
| <span style="color:green">IFGT</span> | implemented |
| <span style="color:green">IFLE</span> | implemented |
| <span style="color:green">IF_ICMPEQ</span> | implemented |
| <span style="color:green">IF_ICMPNE</span> | implemented |
| <span style="color:green">IF_ICMPLT</span> | implemented |
| <span style="color:green">IF_ICMPGE</span> | implemented |
| <span style="color:green">IF_ICMPGT</span> | implemented |
| <span style="color:green">IF_ICMPLE</span> | implemented |
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
| <span style="color:green">PUTSTATIC</span> | implemented |
| GETFIELD | not implemented |
| PUTFIELD | not implemented |
| <span style="color:yellow">INVOKEVIRTUAL</span> | partially implemented |
| <span style="color:yellow">INVOKESPECIAL</span> | partially implemented |
| <span style="color:yellow">INVOKESTATIC</span> | partially implemented |
| INVOKEINTERFACE | not implemented |
| INVOKEDYNAMIC | not implemented |
| NEW | not implemented |
| <span style="color:green">NEWARRAY</span> | implemented |
| <span style="color:green">ANEWARRAY</span> | implemented |
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