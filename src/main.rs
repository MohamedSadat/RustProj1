use std;
use std::arch::x86_64::_mm_stream_sd;
use std::string;
use Sales::CustModel;
//use Sales;
mod Sales;

   
fn main() {
    let mut s = String::from("hello");
let s1 = &s;
    println!("{}",s);
    println!("{}",s1);
let s2=getstr(String::from("Hello"));
println!("{}",s2);

let mut cust =CustModel::customer{name:String::from("Engy"),id:5,haserror:false};
//cust.ResetCust();
let ecust = cust.build();
//let mut emptycust = customer.build();

println!("{:#?}",cust);
println!("{:#?}",ecust);
cust.CopyFrom(&ecust);
println!("After copy");
println!("{:#?}",cust);

}

fn getstr(name:String)->String
{
 let mut s= String::from("Sadat");
 s.push_str(name.as_str());
return  s;
}
