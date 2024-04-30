struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}

struct List<T> {
  first: Option<Node<T>>,
  size: usize,
}

impl<T> List<T> {
  fn empty() -> Self {
    Self {
      first: None,
      size: 0,
    }
  }

  fn is_empty(&self) -> bool {
    self.first.is_none()
  }

  fn first(&self) -> Option<&T> {
    self.first.as_ref().map(|node| &node.data)
  }

  fn get(&self, index: usize) -> Option<&T> {
    self.first.as_ref().and_then(|n| n.get(index))
  }

  fn last(&self) -> Option<&T> {
    self.get(self.size - 1)
  }

  fn insert_first(&mut self, data: T) {
    let next = std::mem::take(&mut self.first);
    let node = Node {
      data,
      next: next.map(Box::new),
    };
    self.first = Some(node);
    self.size += 1;
  }

  fn insert_last(&mut self, data: T) {
    match &mut self.first {
      None => self.first = Some(Node { data, next: None }),
      Some(node) => node.attach(data),
    }
    self.size += 1; 
  }

  fn remove_first(&mut self) {
  }

  fn remove_last(&mut self) {
  }
}

impl<T> Node<T> {
  fn get(&self, index: usize) -> Option<&T> {
    match index {
      0 => Some(&self.data),
      a => self.next.as_ref().and_then(|n| n.get(a - 1)),
    }
  }

  fn attach(&mut self, data: T) {
    match &mut self.next {
      None => self.next = Some(Box::new(Node { data, next: None })),
      Some(next) => next.attach(data),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_empty_list() {
    let list: List<usize> = List::empty();
    let first = list.first();
    assert_eq!(first, None);
    assert_eq!(list.size, 0);
    assert!(list.is_empty());
  }

  #[test]
  fn insert_first() {
    let mut list: List<usize> = List::empty();
    list.insert_first(0);
    assert_eq!(list.first(), Some(0).as_ref());
    assert_eq!(list.last(), Some(0).as_ref());
    assert_eq!(list.size, 1);
    assert!(!list.is_empty());

    list.insert_first(1);
    assert_eq!(list.first(), Some(1).as_ref());
    assert_eq!(list.last(), Some(0).as_ref());
    assert_eq!(list.size, 2);
  }

  #[test]
  fn insert_last() {
    let mut list: List<usize> = List::empty();
    list.insert_first(0);
    assert_eq!(list.first(), Some(0).as_ref());
    assert_eq!(list.last(), Some(0).as_ref());
    assert_eq!(list.size, 1);
    assert!(!list.is_empty());

    list.insert_last(1);
    assert_eq!(list.first(), Some(0).as_ref());
    assert_eq!(list.last(), Some(1).as_ref());
    assert_eq!(list.size, 2);
  }

  #[test]
  fn get_from_index() {
    let list: List<usize> = List::empty();
    assert_eq!(list.get(0), None);

    let mut list: List<usize> = List::empty();
    list.insert_first(0);
    assert_eq!(list.get(0), Some(0).as_ref());

    list.insert_first(1);
    assert_eq!(list.get(0), Some(1).as_ref());
    assert_eq!(list.get(1), Some(0).as_ref());
  }
}
