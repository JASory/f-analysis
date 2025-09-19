/*

    Search algorithms applied to vectors, this simply compartmentalises the algorithms used for CompVector

    split into Parallel, Single-thread, running over hashbuckets, and searching for hash multipliers

*/

pub(crate) mod hash;
pub(crate) mod parallel;
pub(crate) mod sectored;
pub(crate) mod single;

pub(crate) use hash::hash_search;
pub(crate) use parallel::*;
pub(crate) use sectored::*;
pub(crate) use single::*;
