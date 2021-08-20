pub enum ClassFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
    Module,
}

impl ClassFlag {
    pub fn is_set(self: &ClassFlag, flag_val: u16) -> bool {
        match self {
            ClassFlag::Public => flag_val & 0x0001 > 0,
            ClassFlag::Final => flag_val & 0x0010 > 0,
            ClassFlag::Super => flag_val & 0x0020 > 0,
            ClassFlag::Interface => flag_val & 0x0200 > 0,
            ClassFlag::Abstract => flag_val & 0x0400 > 0,
            ClassFlag::Synthetic => flag_val & 0x1000 > 0,
            ClassFlag::Annotation => flag_val & 0x2000 > 0,
            ClassFlag::Enum => flag_val & 0x4000 > 0,
            ClassFlag::Module => flag_val & 0x8000 > 0,
        }
    }
}

pub enum MethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    VarArgs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

impl MethodFlag {
    pub fn is_set(self: &MethodFlag, flag_val: u16) -> bool {
        match self {
            MethodFlag::Public => flag_val & 0x0001 > 0,
            MethodFlag::Private => flag_val & 0x0002 > 0,
            MethodFlag::Protected => flag_val & 0x0004 > 0,
            MethodFlag::Static => flag_val & 0x0008 > 0,
            MethodFlag::Final => flag_val & 0x0010 > 0,
            MethodFlag::Synchronized => flag_val & 0x0020 > 0,
            MethodFlag::Bridge => flag_val & 0x0040 > 0,
            MethodFlag::VarArgs => flag_val & 0x0080 > 0,
            MethodFlag::Native => flag_val & 0x0100 > 0,
            MethodFlag::Abstract => flag_val & 0x0400 > 0,
            MethodFlag::Strict => flag_val & 0x0800 > 0,
            MethodFlag::Synthetic => flag_val & 0x1000 > 0,
        }
    }
}
