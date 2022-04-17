use std::clone;

use crate::*;

#[derive(Default, Debug, Clone)]
pub enum Expr<'a> {
    #[default]
    Void,
    Ident(&'a str),
    Float(f32),
    Uint(u32),
    Unop(&'a str, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Subscript(Box<Expr<'a>>, Box<Expr<'a>>),
    Access(Box<Expr<'a>>, &'a str),
    Binop(Box<Expr<'a>>, &'a str, Box<Expr<'a>>),
    Relop(Box<Expr<'a>>, &'a str, Box<Expr<'a>>),
    List(Vec<Expr<'a>>),
    Ctor(InternedType<'a>, Vec<Expr<'a>>),
}

#[derive(Default, Debug, Clone)]
pub enum Statement<'a> {
    #[default]
    Void,
    Expr(Expr<'a>),
    Decl(Option<&'a str>, InternedType<'a>, &'a str, Expr<'a>),
    If(Expr<'a>, Vec<Statement<'a>>, Option<Vec<Statement<'a>>>),
    Block(Vec<Statement<'a>>),
    While(Expr<'a>, Vec<Statement<'a>>),
    For(
        Option<Box<Statement<'a>>>,
        Option<Expr<'a>>,
        Option<Expr<'a>>,
        Vec<Statement<'a>>,
    ),
}

impl<'a> Statement<'a> {
    pub fn lower(self, ctx: &mut Lower<'a, '_>) {
        match self {
            Statement::Void => (),
            Statement::Expr(expr) => {
                expr.lower(ctx);
            }
            Statement::Decl(_, ty, name, expr) => {
                let id = expr.lower_decl(ty.clone(), ctx);
                ctx.create_var(name, ty, Some(id));
            }
            Statement::Block(block) => {
                ctx.push();
                for stat in block {
                    stat.lower(ctx);
                }
                ctx.pop();
            }
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
                    tail_label.id,
                    post_label.id,
                    LoopControl::default(),
                ));
                ctx.jump(con_label.id);
                ctx.set_label(con_label);
                if let Some(con) = con {
                    let con = con.lower_load(ctx).0;
                    ctx.branch(con, body_label.id, tail_label.id);
                } else {
                    ctx.jump(body_label.id);
                };
                ctx.set_label(body_label);
                ctx.push();
                for stat in body {
                    stat.lower(ctx);
                }
                ctx.pop();
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
                let c = c.lower_load(ctx).0;
                ctx.add_instr(OpSelectionMerge(r_label.id, SelectionControl::None));
                ctx.branch(c, t_label.id, r_label.id);
                ctx.set_label(t_label);
                ctx.push();
                for stat in t {
                    stat.lower(ctx);
                }
                ctx.pop();
                ctx.jump(r_label.id);
                if let Some(f) = f {
                    ctx.set_label(f_label);
                    ctx.push();
                    for stat in f {
                        stat.lower(ctx);
                    }
                    ctx.pop();
                    ctx.jump(r_label.id);
                }
                ctx.set_label(r_label);
                ctx.pop();
            }
            what => unreachable!("{:?}", what),
        }
    }
}

impl<'a> Expr<'a> {
    pub fn lower_decl(self, ty: InternedType<'a>, ctx: &mut Lower<'a, '_>) -> ID {
        match self {
            Expr::List(args) => Expr::Ctor(ty, args).lower(ctx).0,
            _ => self.lower(ctx).0,
        }
    }

    pub fn lower(self, ctx: &mut Lower<'a, '_>) -> (ID, InternedType<'a>) {
        println!("{:?}", self);
        let re = match self {
            Expr::Ident(a) => ctx.get_var(a),
            Expr::Float(val) => {
                ctx.get_const(u32::from_ne_bytes(val.to_ne_bytes()), Scalar::Real(32))
            }
            Expr::Uint(val) => ctx.get_const(val, Scalar::Uint(32)),
            Expr::Access(l, r) => {
                let (id, ty) = l.lower(ctx);
                match ty.ty.as_ref() {
                    Type::Struct(..) | Type::V(..) => ctx.composite_extract2(id, ty, r),
                    Type::Image(..) => match r {
                        "size" => ctx.image_size(id, ty),
                        what => unimplemented!("{:?}", what),
                    },
                    Type::Ptr(pty, _) => match pty.ty.as_ref() {
                        Type::Struct(..) | Type::V(..) => ctx.access_chain2(id, pty.clone(), r),
                        Type::Image(..) => match r {
                            "size" => ctx.image_size(id, ty),
                            what => unimplemented!("{:?}", what),
                        },
                        what => unimplemented!("{:?}", what),
                    },
                    what => unimplemented!("{:?}", what),
                }
            }
            Expr::Ctor(composite, args) => {
                let mut args: Vec<_> = args
                    .into_iter()
                    .enumerate()
                    .map(|(idx, e)| {
                        let (id, ty) = e.lower_load(ctx);
                        let to = composite.type_at_idx(idx as u32);
                        let to = ctx.ctx.get_type(to);
                        ctx.cast(id, ty, to)
                    })
                    .collect();
                // if let Some((t, n)) = composite.get_base_safe().map(type_attr) {
                //     if n != args.len() {
                //         if args.len() == 1 {
                //             args = vec![args.pop().unwrap(); n];
                //         } else {
                //             panic!()
                //         }
                //     }
                // }
                (ctx.cctor(args, composite.id), composite)
            }
            Expr::Call(box Expr::Ident("get_global_id"), args) if args.is_empty() => {
                let ty = ctx.ctx.get_type(Rc::new(Type::V(Scalar::Uint(32), 3)));

                (ctx.get_global_id(), ty)
            }
            what => panic!("{:?}", what),
        };
        re
    }
    pub fn lower_load(self, ctx: &mut Lower<'a, '_>) -> (ID, InternedType<'a>) {
        let (id, ty) = self.lower(ctx);
        ctx.load_if(id, ty)
    }
}
