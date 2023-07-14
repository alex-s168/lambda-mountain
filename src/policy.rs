use crate::ast::*;
use crate::parser::*;
use crate::evaluator::*;

use std::rc::Rc;
use std::collections::HashMap;

pub struct Policy {
   symbols: HashMap<String,Vec<Rhs>>,
}

impl Policy {
   pub fn new() -> Policy {
      Policy {
         symbols: HashMap::new()
      }
   }
   pub fn load(&mut self, input: &str) {
      let input = StringSlice::new(input.to_string());
      for (symbol,rhs) in parse_program(input) {
         if !self.symbols.contains_key(&symbol) {
            self.symbols.insert(symbol.clone(), Vec::new());
         }
         self.symbols.get_mut(&symbol).expect("Policy::load")
                     .push(rhs);
      }
   }
   pub fn hard(&mut self, input: &str) -> Result<Rhs,String> {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = StringSlice::new(input.to_string());
      let input = if self.symbols.contains_key("::pre") {
         StringSlice::new(
            eval_parse(context.clone(), "::pre", input)?.to_string()
         )
      } else {
         input
      };
      let program = parse_rhs(input);
      let post = eval_rhs(context.clone(), &program);
      Result::Ok( post )
   }
   pub fn soft(&mut self, input: &str) -> Result<Rhs,String> {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = StringSlice::new(input.to_string());
      let post = if self.symbols.contains_key("::pre") {
         eval_parse(context.clone(), "::pre", input)?
      } else {
         Rhs::Literal( input.to_string() )
      };
      Result::Ok( post )
   }
   pub fn s_hard(&mut self, input: &str) -> String {
      match self.hard(input) {
         Result::Ok(r) => r.to_string(),
         Result::Err(e) => e
      }
   }
   pub fn s_soft(&mut self, input: &str) -> String {
      match self.soft(input) {
         Result::Ok(r) => r.to_string(),
         Result::Err(e) => e
      }
   }
}

