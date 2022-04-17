use crate::*;

#[derive(Default, Debug, Clone)]
pub struct Context<'a> {
    pub next_id: u32,

    pub type_instr: Vec<Opcode>,
    pub structs: Map<&'a str, InternedType<'a>>,
    pub types: Map<Rc<Type<'a>>, InternedType<'a>>,

    pub constants: Map<(IdType, u32), ID>,
    pub const_instr: Vec<Opcode>,

    pub kernels: Vec<Function<'a>>,
    pub functions: Vec<Function<'a>>,

    pub dec: Vec<Opcode>,
    pub names: Vec<Opcode>,
    pub vars: Vec<Opcode>,
    pub instr: Vec<Opcode>,
    pub inputs: Map<&'a str, ID>, // gl_NumWorkGroups, gl_WorkGroupID, gl_WorkGroupSize, gl_LocalInvocationID
}

impl<'a> Context<'a> {
    pub fn gen(&mut self) -> ID {
        self.next_id += 1;
        ID(self.next_id)
    }

    pub fn get_struct(&mut self, name: &'a str) -> InternedType<'a> {
        if let Some(st) = self.structs.get(name) {
            return st.clone();
        }
        let id = self.gen();
        let st = Mrc::new(Struct {
            name,
            fields: vec![],
            size: 0,
            id,
        });

        let ty = self.create_type(id, Rc::new(Type::Struct(st)));
        self.structs.insert(name, ty.clone());
        ty
    }

    pub fn init_struct(
        &mut self,
        name: &'a str,
        fields: Vec<(&'a str, InternedType<'a>)>,
    ) -> InternedType<'a> {
        let st1 = self.get_struct(name);
        {
            let mut st = st1.as_st().unwrap().borrow_mut();
            let mut offset = 0;
            for (idx, (name, ty)) in fields.into_iter().enumerate() {
                let sz = ty.get_sz();
                st.fields.push(Field {
                    name,
                    ty,
                    idx: idx as u32,
                    offset,
                });
                offset += sz;
            }
            st.size = offset;
        }
        st1
    }

    fn create_type(&mut self, id: ID, ty: Rc<Type<'a>>) -> InternedType<'a> {
        let re = InternedType { id, ty: ty.clone() };
        self.types.insert(ty, re.clone());
        re
    }

    pub fn get_const(&mut self, val: u32, ty: InternedType<'a>) -> ID {
        if let Some(val) = self.constants.get(&(ty.id, val)) {
            return *val;
        }
        let id = self.gen();
        self.constants.insert((ty.id, val), id);
        self.const_instr.push(OpConstant(ty.id, id, val));
        id
    }

    pub fn get_const_u32(&mut self, val: u32) -> ID {
        let ty = self.get_type(Rc::new(Type::S(Scalar::Uint(32))));
        self.get_const(val, ty)
    }

    pub fn get_ptr(&mut self, ty: InternedType<'a>, class: StorageClass) -> InternedType<'a> {
        self.get_type(Rc::new(Type::Ptr(ty, class)))
    }

    pub fn get_type(&mut self, ty: Rc<Type<'a>>) -> InternedType<'a> {
        if let Some(ty) = self.types.get(&ty) {
            return ty.clone();
        }

        let id = self.gen();
        self.create_type(id, ty)
    }

    pub fn func_def(
        &mut self,
        kern: Option<&'a str>,
        name: &'a str,
        inputs: Vec<(&'a str, InternedType<'a>)>,
        outputs: Option<Vec<(&'a str, InternedType<'a>)>>,
        body: Vec<Statement<'a>>,
    ) {
        let f = Function::new(name, inputs, outputs.unwrap_or_default(), body, self);
        if let Some(_) = kern {
            self.kernels.push(f);
        } else {
            self.functions.push(f);
        }
    }

    pub fn go(&mut self) {
        let mut func = self.kernels.pop().unwrap();
        func.lower(self);
    }
}
