trait SortStrategy<T> {
    fn sort(&self, slice: &mut [T]);
}

struct BubbleSort;

impl<T: PartialOrd> SortStrategy<T> for BubbleSort {
    fn sort(&self, slice: &mut [T]) {
        let len = slice.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1)
                }
            }
        }
    }
}

struct Quiksort;

impl<T: PartialOrd> SortStrategy<T> for Quiksort {
    fn sort(&self, slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }
        let pivot = partition(slice);
        let (left, right) = slice.split_at_mut(pivot);
        self.sort(left);
        self.sort(&mut right[1..]);
    }
}

fn partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot = len - 1;
    let mut i = 0;
    for j in 0..len {
        if slice[j] < slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, pivot);
    i
}

struct Sorter<T> {
    strategy: Box<dyn SortStrategy<T>>,
}

impl<T> Sorter<T> {
    fn new(strategy: Box<dyn SortStrategy<T>>) -> Self {
        Self { strategy }
    }

    fn sort(&self, slice: &mut [T]) {
        self.strategy.sort(slice)
    }
}

pub fn sorting() {
    let mut x = vec![3, 3, 2, 4, 0, 9, 1, 5, 4];
    let quik = Sorter::new(Box::new(Quiksort));
    quik.sort(&mut x);
    println!("my quik sort: {:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut data = vec![3, 2, 1, 5, 4];
        let bubble_sort = Sorter::new(Box::new(BubbleSort));
        bubble_sort.sort(&mut data);
        println!("bubble sort result: {:?}", data)
    }

    #[test]
    fn test_quiksort() {
        let mut data = vec![5, 7, 2, 3, 4, 1];
        let quick_sort = Sorter::new(Box::new(Quiksort));
        quick_sort.sort(&mut data);
        println!("quick sort result: {:?}", data);
    }
}
