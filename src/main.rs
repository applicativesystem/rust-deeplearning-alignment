mod args;
use args::AlignmentArgs;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-27
*
* this crate allows you to prepare the vector and the tensor for the deep learning, which
* allows you to input the weights for the three types such as the matched sites, mismatched sites
* and the gapped sites.
*
*
*
* */

fn main() {
    let args = AlignmentArgs::parse();
    println!("The vector for the matched sites have been written");
    let matched:String = alignment_matched(&args.alignment_arg, args.match_arg).unwrap();
    println!("The vector for the mismatch sites have been written: {}", &matched);
    let mismatched: String = alignment_mismatched(&args.alignment_arg, args.mismatch_arg).unwrap();
    println!("The vector for the gapped sites have been written: {}", &mismatched);
    let gapped:String = alignment_gapped(&args.alignment_arg, args.gapped_arg).unwrap();
}

fn alignment_matched(path: &str,  matchedweight:usize) -> Result<String, Box< dyn Error>> {

  #[derive(Debug, Clone)]
  struct DeepLearn {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(path).expect("file not found");
  let fileread = BufReader::new(&fileopen);
  let mut embedded_hold:Vec<DeepLearn> = Vec::new();
  let mut hold_header:Vec<String> = Vec::new();
  let mut hold_sequence:Vec<String> = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not present");
    if line.starts_with(">"){
      hold_header.push(line);
    } else {
      hold_sequence.push(line);
    }
  }
  for i in 0..hold_header.len(){
    embedded_hold.push(DeepLearn{
      header: hold_header[i].clone(),
                       sequence: hold_sequence[i].clone(),
    })
  }
  let mut finalholdseq_multivector = Vec::new();
  let mut finalholdid_multivector:Vec<String> = Vec::new();
  for i in 0..hold_header.len(){
    let mut intermediatehold = Vec::new();
    for j in hold_sequence[i].chars(){
      intermediatehold.push(j);
    }
    finalholdseq_multivector.push(intermediatehold);
    finalholdid_multivector.push(hold_header[i].clone());
  }

  let mut deeplearn_vector_multivector:Vec<Vec<_>> = Vec::new();

  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
       let mut vectoralgebrahold:Vec<usize> = Vec::new();
        if finalholdseq_multivector[i][j].to_string() == "A"
              && finalholdseq_multivector[i+1][j].to_string() == "A" {
              vectoralgebrahold.push(matchedweight);
            deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "T"
           && finalholdseq_multivector[i+1][j].to_string() == "T" {
          vectoralgebrahold.push(matchedweight);
          deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "G"
          && finalholdseq_multivector[i+1][j].to_string() == "G" {
          vectoralgebrahold.push(matchedweight);
          deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "C"
          && finalholdseq_multivector[i+1][j].to_string() == "C" {
          vectoralgebrahold.push(matchedweight);
          deeplearn_vector_multivector.push(vectoralgebrahold);
    }
  }
}
 let mut matched_file = File::create("matched-alignment-vectors.txt").expect("file not found");
 for i in deeplearn_vector_multivector.iter_mut(){
   write!(matched_file,"{:?}", i).expect("file not present")
 }

Ok("alignment vector have been generated".to_string())
}


fn alignment_mismatched(path: &str, mismatchweight: usize) -> Result<String, Box<dyn Error>> {

   #[derive(Debug, Clone)]
  struct DeepLearn {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(path).expect("file not found");
  let fileread = BufReader::new(&fileopen);
  let mut embedded_hold:Vec<DeepLearn> = Vec::new();
  let mut hold_header:Vec<String> = Vec::new();
  let mut hold_sequence:Vec<String> = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not present");
    if line.starts_with(">"){
      hold_header.push(line);
    } else {
      hold_sequence.push(line);
    }
  }
  for i in 0..hold_header.len(){
    embedded_hold.push(DeepLearn{
      header: hold_header[i].clone(),
                       sequence: hold_sequence[i].clone(),
    })
  }
  let mut finalholdseq_multivector = Vec::new();
  let mut finalholdid_multivector:Vec<String> = Vec::new();
  for i in 0..hold_header.len(){
    let mut intermediatehold = Vec::new();
    for j in hold_sequence[i].chars(){
      intermediatehold.push(j);
    }
    finalholdseq_multivector.push(intermediatehold);
    finalholdid_multivector.push(hold_header[i].clone());
  }


  let mut  deeplearn_vector_multivector:Vec<Vec<_>> = Vec::new();

  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
       let mut vectoralgebrahold:Vec<usize> = Vec::new();
        if finalholdseq_multivector[i][j].to_string() == "A"
        && finalholdseq_multivector[i+1][j].to_string() == "T" {
        vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "A"
      && finalholdseq_multivector[i+1][j].to_string() == "G" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "A"
       && finalholdseq_multivector[i+1][j].to_string() == "C" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "T"
       && finalholdseq_multivector[i+1][j].to_string() == "G" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "T"
       && finalholdseq_multivector[i+1][j].to_string() == "A" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "T"
      && finalholdseq_multivector[i+1][j].to_string() == "C" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "G"
      && finalholdseq_multivector[i+1][j].to_string() == "A" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "G"
      && finalholdseq_multivector[i+1][j].to_string() == "C" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "G"
      && finalholdseq_multivector[i+1][j].to_string() == "T" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "C"
      && finalholdseq_multivector[i+1][j].to_string() == "T" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "C"
      && finalholdseq_multivector[i+1][j].to_string() == "G" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        } else if finalholdseq_multivector[i][j].to_string() == "C"
      && finalholdseq_multivector[i+1][j].to_string() == "A" {
      vectoralgebrahold.push(mismatchweight);
      deeplearn_vector_multivector.push(vectoralgebrahold);
        }
}
}
  let mut mismatch_write = File::create("mismatch-alignment-vector.txt").expect("file not present");
  for i in deeplearn_vector_multivector.iter_mut(){
    write!(mismatch_write,"{:?}",i).expect("file not written");
  }
