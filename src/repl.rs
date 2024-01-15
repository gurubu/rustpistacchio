use std::io::*;
use crate::pistasm::*;
use crate::vm::*;
pub struct rpl{
 pub rd:String,
 pub run:bool,
 pub stdin:Stdin,
}
impl rpl{
 pub fn new()->Self{
  Self{rd:String::new(),run:true,stdin:stdin()}
 }
 pub fn read(&mut self){
  self.stdin.read_line(&mut self.rd).unwrap();
 }
 pub fn eval(&mut self,v:&mut v){
  iprs(&self.rd,v);
  self.rd.clear();
 }
 pub fn print(&mut self,v:&mut v){
  v.prn();
 }
}
