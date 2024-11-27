# rust-deeplearning-alignment

 - vector preparation for deep learning for masked alignment prediction
 - given an alignment and weights that you want to propagate for the alignment, it gives you the build in tensor for the given alignment for feed into the neural network.

 ```
 cargo build

 ```
  - to run and build the vectors for import into the neural network
 ```
  λ gauravsablok rust-deeplearning-alignment → λ git master* → ./target/debug/rust-view-aln-form -h
  Usage: rust-view-aln-form <ALIGNMENT_ARG> <GAPPED_ARG> <MISMATCH_ARG> <MATCH_ARG>

  Arguments:
  <ALIGNMENT_ARG>  please provide the path to the alignment file
  <GAPPED_ARG>     please provide the weight for the gapped sites
  <MISMATCH_ARG>   please provide the weight for the mismatch sites
  <MATCH_ARG>      please provide the weight for the matched sites

 Options:
  -h, --help     Print help
  -V, --version  Print version
```

 Gaurav Sablok
