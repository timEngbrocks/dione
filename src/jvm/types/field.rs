use super::Types;

pub enum FieldAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub descriptor: String,
    pub access_flags: u16,
    pub value: Option<Types>,
}

impl Field {
    pub fn new(name: String, descriptor: String, access_flags: u16, value: Option<Types>) -> Self {
        Field {
            name,
            descriptor,
            access_flags,
            value,
        }
    }

    pub fn get_value(&self) -> &Option<Types> {
        &self.value
    }

    pub fn set_value(&mut self, value: Option<Types>) {
        self.value = value;
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & FieldAccessFlags::Public as u16 != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & FieldAccessFlags::Private as u16 != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & FieldAccessFlags::Protected as u16 != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & FieldAccessFlags::Static as u16 != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & FieldAccessFlags::Final as u16 != 0
    }

    pub fn is_volatile(&self) -> bool {
        self.access_flags & FieldAccessFlags::Volatile as u16 != 0
    }

    pub fn is_transient(&self) -> bool {
        self.access_flags & FieldAccessFlags::Transient as u16 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & FieldAccessFlags::Synthetic as u16 != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & FieldAccessFlags::Enum as u16 != 0
    }
}
