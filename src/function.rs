use crate::*;

#[derive(Default, Debug, Clone)]
pub struct Stack<T>(Vec<T>, Vec<usize>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(vec![], vec![])
    }
    pub fn push(&mut self, t: T) {
        self.1.push(self.0.len());
        self.0.push(t);
    }
    pub fn pop(&mut self) {
        self.1.pop();
    }
    pub fn get_mut(&mut self) -> &mut T {
        let idx = self.1.last().unwrap();
        &mut self.0[*idx]
    }
    pub fn get(&self) -> &T {
        let idx = self.1.last().unwrap();
        &self.0[*idx]
    }
}

#[derive(Default, Debug, Clone)]
pub struct Label {
    pub id: ID,
    pub ops: Vec<Opcode>,
}

impl Label {
    pub fn new(id: ID) -> Self {
        Self {
            id,
            ops: vec![OpLabel(id)],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub id: ID,
    pub ty: InternedType<'a>,
}

#[derive(Default, Debug, Clone)]
pub struct Scope<'a> {
    pub variables: Map<&'a str, Variable<'a>>,
}

#[derive(Default, Debug, Clone)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: Vec<Statement<'a>>,
    pub header: Vec<Opcode>,
    pub labels: Vec<Label>,
    pub scopes: Stack<Scope<'a>>,
    pub id: ID,
}

fn split_args<'a>(
    args: Vec<(&'a str, InternedType<'a>)>,
) -> (
    Vec<(&'a str, InternedType<'a>)>,
    Vec<(&'a str, InternedType<'a>)>,
    Vec<(&'a str, InternedType<'a>)>,
) {
    let mut images = vec![];
    let mut buffers = vec![];
    let mut push_constants = vec![];

    for (name, ty) in args {
        match ty.as_ref() {
            Type::Image(..) => images.push((name, ty)),
            Type::Ptr(..) => buffers.push((name, ty)),
            _ => push_constants.push((name, ty)),
        }
    }

    (images, buffers, push_constants)
}

macro_rules! buffer_name_idx {
    ($set:literal, $idx:expr) => {
        match $idx {
            0 => concat!("@Buffer[", $set, "][", 0, "]"),
            1 => concat!("@Buffer[", $set, "][", 1, "]"),
            2 => concat!("@Buffer[", $set, "][", 2, "]"),
            3 => concat!("@Buffer[", $set, "][", 3, "]"),
            4 => concat!("@Buffer[", $set, "][", 4, "]"),
            5 => concat!("@Buffer[", $set, "][", 5, "]"),
            6 => concat!("@Buffer[", $set, "][", 6, "]"),
            7 => concat!("@Buffer[", $set, "][", 7, "]"),
            8 => concat!("@Buffer[", $set, "][", 8, "]"),
            9 => concat!("@Buffer[", $set, "][", 9, "]"),
            _ => panic!(),
        }
    };
}

macro_rules! buffer_name {
    ($set:expr, $idx:expr) => {
        match $set {
            0 => buffer_name_idx!(0, $idx),
            1 => buffer_name_idx!(1, $idx),
            2 => buffer_name_idx!(2, $idx),
            3 => buffer_name_idx!(3, $idx),
            4 => buffer_name_idx!(4, $idx),
            5 => buffer_name_idx!(5, $idx),
            6 => buffer_name_idx!(6, $idx),
            7 => buffer_name_idx!(7, $idx),
            8 => buffer_name_idx!(8, $idx),
            9 => buffer_name_idx!(9, $idx),
            _ => panic!(),
        }
    };
}

