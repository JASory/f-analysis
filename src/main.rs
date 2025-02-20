use f_analysis::{Epz,CompVector,cvec,bseq,WieferichPrime,Interval,Natural,Pseudoprime,BaseSeq,filter::{GenericFilter,Prime},PCGenerator};

/*
 
 weiferich base start stop
 fermat  base   start stop
 strong  base   start stop
 
 filter  file mode   
 
 base     start stop return fermat-bases
 hashtable start stop return hashtable

 start , stop can be replaced with a filename in and a filename out

*/
// Quadratic Residue filter
struct QR;
/*
impl GenericFilter for QR{
   fn filter_check<T: Natural>(x: T) -> bool{
     for i in T::ONE..T::from(10){
          if !x.sprp(i){
             return false;
          }
      }
      return true;
   }
}
*/
fn main(){

    let p = Epz::<2>::ONE;
    let q = Epz::<2>::ONE<<6;
    let mut count = 0u64;
    
   let k = PCGenerator::<Prime>::init(1u64<<32);
   for i in k.to_vector(){
      if i.is_prime(){
         count+=1;
      }
   }
    
    println!("{}",count);

      let env_var = std::env::args().collect::<Vec<String>>();
      
       let base = env_var[2].parse::<u64>().unwrap();
       let sup = env_var[3].parse::<u64>().unwrap();
       let intr = Interval::<u64>::new(2,sup);
      if env_var[1] =="wieferich"{
        println!("{}",intr.wieferich_search(base))
      }
      
      if env_var[1] =="fermat"{
        println!("{}",intr.generate_fermat_rt(base));
      }
      
      if env_var[1]=="strong"{
         let sprp = intr.generate_fermat_rt(base).filter_sprp_rt(base);
         println!("{}",sprp);
      }
      
/**/
/*
let intr = Interval::<u64>::new(2,1u64<<40);
let w = Interval::<u64>::new(2,1u64<<20);
/*
  for i in 257..65535{
      let p= w.wieferich_search(i);
      
      if p.prime_set().len() == 0{
      let mut b = BaseSeq::new(vec![p.base()]);
        b.set_strong_heuristic();
      let sprp = b.generate_pseudoprimes(2u64,1u64<<40,None).unwrap();
      //intr.generate_fermat_rt(i).filter_sprp_rt(i);
      if sprp.len() < 11800{
         println!("{} {}",i,sprp.len());
      }
      
      }
  }
  }
  
  10935 4796277
4860 4796599
760 5115683
12615 4798217
13500 4793216
40560 4796668
43740 4797964

37 17486138
10935 16732363
2 19762642

  */
  //for i in [2,15,52
//].iter(){ // 10935, 4860   12615: 221490  13500 222382, 40560 221780, 43740 222044


  //let mut b = BaseSeq::new(vec![10935]);
  //b.set_strong_heuristic();
  //let sprp = b.generate_pseudoprimes(2,u64::MAX,None).unwrap();
  //sprp.to_file("10935.kpseudo");  67947489  338069  614 318     98,148   5618,
  let sprp = cvec!["/home/jasory/misc/sprp2.bin";u64].load_to_memory().unwrap().filter_range(1u64<<63,u64::MAX).unwrap().filter_bvector(&bseq![3]);
  
  for i in sprp.iter().unwrap(){
     let k = i.factor().unwrap();
     println!("\n{}",k);
     for i in k.factors{
      let ord2 = i.p_ord(2);
      let ord3 = i.p_ord(3);
        println!("p: {} ord2 {} {} ord3 {} {}",i,ord2,ord2.trailing_zeros(),ord3,ord3.trailing_zeros());
     }
     //println!("{}",i.factor().unwrap());
  }
  //.filter_range(2,55245642489452).unwrap().filter_sprp_rt(3168915).filter_sprp_rt(38642);
  
  //println!("{}",sprp.len());
  
  //for i in 0..100{
  //let p = sprp.terminating_search();
  //let q = sprp.filter_bvector(&BaseSeq::new(vec![318, 614]));
  //println!("\n{:?}",p.unwrap());
  
  // (41, 148)

  //}
  */
}
