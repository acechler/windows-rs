use super::*;

pub trait Decode {
    fn decode(file: usize, code: usize) -> Self;
}

pub enum AttributeType {
    MemberRef(MemberRef),
}

impl Decode for AttributeType {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            3 => Self::MemberRef(MemberRef(Row::new(row, file))),
            rest => unimplemented!("{rest:?}"),
        }
    }
}

pub enum HasAttribute {
    MethodDef(MethodDef),
    Field(Field),
    TypeRef(TypeRef),
    TypeDef(TypeDef),
    Param(Param),
    InterfaceImpl(InterfaceImpl),
    MemberRef(MemberRef),
    TypeSpec(TypeSpec),
    GenericParam(GenericParam),
}

impl HasAttribute {
    pub fn encode(&self) -> usize {
        match self {
            Self::MethodDef(row) => (row.0.row + 1) << 5,
            Self::Field(row) => ((row.0.row + 1) << 5) | 1,
            Self::TypeRef(row) => ((row.0.row + 1) << 5) | 2,
            Self::TypeDef(row) => ((row.0.row + 1) << 5) | 3,
            Self::Param(row) => ((row.0.row + 1) << 5) | 4,
            Self::InterfaceImpl(row) => ((row.0.row + 1) << 5) | 5,
            Self::MemberRef(row) => ((row.0.row + 1) << 5) | 6,
            Self::TypeSpec(row) => ((row.0.row + 1) << 5) | 13,
            Self::GenericParam(row) => ((row.0.row + 1) << 5) | 19,
        }
    }
}

#[derive(Clone)]
pub enum HasConstant {
    Field(Field),
}

impl HasConstant {
    pub fn encode(&self) -> usize {
        match self {
            Self::Field(row) => (row.0.row + 1) << 2,
        }
    }
}

#[derive(Clone)]
pub enum MemberForwarded {
    MethodDef(MethodDef),
}

impl MemberForwarded {
    pub fn encode(&self) -> usize {
        match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 1) | 1,
        }
    }
}

pub enum MemberRefParent {
    TypeRef(TypeRef),
}

impl Decode for MemberRefParent {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            1 => Self::TypeRef(TypeRef(Row::new(row, file))),
            rest => unimplemented!("{rest:?}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TypeDefOrRef {
    TypeDef(TypeDef),
    TypeRef(TypeRef),
    TypeSpec(TypeSpec),
}

impl Decode for TypeDefOrRef {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match kind {
            0 => Self::TypeDef(TypeDef(Row::new(row, file))),
            1 => Self::TypeRef(TypeRef(Row::new(row, file))),
            2 => Self::TypeSpec(TypeSpec(Row::new(row, file))),
            rest => unimplemented!("{rest:?}"),
        }
    }
}

pub enum TypeOrMethodDef {
    TypeDef(TypeDef),
}

impl TypeOrMethodDef {
    pub fn encode(&self) -> usize {
        match self {
            Self::TypeDef(value) => (value.0.row + 1) << 1,
        }
    }
}

pub enum ResolutionScope {
    Module(Module),
    ModuleRef(ModuleRef),
    AssemblyRef(AssemblyRef),
    TypeRef(TypeRef),
}

impl Decode for ResolutionScope {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match kind {
            0 => Self::Module(Module(Row::new(row, file))),
            1 => Self::ModuleRef(ModuleRef(Row::new(row, file))),
            2 => Self::AssemblyRef(AssemblyRef(Row::new(row, file))),
            3 => Self::TypeRef(TypeRef(Row::new(row, file))),
            rest => unimplemented!("{rest:?}"),
        }
    }
}
