//use f_analysis::fermat::IntSeq;
//use f_analysis::filter::Base;
use f_analysis::{fermat::{FInteger,FIterator,CompSeq,IntSeq,IntRng},CompVector,Interval,Primes};



fn main() {
   let mut p = Interval::new(1,u64::MAX);

   let start = std::time::Instant::now();
   let q = p.fq_sequence::<IntSeq<u64>>(IntSeq::<u64>::new(Some(1),10).unwrap());
   let stop = start.elapsed();
   //for i in q{
   //  println!("{:?}",i);
   //}
//let stop = start.elapsed();
   println!("{:?}",stop);

/*
  let mut pcount = 0u64;
  let mut failure = 0u64;
   for j in q.iter(){
     if !j.is_prime(){
       failure+=1;
       println!("failed {}",j);
     }
     pcount+=1;
     //println!("{}",j);  
   }
   
   println!("{} {}",pcount, failure);
   */
//   for i in q.iter(){
//     println!("{}",i);
//   }
   //let q = P::new(20771*2);//optimized_sieve(30000);
   //for i in q.iter(){
   //if i == 20771{
   //  println!("Found");
   //  }
   //}
   
   //let q =p.fermat_quotient(5);
   //println!("{:?}",q);
   /*
   


   
   */
   
   //for i in q.iter(){
   //if i == 29131{
   //  println!("Help");
   //}
    // println!("{}",i);
   //}
   //p.set_weak_heuristic();
   //let p = Composite::<u64>::from_file("/home/jasory/sprp2.bin").unwrap();
   //for i in 2..100{
   //let q = p.fermat_quotient(i);//p.rand_base();
   //  println!("{:?}",q);
   //}
    // Generates the base 15 pseudoprimes
   // let semiprimes = Interval::new(0usize, 1usize << 32).generate_fermat::<Base<15>>();
    // Counts the number of pseudoprimes for each base between 100 and 250
   // let base_eval = semiprimes.sprp_eval::<IntSeq<u64>>(Some(100), 150).unwrap();
    // Selects the top 10 bases
    //let strong_base = base_eval.lower_interval(10).unwrap();

    //println!("{}", strong_base);
}
