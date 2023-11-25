use f_analysis::fermat::IntSeq;
use f_analysis::filter::{Base,EPF};
use f_analysis::{CompVector, Interval};


/*

   Note that the vast majority of this example is unnecessary, it simply demonstrates various functions

*/

fn main() {
    
    panic!("
       Read this message - To see an example of the hash computation do the following 
       
       1. Download Feitsma-Galways pseudoprime table from www.cecm.sfu.ca/Pseudoprimes/psps-below-2-to-64.txt.bz2
       2. Unzip it and copy it to the examples folder (the same as this file), without renaming it
       3. Read the source code here for an alternative, if this is too complex or the page no longer exists
       4. Comment out this panic error
       5. Execute using cargo run --release --example hashtable
       6. Wait approximately 30k seconds divided by the number of cores you have available
       
    ");
    
    let file = "examples/psps-below-2-to-64.txt";
    
    // Reads from utf-8 file, checking that each number is composite, 
    // this will fail if any primes exist or the file cannot be read
    let ce = CompVector::<u64>::read_utf8(file).unwrap();
    // Alternate simpler example, this simply generates about half of the sprps less than 2^60
    // let ce = Interval::new(2,1u64<<60).generate_ce(64);
    let ce_len = ce.len();
    
    let psp2 = ce.filter_fermat::<Base<2>>();
    
    // Ensures that the composites are all base-2 psps, skip this check if the simpler option is taken
    assert_eq!(ce_len,psp2.len());
    
    // Reduce the set to Euler-Plumb pseudoprimes
    let epseudo = psp2.filter_fermat::<EPF>();
    
    // Write the reduced set to a binary file for much faster future evaluations
    epseudo.write_binary("epf.bin");
    
    // Write to readable text
    epseudo.write_utf8("epf.txt");
    
    // Compute hashtable, this is the most intensive part, it will use all available cores and may take several hours 
    // Note that this is identical to to_hashtable(Some(262144),Some(1276800789),Some(65535))
    // If you omit the middle argument (the multiplier), it will generate a pseudorandom multiplier, with a different set of bases
    // However this current configuration will always produce the same hashtable
    let hashtable = epseudo.to_hashtable(None,Some(1276800789),None).unwrap(); 
    
    // Proves that the hashtable is in fact deterministic against the composite vector it was constructed against. 
    // In practice this will almost surely never fail, excepting hardware failure. However it has application in 
    // testing if it is deterministic against other composite sets
    if !hashtable.prove(&epseudo){
       panic!("Oh, no a silent error prevented us from producing a deterministic hashtable. Report to https://github.com/JASory/f-analysis/issues immediately")
    }
    
    // Now we write it, this will include the hashfunction used
    hashtable.write_utf8("hashtable.txt");
    
    // And we print it, your terminal may not support printing the table as large as this 
    // so it was written to file before hand to make sure it wasn't wasted computation
    println!("{} \n And written to hashtable file in hashtable.txt",hashtable);
    
    println!("\n You have now replicated machine-prime's hashtable");
    
}
