#![allow(non_camel_case_types,dead_code)]
pub enum vhmi{
 pop,psh,swp,dup,
 add,sub,mul,div,
}
impl vhmi{
 pub fn matcha(self)->u8{
  match self{
   vhmi::pop=>0x0,vhmi::psh=>0x1,
   vhmi::swp=>0x2,vhmi::dup=>0x3,
   vhmi::add=>0x4,vhmi::sub=>0x5,
   vhmi::mul=>0x6,vhmi::div=>0x7,
  }
 }
}
pub struct vhms{
 ds:[i8;256],
 it:[u8;256],
 tr:u8,
 pc:u8,
}
impl vhms{
 pub fn new()->Self{vhms{ds:[0;256],it:[0;256],tr:0,pc:0}}
}
