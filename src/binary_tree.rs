#[derive(Debug)]
pub struct Tree<T> {
  root: Option<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
  left: Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
  data: T,
}

impl<T> Node<T> {
  fn left(&self) -> Option<&T> {
    self.left.as_ref().map(|n| &n.data)
  }

  fn right(&self) -> Option<&T> {
    self.right.as_ref().map(|n| &n.data)
  }


  fn just_add(&mut self, data: T) {
    match (&mut self.left, &self.right) {
      (None, _) => {
        self.add_left(data);
      }
      (Some(_), None) => {
        self.add_right(data);
      }
      (Some(left), Some(_)) => left.just_add(data),
    }
  }

  fn add_right(&mut self, data: T) {
    self.right = Some(Box::new(Node {
      data,
      left: None,
      right: None,
    }))
  }

  fn add_left(&mut self, data: T) {
    self.left = Some(Box::new(Node {
      data,
      left: None,
      right: None,
    }))
  }
}

impl<T> Tree<T> {
  pub fn empty() -> Self {
    Self { root: None }
  }

  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  pub fn root(&self) -> Option<&T> {
    self.root.as_ref().map(|r| &r.data)
  }

  pub fn root_node(&self) -> Option<&Node<T>> {
    self.root.as_ref()
  }

  pub fn add_root(&mut self, data: T) {
    match self.root {
      None => {
        self.root = Some(Node {
          data,
          left: None,
          right: None,
        })
      }
      _ => panic!(),
    }
  }

  fn add_root_and_descend_left(&mut self, data: T) {
    let old_root = std::mem::take(&mut self.root);
    self.root = Some(Node {
      data,
      left: old_root.map(Box::new),
      right: None,
    })
  }

  fn add_root_and_descend_right(&mut self, data: T) {
    let old_root = std::mem::take(&mut self.root);
    self.root = Some(Node {
      data,
      left: None,
      right: old_root.map(Box::new),
    })
  }

  pub fn just_add(&mut self, data: T) {
    match &mut self.root {
      None => self.add_root(data),
      Some(root) => root.just_add(data),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty() {
    let tree: Tree<usize> = Tree::empty();
    assert!(tree.is_empty());
  }

  #[test]
  fn add_root() {
    let mut tree: Tree<usize> = Tree::empty();
    assert_eq!(tree.root(), None);
    tree.add_root(0);
    assert_eq!(tree.root(), Some(0).as_ref());
    assert!(!tree.is_empty());
    println!("{tree:?}");
  }

  #[test]
  fn just_add() {
    let mut tree: Tree<usize> = Tree::empty();
    tree.just_add(0);
    assert_eq!(tree.root(), Some(0).as_ref());
    tree.just_add(1);
    tree.just_add(2);
    assert_eq!(tree.root_node().and_then(|r| r.left()), Some(1).as_ref());
    assert_eq!(tree.root_node().and_then(|r| r.right()), Some(2).as_ref());
  }
}
