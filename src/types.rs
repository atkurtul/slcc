use std::ops::DerefMut;

use crate::*;
pub use Scalar::*;
pub use Type::*;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Scalar {
    Bool,
    Sint(u32),
    Uint(u32),
    Real(u32),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Field<'a> {
    pub name: &'a str,
    pub idx: u32,
    pub offset: u32,
    pub ty: InternedType<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
    pub size: u32,
    pub id: IdType,
}

impl std::hash::Hash for Struct<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> Struct<'a> {
    pub fn get_field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields.iter().find(|f| f.name == name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct InternedType<'a> {
    pub id: IdType,
    pub ty: Rc<Type<'a>>,
}

impl<'a> std::ops::Deref for InternedType<'a> {
    type Target = Rc<Type<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Type<'a> {
    Void,
    Image(Dim),
    S(Scalar),
    V(Scalar, u32),
    M(Scalar, u32, u32),
    Struct(Mrc<Struct<'a>>),
    Arr(InternedType<'a>, u32),
    Ptr(InternedType<'a>, StorageClass),
    Function(Vec<InternedType<'a>>),
}

impl<'a> Type<'a> {
    pub fn get_field_idx(&self, name: &'a str) -> u32 {
        match self {
            Type::Struct(st) => st.borrow().get_field(name).unwrap().idx,
            Type::V(s, n) => match name {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                "w" => 3,
                _ => panic!("unknown field {}", name),
            },
            _ => panic!(),
        }
    }

    pub fn type_at_idx(&self, idx: u32) -> Rc<Type<'a>> {
        match self {
            Type::Struct(st) => st
                .borrow()
                .fields
                .iter()
                .find(|f| f.idx == idx)
                .unwrap()
                .ty
                .ty
                .clone(),
            Type::V(s, n) if idx < *n => Rc::new(Type::S(*s)),
            Type::M(s, n, m) if idx < *m => Rc::new(Type::V(*s, *n)),
            Type::Arr(t, n) if idx < *n => t.ty.clone(),
            _ => panic!(),
        }
    }

    pub fn get_sz(&self) -> u32 {
        use Type::*;
        match self {
            S(s) => s.get_sz(),
            V(s, n) => s.get_sz() * n,
            M(s, n, m) => s.get_sz() * n * m,
            Struct(st) => st.borrow().size,
            Arr(t, n) => t.ty.get_sz() * n,
            _ => panic!(),
        }
    }

    pub fn as_st(&self) -> Option<&Mrc<Struct<'a>>> {
        match self {
            Type::Struct(st) => Some(st),
            _ => None,
        }
    }

    pub fn as_ptr(&self) -> Option<(InternedType<'a>, StorageClass)> {
        match self {
            Type::Ptr(p, c) => Some((p.clone(), *c)),
            _ => None,
        }
    }
}

impl Scalar {
    pub fn get_sz(&self) -> u32 {
        match self {
            Bool => 1,
            Sint(sz) | Uint(sz) | Real(sz) => *sz >> 3,
        }
    }

    pub fn get_relop(&self, op: &str) -> fn(IdType, IdResult, ID, ID) -> Opcode {
        match op {
            "<" => match self {
                Sint(..) => OpSLessThan,
                Uint(..) => OpULessThan,
                Real(..) => OpFOrdLessThan,
                Bool => panic!(),
            },
            ">" => match self {
                Sint(..) => OpSGreaterThan,
                Uint(..) => OpUGreaterThan,
                Real(..) => OpFOrdGreaterThan,
                Bool => panic!(),
            },
            "<=" => match self {
                Sint(..) => OpSLessThanEqual,
                Uint(..) => OpULessThanEqual,
                Real(..) => OpFOrdLessThanEqual,
                Bool => panic!(),
            },
            ">=" => match self {
                Sint(..) => OpSGreaterThanEqual,
                Uint(..) => OpUGreaterThanEqual,
                Real(..) => OpFOrdGreaterThanEqual,
                Bool => panic!(),
            },
            "==" => match self {
                Sint(..) => OpIEqual,
                Uint(..) => OpIEqual,
                Real(..) => OpFOrdEqual,
                Bool => OpLogicalEqual,
            },
            "!=" => match self {
                Sint(..) => OpINotEqual,
                Uint(..) => OpINotEqual,
                Real(..) => OpFOrdNotEqual,
                Bool => OpLogicalNotEqual,
            },
            _ => panic!(),
        }
    }

    pub fn get_binop(&self, op: &str) -> fn(IdType, IdResult, ID, ID) -> Opcode {
        match op {
            "+" => match self {
                Sint(..) => OpIAdd,
                Uint(..) => OpIAdd,
                Real(..) => OpFAdd,
                Bool => panic!(),
            },
            "-" => match self {
                Sint(..) => OpISub,
                Uint(..) => OpISub,
                Real(..) => OpFSub,
                Bool => panic!(),
            },
            "*" => match self {
                Sint(..) => OpIAdd,
                Uint(..) => OpIAdd,
                Real(..) => OpFAdd,
                Bool => panic!(),
            },
            "/" => match self {
                Sint(..) => OpSDiv,
                Uint(..) => OpUDiv,
                Real(..) => OpFDiv,
                Bool => panic!(),
            },
            "%" => match self {
                Sint(..) => OpSMod,
                Uint(..) => OpUMod,
                Real(..) => OpFMod,
                Bool => panic!(),
            },
            "&&" => match self {
                Sint(..) => panic!(),
                Uint(..) => panic!(),
                Real(..) => panic!(),
                Bool => OpLogicalAnd,
            },
            "||" => match self {
                Sint(..) => panic!(),
                Uint(..) => panic!(),
                Real(..) => panic!(),
                Bool => OpLogicalOr,
            },
            "&" => match self {
                Sint(..) => OpBitwiseAnd,
                Uint(..) => OpBitwiseAnd,
                Real(..) => panic!(),
                Bool => OpLogicalOr,
            },
            "|" => match self {
                Sint(..) => OpBitwiseOr,
                Uint(..) => OpBitwiseOr,
                Real(..) => panic!(),
                Bool => OpLogicalOr,
            },
            "^" => match self {
                Sint(..) => OpBitwiseXor,
                Uint(..) => OpBitwiseXor,
                Real(..) => panic!(),
                Bool => OpLogicalNotEqual,
            },
            _ => panic!(),
        }
    }
}
