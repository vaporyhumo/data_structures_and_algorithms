fn bubble_sort(list: &mut [usize]) {
  println!("{list:#?}");
  match list {
    [] => {}
    [_] => {}
    _ => {
      let len = list.len();
      println!("len: {len:?}");
      for _j in 0..len {
        for i in 1..len {
          if list[i-1] > list[i] {
            list.swap(i-1, i);
          }
        }
      }
    }
  }
}

// fn bubble_sort1(list: &mut [usize]) {
//   println!("{list:#?}");
//   match list {
//     [] => {}
//     [_] => {}
//     _ => {
//       let len = list.len();
//       println!("len: {len:?}");
//       for _ in 0..len {
//         for i in 1..len {
//           if list[i-1] > list[i] {
//             list.swap(i-1, i);
//           }
//         }
//       }
//     }
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sort_zero_items() {
    let mut list = vec![];
    bubble_sort(&mut list);
    assert_eq!(list, vec![]);
  }

  #[test]
  fn sort_one_item() {
    let mut list = vec![1];
    bubble_sort(&mut list);
    assert_eq!(list, vec![1]);
  }

  #[test]
  fn sort_multiple_items() {
    let mut list = vec![2, 1];
    bubble_sort(&mut list);
    assert_eq!(list, vec![1, 2]);

    let mut list = vec![1, 2];
    bubble_sort(&mut list);
    assert_eq!(list, vec![1, 2]);

    let mut list = vec![3, 1, 2];
    bubble_sort(&mut list);
    assert_eq!(list, vec![1, 2, 3]);

    let mut list = vec![42, 7, 23, 4, 15, 96, 12, 3, 89, 53];
    bubble_sort(&mut list);
    assert_eq!(list, vec![3, 4, 7, 12, 15, 23, 42, 53, 89, 96]);
  }
}