impl<'a> Function<'a> {
    pub fn new(
        name: &'a str,
        inputs: Vec<(&'a str, InternedType<'a>)>,
        outputs: Vec<(&'a str, InternedType<'a>)>,
        body: Vec<Statement<'a>>,
        ctx: &mut Context<'a>,
    ) -> Self {
        let id = ctx.gen();
        let mut f = Self {
            name,
            body,
            id,
            header: vec![],
            labels: vec![],
            scopes: Stack::new(),
        };
        ctx.names.push(OpName(id, name.to_string()));
        let re = ctx.get_type(Rc::new(Type::Void));
        let fty = ctx.get_type(Rc::new(Type::Function(vec![re.clone()])));
        f.header
            .push(OpFunction(id, re.id, FunctionControl::None, fty.id));
        f.init_args(ctx, inputs, outputs);
        f
    }

    pub fn add_symbol(&mut self, name: &'a str, id: ID, ty: InternedType<'a>) {
        self.scopes
            .get_mut()
            .variables
            .insert(name, Variable { name, id, ty });
    }

    pub fn add_instr(&mut self, instr: Opcode) {
        self.labels.last_mut().unwrap().ops.push(instr);
    }

    fn init_var(
        &mut self,
        name: &'a str,
        ty: InternedType<'a>,
        init: Option<ID>,
        ctx: &mut Context<'a>,
    ) -> ID {
        let id = ctx.gen();
        self.header
            .push(OpVariable(ty.id, id, StorageClass::Function, None));
        if let Some(init) = init {
            self.add_instr(OpStore(id, init, None));
        }
        self.add_symbol(name, id, ty);
        id
    }

    fn init_arg(
        &mut self,
        dec: Vec<Decoration>,
        name: &'a str,
        pty: InternedType<'a>,
        ctx: &mut Context<'a>,
    ) -> ID {
        let (ty, class) = pty.as_ptr().unwrap();
        let id = ctx.gen();

        for dec in dec {
            ctx.dec.push(OpDecorate(id, dec));
        }

        ctx.names.push(OpName(id, name.to_string()));
        ctx.vars.push(OpVariable(pty.id, id, class, None));
        self.add_symbol(name, id, pty);
        id
    }

    fn init_img(
        &mut self,
        set: u32,
        idx: u32,
        dec: Decoration,
        name: &'a str,
        ty: InternedType<'a>,
        ctx: &mut Context<'a>,
    ) {
        let class = StorageClass::UniformConstant;
        let ty = ctx.get_type(Rc::new(Type::Ptr(ty, class)));
        let id = ctx.gen();
        let decs = vec![
            Decoration::DescriptorSet(set),
            Decoration::Binding(idx),
            dec,
        ];
        self.init_arg(decs, name, ty, ctx);
    }

    fn init_buffer(
        &mut self,
        set: u32,
        idx: u32,
        dec: Decoration,
        name: &'a str,
        pty: InternedType<'a>,
        ctx: &mut Context<'a>,
    ) {
        let (ty, class) = pty.as_ptr().unwrap();
        let id = ctx.gen();
        let mut decs = vec![Decoration::DescriptorSet(set), Decoration::Binding(idx)];

        let block = match (class, &dec) {
            (StorageClass::StorageBuffer, Decoration::NonReadable) => {
                decs.push(dec);
                Decoration::BufferBlock
            }
            (StorageClass::UniformConstant, Decoration::NonWritable) => {
                decs.push(dec);
                Decoration::Block
            }
            (StorageClass::StorageBuffer, Decoration::NonWritable) => Decoration::BufferBlock,
            _ => panic!(""),
        };

        ctx.init_struct(buffer_name!(set, idx), vec![(name, ty)]);

        self.init_arg(decs, name, pty, ctx);
    }

    fn init_images(
        &mut self,
        images: Vec<(&'a str, InternedType<'a>)>,
        set: u32,
        dec: Decoration,
        ctx: &mut Context<'a>,
    ) {
        let mut idx = 0;
        for (name, ty) in images {
            self.init_img(set, idx, dec.clone(), name, ty, ctx);
            idx += 1;
        }
    }

    fn init_buffers(
        &mut self,
        buffers: Vec<(&'a str, InternedType<'a>)>,
        set: u32,
        dec: Decoration,
        ctx: &mut Context<'a>,
    ) {
        let mut idx = 0;
        for (name, ty) in buffers {
            self.init_buffer(set, idx, dec.clone(), name, ty, ctx);
            idx += 1;
        }
    }

