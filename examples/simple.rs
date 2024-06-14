use f_analysis::fermat::{IntSeq,FInteger};
use f_analysis::filter::{DBase,Base,SFSqr,SPK};
use f_analysis::{CompVector, Interval,BaseVector,HashTable};

fn main() {
    // Generates the base 15 pseudoprimes
   // let semiprimes = Interval::new(0usize, 1usize << 32).generate_fermat::<Base<15>>();
    // Counts the number of pseudoprimes for each base between 100 and 250
   // let base_eval = semiprimes.sprp_eval::<IntSeq<u64>>(Some(100), 150).unwrap();
    // Selects the top 10 bases
   // let strong_base = base_eval.lower_interval(10).unwrap();
   /*
   let mut x = Interval::new(3,1u64<<40);
   x.set_weak_heuristic();
   let z = x.compute_heuristic(None).unwrap().filter_sprp::<Base<2>>(None);
   //println!("Calculated heuristic");
   let q = CompVector::<u64>::from_file("/home/jasory/sprp2-40.bin").unwrap().load_to_memory().unwrap();
   //let t = q.to_file("/home/jasory/sprp2-.bin");
   println!("Computed heuristic");
   for i in 0..100{
      let fermat_base = z.k_rand(3);
      let res = q.filter_bvector(&fermat_base);
      let ce_count = res.len();
      if ce_count == 0{
         println!("{:?}", fermat_base);
      }
      else{
        println!("Failed with {} counterexamples {}", ce_count,res )
      }
      // filter q with fermatbase
//      println!("{:?}",z.k_5(3))
      
   }
   */
   /*
   let mut ce_64 = Interval::new(2,1u64<<32);
   ce_64.set_strong_heuristic();
   let x = ce_64.compute_heuristic(None).unwrap();
   let x128 = x.to_vector().iter().map(|y| *y as u128).collect::<Vec<u128>>();    let mut f = File::open("foo.txt")?;
   */
   //let mut ce_128 = Interval::new(2,1u128<<32);
   //ce_128.set_strong_heuristic();
   
   /*
   let ht = CompVector::<u64>::from_file("heuristic.bin").unwrap().load_to_memory().unwrap().to_hashtable(Some(262144),Some(1565855935),None).unwrap();//.filter_sprp::<Base<15>>(Some("heuristic-66-15.bin"));
   //ht.to_file("ht-66");
   let sprp2 = CompVector::<u64>::from_file("/home/jasory/sprp2.bin").unwrap().load_to_memory().unwrap();
   let htce = sprp2.filter_hashtable(&ht); //ht.list_failure(&sprp2);
   
   for _ in 0..1000{
   println!("{:?}",htce.bs_rand());
   }
   
   println!("{}",htce.len());
   */
   
  // let bv = BaseVector::<u128>::new(vec![60,3757,7252,168326,1050231]);
  // let ce = CompVector::<u128>::from_file("heuristic-66.bin").unwrap();
  // let bvce = ce.filter_bvector(&bv);
  // println!("{}",bvce.len());
    //let p = HashTable::from_file("ht-66").unwrap();
  //p.to_file("heuristic-test.ht").unwrap();
  //let sprp2 = CompVector::<u64>::from_file("heuristic.bin").unwrap().filter_sprp::<Base<60>>(None).unwrap();
  
  //let mut s = Interval::new(2,u64::MAX);
  //s.set_weak_heuristic();
  //let z = s.compute_heuristic(Some("weak-64.bin")).unwrap();
  
  //let sprp2 = CompVector::<u64>::from_file("/home/jasory/sprp2.bin").unwrap();
  /*
  let five = CompVector::<u64>::from_file("/home/jasory/fbase/3fermat.bin").unwrap().load_to_memory().unwrap().filter_sprp::<Base<3>>(None).unwrap();//.filter_sprp::<Base<5>>(None).unwrap();
 // println!("{}",five);
  
  for i in five.iter().unwrap(){
     if i%2 == 0{
       println!("{}",i)
     }
  }
  */
  //let three = CompVector::<u64>::from_file("/home/jasory/fbase/3fermat.bin").unwrap().filter_sprp::Base<3>(None).unwrap();
  
  //println!("{}",z);
 /*
  let mut z = CompVector::<u64>::new();
  
  z.set_utf8();
  z.set_file("test.txt");
 // println!("{}",z.terminating_search());
  //let mut q = z.load_to_memory().unwrap();
  z.reverse();
 // println!("{}",z);
  println!("{}",z);
  */
  let start = std::time::Instant::now();
  let mut q = CompVector::<u64>::new();
  //q.set_utf8();
 // q.set_memory_max(1<<31);
  q.set_file("epf.bin");
  let sprp2 = q.load_to_memory().unwrap().compute_hashtable(Some(262144),Some(1276800789),None).unwrap();//unwrap().filter_generic::<SFSqr<2>>(None).unwrap().filter_generic::<SFSqr<3>>(None).unwrap().filter_generic::<SFSqr<4>>(None).unwrap();
  // 2- 1009858
  // 3- 1129942
  // 9375-1147253   // 31894014
  // 5-130740
  // 6-2080281
  sprp2.to_file("htsample").unwrap();
  println!("{:?}",start.elapsed()); // 1147899
  // 86328
//  z.sort();
  // B: 5887 V: 4825
//  println!("{}",z);
  
  
  //println!("{}",p.strongest_search(3,10000));
  //let mut count = 0;
  /*
  let start= std::time::Instant::now();
  for _ in 0..100{
  for i in z.iter().unwrap(){
     if i.sprp(3){
       count+=1;
     //  println!("{}",i)
     }
  }
  }
  let stop = start.elapsed();
  println!("{} {} {:?}",z.len(),count, stop);
  */
  //let z = Interval::new(2u64,1u64<<32).generate_fermat::<Base<3>>();
  
  //println!("{}",z);
  //ht.to_file("weak.ht").unwrap();
  
  /*
  //let htce = sprp2.filter_hashtable(&p);
  let ht = sprp2.to_hashtable(Some(32768),None,None).unwrap();
  let z = CompVector::<u64>::from_file("/home/jasory/sprp2.bin").unwrap().filter_sprp::<Base<60>>(None).unwrap();
  let htce = z.filter_hashtable(&ht);
  
  for i in 0..1000{
    println!("{}",htce.bs_rand());
  }
  */
  
  /*
  let parser = |x: &str| -> u64{
      let z = x.chars().filter(|k| k.is_digit(10)).collect::<String>();
      z.parse::<u64>().unwrap()
  };
  
  let splitter = |x: &str| -> Vec<u64>{
      let mut z = x.split(",").collect::<Vec<&str>>();
      z.pop();
      z.iter().map(|y| parser(y)).collect::<Vec<u64>>()
  };
  */
  //let x = "345".parse::<u64>().unwrap();
  
 // println!("{:?}",splitter(" 345 ,5, 56,3 ,3"));
  // let sprp2 = CompVector::<u64>::from_file("/home/jasory/sprp2.bin").unwrap();
  // let k = sprp2.filter_bvector(&bv);
   //bv.to_file("test.bv");
   //let k = BaseVector::<u64>::from_file("test.bv").unwrap();
   
  // println!("{}",bvce.bs_terminating());
   //let mut alpha = CompVector::<u128>::from_file("heuristic-66.bin").unwrap().load_to_memory().unwrap();
   //alpha.set_utf8();
   //alpha.sort();
   //let zepto = alpha.to_file("heuristic.txt").unwrap();
   //let bv = alpha.bs_iterative();
   //println!("{}",bv);
   //let bv = BaseVector::<u64>::new(vec![14861911291535489797, 15203930009347939473, 17604762304033425616, 17455090361288236179, 16000734881807193968, 12962519004447869438, 17896445244787537814, 11053029953967210778, 17223690017368487110]);
   //let ce = alpha.filter_bvector(&bv);
   //println!("{} {}",ce,ce.len())
   //let y = ce_128.compute_heuristic(Some("test2.bin")).unwrap();
   //let z = CompVector::<u128>::from_file("test2.bin").unwrap().load_to_memory().unwrap();//y.load_to_memory().unwrap();
   //let t = y.to_vector();
   /*
   for i in t{
     let p = i.to_bytes();
     let q = u128::from_bytes(&p[..]);
     if i != q{
       println!("{} /= {}",i,q);
     }
   }
   */
   //println!("{}",t == x128);
   //let cefile = CompVector::<u64>::from_file("heuristic.bin").unwrap();//.unwrap().load_to_memory().unwrap().filter_range(2,1u128<<64);
   // println!("{}",cefile.len());    let mut f = File::open("foo.txt")?;
   /*
   let mut t = BaseVector::<u128>::new(vec![2]);
  // println!("{} {}",t.primality(561),t.primality(2047));
   t.set_strong_heuristic();
   let start = std::time::Instant::now();
   // t: 4184s
   let p = t.generate_pseudoprimes(2,1u128<<66,Some("heuristic-66-3.bin")).unwrap();
   let stop = start.elapsed();
   println!("{} counterexamples computed in {:?}",p.len(),stop);
   */
   
   
  
   //let ce = CompVector::<u128>::from_file("heuristic-66-3.bin").unwrap().load_to_memory().unwrap();
  // ht.to_file("heuristic-65.ht").unwrap();
   //println!("{:?}",ce.bs_rand());
  /*
   let cefile = CompVector::<u64>::from_file("heuristic.bin").unwrap();//.load_to_memory().unwrap().filter_range(2,1u128<<66);
   //let ce = CompVector::<u128>::from_file("heuristic-66-15.bin").unwrap().load_to_memory().unwrap();
   println!("{}",cefile.len());
   let ce = cefile.load_to_memory().unwrap().filter_range(2,u64::MAX);
   println!("{}",ce.len());
   //let _ = ce.to_file("heuristic-66-15.bin").unwrap();
   //let sprp2 = CompVector::<u64>::from_file("/home/jasory/sprp2.bin").unwrap().load_to_memory().unwrap().filter_sprp::<Base<15>>(None);
   //let interim = sprp2.to_vector().iter().map(|x| *x as u128).collect::<Vec<u128>>();
   //let sprp15 = CompVector::<u128>::from_vector(interim);
   //let sprp15 = CompVector::<u128>::from_file("sprp15-128.bin").unwrap().load_to_memory().unwrap();
   //let _ = sprp15.to_file("sprp15-128.bin").unwrap();
   //let total = sprp15.len();
   //println!("{}",total);
   //println!("{}",(ce.len() as f64/total as f64));
   
   let ht = ce.to_hashtable(Some(262144),None,None).unwrap();
   ht.to_file("heuristic-66.ht").unwrap();
   let htce = ht.list_failure(&sprp15);
   */
   //for i in 0..100{
   //   println!("{:?}",htce.bs_rand());
   //}
   /*
   let mut steps = 0u64;
   loop {
       steps+=1;
       let ht = ce.to_hashtable(Some(32768),None,None).unwrap();
   //    ht.to_file("heuristic2.ht").unwrap();
       println!("Hashtable computed");
       let mut counter = 0u64;
       let htce = ht.list_failure(&sprp2);
       
       for i in 0..10_000{
      // println!("{}",i);
       let b = BaseVector::<u64>::rand_initialise(4);//u64::gen_k(64).unwrap();
       let fin = htce.filter_bvector(&b);
       
       if fin.len() != 0{
          counter+=1;
         println!("{:?} {}",b,fin.len());
       }
   }
     //  errors.to_file("htres2.bin");
     //  let ratio = errors as f64/total as f64;
       println!("iteration {} errors{}",steps, counter);
   }
   
   */
   
   /*
   let mut count = 0u64;
   let htce = CompVector::<u64>::from_file("htres2.bin").unwrap();
   for i in 0..1000_000{
   //let bv = htce.bs_rand();
   let b = u64::gen_k(64).unwrap();
   let fin = htce.sprp_ce(b);
   if fin.len() != 0{
      count+=1;
      println!("{}",b);
   }
   //println!("{} {:?} {}",htce.len(),bv,fin.len());
   }
   println!("{}",count);
   */
  //  println!("{}",z.element_count() );
    // 129958 2^35
    // 229116       2^37
    // 544578       2^40
}