Ok("mismatchvectors have been generated".to_string())
}


fn alignment_gapped(path: &str, gappedweight: usize) -> Result<String, Box<dyn Error>> {
     #[derive(Debug, Clone)]
  struct DeepLearn {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(path).expect("file not found");
  let fileread = BufReader::new(&fileopen);
  let mut embedded_hold:Vec<DeepLearn> = Vec::new();
  let mut hold_header:Vec<String> = Vec::new();
  let mut hold_sequence:Vec<String> = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not present");
    if line.starts_with(">"){
      hold_header.push(line);
    } else {
      hold_sequence.push(line);
    }
  }
  for i in 0..hold_header.len(){
    embedded_hold.push(DeepLearn{
      header: hold_header[i].clone(),
                       sequence: hold_sequence[i].clone(),
    })
  }
  let mut finalholdseq_multivector = Vec::new();
  let mut finalholdid_multivector:Vec<String> = Vec::new();
  for i in 0..hold_header.len(){
    let mut intermediatehold = Vec::new();
    for j in hold_sequence[i].chars(){
      intermediatehold.push(j);
    }
    finalholdseq_multivector.push(intermediatehold);
    finalholdid_multivector.push(hold_header[i].clone());
  }


  let mut deeplearn_vector_multivector:Vec<Vec<_>> = Vec::new();

  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
       let mut vectoralgebrahold:Vec<usize> = Vec::new();
        if finalholdseq_multivector[i][j].to_string() == "A"
           && finalholdseq_multivector[i+1][j].to_string() == "-" {
            vectoralgebrahold.push(gappedweight);
             deeplearn_vector_multivector.push(vectoralgebrahold);
   } else if finalholdseq_multivector[i][j].to_string() == "T"
       && finalholdseq_multivector[i+1][j].to_string() == "-" {
            vectoralgebrahold.push(gappedweight);
             deeplearn_vector_multivector.push(vectoralgebrahold);
  } else if finalholdseq_multivector[i][j].to_string() == "G"
      && finalholdseq_multivector[i+1][j].to_string() == "-" {
         vectoralgebrahold.push(gappedweight);
          deeplearn_vector_multivector.push(vectoralgebrahold);
   } else if finalholdseq_multivector[i][j].to_string() == "C"
      && finalholdseq_multivector[i+1][j].to_string() == "-" {
         vectoralgebrahold.push(gappedweight);
         deeplearn_vector_multivector.push(vectoralgebrahold);
  }
}
  }
   let mut gapped_write = File::create("gapped-alignment-vector.txt").expect("file not present");
   for i in deeplearn_vector_multivector.iter_mut(){
     write!(gapped_write, "{:?}", i).expect("iter not present");
   }
Ok("gapped alignment vectors have been written".to_string())
}