    fn init_push_constants(&mut self, pc: Vec<(&'a str, InternedType<'a>)>, ctx: &mut Context<'a>) {
        let pc = ctx.init_struct("@PushConstants", pc);
        let pty = ctx.get_type(Rc::new(Type::Ptr(pc.clone(), StorageClass::PushConstant)));
        let id = self.init_arg(vec![Decoration::Block], "@PushConstants", pty, ctx);

        for field in pc.as_st().unwrap().borrow().fields.iter() {
            let idx = ctx.get_const_u32(field.idx);
            let fty = ctx.get_ptr(field.ty.clone(), StorageClass::PushConstant);
            let fid = ctx.gen();
            self.add_instr(OpAccessChain(fty.id, fid, id, vec![idx]));
            self.add_symbol(field.name, fid, fty);
        }
    }

    fn init_args(
        &mut self,
        ctx: &mut Context<'a>,
        inputs: Vec<(&'a str, InternedType<'a>)>,
        outputs: Vec<(&'a str, InternedType<'a>)>,
    ) {
        self.labels.push(Label::new(ctx.gen()));
        self.scopes.push(Scope::default());
        let (output_images, output_buffers, push_constants) = split_args(outputs);
        assert!(push_constants.is_empty());
        let (input_images, input_buffers, push_constants) = split_args(inputs);
        self.init_images(input_images, 0, Decoration::NonWritable, ctx);
        self.init_images(output_images, 1, Decoration::NonReadable, ctx);
        self.init_buffers(input_buffers, 2, Decoration::NonWritable, ctx);
        self.init_buffers(output_buffers, 3, Decoration::NonReadable, ctx);
        self.init_push_constants(push_constants, ctx);
    }

    pub fn lower(&mut self, ctx: &mut Context<'a>) {
        let mut stat = vec![];
        std::mem::swap(&mut stat, &mut self.body);
        for stat in stat {
            stat.lower(&mut Lower { ctx, fun: self });
        }
    }
}

pub struct Lower<'a, 'b> {
    pub ctx: &'b mut Context<'a>,
    pub fun: &'b mut Function<'a>,
}

impl<'a, 'b> Lower<'a, 'b> {
    pub fn load(&mut self, id: ID, ty: IdType) -> ID {
        let re = self.ctx.gen();
        self.fun.add_instr(OpLoad(ty, re, id, None));
        re
    }

    pub fn load_if(&mut self, id: ID, ty: InternedType<'a>) -> (ID, InternedType<'a>) {
        match (id, ty.as_ref()) {
            (id, Type::Ptr(ty, _)) => (self.load(id, ty.id), ty.clone()),
            _ => (id, ty),
        }
    }

    pub fn create_var(&mut self, name: &'a str, ty: InternedType<'a>, init: Option<ID>) -> ID {
        self.fun.init_var(name, ty, init, self.ctx)
    }

    pub fn push(&mut self) {
        self.fun.scopes.push(Scope::default());
    }

    pub fn pop(&mut self) {
        self.fun.scopes.pop();
    }

    pub fn label(&mut self) -> Label {
        Label::new(self.ctx.gen())
    }

    pub fn set_label(&mut self, label: Label) {
        self.fun.labels.push(label);
    }

    pub fn add_instr(&mut self, opc: Opcode) {
        self.fun.add_instr(opc);
    }

    pub fn jump(&mut self, id: ID) {
        self.add_instr(OpBranch(id));
    }

    pub fn branch(&mut self, c: ID, t: ID, f: ID) {
        self.add_instr(OpBranchConditional(c, t, f, vec![]));
    }

    pub fn get_var(&self, name: &'a str) -> (ID, InternedType<'a>) {
        let var = self.fun.scopes.get().variables.get(name).unwrap();
        (var.id, var.ty.clone())
    }

