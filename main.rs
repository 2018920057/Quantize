use std::io;

fn main() {
  let mut input: String = String::new();
  let mut sequence: Vec<i32> = Vec::new();
  
  println!("수열 입력:");
  io::stdin().read_line(&mut input).expect("input error!");
  for i in input.split_whitespace(){
    sequence.push(i.parse().expect("parse error!"));
  }
  sequence.sort();
  println!("입력된 수열:{:?}, 길이:{}",sequence,sequence.len());

  let mut quantized: Vec<i32> = quantize(&sequence);

  println!("양자화된 수열:{:?}",quantized);
  
}

fn quantize(sequence: & Vec<i32>) -> Vec<i32> {
  //TODO
  Vec::new()
}