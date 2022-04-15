pub use slcc::*;



#[derive(Debug)]
pub enum Expr<'a> {
    Ident(&'a str),
    Float(f32),
    Uint(u32),
    Unop(&'a str, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Subscript(Box<Expr<'a>>, Box<Expr<'a>>),
    Access(Box<Expr<'a>>, &'a str),
    Binop(Box<Expr<'a>>, &'a str, Box<Expr<'a>>),
    List(Vec<Expr<'a>>),
    Ctor(Type<'a>, Vec<Expr<'a>>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Decl(Option<&'a str>, Type<'a>, &'a str, Expr<'a>),
    If(Expr<'a>, Vec<Statement<'a>>, Option<Vec<Statement<'a>>>),
    Block(Vec<Statement<'a>>),
    Void,
    While(Expr<'a>, Vec<Statement<'a>>),
    For(
        Option<Box<Statement<'a>>>,
        Option<Expr<'a>>,
        Option<Expr<'a>>,
        Vec<Statement<'a>>,
    ),
}
impl<'a> Statement<'a> {
    pub fn lower(self, ctx: &mut Context<'a>) {
        match self {
            Statement::Expr(expr) => {
                expr.lower(ctx);
            }
            Statement::Decl(_, ty, name, expr) => {
                let expr = expr.lower_decl(ty.clone(), ctx);
                let expr = ctx.load_if(expr);
                ctx.create_variable(ty, name, StorageClass::Function, Some(expr));
            }

            Statement::Block(block) => {
                ctx.push();
                for stat in block {
                    stat.lower(ctx);
                }
                ctx.pop();
            }

            Statement::Void => (),
            Statement::For(init, con, post, body) => {
                ctx.push();
                let header = ctx.label();
                let con_label = ctx.label();
                let body_label = ctx.label();
                let post_label = ctx.label();
                let tail_label = ctx.label();
                let hid = header.id;

                if let Some(init) = init {
                    init.lower(ctx);
                }
                ctx.jump(hid);
                ctx.set_label(header);
                ctx.add_instr(OpLoopMerge(
                    tail_label.id.into(),
                    post_label.id.into(),
                    LoopControl::default(),
                ));
                ctx.jump(con_label.id);

                ctx.set_label(con_label);

                let c = if let Some(con) = con {
                    let c = con.lower(ctx);
                    ctx.load_if(c).1
                } else {
                    ctx.get_bool(true)
                };
                ctx.branch(c, body_label.id, tail_label.id);

                ctx.set_label(body_label);
                for stat in body {
                    stat.lower(ctx);
                }
                ctx.jump(post_label.id);
                ctx.set_label(post_label);
                if let Some(post) = post {
                    post.lower(ctx);
                }
                ctx.jump(hid);
                ctx.set_label(tail_label);
                ctx.pop();
            }

            Statement::If(c, t, f) => {
                ctx.push();
                let t_label = ctx.label();
                let f_label = ctx.label();
                let r_label = ctx.label();

                let c = c.lower(ctx);
                let (_, c, _) = ctx.load_if(c);
                ctx.add_instr(OpSelectionMerge(r_label.id.into(), SelectionControl::None));
                ctx.branch(c, t_label.id, r_label.id);
                ctx.set_label(t_label);
                for stat in t {
                    stat.lower(ctx);
                }
                ctx.jump(r_label.id);
                if let Some(f) = f {
                    ctx.set_label(f_label);
                    for stat in f {
                        stat.lower(ctx);
                    }
                    ctx.jump(r_label.id);
                }
                ctx.set_label(r_label);
                ctx.pop();
            }
            what => unreachable!("{:?}", what),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum BaseType {
    Bool,
    Float,
    Int,
    Uint,
    Float2,
    Int2,
    Uint2,
    Bool2,
    Float3,
    Int3,
    Uint3,
    Bool3,
    Float4,
    Int4,
    Uint4,
    Bool4,
    Image,
    Image2D,
    Image3D,
}

pub fn attr_type(idx: usize, sz: usize) -> BaseType {
    match (idx, sz) {
        (0, 1) => BaseType::Float,
        (0, 2) => BaseType::Float2,
        (0, 3) => BaseType::Float3,
        (0, 4) => BaseType::Float4,
        (1, 1) => BaseType::Int,
        (1, 2) => BaseType::Int2,
        (1, 3) => BaseType::Int3,
        (1, 4) => BaseType::Int4,
        (2, 1) => BaseType::Uint,
        (2, 2) => BaseType::Uint2,
        (2, 3) => BaseType::Uint3,
        (2, 4) => BaseType::Uint4,
        (3, 1) => BaseType::Bool,
        (3, 2) => BaseType::Bool2,
        (3, 3) => BaseType::Bool3,
        (3, 4) => BaseType::Bool4,
        _ => unreachable!(),
    }
}

pub fn type_attr(ty: BaseType) -> (usize, usize) {
    use BaseType::*;

    let idx = match ty {
        Float | Float2 | Float3 | Float4 => 0,
        Int | Int2 | Int3 | Int4 => 1,
        Uint | Uint2 | Uint3 | Uint4 => 2,
        Bool | Bool2 | Bool3 | Bool4 => 3,
        _ => unreachable!(),
    };

    let sz = match ty {
        Float | Int | Uint | Bool => 1,
        Float2 | Int2 | Uint2 | Bool2 => 2,
        Float3 | Int3 | Uint3 | Bool3 => 3,
        Float4 | Int4 | Uint4 | Bool4 => 4,
        _ => unreachable!(),
    };

    (idx, sz)
}

pub fn get_binop(ty: BaseType, op: &str) -> fn(IdType, IdResult, ID, ID) -> Opcode {
    match type_attr(ty) {
        (0, _) => match op {
            "*" => OpFMul,
            "/" => OpFDiv,
            "+" => OpFAdd,
            "-" => OpFSub,
            "<" => OpFOrdLessThan,
            ">" => OpFOrdGreaterThan,
            "<=" => OpFOrdLessThanEqual,
            ">=" => OpFOrdGreaterThanEqual,
            "==" => OpFOrdEqual,
            "!=" => OpFOrdNotEqual,
            "||" => OpLogicalOr,
            "&&" => OpLogicalAnd,
            what => unreachable!("{:?}", what),
        },
        (1, _) => match op {
            "*" => OpIMul,
            "/" => OpSDiv,
            "+" => OpIAdd,
            "-" => OpISub,
            "<" => OpSLessThan,
            ">" => OpSGreaterThan,
            "<=" => OpSLessThanEqual,
            ">=" => OpSGreaterThanEqual,
            "==" => OpIEqual,
            "!=" => OpINotEqual,
            "||" => OpLogicalOr,
            "&&" => OpLogicalAnd,
            _ => unreachable!(),
        },
        (2, _) => match op {
            "*" => OpIMul,
            "/" => OpUDiv,
            "+" => OpIAdd,
            "-" => OpISub,
            "<" => OpULessThan,
            ">" => OpUGreaterThan,
            "<=" => OpULessThanEqual,
            ">=" => OpUGreaterThanEqual,
            "==" => OpIEqual,
            "!=" => OpINotEqual,
            "||" => OpLogicalOr,
            "&&" => OpLogicalAnd,
            _ => unreachable!(),
        },
        (3, _) => match op {
            "==" => OpLogicalEqual,
            "!=" => OpLogicalNotEqual,
            "||" => OpLogicalOr,
            "&&" => OpLogicalAnd,
            _ => unreachable!(),
        },
        _ => unimplemented!(),
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Type<'a> {
    Void,
    Base(BaseType),
    Ptr(Box<Type<'a>>, StorageClass),
    Struct(&'a str),
    Function(Box<Type<'a>>, Vec<Type<'a>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field<'a> {
    name: &'a str,
    ty: Type<'a>,
    idx: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct<'a> {
    name: String,
    fields: Map<&'a str, Field<'a>>,
    id: IdType,
}

pub struct Scope<'a> {
    symbols: Map<&'a str, Symbol<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            symbols: Map::default(),
        }
    }

    pub fn add_symbol(&mut self, name: &'a str, symbol: Symbol<'a>) {
        self.symbols.insert(name, symbol);
    }
}

pub struct Symbol<'a> {
    name: &'a str,
    ty: Type<'a>,
    id: IdResult,
}

#[derive(Copy, Clone)]
pub struct IDGenerator {
    next_id: u32,
}

impl IDGenerator {
    fn gen(&mut self) -> ID {
        self.next_id += 1;
        ID(self.next_id)
    }
}

pub struct Label {
    id: IdResult,
    instr: Vec<Opcode>,
}

impl BaseType {
    fn get_size(self) -> u32 {
        let (w, n) = type_attr(self);
        if w == 3 {
            n as u32
        } else {
            n as u32 * 4
        }
    }
}

impl<'a> Type<'a> {
    fn get_base_of_ptr(&self) -> BaseType {
        match self {
            Type::Ptr(t, _) => t.get_base(),
            _ => unreachable!(),
        }
    }
    fn get_base_mby_ptr(&self) -> BaseType {
        match self {
            Type::Base(base) => *base,
            Type::Ptr(t, _) => match t.as_ref() {
                Type::Base(base) => *base,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            Type::Ptr(_, _) => 8,
            Type::Base(base) => base.get_size(),
            _ => panic!(),
        }
    }

    fn get_base(&self) -> BaseType {
        self.get_base_safe().unwrap()
    }

    fn get_base_safe(&self) -> Option<BaseType> {
        match self {
            Type::Base(base) => Some(*base),
            _ => None,
        }
    }
    fn is_ptr(&self) -> bool {
        match self {
            Type::Ptr(_, _) => true,
            _ => false,
        }
    }
    fn get_pointee(&self) -> Type<'a> {
        match self {
            Type::Ptr(p, _) => *p.clone(),
            what => unreachable!("{:?}", what),
        }
    }
}

impl Label {
    pub fn new() -> Self {
        Self {
            id: ID(0).into(),
            instr: Vec::new(),
        }
    }
    pub fn with_id(id: IdResult) -> Self {
        Self {
            id,
            instr: vec![OpLabel(id)],
        }
    }
    fn push(&mut self, op: Opcode) {
        self.instr.push(op);
    }

    fn prepend(&mut self, mut v: Vec<Opcode>) {
        v.insert(0, OpLabel(self.id));
        std::mem::swap(&mut v, &mut self.instr);
        self.instr.extend(v.into_iter().skip(1));
    }
}

struct LabelStack {
    stack: Vec<Label>,
}

impl LabelStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Label::new()],
        }
    }

    fn label(&mut self, id: IdResult) {
        self.stack.push(Label::with_id(id));
    }

    fn push(&mut self, op: Opcode) {
        self.stack.last_mut().unwrap().push(op);
    }

    fn prepend(&mut self, idx: usize, instr: Vec<Opcode>) {
        self.stack.get_mut(idx).unwrap().prepend(instr);
    }
}

pub struct Function<'a> {
    id: IdResult,
    local_size: (u32, u32, u32),
    name: &'a str,
    labels: LabelStack,
    scope_stack: Vec<usize>,
    scopes: Vec<Scope<'a>>,
    variable_instr: Vec<Opcode>,
    interface: Vec<ID>,
}

impl<'a> Function<'a> {
    pub fn new(id: IdResult, local_size: (u32, u32, u32), name: &'a str) -> Self {
        Self {
            id,
            local_size,
            name,
            labels: LabelStack::new(),
            scope_stack: Vec::new(),
            scopes: Vec::new(),
            interface: Vec::new(),
            variable_instr: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.scope_stack.push(self.scopes.len());
        self.scopes.push(Scope::new());
    }

    pub fn pop(&mut self) {
        self.scope_stack.pop();
    }

    fn get_variable(&self, name: &'a str) -> Option<&Symbol<'a>> {
        for i in (0..self.scope_stack.len()).rev() {
            if let Some(var) = self.scopes[i].symbols.get(name) {
                return Some(var);
            }
        }
        None
    }

    fn label(&mut self, id: IdResult) -> Label {
        Label::with_id(id)
    }

    fn set_label(&mut self, lbl: Label) {
        self.labels.stack.push(lbl);
    }

    fn get_curr_scope(&mut self) -> &mut Scope<'a> {
        let cur = *self.scope_stack.last().unwrap();
        self.scopes.get_mut(cur).unwrap()
    }

    pub fn write_to_module(self, module: &mut Module) {
        let mut labels = self.labels;
        labels.prepend(1, self.variable_instr);

        for label in labels.stack {
            for instr in label.instr {
                module.insert_op(instr);
            }
        }
    }
}

pub struct Context<'a> {
    id: IDGenerator,
    global_scope: Scope<'a>,
    functions: Vec<Function<'a>>,
    function_map: Map<&'a str, usize>,
    types: Map<Type<'a>, IdType>,
    structs: Map<&'a str, Struct<'a>>,
    constants: Map<(IdType, [u8; 4]), IdResult>,
    type_instr: Vec<Opcode>,
    const_instr: Vec<Opcode>,
    decorations: Vec<Opcode>,
    member_names: Vec<Opcode>,
    interface_vars: Vec<Opcode>,
    builtins: Map<&'a str, IdResult>,
}

impl<'a> Context<'a> {
    pub fn into_module(mut self) -> slcc::Module {
        let mut module = slcc::Module::default();
        let bound = self.id.gen().0;
        // module.insert_op(OpCapability(Capability::Kernel));
        // module.insert_op(OpCapability(Capability::Addresses));
        // module.insert_op(OpCapability(Capability::VulkanMemoryModel));
        module.insert_op(OpCapability(Capability::Shader));
        module.insert_op(OpCapability(Capability::ImageQuery));
        module.insert_op(OpCapability(Capability::Image1D));
        // module.insert_op(OpExtension("SPV_KHR_vulkan_memory_model".to_owned()));

        module.insert_op(OpMemoryModel(
            AddressingModel::Logical,
            MemoryModel::GLSL450,
        ));

        for (name, idx) in self.function_map {
            let id = self.functions.get(idx).unwrap().id;
            let local_size = self.functions.get(idx).unwrap().local_size;
            let mut interface = vec![];
            std::mem::swap(
                &mut interface,
                &mut self.functions.get_mut(idx).unwrap().interface,
            );

            module.insert_op(OpEntryPoint(
                ExecutionModel::GLCompute,
                id.into(),
                name.to_owned(),
                interface,
            ));

            module.insert_op(OpExecutionMode(
                id.into(),
                ExecutionMode::LocalSize(local_size.0, local_size.1, local_size.2),
            ));
        }

        for mem in self.member_names {
            module.insert_op(mem);
        }

        for dec in self.decorations {
            module.insert_op(dec);
        }

        for t in self.type_instr {
            module.insert_op(t);
        }

        for opc in self.const_instr {
            module.insert_op(opc);
        }
        for opc in self.interface_vars {
            module.insert_op(opc);
        }

        // module.insert_op(OpExecutionMode);
        for f in self.functions {
            f.write_to_module(&mut module);
        }

        module.finalize(bound);
        module
    }

    pub fn new() -> Self {
        Context {
            id: IDGenerator { next_id: 0 },
            global_scope: Scope::new(),
            functions: vec![],
            types: Map::default(),
            structs: Map::default(),
            constants: Map::default(),
            function_map: Map::default(),
            builtins: Map::default(),
            type_instr: Vec::new(),
            const_instr: Vec::new(),
            decorations: Vec::new(),
            member_names: Vec::new(),
            interface_vars: Vec::new(),
        }
    }

    fn get_bool(&mut self, val: bool) -> IdResult {
        let IdType = self.get_type(Type::Base(BaseType::Bool));
        let re: IdResult = self.id.gen().into();

        let val = if val {
            OpConstantTrue(IdType, re)
        } else {
            OpConstantFalse(IdType, re.into())
        };
        self.const_instr.push(val);
        re
    }

    pub fn get_builtin(&mut self, builtin: &'a str) -> IdResult {
        if let Some(id) = self.builtins.get(&builtin) {
            return *id;
        }

        let id: IdResult = self.id.gen().into();

        let (ty, dec) = match builtin {
            "GlobalInvocationID" => (Type::Base(BaseType::Uint3), BuiltIn::GlobalInvocationId),
            _ => unimplemented!(),
        };
        let ty = Type::Ptr(Box::new(ty), StorageClass::Input);

        let IdType = self.get_type(ty.clone());

        self.interface_vars
            .push(OpVariable(IdType, id.into(), StorageClass::Input, None));

        let name = builtin;
        self.builtins.insert(name, id);
        self.global_scope.add_symbol(name, Symbol { name, ty, id });

        self.decorations
            .push(OpDecorate(id.into(), Decoration::BuiltIn(dec)));

        id
    }

    pub fn push(&mut self) {
        self.functions.last_mut().unwrap().push();
    }

    pub fn pop(&mut self) {
        self.functions.last_mut().unwrap().pop();
    }

    fn get_variable(&self, name: &'a str) -> Option<&Symbol<'a>> {
        match self.functions.last().unwrap().get_variable(name) {
            None => self.global_scope.symbols.get(name),
            v => v,
        }
    }

    fn label(&mut self) -> Label {
        let id = self.id.gen().into();
        self.functions.last_mut().unwrap().label(id)
    }

    fn set_label(&mut self, lbl: Label) {
        self.functions.last_mut().unwrap().set_label(lbl)
    }

    fn get_curr_scope(&mut self) -> &mut Scope<'a> {
        self.functions.last_mut().unwrap().get_curr_scope()
    }

    pub fn assign(&mut self, var: (VT, IdResult, Type<'a>), val: (VT, IdResult, Type<'a>)) {
        let (_, mut val, valt) = self.load_if(val);
        let valt = valt.get_base();
        let ty = var.2.get_base_of_ptr();

        match (type_attr(ty), type_attr(valt)) {
            ((x, n), (y, m)) if m == n => {
                val = self.cast(val, valt, ty);
            }
            ((x, n), (y, 1)) => {
                val = self.cctor(Type::Base(attr_type(x, n)), vec![val; n]).1;
            }
            _ => panic!(),
        }
        self.add_instr(OpStore(var.1.into(), val.into(), None));
    }

    pub fn create_ptr_variable(
        &mut self,
        name: &'a str,
        val: (VT, IdResult, Type<'a>),
    ) -> IdResult {
        let id = val.1;
        let var = Symbol {
            name,
            ty: val.2,
            id,
        };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    pub fn create_variable(
        &mut self,
        ty: Type<'a>,
        name: &'a str,
        class: StorageClass,
        val: Option<(VT, IdResult, Type<'a>)>,
    ) -> IdResult {
        if ty.is_ptr() {
            return self.create_ptr_variable(name, val.unwrap());
        }

        let ty = Type::Ptr(Box::new(ty), class);
        let id: IdResult = self.id.gen().into();

        let IdType = self.get_type(ty.clone());
        self.functions
            .last_mut()
            .unwrap()
            .variable_instr
            .push(OpVariable(IdType, id, class, None));

        if let Some(val) = val {
            let (_, mut val, valt) = self.load_if(val);

            if let Some(valt) = valt.get_base_safe() {
                let ty = ty.get_base_of_ptr();
                match (type_attr(ty), type_attr(valt)) {
                    ((x, n), (y, m)) if m == n => {
                        val = self.cast(val, valt, ty);
                    }
                    ((x, n), (y, 1)) => {
                        val = self.cast(val, valt, attr_type(x, 1));
                        val = self.cctor(Type::Base(attr_type(x, n)), vec![val; n]).1;
                    }
                    _ => panic!(),
                }
            }

            self.add_instr(OpStore(id.into(), val.into(), None));
        }

        let var = Symbol { name, ty, id };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    pub fn get_uint(&mut self, val: u32) -> IdResult {
        self.get_val(BaseType::Uint, val.to_ne_bytes())
    }

    pub fn create_func(
        &mut self,
        size: (u32, u32, u32),
        name: &'a str,
        args: Vec<(&'a str, Type<'a>)>,
        out: Option<Vec<(&'a str, Type<'a>)>>,
        stats: Vec<Statement<'a>>,
    ) -> IdResult {
        let id = self.id.gen().into();
        {
            let id = self.id.gen();
            let IdType = self.get_type(Type::Base(BaseType::Uint3));
            let x = self.get_uint(size.0).into();
            let y = self.get_uint(size.1).into();
            let z = self.get_uint(size.2).into();
            self.const_instr
                .push(OpConstantComposite(IdType, id.into(), vec![x, y, z]));
            self.decorations
                .push(OpDecorate(id, Decoration::BuiltIn(BuiltIn::WorkgroupSize)));
        }

        self.functions.push(Function::new(id, size, name));
        let gid = self.get_builtin("GlobalInvocationID");
        self.functions
            .last_mut()
            .unwrap()
            .interface
            .push(gid.into());

        self.push();
        let ret = self.get_type(Type::Void);

        let mut argsx: Vec<_> = args.iter().map(|x| x.1.clone()).collect();

        if let Some(out) = &out {
            argsx.extend(out.iter().map(|x| x.1.clone()));
        }

        let fty = self
            .get_type(Type::Function(Box::new(Type::Void), vec![]))
            .into();

        self.add_instr(OpFunction(ret, id, FunctionControl::None, fty));

        let lbl = self.label();
        self.set_label(lbl);

        let mut images = vec![];

        let input_block_type_id = self.id.gen();
        let input_block_ptr_type_id = self.id.gen();
        let input_block_id = self.id.gen();

        let mut input_block_fields: Vec<ID> = vec![];

        let mut offset = 0;
        for (name, ty) in args {
            match ty {
                Type::Base(t @ (BaseType::Image | BaseType::Image2D | BaseType::Image3D)) => {
                    images.push((name, t, Decoration::NonWritable))
                }
                _ => {
                    let idx = input_block_fields.len() as u32;

                    self.member_names
                        .push(OpMemberName(input_block_type_id, idx, name.to_owned()));

                    self.decorations.push(OpMemberDecorate(
                        input_block_type_id,
                        idx,
                        Decoration::Offset(offset),
                    ));

                    offset += ty.get_size();
                    input_block_fields.push(self.get_type(ty.clone()).into());

                    let idx = self.get_val(BaseType::Uint, idx.to_ne_bytes());

                    let id = self.ptr_access_chain(
                        input_block_id.into(),
                        ty.clone(),
                        StorageClass::Uniform,
                        idx,
                    );
                    self.create_variable(ty, name, StorageClass::Function, Some(id));
                }
            }
        }
        if !input_block_fields.is_empty() {
            self.decorations
                .push(OpDecorate(input_block_type_id, Decoration::Block));

            self.member_names.push(OpName(
                input_block_type_id,
                format!("{}_input_block_t", name),
            ));
            self.member_names
                .push(OpName(input_block_id, format!("{}_input_block", name)));

            self.type_instr
                .push(OpTypeStruct(input_block_type_id.into(), input_block_fields));
            self.type_instr.push(OpTypePointer(
                input_block_ptr_type_id.into(),
                StorageClass::Uniform,
                input_block_type_id.into(),
            ));

            self.interface_vars.push(OpVariable(
                input_block_ptr_type_id.into(),
                input_block_id.into(),
                StorageClass::Uniform,
                None,
            ));
            self.decorations.push(OpDecorate(
                input_block_id.into(),
                Decoration::DescriptorSet(1),
            ));
            self.decorations
                .push(OpDecorate(input_block_id.into(), Decoration::Binding(0)));
        }

        if let Some(out) = out {
            let output_block_type_id = self.id.gen();
            let output_block_ptr_type_id = self.id.gen();
            let output_block_id = self.id.gen();
            let mut output_block_fields: Vec<ID> = vec![];
            let mut offset = 0;
            for (name, ty) in out {
                match ty {
                    Type::Base(t @ (BaseType::Image | BaseType::Image2D | BaseType::Image3D)) => {
                        images.push((name, t, Decoration::NonReadable))
                    }
                    _ => {
                        let idx = output_block_fields.len() as u32;
                        self.member_names.push(OpMemberName(
                            output_block_type_id,
                            idx,
                            name.to_owned(),
                        ));

                        self.decorations.push(OpMemberDecorate(
                            output_block_type_id,
                            idx,
                            Decoration::Offset(offset),
                        ));

                        offset += ty.get_size();
                        output_block_fields.push(self.get_type(ty.clone()).into());

                        let idx = self.get_val(BaseType::Uint, idx.to_ne_bytes());

                        let id = self.ptr_access_chain(
                            output_block_id.into(),
                            ty.clone(),
                            StorageClass::Uniform,
                            idx,
                        );
                        self.create_variable(ty, name, StorageClass::Function, Some(id));
                    }
                }
            }

            if !output_block_fields.is_empty() {
                self.decorations
                    .push(OpDecorate(output_block_type_id, Decoration::Block));
                self.member_names.push(OpName(
                    output_block_type_id,
                    format!("{}_output_block_t", name),
                ));
                self.member_names
                    .push(OpName(output_block_id, format!("{}_output_block", name)));

                self.type_instr.push(OpTypeStruct(
                    output_block_type_id.into(),
                    output_block_fields,
                ));
                self.type_instr.push(OpTypePointer(
                    output_block_ptr_type_id.into(),
                    StorageClass::Uniform,
                    output_block_type_id.into(),
                ));

                self.interface_vars.push(OpVariable(
                    output_block_ptr_type_id.into(),
                    output_block_id.into(),
                    StorageClass::Uniform,
                    None,
                ));
                self.decorations.push(OpDecorate(
                    output_block_id.into(),
                    Decoration::DescriptorSet(1),
                ));
                self.decorations
                    .push(OpDecorate(output_block_id.into(), Decoration::Binding(1)));
            }
        }

        for (binding, (image_name, ty, dec)) in images.into_iter().enumerate() {
            let id: IdResult = self.id.gen().into();
            let ty = Type::Ptr(Box::new(Type::Base(ty)), StorageClass::UniformConstant);
            let IdType = self.get_type(ty.clone());
            self.decorations.push(OpDecorate(id.into(), dec));
            self.interface_vars
                .push(OpVariable(IdType, id, StorageClass::UniformConstant, None));
            self.member_names
                .push(OpName(id.into(), format!("{}_{}", name, image_name)));

            self.decorations
                .push(OpDecorate(id.into(), Decoration::DescriptorSet(0)));
            self.decorations
                .push(OpDecorate(id.into(), Decoration::Binding(binding as u32)));

            let var = Symbol {
                name: image_name,
                ty,
                id,
            };
            self.get_curr_scope().add_symbol(image_name, var);
        }

        for stat in stats {
            stat.lower(self);
        }

        self.add_instr(OpReturn);
        self.add_instr(OpFunctionEnd);
        self.pop();
        let idx = self.functions.len() - 1;
        self.function_map.insert(name, idx);
        id
    }

    pub fn create_arg(&mut self, name: &'a str, ty: Type<'a>) -> IdResult {
        let id: IdResult = self.id.gen().into();
        let IdType = self.get_type(ty.clone());
        self.add_instr(OpFunctionParameter(IdType, id));
        let var = Symbol { name, ty, id };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    pub fn create_interface_variable(
        &mut self,
        name: &'a str,
        ty: Type<'a>,
        set: u32,
        binding: u32,
    ) -> IdResult {
        let id: IdResult = self.id.gen().into();

        let ty = Type::Ptr(Box::new(ty), StorageClass::UniformConstant);
        let IdType = self.get_type(ty.clone());
        self.interface_vars
            .push(OpVariable(IdType, id, StorageClass::UniformConstant, None));
        self.decorations
            .push(OpDecorate(id.into(), Decoration::DescriptorSet(set)));
        self.decorations
            .push(OpDecorate(id.into(), Decoration::Binding(binding)));
        let var = Symbol { name, ty, id };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    fn get_type(&mut self, ty: Type<'a>) -> IdType {
        if let Some(id) = self.types.get(&ty) {
            return *id;
        }

        use BaseType::*;
        use Type::*;
        let id: IdResult = self.id.gen().into();
        let sampled_type = Base(Float);
        let format = ImageFormat::Rgba8;

        let opc = match &ty {
            Void => OpTypeVoid(id),
            Ptr(t, class) => {
                let ptr = self.get_type(*t.clone());
                OpTypePointer(id, class.clone(), ptr.into())
            }
            Function(ret, args) => {
                let ret: ID = self.get_type(*ret.clone()).into();
                let args: Vec<ID> = args
                    .iter()
                    .map(|arg| self.get_type(arg.clone()).into())
                    .collect();
                OpTypeFunction(id, ret, args)
            }
            Base(t) => match t {
                Image => OpTypeImage(
                    id,
                    self.get_type(sampled_type).into(),
                    Dim::_1D,
                    0,
                    0,
                    0,
                    2,
                    format,
                    None,
                ),
                Image2D => OpTypeImage(
                    id,
                    self.get_type(sampled_type).into(),
                    Dim::_2D,
                    0,
                    0,
                    0,
                    2,
                    format,
                    None,
                ),
                Image3D => OpTypeImage(
                    id,
                    self.get_type(sampled_type).into(),
                    Dim::_3D,
                    0,
                    0,
                    0,
                    2,
                    format,
                    None,
                ),
                _ => match type_attr(*t) {
                    (0, 1) => OpTypeFloat(id, 32),
                    (1, 1) => OpTypeInt(id, 32, 1),
                    (2, 1) => OpTypeInt(id, 32, 0),
                    (3, 1) => OpTypeBool(id),
                    (0, n) => OpTypeVector(id, self.get_type(Base(Float)).into(), n as u32),
                    (1, n) => OpTypeVector(id, self.get_type(Base(Int)).into(), n as u32),
                    (2, n) => OpTypeVector(id, self.get_type(Base(Uint)).into(), n as u32),
                    (3, n) => OpTypeVector(id, self.get_type(Base(Bool)).into(), n as u32),
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        };
        self.type_instr.push(opc);
        self.types.insert(ty, id);
        id
    }

    fn get_val(&mut self, ty: BaseType, val: [u8; 4]) -> IdResult {
        let ty = self.get_type(Type::Base(ty));
        if let Some(id) = self.constants.get(&(ty, val)) {
            return *id;
        }
        let id = self.id.gen();
        self.constants.insert((ty, val), id.into());
        self.const_instr
            .push(OpConstant(ty, id.into(), u32::from_ne_bytes(val)));
        id.into()
    }

    fn branch(&mut self, id: IdResult, t: IdResult, f: IdResult) {
        self.add_instr(OpBranchConditional(id.into(), t.into(), f.into(), vec![]));
    }

    fn jump(&mut self, id: IdResult) {
        self.add_instr(OpBranch(id.into()));
    }

    fn load(&mut self, id: IdResult, ty: Type<'a>) -> IdResult {
        let re = self.id.gen();
        let ty = self.get_type(ty);
        self.add_instr(OpLoad(ty, re.into(), id.into(), None));
        re.into()
    }

    fn store(&mut self, id: IdResult, val: IdResult) {
        self.add_instr(OpStore(id.into(), val.into(), None));
    }

    fn load_if(&mut self, expr: (VT, IdResult, Type<'a>)) -> (VT, IdResult, Type<'a>) {
        match expr {
            (VT::L, id, Type::Ptr(ty, _)) => (VT::R, self.load(id, *ty.clone()), *ty),
            _ => expr,
        }
    }

    fn call(&mut self, func: IdResult, return_type: Type<'a>, args: Vec<ID>) -> IdResult {
        let return_type = self.get_type(return_type);
        let re: IdResult = self.id.gen().into();
        self.add_instr(OpFunctionCall(return_type, re, func.into(), args));
        re
    }

    fn ptr_access_chain(
        &mut self,
        base: IdResult,
        ty: Type<'a>,
        class: StorageClass,
        idx: IdResult,
    ) -> (VT, IdResult, Type<'a>) {
        let re: IdResult = self.id.gen().into();
        let ty = Type::Ptr(Box::new(ty), class);
        let IdType = self.get_type(ty.clone());
        self.add_instr(OpInBoundsAccessChain(
            IdType,
            re,
            base.into(),
            vec![idx.into()],
        ));
        (VT::L, re, ty)
    }

    fn vec_dyn_extract(
        &mut self,
        base: IdResult,
        vec: BaseType,
        var: IdResult,
    ) -> (VT, IdResult, Type<'a>) {
        let ty = Type::Base(vec).underlying();
        let IdType = self.get_type(ty.clone());
        let re: IdResult = self.id.gen().into();
        self.add_instr(OpVectorExtractDynamic(IdType, re, base.into(), var.into()));
        (VT::R, re, ty)
    }

    fn vec_composite_extract(
        &mut self,
        base: IdResult,
        vec: BaseType,
        var: &'a str,
    ) -> (VT, IdResult, Type<'a>) {
        let idx: u32 = match var {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            "w" => 3,
            _ => panic!("Unknown vector component"),
        };

        let ty = match vec {
            BaseType::Float2 => BaseType::Float,
            BaseType::Float3 => BaseType::Float,
            BaseType::Float4 => BaseType::Float,
            BaseType::Uint2 => BaseType::Uint,
            BaseType::Uint3 => BaseType::Uint,
            BaseType::Uint4 => BaseType::Uint,
            _ => unreachable!(),
        };

        let IdType = self.get_type(Type::Base(ty));
        let re: IdResult = self.id.gen().into();
        self.add_instr(OpCompositeExtract(IdType, re, base.into(), vec![idx]));
        (VT::R, re, Type::Base(ty))
    }

    fn vec_access_chain_val(
        &mut self,
        base: IdResult,
        vec: BaseType,
        class: StorageClass,
        idx: IdResult,
    ) -> (VT, IdResult, Type<'a>) {
        let ty = match vec {
            BaseType::Float2 => BaseType::Float,
            BaseType::Float3 => BaseType::Float,
            BaseType::Float4 => BaseType::Float,
            BaseType::Uint2 => BaseType::Uint,
            BaseType::Uint3 => BaseType::Uint,
            BaseType::Uint4 => BaseType::Uint,
            BaseType::Int2 => BaseType::Int,
            BaseType::Int3 => BaseType::Int,
            BaseType::Int4 => BaseType::Int,
            vec => unreachable!("{:?}", vec),
        };

        let IdType = self.get_type(Type::Ptr(Box::new(Type::Base(ty)), class));
        let re: IdResult = self.id.gen().into();
        self.add_instr(OpInBoundsAccessChain(
            IdType,
            re,
            base.into(),
            vec![idx.into()],
        ));
        (VT::L, re, Type::Ptr(Box::new(Type::Base(ty)), class))
    }

    fn vec_access_chain(
        &mut self,
        base: IdResult,
        vec: BaseType,
        class: StorageClass,
        idx: u32,
    ) -> (VT, IdResult, Type<'a>) {
        let ty = match vec {
            BaseType::Float2 => BaseType::Float,
            BaseType::Float3 => BaseType::Float,
            BaseType::Float4 => BaseType::Float,
            BaseType::Uint2 => BaseType::Uint,
            BaseType::Uint3 => BaseType::Uint,
            BaseType::Uint4 => BaseType::Uint,
            BaseType::Int2 => BaseType::Int,
            BaseType::Int3 => BaseType::Int,
            BaseType::Int4 => BaseType::Int,
            vec => unreachable!("{:?}", vec),
        };
        let idx = self.get_val(BaseType::Uint, idx.to_ne_bytes());
        self.vec_access_chain_val(base, vec, class, idx)
    }

    fn vec_access_chain_name(
        &mut self,
        base: IdResult,
        vec: BaseType,
        class: StorageClass,
        var: &'a str,
    ) -> (VT, IdResult, Type<'a>) {
        let idx: u32 = match var {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            "w" => 3,
            _ => panic!("Unknown vector component"),
        };
        self.vec_access_chain(base, vec, class, idx)
    }

    fn access_chain(
        &mut self,
        base: IdResult,
        ty: &'a str,
        var: &'a str,
    ) -> (VT, IdResult, Type<'a>) {
        let re: IdResult = self.id.gen().into();
        let field = self.find_field(ty, var);
        let ty = field.ty;
        let IdType = self.get_type(ty.clone());
        let idx = self.get_val(BaseType::Uint, field.idx.to_ne_bytes());
        self.add_instr(OpInBoundsAccessChain(
            IdType,
            re,
            base.into(),
            vec![idx.into()],
        ));
        (VT::L, re, ty)
    }

    fn find_field(&self, ty: &'a str, var: &'a str) -> Field<'a> {
        self.structs
            .get(ty)
            .unwrap()
            .fields
            .get(var)
            .unwrap()
            .clone()
    }

    fn composite_extract(
        &mut self,
        base: IdResult,
        ty: &'a str,
        var: &'a str,
    ) -> (VT, IdResult, Type<'a>) {
        let re: IdResult = self.id.gen().into();
        let field = self.find_field(ty, var);
        let ty = field.ty;
        let IdType = self.get_type(ty.clone());
        self.add_instr(OpCompositeExtract(IdType, re, base.into(), vec![field.idx]));
        (VT::R, re, ty)
    }

    fn cctor(&mut self, ty: Type<'a>, args: Vec<IdResult>) -> (VT, IdResult, Type<'a>) {
        if type_attr(ty.get_base()).1 == 1 {
            panic!();
        }

        let re: IdResult = self.id.gen().into();
        let IdType = self.get_type(ty.clone());
        self.add_instr(OpCompositeConstruct(
            IdType,
            re,
            args.iter().map(|arg| (*arg).into()).collect(),
        ));
        (VT::R, re, ty)
    }

    fn arithmetics(
        &mut self,
        lhs: IdResult,
        rhs: IdResult,
        ty: BaseType,
        ret: Type<'a>,
        inst: fn(IdType, IdResult, ID, ID) -> Opcode,
    ) -> (VT, IdResult, Type<'a>) {
        let IdType = self.get_type(ret.clone());
        let re: IdResult = self.id.gen().into();
        self.add_instr(inst(IdType, re, lhs.into(), rhs.into()));
        (VT::R, re, ret)
    }

    fn get_image_size(&mut self, img: IdResult, ty: BaseType) -> (VT, IdResult, Type<'a>) {
        let re: IdResult = self.id.gen().into();
        let sz = match ty {
            BaseType::Image => BaseType::Int,
            BaseType::Image2D => BaseType::Int2,
            BaseType::Image3D => BaseType::Int3,
            _ => unreachable!(),
        };
        let IdType = self.get_type(Type::Base(sz));
        self.add_instr(OpImageQuerySize(IdType, re, img.into()));

        (VT::R, re, Type::Base(sz))
    }

    fn write_image(
        &mut self,
        img: IdResult,
        ty: Type<'a>,
        coord: (VT, IdResult, Type<'a>),
        color: (VT, IdResult, Type<'a>),
    ) {
        let (_, coord, dim) = self.load_if(coord);
        let (_, mut color, fmt) = self.load_if(color);

        let fmt = fmt.get_base();

        match (ty.get_base(), dim.get_base()) {
            (
                BaseType::Image3D,
                BaseType::Float4 | BaseType::Float3 | BaseType::Uint3 | BaseType::Int3,
            ) => (),
            (
                BaseType::Image2D,
                BaseType::Float4 | BaseType::Float2 | BaseType::Uint2 | BaseType::Int2,
            ) => (),
            (
                BaseType::Image,
                BaseType::Float4 | BaseType::Float | BaseType::Uint | BaseType::Int,
            ) => (),
            (ty, dim) => unreachable!("{:?} {:?}", ty, dim),
        };

        match type_attr(fmt) {
            (0, 1) => {
                color = self.cctor(Type::Base(BaseType::Float4), vec![color; 4]).1;
            }
            (x, 1) => {
                color = self.cast(color, fmt, BaseType::Float);
                color = self.cctor(Type::Base(BaseType::Float4), vec![color; 4]).1;
            }
            (x, 4) => {
                color = self.cast(color, fmt, BaseType::Float4);
            }
            what => unreachable!("{:?}", what),
        }

        self.add_instr(OpImageWrite(img.into(), coord.into(), color.into(), None));
    }

    fn read_image(
        &mut self,
        img: IdResult,
        ty: Type<'a>,
        coord: (VT, IdResult, Type<'a>),
    ) -> (VT, IdResult, Type<'a>) {
        let (_, coord, dim) = self.load_if(coord);

        match (ty.get_base(), dim.get_base()) {
            (BaseType::Image3D, BaseType::Float3 | BaseType::Uint3 | BaseType::Int3) => (),
            (BaseType::Image2D, BaseType::Float2 | BaseType::Uint2 | BaseType::Int2) => (),
            (BaseType::Image, BaseType::Float | BaseType::Uint | BaseType::Int) => (),
            (ty, dim) => unreachable!("{:?} {:?}", ty, dim),
        };

        let re = self.id.gen().into();
        let IdType = self.get_type(Type::Base(BaseType::Float4));
        self.add_instr(OpImageRead(IdType, re, img.into(), coord.into(), None));

        (VT::R, re, Type::Base(BaseType::Float4))
    }

    fn negate(
        &mut self,
        base: IdResult,
        ty: BaseType,
        inst: fn(IdType, IdResult, ID) -> Opcode,
    ) -> (VT, IdResult, Type<'a>) {
        let re: IdResult = self.id.gen().into();
        let IdType = self.get_type(Type::Base(ty));
        self.add_instr(inst(IdType, re, base.into()));
        (VT::R, re, Type::Base(ty))
    }

    fn add_instr(&mut self, opc: Opcode) {
        self.functions.last_mut().unwrap().labels.push(opc);
    }

    fn cast(&mut self, id: IdResult, from: BaseType, to: BaseType) -> IdResult {
        assert!(type_attr(from).1 == type_attr(to).1);
        if (from == to) {
            return id;
        }

        let id = id.into();
        let re: IdResult = self.id.gen().into();

        let IdType = self.get_type(Type::Base(to));

        let op = match (type_attr(from).0, type_attr(to).0) {
            (0, 1) => OpConvertFToS(IdType, re, id),
            (0, 2) => OpConvertFToU(IdType, re, id),
            (1, 0) => OpConvertSToF(IdType, re, id),
            (2, 0) => OpConvertUToF(IdType, re, id),
            (1, 2) => OpBitcast(IdType, re, id),
            (2, 1) => OpBitcast(IdType, re, id),
            _ => unreachable!(),
        };

        self.add_instr(op);
        re
    }

    fn common_type_cast(
        &mut self,
        l: (VT, IdResult, Type<'a>),
        r: (VT, IdResult, Type<'a>),
    ) -> (IdResult, IdResult, BaseType) {
        let (_, mut li, lt) = self.load_if(l);
        let (_, mut ri, rt) = self.load_if(r);

        let (mut lt, mut rt) = match (lt, rt) {
            (Type::Base(lt), Type::Base(rt)) => (lt, rt),
            what => unreachable!("{:?}", what),
        };

        if lt == rt {
            return (li, ri, lt);
        }

        type_attr(lt);

        let t = match (type_attr(lt), type_attr(rt)) {
            ((x, a), (y, b)) if a == b => {
                let t = attr_type(x.min(y), a);
                li = self.cast(li, lt, t);
                ri = self.cast(ri, rt, t);
                t
            }
            ((x, 1), (y, n)) => {
                if x < y {
                    let pt = rt;
                    rt = attr_type(x, n);
                    ri = self.cast(ri, pt, rt);
                }
                if y < x {
                    li = self.cast(li, lt, attr_type(y, 1));
                }
                li = self.cctor(Type::Base(rt), vec![li; n]).1;
                rt
            }
            ((x, n), (y, 1)) => {
                if x < y {
                    ri = self.cast(ri, rt, attr_type(x, 1));
                }
                if y < x {
                    let pt = lt;
                    lt = attr_type(y, n);
                    li = self.cast(li, pt, lt);
                }
                ri = self.cctor(Type::Base(lt), vec![ri; n]).1;
                lt
            }

            what => unreachable!("{:?}", what),
        };

        (li, ri, t)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VT {
    L,
    R,
    None,
}

impl<'a> Type<'a> {
    fn underlying(&self) -> Type<'a> {
        match self {
            Type::Ptr(t, _) => *t.clone(),
            Type::Base(t) => match type_attr(*t) {
                (0, n) if n > 1 => Type::Base(BaseType::Float),
                (1, n) if n > 1 => Type::Base(BaseType::Int),
                (2, n) if n > 1 => Type::Base(BaseType::Uint),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn type_at_idx(&self, idx: u32, ctx: &Context<'a>) -> Type<'a> {
        match self {
            Type::Base(BaseType::Float4) if idx < 4 => Type::Base(BaseType::Float),
            Type::Base(BaseType::Float3) if idx < 3 => Type::Base(BaseType::Float),
            Type::Base(BaseType::Float2) if idx < 2 => Type::Base(BaseType::Float),
            Type::Base(BaseType::Uint4) if idx < 4 => Type::Base(BaseType::Uint),
            Type::Base(BaseType::Uint3) if idx < 3 => Type::Base(BaseType::Uint),
            Type::Base(BaseType::Uint2) if idx < 2 => Type::Base(BaseType::Uint),
            Type::Base(BaseType::Int4) if idx < 4 => Type::Base(BaseType::Int),
            Type::Base(BaseType::Int3) if idx < 3 => Type::Base(BaseType::Int),
            Type::Base(BaseType::Int2) if idx < 2 => Type::Base(BaseType::Int),
            Type::Struct(name) => ctx
                .structs
                .get(name)
                .unwrap()
                .fields
                .iter()
                .find(|(_, field)| field.idx == idx)
                .unwrap()
                .1
                .ty
                .clone(),
            Type::Ptr(t, _) => *t.clone(),
            what => unreachable!("{:?}", what),
        }
    }
}

impl<'a> Expr<'a> {
    pub fn lower_decl(
        self,
        composite: Type<'a>,
        ctx: &mut Context<'a>,
    ) -> (VT, IdResult, Type<'a>) {
        match self {
            Expr::List(args) => Expr::Ctor(composite, args).lower(ctx),
            _ => self.lower(ctx),
        }
    }

    pub fn lower(self, ctx: &mut Context<'a>) -> (VT, IdResult, Type<'a>) {
        // println!("{:?}", self);
        match self {
            Expr::Ident(a) => {
                let var = ctx.get_variable(a).expect(a);
                (VT::L, var.id, var.ty.clone())
            }
            Expr::Float(val) => (
                VT::R,
                ctx.get_val(BaseType::Float, val.to_ne_bytes()),
                Type::Base(BaseType::Float),
            ),
            Expr::Uint(val) => (
                VT::R,
                ctx.get_val(BaseType::Uint, val.to_ne_bytes()),
                Type::Base(BaseType::Uint),
            ),

            Expr::Ctor(composite, args) => {
                let mut args: Vec<_> = args
                    .into_iter()
                    .enumerate()
                    .map(|(idx, e)| {
                        let e = e.lower(ctx);
                        let (_, id, ty) = ctx.load_if(e);
                        ctx.cast(
                            id,
                            ty.get_base(),
                            composite.type_at_idx(idx as u32, ctx).get_base(),
                        )
                    })
                    .collect();
                if let Some((t, n)) = composite.get_base_safe().map(type_attr) {
                    if n != args.len() {
                        if args.len() == 1 {
                            args = vec![args.pop().unwrap(); n];
                        } else {
                            panic!()
                        }
                    }
                }
                ctx.cctor(composite, args)
            }
            Expr::Unop(op, expr) => match op {
                "&" => match expr.lower(ctx) {
                    (VT::L, id, Type::Ptr(ty, class)) => (VT::R, id, Type::Ptr(ty, class)),
                    _ => unreachable!(),
                },
                "*" => match expr.lower(ctx) {
                    (VT::R, id, Type::Ptr(ty, class)) => (VT::L, id, Type::Ptr(ty, class)),
                    (VT::L, id, Type::Ptr(ty, class)) => (VT::L, ctx.load(id, *ty.clone()), *ty),
                    _ => unreachable!(),
                },
                "-" => {
                    let x = expr.lower(ctx);
                    let (_, x, t) = ctx.load_if(x);
                    let t = t.get_base();
                    match type_attr(t) {
                        (0, _) => ctx.negate(x, t, OpFNegate),
                        (1, _) => ctx.negate(x, t, OpSNegate),
                        (2, n) => {
                            let nt = attr_type(1, n);
                            let x = ctx.cast(x, t, nt);
                            ctx.negate(x, nt, OpSNegate)
                        }
                        _ => unreachable!(),
                    }
                }
                "++" => match expr.lower(ctx) {
                    (VT::L, id, Type::Ptr(box Type::Base(t), class)) => {
                        let lid = ctx.load(id, Type::Base(t));
                        let one = ctx.get_val(t, 1u32.to_ne_bytes());
                        let re = ctx.arithmetics(lid, one, t, Type::Base(t), OpIAdd).1;
                        ctx.store(id, re);
                        (VT::L, id, Type::Ptr(Box::new(Type::Base(t)), class))
                    }
                    what => unreachable!("{:?}", what),
                },
                _ => unreachable!("{:?}{:#?}", op, expr),
            },
            Expr::Call(f, args) => {
                match &*f {
                    Expr::Ident("get_image_size") => {
                        assert!(args.len() == 1);
                        let mut args = args;
                        let (_, img, ty) = args.pop().unwrap().lower(ctx);
                        let img = ctx.load(img, ty.get_pointee());
                        match ty.get_pointee() {
                            Type::Base(ty) => return ctx.get_image_size(img, ty),
                            _ => unreachable!(),
                        }
                    }
                    Expr::Ident("get_global_id") => {
                        assert!(args.len() == 1);
                        let mut args = args;
                        let (_, idx, ty) = args.pop().unwrap().lower(ctx);
                        let id = ctx.get_builtin("GlobalInvocationID");
                        let id = ctx.load(id, Type::Base(BaseType::Uint3));
                        return ctx.vec_dyn_extract(id, BaseType::Uint3, idx);
                    }
                    Expr::Ident("write_image") => {
                        assert!(args.len() == 3);
                        let mut args = args;
                        let color = args.pop().unwrap().lower(ctx);
                        let coord = args.pop().unwrap().lower(ctx);
                        let (_, img, ty) = args.pop().unwrap().lower(ctx);
                        let img = ctx.load(img, ty.get_pointee());
                        ctx.write_image(img, ty.get_pointee(), coord, color);
                        return (VT::None, ID(0).into(), Type::Void);
                    }
                    Expr::Ident("read_image") => {
                        assert!(args.len() == 2);
                        let mut args = args;
                        let coord = args.pop().unwrap().lower(ctx);
                        let (_, img, ty) = args.pop().unwrap().lower(ctx);
                        let img = ctx.load(img, ty.get_pointee());
                        return ctx.read_image(img, ty.get_pointee(), coord);
                    }
                    _ => (),
                };

                let (_, fid, fty) = f.lower(ctx);
                match fty {
                    Type::Function(ret, sig) => {
                        let ret = *ret;
                        assert!(sig.len() == args.len());
                        let args = args
                            .into_iter()
                            .zip(sig.into_iter())
                            .map(|(arg, ty)| match arg.lower(ctx) {
                                (VT::L, id, ty2) if ty2 == ty => ctx.load(id, ty),
                                (VT::R, id, ty2) if ty2 == ty => id,
                                _ => unreachable!(),
                            })
                            .map(|id| id.into())
                            .collect::<Vec<ID>>();
                        (VT::R, ctx.call(fid, ret.clone(), args), ret)
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Subscript(l, r) => {
                let r = r.lower(ctx);
                let r = ctx.load_if(r);
                let idx = r.1;

                match l.lower(ctx) {
                    (VT::L, id, Type::Ptr(ty, class)) => ctx.ptr_access_chain(id, *ty, class, idx),
                    (VT::R, id, Type::Ptr(ty, class)) => ctx.ptr_access_chain(id, *ty, class, idx),
                    _ => unimplemented!(),
                }
            }
            Expr::Access(l, r) => match l.lower(ctx) {
                (VT::R, id, Type::Struct(name)) => ctx.composite_extract(id, name, r),
                (VT::L, id, Type::Struct(name)) => ctx.access_chain(id, name, r),
                (VT::L, id, Type::Ptr(ty, class)) => {
                    ctx.vec_access_chain_name(id, ty.get_base(), class, r)
                }
                (VT::R, id, Type::Base(base_type)) => ctx.vec_composite_extract(id, base_type, r),
                what => unimplemented!("{:?}", what),
            },
            Expr::Binop(l, op, r) => {
                if "=" == op {
                    let l = l.lower(ctx);
                    let r = r.lower_decl(l.2.get_pointee(), ctx);
                    ctx.assign(l, r);
                    return (VT::None, ID(0), Type::Void);
                }

                let l = l.lower(ctx);
                let r = r.lower(ctx);
                match op {
                    "+=" | "-=" | "*=" | "/=" => {
                        if l.0 == VT::R {
                            panic!()
                        };
                        let lt = l.2.get_base_of_ptr();
                        let li = ctx.load(l.1, Type::Base(lt));
                        let (_, ri, rt) = ctx.load_if(r);
                        let ri = ctx.cast(ri, rt.get_base(), lt);
                        let val = ctx
                            .arithmetics(li, ri, lt, Type::Base(lt), get_binop(lt, &op[..1]))
                            .1;

                        ctx.store(l.1, val);
                        return (VT::L, l.1, l.2);
                    }
                    _ => (),
                };

                let (li, ri, ty) = ctx.common_type_cast(l, r);

                let opc = get_binop(ty, op);

                match op {
                    "<" | ">" | "<=" | ">=" | "==" | "!=" | "||" | "&&" => {
                        ctx.arithmetics(li, ri, ty, Type::Base(BaseType::Bool), opc)
                    }
                    _ => ctx.arithmetics(li, ri, ty, Type::Base(ty), opc),
                }
            }

            _ => unreachable!(),
        }
    }
}
