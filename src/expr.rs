pub use crate::spirv::*;

pub use std::collections::HashMap;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Result,
    hash::Hash,
};

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
}

#[derive(Debug)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Decl(Option<&'a str>, Type<'a>, &'a str, Expr<'a>),
    Assign(Expr<'a>, Expr<'a>),
    If(Expr<'a>, Vec<Statement<'a>>, Option<Vec<Statement<'a>>>),
    Block(Vec<Statement<'a>>),
    Void,
    While(Expr<'a>, Vec<Statement<'a>>),
    For(
        Option<Expr<'a>>,
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
                ctx.create_variable(ty, name, StorageClass::Function(), Some(expr));
            }
            Statement::Assign(l, r) => {
                let l = l.lower(ctx);
                let r = r.lower_decl(l.2.get_pointee(), ctx);
                ctx.assign(l, r);
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
                let con_label = ctx.label();
                let post_label = ctx.label();
                let body_label = ctx.label();
                let tail_label = ctx.label();

                let cid = con_label.id;
                ctx.jump(cid);

                init.map(|expr| expr.lower(ctx));
                ctx.set_label(con_label);

                match con {
                    Some(con) => {
                        let c = con.lower(ctx).1;
                        ctx.branch(c, body_label.id, tail_label.id)
                    }
                    _ => {
                        ctx.jump(body_label.id);
                    }
                };
                ctx.set_label(body_label);
                for stat in body {
                    stat.lower(ctx);
                }
                ctx.jump(post_label.id);

                ctx.set_label(post_label);
                if let Some(post) = post {
                    post.lower(ctx);
                }
                ctx.jump(cid);

                ctx.set_label(tail_label);
            }

            Statement::If(c, t, f) => {
                let t_label = ctx.label();
                let f_label = ctx.label();
                let r_label = ctx.label();

                let c = c.lower(ctx);
                let (_, c, _) = ctx.load_if(c);
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

pub fn get_binop(ty: BaseType, op: &str) -> fn(TypeID, ResultID, ID, ID) -> Opcode {
    use Opcode::*;

    match type_attr(ty) {
        (0, _) => match op {
            "*" => FMul,
            "/" => FDiv,
            "+" => FAdd,
            "-" => FSub,
            "<" => FOrdLessThan,
            ">" => FOrdGreaterThan,
            "<=" => FOrdLessThanEqual,
            ">=" => FOrdGreaterThanEqual,
            "==" => FOrdEqual,
            "!=" => FOrdNotEqual,
            "||" => LogicalOr,
            "&&" => LogicalAnd,
            what => unreachable!("{:?}", what),
        },
        (1, _) => match op {
            "*" => IMul,
            "/" => SDiv,
            "+" => IAdd,
            "-" => ISub,
            "<" => SLessThan,
            ">" => SGreaterThan,
            "<=" => SLessThanEqual,
            ">=" => SGreaterThanEqual,
            "==" => IEqual,
            "!=" => INotEqual,
            "||" => LogicalOr,
            "&&" => LogicalAnd,
            _ => unreachable!(),
        },
        (2, _) => match op {
            "*" => IMul,
            "/" => UDiv,
            "+" => IAdd,
            "-" => ISub,
            "<" => ULessThan,
            ">" => UGreaterThan,
            "<=" => ULessThanEqual,
            ">=" => UGreaterThanEqual,
            "==" => IEqual,
            "!=" => INotEqual,
            "||" => LogicalOr,
            "&&" => LogicalAnd,
            _ => unreachable!(),
        },
        (3, _) => match op {
            "==" => LogicalEqual,
            "!=" => LogicalNotEqual,
            "||" => LogicalOr,
            "&&" => LogicalAnd,
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
    name: &'a str,
    fields: HashMap<&'a str, Field<'a>>,
    id: TypeID,
}

pub struct Scope<'a> {
    symbols: HashMap<&'a str, Symbol<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, name: &'a str, symbol: Symbol<'a>) {
        self.symbols.insert(name, symbol);
    }
}

pub struct Symbol<'a> {
    name: &'a str,
    ty: Type<'a>,
    id: ResultID,
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
    id: ResultID,
    instr: Vec<Opcode>,
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
    fn get_base(&self) -> BaseType {
        match self {
            Type::Base(base) => *base,
            _ => unreachable!(),
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
    pub fn with_id(id: ResultID) -> Self {
        Self {
            id,
            instr: vec![Opcode::Label(id)],
        }
    }
    fn push(&mut self, op: Opcode) {
        self.instr.push(op);
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

    fn label(&mut self, id: ResultID) {
        self.stack.push(Label::with_id(id));
    }

    fn push(&mut self, op: Opcode) {
        self.stack.last_mut().unwrap().push(op);
    }
}

pub struct Function<'a> {
    id: ResultID,
    name: &'a str,
    labels: LabelStack,
    scope_stack: Vec<usize>,
    scopes: Vec<Scope<'a>>,
    variable_instr: Vec<Opcode>,
    builtins: HashMap<&'a str, ResultID>,
    decorations: Vec<Opcode>,
}

impl<'a> Function<'a> {
    pub fn new(id: ResultID, name: &'a str) -> Self {
        Self {
            id,
            name,
            labels: LabelStack::new(),
            scope_stack: Vec::new(),
            scopes: Vec::new(),
            variable_instr: Vec::new(),
            decorations: Vec::new(),
            builtins: HashMap::new(),
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

    fn label(&mut self, id: ResultID) -> Label {
        Label::with_id(id)
    }

    fn set_label(&mut self, lbl: Label) {
        self.labels.stack.push(lbl);
    }

    fn get_curr_scope(&mut self) -> &mut Scope<'a> {
        let cur = *self.scope_stack.last().unwrap();
        self.scopes.get_mut(cur).unwrap()
    }

    pub fn write_to_module(mut self, module: &mut Module) {
        self.labels
            .stack
            .get_mut(1)
            .unwrap()
            .instr
            .extend(self.variable_instr.into_iter());
        let id = self.labels.stack.get(2).unwrap().id;
        self.labels
            .stack
            .get_mut(1)
            .unwrap()
            .push(Opcode::Branch(id.into()));
        for label in self.labels.stack {
            for instr in label.instr {
                module.insert_op(instr);
            }
        }
    }

    pub fn get_builtin(&mut self, builtin: &'a str, ctx: &mut Context<'a>) -> ResultID {
        if let Some(id) = self.builtins.get(&builtin) {
            return *id;
        }

        let id: ResultID = ctx.id.gen().into();

        let (ty, dec) = match builtin {
            "GlobalInvocationID" => (Type::Base(BaseType::Uint3), BuiltIn::GlobalInvocationId()),
            _ => unimplemented!(),
        };
        let ty = Type::Ptr(Box::new(ty), StorageClass::Function());

        let typeid = ctx.get_type(ty.clone());

        self.variable_instr.push(Opcode::Variable(
            typeid,
            id.into(),
            StorageClass::Function(),
            None,
        ));

        let name = builtin;
        self.builtins.insert(name, id);
        self.get_curr_scope()
            .add_symbol(name, Symbol { name, ty, id });

        self.decorations
            .push(Opcode::Decorate(id.into(), Decoration::BuiltIn(dec)));

        id
    }
}

pub struct Context<'a> {
    id: IDGenerator,
    global_scope: Scope<'a>,
    functions: Vec<Function<'a>>,
    function_map: HashMap<&'a str, ResultID>,
    types: HashMap<Type<'a>, TypeID>,
    structs: HashMap<&'a str, Struct<'a>>,
    constants: HashMap<(TypeID, [u8; 4]), ResultID>,
    type_instr: Vec<Opcode>,
}

impl<'a> Context<'a> {
    pub fn into_module(mut self) -> Module {
        let mut module = Module::default();
        let bound = self.id.gen().0;
        module.insert_op(Opcode::Capability(Capability::Kernel()));
        module.insert_op(Opcode::Capability(Capability::Addresses()));

        module.insert_op(Opcode::MemoryModel(
            AddressingModel::Physical32(),
            MemoryModel::OpenCL(),
        ));

        for (name, id) in self.function_map {
            module.insert_op(Opcode::EntryPoint(
                ExecutionModel::Kernel(),
                id.into(),
                name.to_owned(),
                vec![],
            ));
        }

        for f in &self.functions {
            for dec in &f.decorations {
                module.insert_op(dec.clone());
            }
        }

        for t in self.type_instr {
            module.insert_op(t);
        }

        for ((ty, val), id) in self.constants {
            module.insert_op(Opcode::Constant(ty, id, u32::from_ne_bytes(val)));
        }

        // module.insert_op(Opcode::ExecutionMode());
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
            types: HashMap::new(),
            structs: HashMap::new(),
            constants: HashMap::new(),
            function_map: HashMap::new(),
            type_instr: Vec::new(),
        }
    }

    fn get_bool(&mut self, val: bool) -> ResultID {
        let typeid = self.get_type(Type::Base(BaseType::Bool));
        let re: ResultID = self.id.gen().into();

        let val = if val {
            Opcode::ConstantTrue(typeid, re)
        } else {
            Opcode::ConstantFalse(typeid, re.into())
        };
        self.add_instr(val);
        re
    }

    pub fn get_builtin(&mut self, builtin: &'a str) -> ResultID {
        let mut f = self.functions.pop().unwrap();
        let b = f.get_builtin(builtin, self);
        self.functions.push(f);
        b
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

    pub fn assign(&mut self, var: (VT, ResultID, Type<'a>), val: (VT, ResultID, Type<'a>)) {
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
        self.add_instr(Opcode::Store(var.1.into(), val.into(), None));
    }

    pub fn create_variable(
        &mut self,
        ty: Type<'a>,
        name: &'a str,
        class: StorageClass,
        val: Option<(VT, ResultID, Type<'a>)>,
    ) -> ResultID {
        let id: ResultID = self.id.gen().into();
        let ty = Type::Ptr(Box::new(ty), class);
        let typeid = self.get_type(ty.clone());
        self.functions
            .last_mut()
            .unwrap()
            .variable_instr
            .push(Opcode::Variable(typeid, id, class, None));
        if let Some(val) = val {
            let (_, mut val, valt) = self.load_if(val);
            let valt = valt.get_base();
            let ty = ty.get_base_of_ptr();
            match (type_attr(ty), type_attr(valt)) {
                ((x, n), (y, m)) if m == n => {
                    val = self.cast(val, valt, ty);
                }
                ((x, n), (y, 1)) => {
                    val = self.cctor(Type::Base(attr_type(x, n)), vec![val; n]).1;
                }
                _ => panic!(),
            }
            self.add_instr(Opcode::Store(id.into(), val.into(), None));
        }
        let var = Symbol { name, ty, id };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    pub fn create_func(
        &mut self,
        name: &'a str,
        args: Vec<(&'a str, Type<'a>)>,
        out: Option<Vec<(&'a str, Type<'a>)>>,
        stats: Vec<Statement<'a>>,
    ) -> ResultID {
        let id = self.id.gen().into();
        self.functions.push(Function::new(id, name));

        self.push();
        let ret = self.get_type(Type::Void);

        let mut argsx: Vec<_> = args.iter().map(|x| x.1.clone()).collect();

        if let Some(out) = &out {
            argsx.extend(out.iter().map(|x| x.1.clone()));
        }

        let fty = self
            .get_type(Type::Function(Box::new(Type::Void), argsx))
            .into();

        self.add_instr(Opcode::Function(ret, id, FunctionControl::None(), fty));

        let mut interface: Vec<ID> = vec![];

        for (name, ty) in args {
            interface.push(self.create_arg(name, ty).into());
        }

        if let Some(out) = out {
            for (name, ty) in out {
                interface.push(self.create_arg(name, ty).into());
            }
        }
        let lbl = self.label();
        self.set_label(lbl);
        let lbl = self.label();
        self.set_label(lbl);

        for stat in stats {
            stat.lower(self);
        }

        self.add_instr(Opcode::Return());
        self.add_instr(Opcode::FunctionEnd());
        self.pop();

        self.function_map.insert(name, id);
        id
    }

    pub fn create_arg(&mut self, name: &'a str, ty: Type<'a>) -> ResultID {
        let id: ResultID = self.id.gen().into();

        let ty2 = match ty.clone() {
            ty @ Type::Base(BaseType::Image) => ty,
            ty @ Type::Base(BaseType::Image2D) => ty,
            ty @ Type::Base(BaseType::Image3D) => ty,
            ty => ty,
        };

        let typeid = self.get_type(ty2);
        self.add_instr(Opcode::FunctionParameter(typeid, id));
        let var = Symbol { name, ty, id };
        self.get_curr_scope().add_symbol(name, var);
        id
    }

    fn get_type(&mut self, ty: Type<'a>) -> TypeID {
        if let Some(id) = self.types.get(&ty) {
            return *id;
        }

        use BaseType::*;
        use Type::*;
        let id: ResultID = self.id.gen().into();

        let opc = match &ty {
            Void => Opcode::TypeVoid(id),
            Ptr(t, class) => {
                let ptr = self.get_type(*t.clone());
                Opcode::TypePointer(id, class.clone(), ptr.into())
            }
            Function(ret, args) => {
                let ret: ID = self.get_type(*ret.clone()).into();
                let args: Vec<ID> = args
                    .iter()
                    .map(|arg| self.get_type(arg.clone()).into())
                    .collect();
                Opcode::TypeFunction(id, ret, args)
            }
            Base(t) => match t {
                Image => Opcode::TypeImage(
                    id,
                    self.get_type(Void).into(),
                    Dim::_1D(),
                    0,
                    0,
                    0,
                    2,
                    ImageFormat::Unknown(),
                    None,
                ),
                Image2D => Opcode::TypeImage(
                    id,
                    self.get_type(Void).into(),
                    Dim::_2D(),
                    0,
                    0,
                    0,
                    2,
                    ImageFormat::Unknown(),
                    None,
                ),
                Image3D => Opcode::TypeImage(
                    id,
                    self.get_type(Void).into(),
                    Dim::_3D(),
                    0,
                    0,
                    0,
                    2,
                    ImageFormat::Unknown(),
                    None,
                ),
                _ => match type_attr(*t) {
                    (0, 1) => Opcode::TypeFloat(id, 32),
                    (1, 1) => Opcode::TypeInt(id, 32, 0),
                    (2, 1) => return self.get_type(Base(Int)),
                    (3, 1) => Opcode::TypeBool(id),
                    (0, n) => Opcode::TypeVector(id, self.get_type(Base(Float)).into(), n as u32),
                    (1, n) => Opcode::TypeVector(id, self.get_type(Base(Int)).into(), n as u32),
                    (2, n) => return self.get_type(Base(attr_type(1, n))),
                    (3, n) => Opcode::TypeVector(id, self.get_type(Base(Bool)).into(), n as u32),
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        };
        self.type_instr.push(opc);
        self.types.insert(ty, TypeID(id.0));
        TypeID(id.0)
    }

    fn get_val(&mut self, ty: BaseType, val: [u8; 4]) -> ResultID {
        let ty = self.get_type(Type::Base(ty));
        if let Some(id) = self.constants.get(&(ty, val)) {
            return *id;
        }
        let id = self.id.gen();
        self.constants.insert((ty, val), id.into());
        id.into()
    }

    fn branch(&mut self, id: ResultID, t: ResultID, f: ResultID) {
        self.add_instr(Opcode::BranchConditional(
            id.into(),
            t.into(),
            f.into(),
            vec![],
        ));
    }

    fn jump(&mut self, id: ResultID) {
        self.add_instr(Opcode::Branch(id.into()));
    }

    fn load(&mut self, id: ResultID, ty: Type<'a>) -> ResultID {
        let re = self.id.gen();
        let ty = self.get_type(ty);
        self.add_instr(Opcode::Load(ty, re.into(), id.into(), None));
        re.into()
    }

    fn store(&mut self, id: ResultID, val: ResultID) {
        self.add_instr(Opcode::Store(id.into(), val.into(), None));
    }

    fn load_if(&mut self, mut expr: (VT, ResultID, Type<'a>)) -> (VT, ResultID, Type<'a>) {
        match expr {
            (VT::L, id, Type::Ptr(ty, _)) => (VT::R, self.load(id, *ty.clone()), *ty),
            _ => expr,
        }
    }

    fn call(&mut self, func: ResultID, return_type: Type<'a>, args: Vec<ID>) -> ResultID {
        let return_type = self.get_type(return_type);
        let re: ResultID = self.id.gen().into();
        self.add_instr(Opcode::FunctionCall(return_type, re, func.into(), args));
        re
    }

    fn ptr_access_chain(
        &mut self,
        base: ResultID,
        ty: Type<'a>,
        class: StorageClass,
        idx: ResultID,
    ) -> (VT, ResultID, Type<'a>) {
        let re: ResultID = self.id.gen().into();
        let ty = Type::Ptr(Box::new(ty), class);
        let typeid = self.get_type(ty.clone());
        self.add_instr(Opcode::InBoundsAccessChain(
            typeid,
            re,
            base.into(),
            vec![idx.into()],
        ));
        (VT::L, re, ty)
    }

    fn vec_dyn_extract(
        &mut self,
        base: ResultID,
        vec: BaseType,
        var: ResultID,
    ) -> (VT, ResultID, Type<'a>) {
        let ty = Type::Base(vec).underlying();
        let typeid = self.get_type(ty.clone());
        let re: ResultID = self.id.gen().into();
        self.add_instr(Opcode::VectorExtractDynamic(
            typeid,
            re,
            base.into(),
            var.into(),
        ));
        (VT::R, re, ty)
    }

    fn vec_composite_extract(
        &mut self,
        base: ResultID,
        vec: BaseType,
        var: &'a str,
    ) -> (VT, ResultID, Type<'a>) {
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

        let typeid = self.get_type(Type::Base(ty));
        let re: ResultID = self.id.gen().into();
        self.add_instr(Opcode::CompositeExtract(typeid, re, base.into(), vec![idx]));
        (VT::R, re, Type::Base(ty))
    }

    fn vec_access_chain_val(
        &mut self,
        base: ResultID,
        vec: BaseType,
        class: StorageClass,
        idx: ResultID,
    ) -> (VT, ResultID, Type<'a>) {
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

        let typeid = self.get_type(Type::Ptr(Box::new(Type::Base(ty)), class));
        let re: ResultID = self.id.gen().into();
        self.add_instr(Opcode::InBoundsAccessChain(
            typeid,
            re,
            base.into(),
            vec![idx.into()],
        ));
        (VT::L, re, Type::Ptr(Box::new(Type::Base(ty)), class))
    }

    fn vec_access_chain(
        &mut self,
        base: ResultID,
        vec: BaseType,
        class: StorageClass,
        idx: u32,
    ) -> (VT, ResultID, Type<'a>) {
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
        base: ResultID,
        vec: BaseType,
        class: StorageClass,
        var: &'a str,
    ) -> (VT, ResultID, Type<'a>) {
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
        base: ResultID,
        ty: &'a str,
        var: &'a str,
    ) -> (VT, ResultID, Type<'a>) {
        let re: ResultID = self.id.gen().into();
        let field = self.find_field(ty, var);
        let ty = field.ty;
        let typeid = self.get_type(ty.clone());
        let idx = self.get_val(BaseType::Uint, field.idx.to_ne_bytes());
        self.add_instr(Opcode::InBoundsAccessChain(
            typeid,
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
        base: ResultID,
        ty: &'a str,
        var: &'a str,
    ) -> (VT, ResultID, Type<'a>) {
        let re: ResultID = self.id.gen().into();
        let field = self.find_field(ty, var);
        let ty = field.ty;
        let typeid = self.get_type(ty.clone());
        self.add_instr(Opcode::CompositeExtract(
            typeid,
            re,
            base.into(),
            vec![field.idx],
        ));
        (VT::R, re, ty)
    }

    fn cctor(&mut self, ty: Type<'a>, args: Vec<ResultID>) -> (VT, ResultID, Type<'a>) {
        if type_attr(ty.get_base()).1 == 1 {
            panic!();
        }

        let re: ResultID = self.id.gen().into();
        let typeid = self.get_type(ty.clone());
        self.add_instr(Opcode::CompositeConstruct(
            typeid,
            re,
            args.iter().map(|arg| (*arg).into()).collect(),
        ));
        (VT::R, re, ty)
    }

    fn arithmetics(
        &mut self,
        lhs: ResultID,
        rhs: ResultID,
        ty: BaseType,
        ret: Type<'a>,
        inst: fn(TypeID, ResultID, ID, ID) -> Opcode,
    ) -> (VT, ResultID, Type<'a>) {
        let typeid = self.get_type(ret.clone());
        let re: ResultID = self.id.gen().into();
        self.add_instr(inst(typeid, re, lhs.into(), rhs.into()));
        (VT::R, re, ret)
    }

    fn get_image_size(&mut self, img: ResultID, ty: BaseType) -> (VT, ResultID, Type<'a>) {
        let re: ResultID = self.id.gen().into();
        let sz = match ty {
            BaseType::Image => BaseType::Uint,
            BaseType::Image2D => BaseType::Uint2,
            BaseType::Image3D => BaseType::Uint3,
            _ => unreachable!(),
        };
        let typeid = self.get_type(Type::Base(sz));
        self.add_instr(Opcode::ImageQuerySize(typeid, re, img.into()));

        (VT::R, re, Type::Base(sz))
    }

    fn write_image(
        &mut self,
        img: ResultID,
        ty: Type<'a>,
        coord: (VT, ResultID, Type<'a>),
        color: (VT, ResultID, Type<'a>),
    ) {
        let (_, coord, dim) = self.load_if(coord);
        let (_, color, fmt) = self.load_if(color);

        let fmt = fmt.get_base();

        match (ty.get_base(), dim.get_base()) {
            (BaseType::Image3D, BaseType::Float3 | BaseType::Uint3 | BaseType::Int3) => (),
            (BaseType::Image2D, BaseType::Float2 | BaseType::Uint2 | BaseType::Int2) => (),
            (BaseType::Image, BaseType::Float | BaseType::Uint | BaseType::Int) => (),
            (ty, dim) => unreachable!("{:?} {:?}", ty, dim),
        };

        self.add_instr(Opcode::ImageWrite(
            img.into(),
            coord.into(),
            color.into(),
            None,
        ));
    }

    fn read_image(
        &mut self,
        img: ResultID,
        ty: Type<'a>,
        coord: (VT, ResultID, Type<'a>),
    ) -> (VT, ResultID, Type<'a>) {
        let (_, coord, dim) = self.load_if(coord);

        match (ty.get_base(), dim.get_base()) {
            (BaseType::Image3D, BaseType::Float3 | BaseType::Uint3 | BaseType::Int3) => (),
            (BaseType::Image2D, BaseType::Float2 | BaseType::Uint2 | BaseType::Int2) => (),
            (BaseType::Image, BaseType::Float | BaseType::Uint | BaseType::Int) => (),
            (ty, dim) => unreachable!("{:?} {:?}", ty, dim),
        };

        let re = self.id.gen().into();
        let typeid = self.get_type(Type::Base(BaseType::Float4));
        self.add_instr(Opcode::ImageRead(
            typeid,
            re,
            img.into(),
            coord.into(),
            None,
        ));

        (VT::R, re, Type::Base(BaseType::Float4))
    }

    fn negate(
        &mut self,
        base: ResultID,
        ty: BaseType,
        inst: fn(TypeID, ResultID, ID) -> Opcode,
    ) -> (VT, ResultID, Type<'a>) {
        let re: ResultID = self.id.gen().into();
        let typeid = self.get_type(Type::Base(ty));
        self.add_instr(inst(typeid, re, base.into()));
        (VT::R, re, Type::Base(ty))
    }

    fn add_instr(&mut self, opc: Opcode) {
        self.functions.last_mut().unwrap().labels.push(opc);
    }

    fn cast(&mut self, id: ResultID, from: BaseType, to: BaseType) -> ResultID {
        assert!(type_attr(from).1 == type_attr(to).1);
        if (from == to) {
            return id;
        }

        match (type_attr(from).0, type_attr(to).0) {
            (1, 2) => return id,
            (2, 1) => return id,
            _ => (),
        };

        let id = id.into();
        let re: ResultID = self.id.gen().into();

        let typeid = self.get_type(Type::Base(to));

        let op = match (type_attr(from).0, type_attr(to).0) {
            (0, 1) => Opcode::ConvertFToS(typeid, re, id),
            (0, 2) => Opcode::ConvertFToU(typeid, re, id),
            (1, 0) => Opcode::ConvertSToF(typeid, re, id),
            (2, 0) => Opcode::ConvertUToF(typeid, re, id),
            _ => unreachable!(),
        };

        self.add_instr(op);
        re
    }

    fn common_type_cast(
        &mut self,
        l: (VT, ResultID, Type<'a>),
        r: (VT, ResultID, Type<'a>),
    ) -> (ResultID, ResultID, BaseType) {
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
    ) -> (VT, ResultID, Type<'a>) {
        match self {
            Expr::List(args) => {
                let args: Vec<_> = args
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
                ctx.cctor(composite, args)
            }
            _ => self.lower(ctx),
        }
    }

    pub fn lower(self, ctx: &mut Context<'a>) -> (VT, ResultID, Type<'a>) {
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
                        (0, _) => ctx.negate(x, t, Opcode::FNegate),
                        (1, _) => ctx.negate(x, t, Opcode::SNegate),
                        (2, n) => {
                            let nt = attr_type(1, n);
                            let x = ctx.cast(x, t, nt);
                            ctx.negate(x, nt, Opcode::SNegate)
                        }
                        _ => unreachable!(),
                    }
                }
                "++" => match expr.lower(ctx) {
                    (VT::L, id, Type::Ptr(box Type::Base(t), class)) => {
                        let lid = ctx.load(id, Type::Base(t));
                        let one = ctx.get_val(t, 1u32.to_ne_bytes());
                        let re = ctx.arithmetics(lid, one, t, Type::Base(t), Opcode::IAdd).1;
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
                        match ty {
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
                        ctx.write_image(img, ty, coord, color);
                        return (VT::None, ID(0).into(), Type::Void);
                    }
                    Expr::Ident("read_image") => {
                        assert!(args.len() == 2);
                        let mut args = args;
                        let coord = args.pop().unwrap().lower(ctx);
                        let (_, img, ty) = args.pop().unwrap().lower(ctx);
                        return ctx.read_image(img, ty, coord);
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

                use Opcode::*;

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
