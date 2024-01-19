#![allow(dead_code,non_camel_case_types)]
use pest_derive::Parser;
use crate::vm::*;
pub enum dir{l,r}
#[derive(Parser)]
#[grammar="fange.pest"]
pub struct fange{
 siz:usize,
 dir:dir,
}
impl fange{
 pub fn init(s:usize)->Self{Self{siz:s,dir:dir::r}}
 pub fn mv(&mut self,vm:&mut v){
  match self.dir{
   dir::r=>vm.ip+=1,
   dir::l=>vm.ip-=1,
  } 
 }
}
