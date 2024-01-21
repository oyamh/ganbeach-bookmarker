use std::{cell::RefCell, rc::Rc};

/// RewritableResult: callback内で使える値を構造体内で参照する。
#[derive(Debug)]
pub struct RewritableResult<T>(Rc<RefCell<Option<T>>>);

impl<T> RewritableResult<T> {
    pub fn new(result: Option<T>) -> Self {
        Self(Rc::new(RefCell::new(result)))
    }

    pub fn replace(&mut self, result: Option<T>) -> Option<T> {
        self.0.replace(result)
        // self.0.borrow_mut().replace(result)
    }

    pub fn is_some(&self) -> bool {
        self.0.borrow_mut().is_some()
        //self.0.borrow().is_some();
    }
}

impl<T> Clone for RewritableResult<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for RewritableResult<T> {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

//TODO: replaceのテストを書く。
#[cfg(test)]
mod test {
    use super::RewritableResult;

    #[test]
    fn should_replace_value() {
        let mut result = RewritableResult::new(Some(1));
        let old_value = result.replace(Some(2));
        assert_eq!(old_value, Some(1));
    }

    #[test]
    fn should_replace_with_none() {
        let mut result = RewritableResult::new(None);
        let init_none = result.replace(Some(2));
        assert_eq!(init_none, None);
        let some_2 = result.replace(None);
        assert_eq!(some_2, Some(2));
        let none = result.replace(None);
        assert_eq!(none, None);
    }
}
