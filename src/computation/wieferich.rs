use crate::computation::compio::*;
use crate::Interval;

pub struct WieferichEval{
   base: u64,
   index: u64,
   lowerbound: u64,
   upperbound: u64,
   folder: String
}

impl WieferichEval{
   pub fn initialise(base: u64, lowerbound: u64, upperbound: u64, folder: &str){
      assert!(lowerbound<upperbound);
      std::fs::create_dir(folder);
      let basefile = folder.to_string()+"/base";
      write_single(base,&basefile);
      let boundfile = folder.to_string()+"/bound";
      let idxfile = folder.to_string()+"/index";
      write_single(lowerbound,&idxfile);
      write_bounds(lowerbound,upperbound,&boundfile);
      create_file(&(folder.to_string()+"/primes"));
   }
   
   pub fn recover(folder: &str) -> Self{
      let base = load_idx(&(folder.to_string()+"/base")).unwrap();
      let index = load_idx((&(folder.to_string()+"/index"))).unwrap();
      let (lowerbound,upperbound) = load_bounds(&(folder.to_string()+"/bound")).unwrap();
      Self{base,index,lowerbound,upperbound,folder: folder.to_string()}
   }
   
   pub fn run(&self){
      const STRIDE : u64 = 1u64<<28;
      let mut start = self.index;
      loop {
         let interval = Interval::<u64>::new(start,start+STRIDE);
         let wieferichs = interval.wieferich_search(self.base);
         if wieferichs.cardinality() != 0{
            let pstring = wieferichs.iter().map(|p|p.to_string()).collect::<Vec<String>>();
            let dataout = pstring.join("\n")+"\n";
            appender(&dataout,&(self.folder.clone()+"/primes"));
         }
         write_single(start,&(self.folder.clone()+"/index"));
         
         start+=STRIDE;

         if start >= self.upperbound{
           break;
         }
      }
      // Split the large computation over several intervals
      
   }  
        
  }
