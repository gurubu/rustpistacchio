#![allow(non_camel_case_types,unused_imports)]
use sdl2::{Sdl,EventPump};
use sdl2::render::Canvas;
use sdl2::video::{Window,WindowContext};
use sdl2::pixels::Color;
use crate::vm::v;
pub struct sdl{
 pub cnv:Canvas<Window>,
 pub evt:EventPump,
 pub run:bool,
}
impl sdl{
 pub fn init()->Self{
  let sdl=sdl2::init().unwrap(); 
  let sub=sdl.video().unwrap();
  let win=match sub.window("pistasm",320,240)
                   .position_centered()
                   .build(){
                     Ok(win)=>win,
                     Err(err)=>panic!("window no")};
  let mut cnv=win.into_canvas().present_vsync().build().unwrap();
  let evt=sdl.event_pump().unwrap();
  cnv.set_draw_color(Color::RGB(0,0,0));
  cnv.clear();
  sdl{cnv,evt,run:true}
 }
 pub fn setclr(&mut self,r:u8,g:u8,b:u8){
  self.cnv.set_draw_color(Color::RGB(r,g,b));
 }
 pub fn clrcnv(&mut self){
  self.cnv.clear();
 }
 pub fn rdrcnv(&mut self){
  self.cnv.present();
 }
}
impl v{
 pub fn clr(&mut self){let r = self.ppr();let g = self.ppr();let b=self.ppr();
  self.sc.as_mut().unwrap().setclr(r as u8,g as u8,b as u8);
  self.sc.as_mut().unwrap().clrcnv();
  self.sc.as_mut().unwrap().rdrcnv();}
 
}

 

