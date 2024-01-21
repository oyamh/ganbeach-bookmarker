use crate::hooks::use_lists_context::use_lists_context;
use crate::hooks::use_oninput::use_oninput;
use domain::{SuggestInfo, Title, TypeCode};
use std::{collections::HashMap, fmt::Debug, ops::ControlFlow, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_suggest(input_ref: &NodeRef, type_code: TypeCode) -> UseStateHandle<SuggestResults> {
    let lists_ctx = use_lists_context();
    let results = use_state(|| SuggestResults::default());

    {
        log::debug!("use_suggest");
        let lists_ctx = lists_ctx.clone();
        let lists_ctx_2 = lists_ctx.clone();
        let results = results.clone();
        use_oninput(
            input_ref,
            move |e| {
                let lists_ctx = lists_ctx.clone();

                let input_target = e
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                input_target.map(|input_target| {
                    let value = input_target.value();

                    log::debug!("value: {}", &value);

                    //TODO?: e.codeによってlists_ctx.tags OR results.map()を使うか分ける。
                    //削除方法はDeleteとBackSpaceの他に、選択からの切り取りがある。その切り取りに対してInput Eventでは対応できない(はず)。よって見送り。

                    let value: &str = &value;
                    let splited_last_value = value
                        .rsplit_once(",")
                        .map_or(value, |tupple| tupple.1.trim_start());
                    // let splited_last_value = value
                    //     .rsplit_once(",")
                    //     .map_or(&value as &str, |tupple| tupple.1.trim_start());

                    let list_infos = suggest(
                        &splited_last_value,
                        lists_ctx.inner().typed_lists_iter(type_code),
                    );
                    // log::debug!("lists_ctx: {:?}", &lists_ctx);
                    // log::debug!("list_infos: {:?}", &list_infos);
                    results.set(list_infos);
                });
            },
            lists_ctx_2,
        );
    }
    results
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

impl Debug for SuggestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_list().entries(self.0.iter()).finish()
        f.write_fmt(format_args!("SuggestResults[{:?}]", self.0))
    }
}

fn suggest<'a, T, I>(word: &'a str, lists_iter: I) -> SuggestResults
where
    T: SuggestInfo + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut min_score = std::i32::MAX;
    let mut max_score = 0;
    let score_map: HashMap<i32, Vec<(&Title, u32)>> =
        lists_iter.fold(HashMap::new(), |mut accum, list| {
            let list_title = list.title();
            let child_count = list.child_count();
            if let Some(score) = fuzzy_match(list_title, word) {
                if score < min_score {
                    min_score = score;
                }
                if score > max_score {
                    max_score = score;
                }
                accum
                    .entry(score)
                    .or_insert_with(|| vec![])
                    .push((list_title, child_count));
                // if let Some(same_scores) = accum.get_mut(&score) {
                //     same_scores.push((list_title, child_count));
                // } else {
                //     accum.insert(score, vec![(list_title, child_count)]);
                // }
            }
            accum
        });

    let results = (min_score..max_score + 1)
        .rev()
        .filter(|i| score_map.get(&i).is_some())
        .map(|i| score_map.get(&i).unwrap())
        .flatten()
        .map(|(title, child_count)| {
            Rc::new(SuggestResult {
                title: title.to_owned().clone(),
                count: *child_count,
            })
        })
        .collect::<Vec<Rc<SuggestResult>>>();
    SuggestResults(results)
}

#[test]
fn should_suggest() {
    use domain::{List, Lists};

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
    println!("lists={lists:#?}");
    let titles = suggest("vi", lists.typed_lists_iter(TypeCode::Folder));
    println!("titles={titles:?}");
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
    let titles = suggest("moe", lists.typed_lists_iter(TypeCode::Folder));
    println!("titles: {titles:?}");
    assert_eq!(titles, SuggestResults::from(vec!["molestiae", "maiores"]));
}

#[allow(dead_code)]
fn suggest_titles<'a, I>(word: &'a str, lists_iter: I) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    let mut min_score = std::i32::MAX;
    let mut max_score = 0;
    let score_map: HashMap<i32, Vec<&str>> = lists_iter.fold(HashMap::new(), |mut accum, title| {
        if let Some(score) = fuzzy_match(title, word) {
            if score < min_score {
                min_score = score;
            }
            if score > max_score {
                max_score = score;
            }
            if let Some(same_scores) = accum.get_mut(&score) {
                same_scores.push(title);
            } else {
                accum.insert(score, vec![title]);
            }
        }
        accum
    });
    // log::debug!("score_map: {score_map:?}");
    // println!("score_map: {score_map:?}");

    (min_score..max_score + 1)
        .rev()
        .filter(|i| score_map.get(&i).is_some())
        .map(|i| score_map.get(&i).unwrap())
        .flatten()
        .map(|title| String::from(*title))
        .collect::<Vec<String>>()
}

#[test]
fn should_suggest_titles() {
    let v = vec!["vitae", "veniam"];
    let iter = v.iter().map(|s| *s);
    let titles = suggest_titles("vi", iter);
    println!("titles: {titles:?}");
    assert_eq!(titles, vec!["vitae", "veniam"]);

    let v = vec!["commodi", "molestiae", "temporibus", "maiores"];
    let iter = v.iter().map(|s| *s);
    let titles = suggest_titles("moe", iter);
    println!("titles: {titles:?}");
    assert_eq!(titles, vec!["molestiae", "maiores"]);
}

