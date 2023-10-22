use crate::jvm::instructions::InstructionStream;

pub enum MethodAccesFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

pub struct Method {
    name: String,
    descriptor: String,
    access_flags: u16,
    max_locals: u16,
    max_stack: u16,
    instruction_stream: InstructionStream,
    native: bool,
}

impl Method {
    pub fn new(
        name: String,
        descriptor: String,
        access_flags: u16,
        max_locals: u16,
        max_stack: u16,
        instruction_stream: InstructionStream,
        native: bool,
    ) -> Self {
        Method {
            name,
            descriptor,
            access_flags,
            max_locals,
            max_stack,
            instruction_stream,
            native,
        }
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & MethodAccesFlags::Public as u16 != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & MethodAccesFlags::Private as u16 != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & MethodAccesFlags::Protected as u16 != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & MethodAccesFlags::Static as u16 != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & MethodAccesFlags::Final as u16 != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flags & MethodAccesFlags::Synchronized as u16 != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.access_flags & MethodAccesFlags::Bridge as u16 != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags & MethodAccesFlags::Varargs as u16 != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & MethodAccesFlags::Native as u16 != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & MethodAccesFlags::Abstract as u16 != 0
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags & MethodAccesFlags::Strict as u16 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & MethodAccesFlags::Synthetic as u16 != 0
    }
}