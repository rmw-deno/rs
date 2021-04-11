use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash(v: Vec<u8>) -> Vec<u8> {
  return blake3::hash(&v).as_bytes().to_vec();
}

#[wasm_bindgen]
pub fn hash_leading_zero(v: Vec<u8>, n: u32) -> Vec<u8> {
  let mut token = vec![0u8];
  loop {
    let mut txt = token.clone();
    txt.extend(&v);

    let h = hash(txt);
    let mut leading_zeros: u32 = 0;
    for i in h.iter() {
      let t = i.leading_zeros();
      leading_zeros += t;
      if t != 8 {
        break;
      }
    }
    if leading_zeros >= n {
      // println!("{} {}", String::from_utf8_lossy(&v), hex::encode(&v));
      break;
    }
    vec_incr(&mut token);
  }
  token
}
 
pub fn vec_incr(b256: &mut Vec<u8>) {
  let mut i = b256.len();

  loop {
    i -= 1;
    let n = b256[i];
    if n == 255 {
      b256[i] = 0;
    } else {
      b256[i] = n + 1;
      return;
    }
    if i == 0 {
      break;
    }
  }

  b256.insert(0, 1);
}
