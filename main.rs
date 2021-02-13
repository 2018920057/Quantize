
use std::io;
use std::cmp;

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
  //pSum: A[i]까지의 합, pSqSum: A[i]^2까지의 합 저장
  let mut pSum: Vec<i32> = vec![sequence[0]];
  let mut pSqSum: Vec<i32> = vec![sequence[0].pow(2)];
  for i in 1..sequence.len() {
    pSum.push(pSum[i-1]+sequence[i]);
    pSqSum.push(pSqSum[i-1]+sequence[i].pow(2));
  }
  println!("{}",minError(&sequence, 0, sequence.len()-1, &pSum, &pSqSum));
  println!("사용할 숫자의 수:");
  io::stdin().read_line(&mut input).expect("input error!");
  let kind: i32 = input.trim().parse().expect("parse error!");

  let sse = quantize(&sequence, &pSum, &pSqSum, 0, kind);
  println!("오차 제곱의 합의 최소치(SSE): {}",sse);
  
}

fn quantize(sequence: & Vec<i32>, pSum: & Vec<i32>, pSqSum: & Vec<i32>, from: usize, kind: i32) -> i32 {
  //ret: sse, len: 입력배열의 길이
  let mut ret: i32 = 999999999;
  let len: usize = sequence.len();
  //기저 사례: 모든 숫자가 양자화되었을 때
  if from==len-1 {return 0;}
  //기저 사례: 숫자가 남았는데 더 못 묶을 때 
  if kind==0 {return ret;}
  //조각의 길이를 변화시켜가며 최소치 찾기
  for i in 1..len-from+1 {
    ret= cmp::min(ret,minError(&sequence, from, from+i-1, pSum, pSqSum)+quantize(sequence, pSum, pSqSum, from+i,kind-1));
  }
  ret
}

fn minError(sequence: & Vec<i32>, lo: usize, hi: usize, pSum: &Vec<i32>, pSqSum: &Vec<i32>) -> i32{
  let sum: i32 = pSum[hi] - if lo==0 {0} else {pSum[lo]};
  let sqSum: i32 = pSqSum[hi] - if lo==0 {0} else {pSqSum[lo]};
  let m: i32 = (((sum as f64)/(hi-lo+1)as f64)+0.5) as i32;
  sqSum - 2*m*sum + m*m*(hi-lo+1)as i32
}