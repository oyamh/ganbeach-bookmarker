use crate::Title;
use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, HashMap},
    default::Default,
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, ControlFlow, SubAssign},
    rc::Rc,
};

pub trait SuggestInfo {
    fn title(&self) -> &Title;
    fn child_count(&self) -> u32;
}

pub trait MatchCommand<'a> {
    fn score<T>(&mut self, info: &'a T) -> Option<Score>
    where
        T: SuggestInfo + 'a;
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Score(pub i32);

impl Score {
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }
}

impl Add for Score {
    type Output = Score;
    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

impl Into<i32> for Score {
    fn into(self) -> i32 {
        self.0
    }
}

impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
pub struct ScoreMap<'a, T>
where
    T: Default,
{
    // scores: BinaryHeap<Reverse<Score>>,
    map: HashMap<Score, Vec<&'a T>>,
}

impl<'a, T> ScoreMap<'a, T>
where
    T: SuggestInfo + 'a + Default + Debug,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_title(&mut self, score: Score, item: &'a T) {
        // self.scores.push(Reverse(score));
        self.map.entry(score).or_insert(Vec::new()).push(item);
    }

    pub fn sort_by_scores(&self) -> impl Iterator<Item = &&T> {
        //TODO: into_iter_sortedかdrain_sortedが標準化されたらinto_sorted_vecと入れ替える。
        // self.scores
        //     .into_sorted_vec()
        //     .into_iter()
        //     .flat_map(move |score| self.map.remove(&score.0).unwrap())

        let mut v: Vec<(&Score, &Vec<&T>)> = self.map.iter().collect();
        v.sort_by_key(|(score, _infos)| Reverse(*score));
        v.into_iter().flat_map(|(_score, infos)| infos)
    }

    pub fn entry(&mut self, key: Score) -> Entry<'_, Score, Vec<&'a T>> {
        self.map.entry(key)
    }
}

#[cfg(test)]
mod sort_test {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn should_sort_by_scores() {
        unimplemented!();
    }

