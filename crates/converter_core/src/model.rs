use serde::{Deserialize,Serialize}; use crate::format::FileFormat;
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct ConversionRequest{pub input:String,pub output:String,pub target:FileFormat}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct ConversionResult{pub output:String,pub bytes_written:u64,pub warnings:Vec<String>}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)] pub enum JobStatus{Queued,Running,Success,Failed,Cancelled}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct ConversionJob{pub id:String,pub request:ConversionRequest,pub status:JobStatus,pub progress:u8,pub error:Option<String>}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct BatchPlan{pub jobs:Vec<ConversionRequest>}
