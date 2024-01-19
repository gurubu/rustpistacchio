#![allow(non_camel_case_types,dead_code)]
pub enum vhmi{
 pop,psh,swp,dup,
 add,sub,mul,div,
}
impl vhmi{
 pub fn matcha(self)->u8{
  match self{
   vhmi::pop=>0,vhmi::psh=>1,
   vhmi::swp=>2,vhmi::dup=>3,
   vhmi::add=>4,vhmi::sub=>5,
   vhmi::mul=>6,vhmi::div=>7,
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
