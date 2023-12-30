/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

G: A Basic Codegen

*/

use crate::*;
use punc::*;

struct StructuredExpression {
   //Put Cons in %rdi idk
   //Put Atoms in %rsi idk
   //Nil if neither %rsi nor %rdi
   term: Term,
   labels: Vec<Term>,
}

static mut UUID_COUNTER: usize = 0;
fn uuid() -> String {
   let id = unsafe { UUID_COUNTER += 1; UUID_COUNTER };
   format!("uuid_{}", id)
}

fn introduce_constant(s: &S) -> StructuredExpression {
   if is_nil(s) {
      StructuredExpression {
         term: punc!(
            (xor %rdi %rdi)
            (xor %rsi %rsi)
         ),
         labels: vec![]
      }
   } else if is_atom(s) {
      let id = uuid();
      StructuredExpression {
         term: punc!(
            (xor %rdi %rdi)
            (mov {Term::var(&format!("@{}",id))} %rsi)
         ),
         labels: vec![punc!(
            (label {Term::var(&id)} 
               (.asciz {Term::var(&format!("\"{}\"",s))})
               (.zero 1)
            )
         )]
      }
   } else {
      panic!("Term is not a constant: {}", s)
   }
}

fn safe_compile_expression(e: &S, tt: &S) -> S {
   unimplemented!("g::safe_compile_expression {} : {}", e, tt)
}

pub fn compile(cfg: &str, s: &S) {
  for (k,v) in kv_iter(s) {
     let e = safe_compile_expression(&v, &typ("Block"));
     println!("g::compile: {} := {}", k, e);
  }
  unimplemented!("g::compile")
  /*
  TODO: fix prelude and verify output before sending to PunC

  let t = compile_expression(s);

  let mut label_t = punc!( label nil_as_string (.asciz "()") (.zero 1) );
  for label in t.labels {
     label_t = punc!( {label} {label_t} );
  }

  let program = punc!(
      (.global _start)
      (.text)
      (label _start
         { t.term }
         (call print)
         (mov @60 %rax)
         (xor %rdi %rdi)
         (syscall)
      )
      ({ include!("../stdlib/prelude.rs") })
      ({label_t})
   );
   println!("program {}", program.to_string());
   program.compile(cfg);
   */
}