    pub fn get_const(&mut self, val: u32, ty: Scalar) -> (ID, InternedType<'a>) {
        let ty = self.ctx.get_type(Rc::new(Type::S(ty)));
        let val = self.ctx.get_const(val, ty.clone());
        (val, ty)
    }

    pub fn composite_extract(
        &mut self,
        id: ID,
        ty: InternedType<'a>,
        idx: u32,
    ) -> (ID, InternedType<'a>) {
        let re = self.ctx.gen();
        let fty = self.ctx.get_type(ty.type_at_idx(idx));
        self.fun
            .add_instr(OpCompositeExtract(fty.id, re, id, vec![idx]));
        (re, fty)
    }

    pub fn composite_extract2(
        &mut self,
        id: ID,
        ty: InternedType<'a>,
        name: &'a str,
    ) -> (ID, InternedType<'a>) {
        let idx = ty.get_field_idx(name);
        self.composite_extract(id, ty, idx)
    }

    pub fn access_chain(
        &mut self,
        id: ID,
        pty: InternedType<'a>,
        idx: u32,
    ) -> (ID, InternedType<'a>) {
        let (ty, class) = pty.as_ptr().unwrap();
        let re = self.ctx.gen();
        let fty = self.ctx.get_type(ty.type_at_idx(idx));
        let fty = self.ctx.get_ptr(fty, class);
        let idx = self.ctx.get_const_u32(idx);
        self.fun
            .add_instr(OpInBoundsAccessChain(fty.id, re, id, vec![idx]));
        (re, fty)
    }

    pub fn access_chain2(
        &mut self,
        id: ID,
        pty: InternedType<'a>,
        name: &'a str,
    ) -> (ID, InternedType<'a>) {
        let (ty, class) = pty.as_ptr().unwrap();
        let idx = ty.get_field_idx(name);
        self.composite_extract(id, ty, idx)
    }

    pub fn subscript(&mut self, id: ID, fty: InternedType<'a>, idx: ID) -> (ID, InternedType<'a>) {
        let re = self.ctx.gen();
        self.fun.add_instr(OpAccessChain(fty.id, re, id, vec![idx]));
        (re, fty)
    }

    pub fn image_size(&mut self, id: ID, ty: InternedType<'a>) -> (ID, InternedType<'a>) {
        match ty.ty.as_ref() {
            Type::Image(dim) => {
                let ty = match dim {
                    Dim::_1D => self.ctx.get_type(Rc::new(Type::S(Scalar::Uint(32)))),
                    Dim::_2D => self.ctx.get_type(Rc::new(Type::S(Scalar::Uint(32)))),
                    Dim::_3D => self.ctx.get_type(Rc::new(Type::S(Scalar::Uint(32)))),
                    _ => panic!(""),
                };
                let re = self.ctx.gen();
                self.add_instr(OpImageQuerySize(ty.id, re, id));
                (re, ty)
            }
            Type::Ptr(pty, class) => match pty.ty.as_ref() {
                Type::Image(dim) => {
                    let id = self.load(id, pty.id);
                    self.image_size(id, pty.clone())
                }
                _ => panic!(""),
            },

            _ => panic!("image_size"),
        }
    }

    pub fn cast(&mut self, id: ID, from: InternedType<'a>, to: InternedType<'a>) -> ID {
        if (from == to) {
            return id;
        }

        id
    }

    pub fn cctor(&mut self, args: Vec<ID>, ty: IdType) -> ID {
        let re = self.ctx.gen();
        self.add_instr(OpCompositeConstruct(ty, re, args));
        re
    }

    pub fn get_global_id(&mut self) -> (ID, InternedType<'a>) {
        if let Some(id) = self.ctx.inputs.get("get_global_id") {
            return id;
        }
        let id = self.ctx.gen();
        self.ctx.inputs.insert("get_global_id", id);
        self.fun.header.push(OpVariable())
    }
}