fn fuzzy_match<T>(word: T, pattern: &str) -> Option<i32>
where
    T: AsRef<str>,
{
    match contains_chars(&word, pattern) {
        true => Some(get_fuzzy_score(&word, pattern)),
        false => None,
    }
}

#[derive(Debug, Default)]
struct Score {
    total_score: i32,
    chain_count: u32, // 文字一致連鎖数
    in_gap: bool,     // 不一致の連鎖中フラグ
}

const SCORE_MATCH: i32 = 16; // 文字一致ボーナス値
const SCORE_CHAIN: i32 = 4; // 一致連鎖ボーナス値
const SCORE_FIRST_LETTER: i32 = 8; // 頭文字一致ボーナス値
                                   // const SCORE_FIRST_LETTER: i32 = 2; // 頭文字一致ボーナス値
                                   // const SCORE_FIRST_CHAR: i32 = 8; // 一文字目ボーナス値
const SCORE_GAP_START: i32 = -3; // 不一致ペナルティ値
const SCORE_GAP_EXTENSION: i32 = -1; // 不一致連鎖ペナルティ値

fn get_fuzzy_score<T>(target: T, pattern: &str) -> i32
where
    T: AsRef<str>,
{
    let mut pattern_iter = pattern.chars();
    let mut matching_char = pattern_iter.next().unwrap();
    // println!("get_fuzzy_score: {}", target);
    let score = target
        .as_ref()
        .chars()
        .enumerate()
        .fold(Score::default(), |accum, (i, c)| {
            let Score {
                total_score,
                chain_count,
                in_gap,
            } = accum;
            if c == matching_char {
                // println!("{c} == {matching_char}");
                let score_matching = if i == 0 {
                    // println!("+{} SCORE_FIRST_LETTER", SCORE_FIRST_LETTER);
                    SCORE_FIRST_LETTER
                } else if 0 < chain_count {
                    // println!("+{} SCORE_CHAIN", SCORE_CHAIN);
                    SCORE_CHAIN
                } else {
                    0
                };
                // println!("+{} SCORE_MATCH", SCORE_MATCH);
                matching_char = match pattern_iter.next() {
                    Some(c) => c,
                    None => ' ',
                };
                Score {
                    total_score: total_score + SCORE_MATCH + score_matching,
                    chain_count: chain_count + 1,
                    in_gap: false,
                }
            } else {
                // println!("{c} != {matching_char}");
                let score_gap = if in_gap {
                    // println!("{} SCORE_GAP_EXTENSION", SCORE_GAP_EXTENSION);
                    SCORE_GAP_EXTENSION
                } else {
                    // println!("{} SCORE_GAP_START", SCORE_GAP_START);
                    SCORE_GAP_START
                };
                Score {
                    total_score: total_score + score_gap,
                    chain_count: 0,
                    in_gap: true,
                }
            }
        });
    // let total = score.total_score;
    // println!("total: {total:?}");
    score.total_score
}

#[test]
fn should_get_fuzzy_score() {
    let higher_score = get_fuzzy_score("test", "test");
    let lower_score = get_fuzzy_score("-test", "test");
    assert!(higher_score > lower_score);

    let higher_score = get_fuzzy_score("test", "test");
    let lower_score = get_fuzzy_score("test-", "test");
    assert!(higher_score > lower_score);

    let higher_score = get_fuzzy_score("vitae", "vi");
    let lower_score = get_fuzzy_score("veniam", "vi");
    // println!("h: {higher_score}, l: {lower_score}");
    assert!(higher_score > lower_score);
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

/// 全ての文字を含む範囲を調べる。Some((開始位置, 終端装置))を返す。完全一致ではないので注意。あくまで文字が順番通りに含まれているかどうか。
#[allow(dead_code)]
fn get_chars_range(target: &str, pattern: &str) -> Option<(u32, u32)> {
    log::debug!("get_chars_range");
    // let mut pattern_iter = pattern.char_indices().map(|(i, c)| c);
    let mut pattern_iter = pattern.chars();
    let mut matching_char = match pattern_iter.next() {
        Some(c) => c,
        None => return None,
    };
    //TODO: try_foldを使う。breakできるようにする。
    //.chars().enumerate()
    let end_index = target.char_indices().fold(-1, |accum, (index, c)| {
        if c != matching_char {
            return accum;
        }
        match pattern_iter.next() {
            // 次の文字の検索を続ける。
            Some(c) => {
                matching_char = c;
                return -1;
            }
            // Noneならば全て一致した。最後に一致した文字のindexを返す。
            None => {
                return index as i32;
            }
        };
    });
    // endが-1の場合は不完全一致なのでNoneを返す。
    if end_index == -1 {
        return None;
    }
    let first_char = pattern.chars().nth(0).unwrap();
    let begin_index = target.find(first_char).unwrap();
    Some((begin_index as u32, end_index as u32))
}

#[test]
fn should_search_exact_match() {
    assert_eq!(get_chars_range("testing", "test"), Some((0, 3)));
    assert_eq!(get_chars_range("t---testing", "test"), Some((0, 7)));
    // 日本語は3バイト
    assert_eq!(get_chars_range("テスト一致", "テスト"), Some((0, 6)));
    assert_eq!(get_chars_range("一致テスト", "テスト"), Some((6, 12)));
}
