#![allow(non_camel_case_types,dead_code)]
use std::{fmt::Display, collections::HashMap};
use crate::gfx::*;
pub struct llv{
 pub ds:[u8;16],
 pub ti:[u8;16],
}
impl llv{
 pub fn inittbl(&mut self){}
}

pub struct v{
 pub ds:Vec<i32>,
 pub is:Vec<ins>,
 pub ip:usize,
 pub rn:bool,
 pub lb:HashMap<String,usize>,
 pub sc:Option<sdl>,
}
#[derive(Clone,Copy)]
pub enum ins{
 pop,psh(i32),swp,dup,
 add,sub,mul,div,nil,
 jmp(usize),ji0(usize),ji1(usize),
 jie(usize),jig(usize),jil(usize),
 eql,lth,gth,
 hlt, 
 scr,
}
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
   Self::jie(u)=>String::from(format!("jie {}",u)),
   Self::jig(u)=>String::from(format!("jig {}",u)),
   Self::jil(u)=>String::from(format!("jil {}",u)),
   Self::eql=>String::from("eql"),
   Self::lth=>String::from("eql"),
   Self::gth=>String::from("eql"),
   Self::hlt=>String::from("hlt"),
   Self::scr=>String::from("scr"),
  }
 }
}
pub fn empty(){println!("not enough cell on stack!");}
impl v{
 pub fn new(dsp:bool)->Self{
  match dsp{
   true=>Self{ds:vec![],is:vec![],ip:0,
              rn:true,lb:HashMap::new(),
              sc:Some(sdl::init())},
   false=>Self{ds:vec![],is:vec![],ip:0,
              rn:true,lb:HashMap::new(),
              sc:None}}
 }
 pub fn stklen(&self)->usize{self.ds.len()}
 pub fn chkstk(&self,n:usize)->bool{if self.stklen()<n{false}else{true}}
 pub fn top(&self)->i32{*self.ds.last().unwrap()}
 pub fn sno(&self)->i32{self.ds[self.ds.len()-2]}
 pub fn rmn(&self)->i32{self.ds[self.ds.len()-3]}
 pub fn frm(&self)->usize{self.ip}
 pub fn psh(&mut self,i:i32){self.ds.push(i);}
 pub fn pop(&mut self){if self.chkstk(1){self.ds.pop();}else{empty();}}
 pub fn ppr(&mut self)->i32{self.ds.pop().unwrap()}
 pub fn swp(&mut self){let r=self.ppr();let s=self.ppr();self.psh(r);self.psh(s);}
 pub fn dup(&mut self){self.psh(self.top())}
 pub fn add(&mut self){let l=self.ppr();let r=self.ppr();self.psh(l+r);}
 pub fn sub(&mut self){let l=self.ppr();let r=self.ppr();self.psh(l-r);}
 pub fn mul(&mut self){let l=self.ppr();let r=self.ppr();self.psh(l*r);}
 pub fn div(&mut self){let l=self.ppr();let r=self.ppr();self.psh(l/r);}
 pub fn prn(&self){println!("{:?}",self.ds)}
 pub fn ins(&mut self,i:ins){self.is.push(i);}
 pub fn nil(&mut self){}
 pub fn lbl(&mut self,l:(String,usize)){self.lb.insert(l.0,l.1);}
 pub fn jmp(&mut self,u:usize){self.ip = u-1;}
 pub fn ji0(&mut self,u:usize){if self.top()==0{self.ip = u-1;}}
 pub fn ji1(&mut self,u:usize){if self.top()==1{self.ip = u-1;}} 
 pub fn jie(&mut self,u:usize){if self.top()==self.sno(){self.ip = u-1;}}
 pub fn jig(&mut self,u:usize){if self.top()>self.sno(){self.ip = u-1;}}
 pub fn jil(&mut self,u:usize){if self.top()<self.sno(){self.ip = u-1;}}
 pub fn eql(&mut self){if self.top()==self.sno(){self.psh(1)}else{self.psh(0)}}
 pub fn lth(&mut self){if self.top() <self.sno(){self.psh(1)}else{self.psh(0)}}
 pub fn gth(&mut self){if self.top() >self.sno(){self.psh(1)}else{self.psh(0)}}
 pub fn hlt(&mut self){self.rn=false;}
 pub fn quit(&mut self){self.rn=false;}
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
   ins::jie(u)=>self.jie(u),
   ins::jig(u)=>self.jig(u),
   ins::jil(u)=>self.jil(u),
   ins::eql=>self.eql(),
   ins::lth=>self.lth(),
   ins::gth=>self.gth(),
   ins::hlt=>self.hlt(),
   ins::scr=>self.clr(),
  }
 }
  pub fn exe(&mut self){
  loop{
   println!("{}",self.is[self.ip].disp());
   self.matchins(self.is[self.ip]);
   if self.rn == false{
    break;
   }
   println!("{:?}",self.ds);
   if self.ip+1<self.is.len(){
    self.ip+=1;
   }
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
