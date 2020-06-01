use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<I> {
    current: Option<usize>,
    stack: Vec<I>
}

pub struct StackIntoIter<I> {
    count: usize,
    current: Option<usize>,
    iter: std::vec::IntoIter<I>
}

impl<I> Default for Stack<I> {
    fn default() -> Self {
        Self {
            current: None,
            stack: Vec::new(),
        }
    }
}

impl<I> From<Vec<I>> for Stack<I> {
    fn from(stack: Vec<I>) -> Self {
        if stack.is_empty() { return Stack::default() }
        Self { current: Some(0), stack }
    }
}

impl<I> IntoIterator for Stack<I> {
    type Item = (bool, I);
    type IntoIter = StackIntoIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        StackIntoIter { count: 0, current: self.current, iter: self.stack.into_iter() }
    }
}

impl<I> Iterator for StackIntoIter<I> {
    type Item = (bool, I);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        self.count = self.count + 1;
        Some((self.current? == self.count - 1, item))
    }
}

impl<I> Deref for Stack<I> {
    type Target = Vec<I>;

    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl<I> FromIterator<(bool, I)> for Stack<I> {
    fn from_iter<T: IntoIterator<Item=(bool, I)>>(iter: T) -> Self {
        let mut count = 0;
        let mut current = None;
        let stack = iter.into_iter()
            .map(|(is_current, item)| {
                if is_current { current.replace(count); }
                count = count + 1;
                item
            })
            .collect::<Vec<I>>();
        Self { current, stack }
    }
}

impl<I> Stack<I> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_current<F: FnOnce(I) -> I>(mut self, replace: F) -> Self {
        if let Some(current) = self.current {
            let old_item = self.stack.remove(current);
            let new_item = replace(old_item);
            self.stack.insert(current, new_item);
        }
        self
    }

    pub fn get_current(&self) -> Option<&I> {
        self.current.as_ref()
            .and_then(|current| self.stack.get(*current))
    }

    pub fn add(mut self, item: I) -> Self {
        let current = self.current.unwrap_or(0);
        self.stack.insert(current, item);
        self.current = Some(current);
        self
    }

    pub fn set_current(mut self, index: usize) -> Self {
        if self.stack.len() > index {
            self.current = Some(index);
        }
        self
    }

    pub fn remove_by<F>(mut self, predicate: F) -> Self where F: Fn(&I) -> bool {
        self.stack.iter()
            .position(|item| predicate(item))
            .map(|item| self.stack.remove(item));
        self
    }

    pub fn next(mut self) -> Self {
        self.current = self.current.map(|current| {
            if current == self.stack.len() - 1 { 0 } else { current + 1 }
        });
        self
    }

    pub fn previous(mut self) -> Self {
        self.current = self.current.map(|current| {
            if current == 0 { self.stack.len() - 1 } else { current - 1 }
        });
        self
    }
}

#[cfg(test)]
mod stack_tests {
    use crate::stack::Stack;

    #[test]
    fn test() {
        let stack = Stack::new();
        let stack = stack.add(1);
        let stack = stack.add(2);
        let stack = stack.add(3);
        let stack = stack.into_iter()
            .collect::<Stack<i32>>();
        let expected: Stack<i32> = vec![3, 2, 1].into();
        assert_eq!(expected, stack);
    }
}
