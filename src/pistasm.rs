#![allow(non_camel_case_types)]
use pest::Parser;
use pest_derive::Parser;
use crate::vm::*;
#[derive(Parser)]
#[grammar="pistasm.pest"]
pub struct pistasm;
pub fn prs(s:&String,v:&mut v){
 let p=pistasm::parse(Rule::prg,&s)
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
   Rule::jmp=>{for y in x.clone().into_inner(){
    match y.as_rule(){
     Rule::num=>v.ins(ins::jmp(y.as_str().parse::<usize>().unwrap())),
     _=>(),
    }
   }},
   _=>v.ins(ins::nil),
  }
 }
}
pub fn iprs(s:&String,v:&mut v){
 let p=pistasm::parse(Rule::prg,&s.trim())
       .expect("tamere").next().unwrap();
 for x in p.into_inner(){
  match x.as_rule(){
   Rule::pop=>v.pop(),
   Rule::num=>v.psh(x.as_str().parse::<i32>().unwrap()),
   Rule::swp=>v.swp(),
   Rule::dup=>v.dup(),
   Rule::add=>v.add(),
   Rule::mul=>v.mul(),
   Rule::sub=>v.sub(),
   Rule::div=>v.div(),
   // Rule::pop=>println!("pop"),
   // Rule::num=>println!("num"),
   // Rule::swp=>println!("swp"),
   // Rule::dup=>println!("dup"),
   // Rule::add=>println!("add"),
   // Rule::mul=>println!("mul"),
   // Rule::sub=>println!("sub"),
   // Rule::div=>println!("div"),
      _=>(),
  }
 }
}
