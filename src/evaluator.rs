use crate::ast::*;

use std::rc::Rc;
use std::collections::HashMap;
use im_lists::list::*;

#[derive(Clone)]
pub struct Context {
   globals: Rc<HashMap<String,Vec<Rhs>>>,
   locals: List<(String,Rhs)>,
   is_null: bool,
}

impl Context {
   pub fn new(policy: Rc<HashMap<String,Vec<Rhs>>>) -> Context {
      Context {
         globals: policy,
         locals: List::new(),
         is_null: false,
      }
   }
   pub fn null() -> Context {
      Context {
         globals: Rc::new(HashMap::new()),
         locals: List::new(),
         is_null: true,
      }
   }
   pub fn bind(self, symbol: String, term: Rhs) -> Context {
      let locals = List::cons((symbol, term), self.locals);
      Context {
         globals: self.globals.clone(),
         locals: locals,
         is_null: false,
      }
   }
   pub fn lookup(self, symbol: &str) -> Rhs {
      for (k,v) in self.locals {
         if k==symbol {
            return v.clone();
         }
      }
      if let Some(vs) = self.globals.get(symbol) {
         if vs.len()==1 {
            return vs[0].clone();
         } else {
            return Rhs::Poly(vs.clone());
         }
      }
      Rhs::Variable(symbol.to_string())
   }
}

pub fn eval_parse(context: Context, rule: &str, input: StringSlice) -> Result<Rhs,String> {
   for rhs in context.globals.get(rule).expect(rule) {
      if let Rhs::Lambda(lhs,rhs) = rhs {
         let context = destructure_literal(context.clone(), lhs, input.clone());
         if !context.is_null {
            let rhs = eval_rhs(context.clone(), rhs)?;
            let rhs = if let Rhs::App(rs) = rhs {
               let mut s = String::new();
               for rv in rs {
                  if let Rhs::Literal(l) = rv {
                     s.push_str(&l);
                  } else {
                     s.push_str(&rv.to_string());
                  }
               }
               Rhs::Literal(s)
            } else { rhs };
            return Result::Ok( rhs.clone() )
         }
      } else {
         return Result::Ok( rhs.clone() )
      }
   }
   Result::Err( format!("Parse Error [{}]: {}", rule, input.to_string()) )
}

pub fn destructure_literal(context: Context, lhs: &[Rhs], input: StringSlice) -> Context {
   if lhs.len()==0 {
      if input.len()==0 {
         context
      } else {
         Context::null()
      }
   } else if let Rhs::Literal(v) = &lhs[0] {
      if input.starts_with(v) {
         destructure_literal( context, &lhs[1..], input.after(v.len()) )
      } else {
         Context::null()
      }
   } else if let Rhs::Literal(v) = &lhs[lhs.len()-1] {
      if input.ends_with(v) {
         destructure_literal( context, &lhs[..lhs.len()-1], input.before(v.len()) )
      } else {
         Context::null()
      }
   } else if let Rhs::Variable(v) = &lhs[0] {
      context.bind(v.clone(), Rhs::Literal(input.to_string()) )
   } else {
      unimplemented!("evaluator::destructure_literal {}", lhs[0])
   }
}

pub fn eval_lazy(context: Context, f: Rhs, xs: &[Rhs]) -> Result<Rhs,String> {
   let f = eval_rhs(context.clone(), &[f])?;
   if let Rhs::Lambda(lhs,rhs) = &f {
      let inner_context = destructure_rhs(context.clone(), lhs, xs);
      if inner_context.is_null {
         return Result::Err( format!("Pattern Match Failure: {}", Rhs::App(xs.to_vec())) )
      }
      eval_rhs(inner_context, rhs)
   } else if let Rhs::Poly(ps) = &f {
      for p in ps {
      if let Rhs::Lambda(lhs,rhs) = p {
         let inner_context = destructure_rhs(context.clone(), lhs, xs);
         if !inner_context.is_null {
            return eval_rhs(inner_context, rhs);
         }
      }}
      Result::Err( format!("Pattern Match Failure: {}", Rhs::App(xs.to_vec())) )
   } else {
      let mut r = xs.to_vec();
      r.insert(0, f);
      Result::Ok(Rhs::App(r))
   }
}

