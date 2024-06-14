use f_analysis::fermat::IntSeq;
use f_analysis::filter::{Base,EPF};
use f_analysis::{CompVector, Interval};


/*

   Note that the vast majority of this example is unnecessary, it simply demonstrates various functions

*/

fn main() {
    /*
    panic!("
       Read this message - To see an example of the hash computation do the following 
       
       1. Download Feitsma-Galways pseudoprime table from www.cecm.sfu.ca/Pseudoprimes/psps-below-2-to-64.txt.bz2
       2. Unzip it and copy it to the examples folder (the same as this file), without renaming it
       3. Read the source code here for an alternative, if this is too complex or the page no longer exists
       4. Comment out this panic error
       5. Execute using cargo run --release --example hashtable
       6. Wait approximately 30k seconds divided by the number of cores you have available
       
    ");
    */
    let file = "/home/jasory/psps-below-2-to-64.txt";
    
    
    // Initialise new CompVector, normally one would use from_vector or from_file 
    // except we need to read a utf8 file so we have to set the flag before we read the file
    let mut ce = CompVector::<u64>::new();
    // Read from a utf8 file versus the default of binary
    ce.set_utf8();
    // Assign file to CompVector were we will load it to memory
    ce.set_file(file);
    // Load to memory
    let ce2 = ce.load_to_memory().unwrap();

    // Reduce the set to Euler-Plumb pseudoprimes
    // the euler-plumb filter currently only uses filter_generic
    let mut epseudo = ce.filter_generic::<EPF>(None).unwrap();
    
    // Epseudo writes to utf-8 file, this is easy  to read in pagers
    // This function actually returns another CompVector handling this file, 
    //but we already have the values in memory so we can discard it
    let _discard = epseudo.to_file("epf.txt").unwrap();
    
    epseudo.set_binary();
    // Write the reduced set to a binary file for much faster future evaluations
    let _discard = epseudo.to_file("epf.bin").unwrap();
    
    // Compute hashtable, this is the most intensive part, it will use all available cores and may take several hours 
    // Note that this is identical to to_hashtable(Some(262144),Some(1276800789),Some(65535))
    // If you omit the middle argument (the multiplier), it will generate a pseudorandom multiplier, with a different set of bases
    // However this current configuration will always produce the same hashtable
    let hashtable = epseudo.compute_hashtable(Some(262144),Some(1276800789),None).unwrap(); 
    
    // Uncomment line below to construct a hashtable with 262144 but a different multiplier
    // let hashtable = epseudo.compute_hashtable(Some(262144),None,None).unwrap();
    
    // Uncomment line below to construct a hashtable with the most convenient values 
    // let hashtable = epseudo.compute_hashtable(None,None,None).unwrap();
    
    // Proves that the hashtable is in fact deterministic against the composite vector it was constructed against. 
    // In practice this will almost surely never fail, excepting hardware failure. However it has application in 
    // testing if it is deterministic against other composite sets
    if epseudo.filter_hashtable(&hashtable).len() !=0{
       panic!("Oh, no a silent error prevented us from producing a deterministic hashtable. Report to https://github.com/JASory/f-analysis/issues immediately")
    }
    
    // Now we write it, this will include the hashfunction used
    hashtable.to_file("epf-64.ht");
    
    // And we print it, your terminal may not support printing the table as large as this 
    // so it was written to file before hand to make sure it wasn't wasted computation
    println!("{} \n And written to hashtable file in hashtable.txt",hashtable);
    
    println!("\n You have now replicated machine-prime's hashtable");
    
}
