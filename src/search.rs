/*
  
    Search algorithms applied to vectors, this simply compartmentalises the algorithms used for CompVector
    
    split into Parallel, Single-thread, running over hashbuckets, and searching for hash multipliers

*/

pub(crate) mod parallel;
pub(crate) mod single;
pub(crate) mod sectored;
pub(crate) mod hash;

pub(crate) use parallel::*;
pub(crate) use single::*;
pub(crate) use sectored::*;
pub(crate) use hash::hash_search;
