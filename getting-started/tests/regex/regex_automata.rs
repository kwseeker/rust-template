use regex_automata::{meta::Regex, Match};

#[test]
pub fn regex_automata_usage() {
    let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let text = b"2018-12-24 2016-10-08";
    let matches: Vec<Match> = re.find_iter(text).collect();
    //输出的是索引范围集合
    // assert_eq!(matches, vec![(0, 10), (11, 21)]);

    let re = Regex::new("foo[0-9]+").unwrap();
    let haystack = "foo1 foo12 foo123";
    let matches: Vec<Match> = re. find_iter(haystack).collect();
    assert_eq!(matches, vec![
        Match::must(0, 0..4),
        Match::must(0, 5..10),
        Match::must(0, 11..17),
    ]);
}