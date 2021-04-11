import * as blake3 from '../../blake3.js';
import * as hex from "https://deno.land/std/encoding/hex.ts";
import {encode} from "https://deno.land/std/encoding/utf8.ts"

var txt = "test"
console.log(txt, hex.encodeToString(blake3.hash(encode(txt))));

var times = 0;
var cost = 0

while(++times){
  var txt = times.toString(); 
  var begin = new Date().getTime() - 0;
  blake3.hash_leading_zero(encode(txt),18);
  cost += new Date().getTime() - begin;
  console.log(times, Math.round(cost/times)/1000)
}
// for(var i of ["hi", "test"]){
//   console.log();
// }
