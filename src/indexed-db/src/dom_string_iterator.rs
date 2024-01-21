use web_sys::DomStringList;

pub struct DomStringIterator {
    inner: DomStringList,
    index: u32,
}

impl From<DomStringList> for DomStringIterator {
    fn from(list: DomStringList) -> Self {
        Self {
            inner: list,
            index: 0,
        }
    }
}

impl<'a> Iterator for DomStringIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.item(self.index).map(|s| {
            self.index += 1;
            s
        })
    }
}

// mod test {
//     struct MyList(Vec<u32>);

//     impl MyList {
//         fn item(&self, index: usize) -> Option<&u32> {
//             self.0.get(index)
//         }
//     }

//     struct MyIterator {
//         inner: MyList,
//         index: usize,
//     }

//     impl From<MyList> for MyIterator {
//         fn from(list: MyList) -> Self {
//             Self {
//                 inner: list,
//                 index: 0,
//             }
//         }
//     }

//     impl<'a> Iterator for MyIterator {
//         type Item = u32;
//         fn next(&mut self) -> Option<Self::Item> {
//             self.inner.item(self.index).map(|s| {
//                 self.index += 1;
//                 *s
//             })
//             // if let Some(item) = self.inner.item(self.index) {
//             //     self.index += 1;
//             //     Some(item)
//             // } else {
//             //     None
//             // }
//         }
//     }

//     #[test]
//     fn should_item_iter() {
//         let mylist = MyList(vec![1, 2, 3, 4, 5]);
//         // let myiter = Into::<MyIterator>::into(mylist);
//         let myiter: MyIterator = mylist.into();

//         println!("begin");

//         myiter.for_each(|n| {
//             println!("{:?}", n);
//             println!("next...");
//         });

//         println!("end");
//     }
// }