pub fn eval_rhs(mut context: Context, rhs: &[Rhs]) -> Result<Rhs,String> {
   println!("eval_rhs {}", Rhs::App(rhs.to_vec()));
   if rhs.len()==0 {
      return Result::Ok( Rhs::App(Vec::new()) );
   }
   if rhs.len()==1 {
      return Result::Ok( if let Rhs::Variable(v) = &rhs[0] {
         context.lookup(v)
      } else if let Rhs::App(gs) = &rhs[0] {
         eval_rhs(context, gs)?
      } else {
         rhs[0].clone()
      });
   }
   if let [Rhs::Variable(op), x, ps] = rhs {
   if op == "match" {
      let x = eval_rhs(context.clone(), &[x.clone()])?;
      if let Rhs::App(ps) = ps {
      for p in ps {
      if let Rhs::Lambda(lhs,rhs) = p {
         let inner_context = destructure_rhs(context.clone(), lhs, &[x.clone()]);
         if !inner_context.is_null {
            return eval_rhs(inner_context, rhs);
         }
      }}}
      return Result::Err( format!("Pattern Match Failure: {}", Rhs::App(vec![
         Rhs::Variable("match".to_string()),
         x.clone(),
         ps.clone(),
      ])) )
   }}
   if let [Rhs::Variable(op), cs, x] = rhs {
   if op == "ctx" {
      let x = eval_rhs(context.clone(), &[x.clone()])?;
      let cs = eval_rhs(context.clone(), &[x.clone()])?;
      if let Rhs::App(cs) = cs {
      for c in cs {
      if let Rhs::App(c) = c {
      if let [Rhs::Variable(cv), ct] = &c[..] {
         context = context.bind(cv.clone(), ct.clone());
      }}}}
      return eval_rhs(context.clone(), &[x.clone()]);
   }}
   if let [Rhs::Variable(op), Rhs::Variable(v), t] = &rhs[..2] {
   if op == "let" {
      let t = eval_rhs(context.clone(), &[t.clone()])?;
      context = context.bind(v.clone(), t.clone());
      return eval_rhs(context.clone(), &rhs[2..]);
   }}
   let mut gs = Vec::new();
   for g in rhs {
      gs.push( eval_rhs(context.clone(), &[g.clone()])? );
   }
   if let Rhs::Lambda(lhs,rhs) = &gs[0] {
      let inner_context = destructure_rhs(context.clone(), lhs, &gs[1..]);
      if inner_context.is_null {
         return Result::Err( format!("Pattern Match Failure: {}", Rhs::App(gs)) )
      }
      return eval_rhs(inner_context, rhs);
   } else if let Rhs::Poly(ps) = &gs[0] {
      for p in ps {
      if let Rhs::Lambda(lhs,rhs) = p {
         let inner_context = destructure_rhs(context.clone(), lhs, &gs[1..]);
         if !inner_context.is_null {
            return eval_rhs(inner_context, rhs);
         }
      }}
      return Result::Err( format!("Pattern Match Failure: {}", Rhs::App(gs)) );
   } else {
      return Result::Ok( Rhs::App(gs) );
   }
}

pub fn destructure_rhs(mut context: Context, lhs: &[Rhs], rhs: &[Rhs]) -> Context {
   if lhs.len()==3 {
   if let [Rhs::Literal(lop),Rhs::Variable(lv),Rhs::Literal(lc)] = lhs {
   if lop=="~" {
   if let Rhs::Literal(rv) = &rhs[0] {
      if lc=="Int" && rv.chars().all(|c| c.is_digit(10) || c=='_') {
         return context.bind(lv.clone(), rhs[0].clone());
      } else if lc=="Float" && rv.chars().all(|c| c.is_digit(10) || c=='_' || c=='.') {
         return context.bind(lv.clone(), rhs[0].clone());
      } else {
         return Context::null();
      }
   }}}}
   if lhs.len() != rhs.len() {
      return Context::null();
   }
   for (l,r) in std::iter::zip(lhs,rhs) {
      if context.is_null {
         return context;
      }
      if let (Rhs::App(ls),Rhs::App(rs)) = (l,r) {
         context = destructure_rhs(context,ls,rs);
      } else if let (Rhs::Lambda(llhs,lrhs),Rhs::Lambda(rlhs,rrhs)) = (l,r) {
         context = destructure_rhs(context,llhs,rlhs);
         if context.is_null {
            return context;
         }
         if lrhs.len()==1 {
            context = destructure_rhs(context,lrhs,&[Rhs::App(rrhs.to_vec())]);
         } else {
            context = destructure_rhs(context,lrhs,rrhs);
         }
      } else if let (Rhs::Variable(lv),rv) = (l,r) {
         context = context.bind(lv.clone(), rv.clone());
      } else if let (Rhs::Literal(ll),Rhs::Literal(rl)) = (l,r) {
         if ll != rl {
            return Context::null();
         }
      } else if let (Rhs::App(ls),Rhs::Literal(_)) = (l,r) {
         context = destructure_rhs(context, &ls, &[r.clone()]);
      } else {
         return Context::null();
      }
   }
   context
}

