pub mod CustModel
{
    #[derive(Debug)]
pub struct customer{
   pub name:String,
   pub id:i32,
   pub haserror:bool
}
impl customer{
  pub  fn build(&self)->customer{
        customer { name: String::from(""), id: 0, haserror: false }

    }
   pub fn ResetCust(&mut self){
        self.name = String::from("_");
        self.id=0;
        self.haserror=false;
    }
   pub fn CopyFrom(&mut self , source:&customer){
        self.name=String::from(source.name.as_str());
        self.id=source.id;
        self.haserror=source.haserror;
    }
   pub fn ToString(&mut self)->String{
        let mut r = String::from(self.name.as_str());
//r.push_str(self.id.to_string());

        return r;
    }
}

}