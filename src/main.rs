// Copyright (c) 2019 - 2023 Leap Dot Cloud Limited <info@leap.cloud> - All rights reserved
// All content is proprietary and confidential
// Unauthorized use, copy and / or distribution of this project and any of its files, via any medium is strictly prohibited
// No warranty provided, explicit or implicit
// See file LICENSE for full license details

fn concatenate_strings(a: &str, b: &str) -> String {
  let mut result = String::new();
  result.push_str(a);
  result.push_str(b);
  result
}

fn main(){
  println!("{}", concatenate_strings("Blah", " fuh!"));
  
}
