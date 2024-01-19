#![allow(non_camel_case_types)]
use crate::vm::*;
use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar="fraise.pest"]
pub struct fraise;
pub fn prs(s:&String,v:&mut v){
 let p=fraise::parse(Rule::prg,&s)
       .expect("tamere").next().unwrap();
 for x in p.into_inner(){
  match x.as_rule(){
   Rule::pop=>v.ins(ins::pop),
   Rule::num=>v.ins(ins::psh(x.as_str().parse::<i32>().unwrap())),
   Rule::swp=>v.ins(ins::swp),
   Rule::dup=>v.ins(ins::dup),
   Rule::add=>v.ins(ins::add),
   Rule::mul=>v.ins(ins::mul),
   Rule::sub=>v.ins(ins::sub),
   Rule::div=>v.ins(ins::div),
   Rule::eql=>v.ins(ins::eql),
   Rule::lth=>v.ins(ins::lth),
   Rule::gth=>v.ins(ins::gth),
   _=>(),
  }
 }
}
