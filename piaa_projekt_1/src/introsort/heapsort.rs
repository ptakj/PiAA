pub fn left_and_right_child_index (i: usize) -> (usize, usize)
{
    (2*i+1, 2*i+2)
}

pub fn last_parent<T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T]) -> usize {
    if to_max_heap.len() == 1{
        return 0;
    } 
    (to_max_heap.len()-2)/2
}

pub fn maxx <T: std::cmp::PartialOrd + Clone + Default + Copy>(a: T, b: T, c: T)-> T{
    if(a >=b && a>=c){a} else
    if(b>=a && b>=c){b} else
    {c}
}


pub fn heapify<T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T], current: usize) {
    let ln = to_max_heap.len();
    let (l, r) = left_and_right_child_index(current);

    if r < ln {
        let mx = maxx(to_max_heap[l], to_max_heap[r], to_max_heap[current]);
        if mx == to_max_heap[l] {
            to_max_heap.swap(current, l);
            sink_down(to_max_heap, l);
        } else if mx == to_max_heap[r] {
            to_max_heap.swap(current, r);
            sink_down(to_max_heap, r);
        }
    } else if l < ln {
        if to_max_heap[l] > to_max_heap[current] {
            to_max_heap.swap(l, current);
            sink_down(to_max_heap, l);
        }
    }
}

pub fn sink_down<T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T], current: usize) {
    let ln = to_max_heap.len();
    let (l, r) = (2 * current + 1, 2 * current + 2);
    let mut largest = current;

    if l < ln && to_max_heap[l] > to_max_heap[largest] {
        largest = l;
    }

    if r < ln && to_max_heap[r] > to_max_heap[largest] {
        largest = r;
    }

    if largest != current {
        to_max_heap.swap(current, largest);
        sink_down(to_max_heap, largest);
    }
}
pub fn my_heap_sort<T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T]) {
    let ln = to_max_heap.len();
    if ln < 2 {
        return;
    }


    for i in (0..=ln / 2).rev() {
        heapify(to_max_heap, i);
    }

    for i in (1..ln).rev() {
        to_max_heap.swap(0, i);
        heapify(&mut to_max_heap[..i], 0);
    }
}
