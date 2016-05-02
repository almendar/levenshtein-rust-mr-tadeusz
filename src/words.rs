use std::collections::HashSet;

pub fn split_to_words(content: &str) -> HashSet<String> {
    let filter_chars: HashSet<char> = [' ', '.', ',', '-', ':', ';', '!', '—', '(', ')', '»',
                                        '«', '?']
                                           .into_iter()
                                           .cloned()
                                           .collect();


    let acc: HashSet<String> = content.lines()
                                      .flat_map(|x| x.split_whitespace())
                                      .map(|y| {
                                          y.chars()
                                           .filter(|x| !filter_chars.contains(x))
                                           .collect::<String>()
                                      })
                                      .map(|x| x.to_lowercase())
                                      .filter(|x| !x.is_empty())
                                      .collect();
    return acc;

}


#[test]
fn test_splitting_to_words() {
  let sample_text = "Some text \n
  that doesn't fit into buffers!";
  let samples: Vec<&str> = vec!["some", "text", "that", "doesn't", "fit", "into", "buffers"];
  let expected_ressult: HashSet<String> = samples.iter().map(|x| x.to_string()).collect();
  let what_we_received: HashSet<String> = split_to_words(&sample_text);
  assert_eq!(
    what_we_received,
    expected_ressult
  )
}
