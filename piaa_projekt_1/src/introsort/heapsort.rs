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


pub fn heapify <T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T], current: usize)
{

    let (l, r): (usize, usize) = left_and_right_child_index(current);    
    if r < to_max_heap.len(){
        let mut mx  = maxx(to_max_heap[l], to_max_heap[r], to_max_heap[current]);
        if   mx == to_max_heap[l] {
            to_max_heap.swap(current,l);
            sink_down(to_max_heap,l);
        }
        else
        {
            if mx == to_max_heap[r] {
                to_max_heap.swap(current, r);
                sink_down(to_max_heap,r);
            }
        }
    }
    else {
        let mut mx  = to_max_heap[current];
        if mx < to_max_heap[l]{
            to_max_heap.swap(l,current);
        }
    }
    if current > 0 {
    heapify(to_max_heap, current-1)
    }
}

pub fn sink_down<T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T], current: usize){
    if current >= to_max_heap.len(){
        return
    }

    let (l, r): (usize, usize) = left_and_right_child_index(current);    
    if r < to_max_heap.len(){
        let mut mx  = maxx(to_max_heap[l], to_max_heap[r], to_max_heap[current]);
        if   mx == to_max_heap[l] {
            to_max_heap.swap(current,l);
            sink_down(to_max_heap, l);            
        }
        else
        {
            if mx == to_max_heap[r] {
                to_max_heap.swap(current, r);
                sink_down(to_max_heap, r);
            }
        }
    }
    else {
        let mut mx  = to_max_heap[current];
        if mx < to_max_heap[l]{
            to_max_heap.swap(l,current);
        }
    }
}

pub fn my_heap_sort <T: std::cmp::PartialOrd + Clone + Default + Copy>(to_max_heap: &mut [T])
{
    let ln = to_max_heap.len();
    let mut placeholder: usize = to_max_heap.len()-1;
    for i in (0..ln).rev() {
        let lp = last_parent(to_max_heap);
        heapify(to_max_heap, lp);
        to_max_heap.swap(0, i);
        placeholder = placeholder -1;
    } 
}


