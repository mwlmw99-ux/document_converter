use crate::{error::CoreError,model::{ConversionRequest,ConversionResult}};
pub trait ConversionEngine:Send+Sync{fn can_convert(&self,input:crate::format::FileFormat,target:crate::format::FileFormat)->bool;fn convert(&self,r:&ConversionRequest)->Result<ConversionResult,CoreError>;}
pub struct Registry{pub engines:Vec<Box<dyn ConversionEngine>>} impl Registry{pub fn new()->Self{Self{engines:Vec::new()}} pub fn register<E:ConversionEngine+'static>(&mut self,e:E){self.engines.push(Box::new(e));}}
