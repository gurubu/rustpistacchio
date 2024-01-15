#![allow(non_camel_case_types,dead_code)]
use std::fmt::Display;
#[derive(Clone,Copy)]
pub enum ins{
 pop,psh(i32),swp,dup,
 add,sub,mul,div,nil,
 jmp(usize),ji0(usize),ji1(usize)}
impl ins{
 pub fn disp(&self)->String{
  match self{
   Self::pop=>String::from("pop"),
   Self::psh(i)=>String::from(format!("psh {}",i)),
   Self::swp=>String::from("swp"),
   Self::add=>String::from("add"),
   Self::sub=>String::from("sub"),
   Self::mul=>String::from("mul"),
   Self::div=>String::from("div"),
   Self::dup=>String::from("dup"),
   Self::nil=>String::from("nil"),
   Self::jmp(u)=>String::from(format!("jmp {}",u)),
   Self::ji0(u)=>String::from(format!("ji0 {}",u)),
   Self::ji1(u)=>String::from(format!("ji1 {}",u)),
  }
}
}
pub struct v{
 pub ds:Vec<i32>,
 pub is:Vec<ins>,
 pub ip:usize}
impl v{
 pub fn new()->Self{Self{ds:vec![],is:vec![],ip:0}}
 pub fn top(&self)->i32{*self.ds.last().unwrap()}
 pub fn frm(&self)->usize{self.ip}
 pub fn psh(&mut self,i:i32){self.ds.push(i);}
 pub fn pop(&mut self){self.ds.pop();}
 pub fn por(&mut self)->i32{let r=self.top();self.pop();r}
 pub fn swp(&mut self){let r=self.por();let s=self.por();self.psh(r);self.psh(s);}
 pub fn dup(&mut self){self.psh(self.top())}
 pub fn add(&mut self){let l=self.por();let r=self.por();self.psh(l+r);}
 pub fn sub(&mut self){let l=self.por();let r=self.por();self.psh(l-r);}
 pub fn mul(&mut self){let l=self.por();let r=self.por();self.psh(l*r);}
 pub fn div(&mut self){let l=self.por();let r=self.por();self.psh(l/r);}
 pub fn prn(&self){println!("{:?}",self.ds)}
 pub fn ins(&mut self,i:ins){self.is.push(i);}
 pub fn nil(&mut self){}
 pub fn jmp(&mut self,u:usize){
  self.ip = u;
 }
 pub fn ji0(&mut self,u:usize){
  if self.top()==0{
   self.ip = u;
  }
 }
 pub fn ji1(&mut self,u:usize){
  if self.top()==1{
   self.ip = u;
  }
 } 
 pub fn matchins(&mut self,i:ins){
   match i{
   ins::pop=>self.pop(),
   ins::psh(i)=>self.psh(i),
   ins::swp=>self.swp(),
   ins::dup=>self.dup(),
   ins::add=>self.add(),
   ins::sub=>self.sub(),
   ins::mul=>self.mul(),
   ins::div=>self.div(),
   ins::nil=>self.nil(),
   ins::jmp(u)=>self.jmp(u),
   ins::ji0(u)=>self.ji0(u),
   ins::ji1(u)=>self.ji1(u),
  }
 }
 pub fn mins(&mut self){
  for x in self.is.clone().iter(){
   match x{
    ins::pop=>self.pop(),
    ins::psh(i)=>self.psh(*i),
    ins::swp=>self.swp(),
    ins::dup=>self.dup(),
    ins::add=>self.add(),
    ins::sub=>self.sub(),
    ins::mul=>self.mul(),
    ins::div=>self.div(),
    ins::nil=>self.nil(),
    ins::jmp(u)=>self.jmp(*u),
    ins::ji0(u)=>self.ji0(*u),
    ins::ji1(u)=>self.ji1(*u),
   }
  }
 }
 pub fn exe(&mut self){
  loop{
   self.matchins(self.is[self.ip]);
  }
 }
 pub fn pvec(&mut self,vi:&Vec<ins>){
  for x in vi.iter(){
   self.ins(*x);
  }
 }
 pub fn exvi(&mut self,v:&Vec<ins>){
  println!("exvi");
  for x in 0..v.len(){
   v[x].disp();
   self.matchins(v[x]);
  }
 }
}

