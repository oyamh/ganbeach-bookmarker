// use super::List;
use crate::gooscut::Lists as PbLists;
use domain::Lists;

impl From<PbLists> for Lists {
    fn from(value: PbLists) -> Lists {
        let inner = value
            .lists
            .into_iter()
            .map(|list| (list.bookmark_id.into(), list.into()))
            .collect();
        Lists(inner)
    }
}

// #[derive(Debug)]
// pub struct Lists {
//     inner: Vec<List>,
// }

// impl<'a> IntoIterator for &'a Lists {
//     type Item = &'a List;
//     type IntoIter = ListsIter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         ListsIter {
//             lists: self,
//             index: 0,
//         }
//     }
// }

// pub struct ListsIter<'a> {
//     lists: &'a Lists,
//     index: usize,
// }

// impl<'a> Iterator for ListsIter<'a> {
//     type Item = &'a List;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= self.lists.inner.len() {
//             None
//         } else {
//             self.index += 1;
//             Some(&self.lists.inner[self.index - 1])
//         }
//     }
// }

// // impl<I: Iterator> IntoIterator for Lists {
// //     type Item = I::Item;
// //     type IntoIter = I;
// //     fn into_iter(self) -> I {
// //         ListRepeater {
// //             iter: self.lists.iter(),
// //         }
// //     }
// // }

// // impl<'a> ListsProvider for Lists {
// //     type List = &'a List;
// //     type Iterator = ListRepeater<'a>;
// //     fn iter(&self) -> Self::Iterator {
// //         ListRepeater {
// //             iter: self.lists.iter(),
// //         }
// //     }
// // }

// // impl ListsProvider for Lists {
// //     type List = List;
// //     fn provide(&self) -> Vec<Self::List> {
// //         self.0
// //     }
// // }

// struct ListRepeater<'a> {
//     iter: std::slice::Iter<'a, List>,
// }

// impl<'a> Iterator for ListRepeater<'a> {
//     type Item = &'a List;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }
