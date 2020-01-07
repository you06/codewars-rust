use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
  
  // MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".
  fn new() -> MorseDecoder {
    MorseDecoder{
      morse_code: HashMap::new(),
    }
  }

  fn parse(&self, encoded: &String) -> String {
    if encoded == " " {
      return " ".to_owned();
    }
    match self.morse_code.get(encoded) {
      Some(decoded) => decoded.to_owned(),
      None => "".to_owned()
    }
  }

  fn decode_morse(&self, encoded: &str) -> String {
    let (
      mut decode,
      mut chars,
      mut start_flag
    ) = (
      Vec::new() as Vec<String>,
      vec![],
      true
    );
    // I don't want to write double-split code
    for ch in encoded.trim().chars() {
      if start_flag {
        start_flag = false;
        chars.push(ch);
      } else {
        if ch != ' ' {
          chars.push(ch);
        } else {
          decode.push(self.parse(&chars.clone().into_iter().collect()));
          chars.clear();
          start_flag = true;
        }
      }
    }
    // parse the last character
    decode.push(self.parse(&chars.clone().into_iter().collect()));

    decode.join("")
  }

  // fn decode_morse(&self, encoded: &str) -> String {
  //   let decode: Vec<String> = encoded
  //                               .trim()
  //                               .split_whitespace()
  //                               .map(|c| self.parse(c))
  //                               .collect();
  //   decode.join("")
  // }
  
}

pub fn test() {
  let decoder = MorseDecoder::new();
  assert_eq!(decoder.decode_morse(" .... . -.--   .--- ..- -.. ."), "HEY JUDE");
}
