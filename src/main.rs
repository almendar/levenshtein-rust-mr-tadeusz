mod levenstein;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::io;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::env;

fn read() -> io::Result<String> {
  let mut buffer: String = String::new();
  let path = Path::new("./pan-tadeusz.txt");
  let mut file = try!(File::open(path));
  let _ = file.read_to_string(&mut buffer);
  return Ok(buffer);
}

#[allow(dead_code)]
fn write_to_file(content: &HashSet<String>) {
  match File::create("pan-tadeusz-words.txt") {
      Ok(mut f) => for i in content {
        let _ = f.write_all(i.as_bytes());
        let _ = f.write_all(b"\n");
      },
      Err(e) => println!("{}", e),
  };
}

fn split_to_words(content: &str) -> HashSet<String> {
    let filter_chars: BTreeSet<char> = [' ', '.', ',','-',':'
		,';','!','—', '(', ')','»','«', '?'].iter().cloned().collect();

    let acc: HashSet<String> = content.lines()
        .flat_map(|x| x.split_whitespace())
        .map(|y| y.chars().filter(|x| !filter_chars.contains(x)).collect::<String>())
        .map(|x| x.to_lowercase())
        .filter(|x| !x.is_empty())
        .collect();
    return acc;
}


fn print_fixed_word(word_to_fix: &String) {
  match read() {
    Ok(book) => {
      let words: HashSet<String> = split_to_words(&book);
      let mut computated_distances: Vec<(&String, usize)> = words.iter().map(
        |x| (x, levenstein::distance(x, &word_to_fix.to_lowercase()))
      ).collect();
      computated_distances.sort_by_key(|&(_,y)| y);
      let best_five: Vec<&(&String, usize)> = computated_distances.iter().take(10).collect();
      for top_word in best_five {
        println!("{}, {}", top_word.0, top_word.1);
      };
    },
    Err(_) => ()
  }
}


fn main() {
  let mut args = env::args();
  args.next();
  match args.next() {
    Some(word_to_fix) => print_fixed_word(&word_to_fix),
    None => {
      println!("Pass word as a cmd line argument to see nearest neighbours");
      ();
    }
  };

}
