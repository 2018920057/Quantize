use std::io;

fn main() {
  let mut input: String = String::new();
  let mut sequence: Vec<i32> = Vec::new();
  let kind: i32;

  println!("양자화할 수열 입력:");
  io::stdin().read_line(&mut input).expect("input error!");
  for i in input.split_whitespace(){
    sequence.push(i.parse().expect("parse error!"));
  }
  input.clear();
  sequence.sort();
  println!("입력된 수열:{:?}, 길이:{}",sequence,sequence.len());


  println!("사용할 숫자의 수:");
  io::stdin().read_line(&mut input).expect("input error!");
  let kind: i32 = input.trim().parse().expect("parse error!");

  let sse = quantize(&sequence, kind);
  println!("오차 제곱의 합의 최소치(SSE): {}",sse);
  
}

fn quantize(sequence: & Vec<i32>, kind: i32) -> i32 {
  //ret: sse, len: 입력배열의 길이
  let mut ret: i32 = 999999999;
  let len: usize = sequence.len();
  //pSum: A[i]까지의 합, pSqSum: A[i]^2까지의 합 저장
  let mut pSum: Vec<i32> = vec![sequence[0]];
  let mut pSqSum: Vec<i32> = vec![sequence[0].pow(2)];
  for i in 1..len {
    pSum.push(pSum[i-1]+sequence[i]);
    pSqSum.push(pSqSum[i-1]+sequence[i].pow(2));
  }
  //TODO
  ret
}

fn minError(sequence: & Vec<i32> ,lo: usize, hi: usize) -> i32{
  let mut ret: i32 = 999999999;
  //TODO
  ret
}