use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct AlignmentArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
    /// please provide the weight for the gapped sites
    pub gapped_arg: usize,
    /// please provide the weight for the mismatch sites
    pub mismatch_arg: usize,
    /// please provide the weight for the matched sites
    pub match_arg: usize,

}
