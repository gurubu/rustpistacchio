use std::fs;
use rustpistacchio::vm::*;
use rustpistacchio::pistasm::*;
use rustpistacchio::repl::*;
pub fn main(){
 // let mut r=rpl::new();
 let mut v=v::new();
 // while r.run{
 //  r.read();
 //  r.eval(&mut v);
 //  r.print(&mut v);
 // }
 let f=fs::read_to_string("pistest.pasm").unwrap();
 prs(&f,&mut v);
 v.exe();
 println!("{:?}",v.ds);
}