    #[test]
    fn should_sort_binaryheap() {
        let mut scores: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

        scores.push(Reverse(2));
        scores.push(Reverse(2));
        scores.push(Reverse(1));
        scores.push(Reverse(5));
        scores.push(Reverse(4));
        scores.push(Reverse(3));
        scores.push(Reverse(6));

        let mut map = HashMap::<u32, Vec<&str>>::new();
        map.entry(2).or_insert(Vec::new()).push("test2");
        map.entry(2).or_insert(Vec::new()).push("test2");
        map.entry(1).or_insert(Vec::new()).push("test1");
        map.entry(5).or_insert(Vec::new()).push("test5");
        map.entry(4).or_insert(Vec::new()).push("test4");
        map.entry(3).or_insert(Vec::new()).push("test3");
        map.entry(6).or_insert(Vec::new()).push("test6");

        let v = scores.into_sorted_vec();
        dbg!(&v);

        // let result = v
        //     .iter()
        //     .map(|i| map.get(i).unwrap())
        //     .flatten()
        //     .collect::<Vec<&&str>>();
        // dbg!(result);

        // let result = v
        //     .into_iter()
        //     .map(|i| map.remove(&i).unwrap())
        //     .flatten()
        //     .collect::<Vec<&str>>();
        // dbg!(result);

        // let result = v
        //     .into_iter()
        //     .flat_map(|i| map.remove(&i.0).unwrap())
        //     .collect::<Vec<&str>>();
        // dbg!(result);

        let result = v
            .into_iter()
            .map(|i| map.remove(&i.0).or(None))
            .filter(|x| x.is_some())
            .flatten()
            .flatten()
            .collect::<Vec<&str>>();
        dbg!(result);

        // let result = scores.iter().map(|i| {
        //     dbg!(i);
        //     i
        // });
        // dbg!(result);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SuggestResult {
    pub title: Title,
    pub count: u32,
}

impl From<String> for SuggestResult {
    fn from(src: String) -> Self {
        Self {
            title: src.into(),
            count: 0,
        }
    }
}

impl From<&str> for SuggestResult {
    fn from(src: &str) -> Self {
        Self::from(src.to_string())
    }
}

impl<T> From<&&T> for SuggestResult
where
    T: SuggestInfo,
{
    fn from(src: &&T) -> Self {
        Self {
            title: src.title().to_owned().clone(),
            count: src.child_count(),
        }
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct SuggestResults(Vec<Rc<SuggestResult>>);

impl SuggestResults {
    pub fn len(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn iter(&self) -> impl Iterator<Item = &Rc<SuggestResult>> {
        self.0.iter()
    }
}

impl From<Vec<String>> for SuggestResults {
    fn from(src: Vec<String>) -> Self {
        Self(
            src.iter()
                .map(|title| Rc::new(SuggestResult::from(title.to_owned())))
                .collect(),
        )
    }
}

impl From<Vec<&str>> for SuggestResults {
    fn from(src: Vec<&str>) -> Self {
        Self(
            src.iter()
                .map(|title| Rc::new(SuggestResult::from(*title)))
                .collect(),
        )
    }
}

impl From<Vec<Rc<SuggestResult>>> for SuggestResults {
    fn from(src: Vec<Rc<SuggestResult>>) -> Self {
        Self(src)
    }
}

// impl<A> FromIterator<&A> for SuggestResults
// where
//     A: SuggestInfo,
// {
//     fn from_iter<T: IntoIterator<Item = &A>>(iter: T) -> Self {
//         Self::from(Vec::from_iter(iter.into_iter().map(|info| {
//             Rc::new(SuggestResult {
//                 title: info.title().clone(),
//                 count: info.child_count(),
//             })
//         })))
//     }
// }

impl Debug for SuggestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_list().entries(self.0.iter()).finish()
        f.write_fmt(format_args!("SuggestResults[{:?}]", self.0))
    }
}

#[derive(Debug, Default)]
pub struct SuggestCommand<'a, T, I>
where
    T: SuggestInfo + 'a,
    I: Iterator<Item = &'a T>,
{
    list_iter: I,
}

impl<'a, T, I> SuggestCommand<'a, T, I>
where
    T: SuggestInfo + 'a,
    I: Iterator<Item = &'a T>,
{
    pub fn new(list_iter: I) -> Self {
        Self { list_iter }
    }
}

impl<'a, I, T> SuggestCommand<'a, T, I>
where
    T: SuggestInfo + 'a + Default + Debug,
    I: Iterator<Item = &'a T>,
{
    pub fn suggest(self, mut match_command: impl MatchCommand<'a>) -> SuggestResults {
        self.list_iter
            .fold(ScoreMap::new(), |mut score_map, list| {
                match match_command.score(list) {
                    Some(score) if score.is_negative() => {}
                    Some(score) => {
                        score_map.insert_title(score, list);
                    }
                    None => {}
                }
                score_map
            })
            .sort_by_scores()
            .map(|list| Rc::new(SuggestResult::from(list)))
            .collect::<Vec<Rc<SuggestResult>>>()
            .into()
    }
}

#[cfg(test)]
mod suggest_test {
    use super::*;
    use crate::{List, Lists, TypeCode};

    #[test]
    fn should_suggest_by_command() {
        let v = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_title("vitae".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
            List::builder()
                .set_bookmark_id(2)
                .set_title("veniam".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
        ];
        let lists = Lists::from(v);

        let command = SuggestCommand::new(lists.typed_lists_iter(TypeCode::Folder));
        let match_command = FuzzyMatchCommand::new("vi");
        let titles = command.suggest(match_command);
        assert_eq!(titles, SuggestResults::from(vec!["vitae", "veniam"]));

        let v = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_title("commodi".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
            List::builder()
                .set_bookmark_id(2)
                .set_title("molestiae".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
            List::builder()
                .set_bookmark_id(3)
                .set_title("temporibus".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
            List::builder()
                .set_bookmark_id(4)
                .set_title("maiores".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
        ];
        let lists = Lists::from(v);
        let command = SuggestCommand::new(lists.typed_lists_iter(TypeCode::Folder));
        let match_command = FuzzyMatchCommand::new("moe");
        let titles = command.suggest(match_command);
        assert_eq!(titles, SuggestResults::from(vec!["molestiae", "maiores"]));
    }
}

#[derive(Debug, Default)]
struct EmptyMatchCommand<'a, F>
where
    F: Fn(&Title, &'a str) -> Option<Score>,
{
    pattern: &'a str,
    match_fn: F,
}

impl<'a, F> MatchCommand<'a> for EmptyMatchCommand<'a, F>
where
    F: Fn(&Title, &'a str) -> Option<Score>,
{
    fn score<T>(&mut self, info: &'a T) -> Option<Score>
    where
        T: SuggestInfo + 'a,
    {
        (self.match_fn)(info.title(), self.pattern)
    }
}

#[derive(Debug, Default)]
pub struct FuzzyMatchCommand<'a> {
    pattern: &'a str,
}

impl<'a> FuzzyMatchCommand<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Self { pattern }
    }
}

impl<'a> MatchCommand<'a> for FuzzyMatchCommand<'a> {
    fn score<T>(&mut self, info: &'a T) -> Option<Score>
    where
        T: SuggestInfo + 'a,
    {
        match contains_chars(info.title(), self.pattern) {
            true => Some(FuzzyScoreCommand::new().calc(info.title(), self.pattern)),
            false => None,
        }
    }
}

#[cfg(test)]
mod match_command_test {
    use super::*;
    use crate::{List, TypeCode};

    #[test]
    fn should_score() {
        let v = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_title("vitae".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
            List::builder()
                .set_bookmark_id(2)
                .set_title("veniam".to_string())
                .set_type_code(TypeCode::Folder.into())
                .build(),
        ];

        let mut command = FuzzyMatchCommand::new("vi");
        let result = command.score(&v[0]);
        assert_eq!(result, Some(Score(39)));

        let result2 = command.score(&v[1]);
        assert_eq!(result2, Some(Score(32)));
    }
}

const SCORE_MATCH: Score = Score(16); // 文字一致ボーナス値
const SCORE_CHAIN: Score = Score(4); // 一致連鎖ボーナス値
const SCORE_FIRST_LETTER: Score = Score(8); // 頭文字一致ボーナス値
                                            // const SCORE_FIRST_LETTER: Score = Score(2); // 頭文字一致ボーナス値
                                            // const SCORE_FIRST_CHAR: Score = Score(8); // 一文字目ボーナス値
const SCORE_GAP_START: Score = Score(-3); // 不一致ペナルティ値
const SCORE_GAP_EXTENSION: Score = Score(-1); // 不一致連鎖ペナルティ値

#[derive(Debug, Default)]
struct FuzzyScoreCommand {
    total_score: Score,
    chain_count: u32, // 文字一致連鎖数
    in_gap: bool,     // 不一致の連鎖中フラグ
    is_first_letter: bool,
}

impl FuzzyScoreCommand {
    pub fn new() -> Self {
        Self {
            is_first_letter: true,
            ..Self::default()
        }
    }

    fn calc<T>(&mut self, target: T, pattern: &str) -> Score
    where
        T: AsRef<str>,
    {
        let mut pattern_iter = pattern.chars();
        let mut matching_char = match pattern_iter.next() {
            Some(c) => c,
            // None => return Score(0),
            None => ' ',
        };
        target.as_ref().chars().for_each(|target_char| {
            if target_char == matching_char {
                self.bonus();
                matching_char = match pattern_iter.next() {
                    Some(c) => c,
                    None => ' ',
                };
            } else {
                self.penalty();
            }
        });
        self.total_score
    }

    // fn next_char(&self, mut iter: impl Iterator<Item = char>) -> char {
    //     match iter.next() {
    //         Some(c) => c,
    //         None => ' ',
    //     }
    // }

    fn bonus(&mut self) {
        let score = if self.is_first_letter {
            SCORE_FIRST_LETTER
        } else if 0 < self.chain_count {
            SCORE_CHAIN
        } else {
            Score(0)
        };
        self.total_score += SCORE_MATCH + score;
        self.chain_count += 1;
        self.in_gap = false;
        self.is_first_letter = false;
    }

    fn penalty(&mut self) {
        let score = if self.in_gap {
            SCORE_GAP_EXTENSION
        } else {
            SCORE_GAP_START
        };
        self.total_score += score;
        self.chain_count = 0;
        self.in_gap = true;
        self.is_first_letter = false;
    }
}

#[cfg(test)]
mod calc_test {
    use super::*;

    #[test]
    fn should_calc_score() {
        let mut result = Score(0) + Score(-5);
        dbg!(result);
        assert_eq!(result, Score(-5));

        result += Score(-5);
        dbg!(result);
        assert_eq!(result, Score(-10));
    }

    #[test]
    fn should_get_fuzzy_score() {
        let higher_score = FuzzyScoreCommand::new().calc("test", "test");
        let lower_score = FuzzyScoreCommand::new().calc("-test", "test");
        assert!(higher_score > lower_score);

        let higher_score = FuzzyScoreCommand::new().calc("test", "test");
        let lower_score = FuzzyScoreCommand::new().calc("test-", "test");
        assert!(higher_score > lower_score);

        let higher_score = FuzzyScoreCommand::new().calc("vitae", "vi");
        let lower_score = FuzzyScoreCommand::new().calc("veniam", "vi");
        dbg!("vitae", higher_score);
        dbg!("veniam", lower_score);
        assert!(higher_score > lower_score);
    }
}

/// patternの全ての文字がその順番通りにtargetに含まれているかを調べる。完全一致ではないので注意。あくまで文字が順番通り含まれているかどうか。
fn contains_chars<T>(target: T, pattern: &str) -> bool
where
    T: AsRef<str>,
{
    let mut pattern_iter = pattern.chars();
    let matching_char = match pattern_iter.next() {
        Some(c) => c,
        None => return false,
    };
    // ControlFlowはResultやOptionと同じようなenum型。
    let result = target.as_ref().chars().try_fold(matching_char, |accum, c| {
        if c != accum {
            return ControlFlow::Continue(accum);
        }
        match pattern_iter.next() {
            Some(c) => ControlFlow::Continue(c),
            None => ControlFlow::Break(true),
        }
    });
    match result {
        ControlFlow::Continue(_c) => false,
        ControlFlow::Break(b) => b,
    }
}

#[cfg(test)]
mod fuzzy_search_test {
    use super::*;

    #[test]
    fn should_contains_chars() {
        // contains_chars("testing", "test");
        assert_eq!(contains_chars("testing", "test"), true);
        assert_eq!(contains_chars("teテストsting", "test"), true);
        assert_eq!(contains_chars("テストtesting", "test"), true);
        assert_eq!(contains_chars("テストtesting", "テスト"), true);
        assert_eq!(contains_chars("testingテスト", "テスト"), true);

        assert_eq!(contains_chars("tes-ing", "test"), false);
        assert_eq!(contains_chars("tetsing", "test"), false);
    }
}
