use f_analysis::fermat::IntSeq;
use f_analysis::filter::Base;
use f_analysis::{CompVector, Interval};

fn main() {
    // Generates the base 15 pseudoprimes
    let semiprimes = Interval::new(0usize, 1usize << 32).generate_fermat::<Base<15>>();
    // Counts the number of pseudoprimes for each base between 100 and 250
    let base_eval = semiprimes.sprp_eval::<IntSeq<u64>>(Some(100), 150).unwrap();
    // Selects the top 10 bases
    let strong_base = base_eval.lower_interval(10).unwrap();

    println!("{}", strong_base);
}
