const N: usize = 5;
const MAX_SWAPS_QUADRATIC: usize = (N - 1) * N / 2;
const MAX_SWAPS_LINEAR: usize = N - 1;
const MAX_SWAPS_LINEAR_CYCLE_SORT: usize = N;
const MAX_SWAPS_QUADRATIC_QUICKSORT: usize = N * (N + 1) / 2 - 1;

const QUADRATIC_SWAPS_STAGES: usize = 1 + MAX_SWAPS_QUADRATIC;
const LINEAR_SWAPS_STAGES: usize = 1 + MAX_SWAPS_LINEAR;
const LINEAR_SWAPS_CYCLE_SORT_STAGES: usize = 1 + MAX_SWAPS_LINEAR_CYCLE_SORT;
const QUADRATIC_QUICKSORT_SWAPS_STAGES: usize = 1 + MAX_SWAPS_QUADRATIC_QUICKSORT;

#[derive(Debug)]
pub struct SortRecording<const MAX_STAGES: usize> {
    pub stages: [[u32; N]; MAX_STAGES],
    pub count: usize,
}

pub type SortRecordingQuadradtic = SortRecording<{ QUADRATIC_SWAPS_STAGES }>;
pub type SortRecordingLinear = SortRecording<{ LINEAR_SWAPS_STAGES }>;
pub type SortRecordingQuadraticQuickSort = SortRecording<{ QUADRATIC_QUICKSORT_SWAPS_STAGES }>;
pub type SortRecordingLinearCycleSort = SortRecording<{ LINEAR_SWAPS_CYCLE_SORT_STAGES }>;

impl<const MAX_STAGES: usize> SortRecording<MAX_STAGES> {
    fn new() -> Self {
        SortRecording {
            stages: [[0; N]; MAX_STAGES],
            count: 0,
        }
    }

    fn push(&mut self, input: &[u32; N]) {
        self.stages[self.count].copy_from_slice(input);
        self.count += 1;
    }

    pub fn get_stage(&self, i: usize) -> [u32; N] {
        self.stages[if i < self.count { i } else { self.count - 1 }]
    }

}

pub fn bubble_sort(input: &mut [u32; N]) -> SortRecordingQuadradtic {
    let mut sort_recording = SortRecordingQuadradtic::new();
    sort_recording.push(&input);
    let mut input_len = input.len();
    while input_len > 1 {
        let mut sorted_len = 0;
        for i in 1..=input_len - 1 {
            if input[i - 1] > input[i] {
                input.swap(i - 1, i);
                sort_recording.push(&input);
                sorted_len = i;
            }
        }
        input_len = sorted_len;
    }
    return sort_recording;
}

pub fn insertion_sort(input: &mut [u32; N]) -> SortRecordingQuadradtic {
    let mut sort_recording = SortRecordingQuadradtic::new();
    sort_recording.push(&input);
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j, j - 1);
            sort_recording.push(&input);
            j -= 1;
        }
    }
    return sort_recording;
}

pub fn selection_sort(input: &mut [u32; N]) -> SortRecordingLinear {
    let mut sort_recording = SortRecordingLinear::new();
    sort_recording.push(&input);
    let input_length = input.len();
    for i in 0..input_length {
        let mut min_i = i;
        for j in (i + 1)..input_length {
            if input[j] < input[min_i] {
                min_i = j;
            }
        }
        if min_i != i {
            input.swap(i, min_i);
            sort_recording.push(&input);
        }
    }
    return sort_recording;
}

/* fn main() {
    println!("{:?}", cycle_sort(&mut [3, 5, 1, 2, 4]));
    println!("{:?}", bubble_sort(&mut [3, 5, 1, 2, 4]));
    println!("{:?}", quick_sort(&mut [3, 5, 1, 2, 4]));
    println!("{:?}", insertion_sort(&mut [3, 5, 1, 2, 4]));
    println!("{:?}", selection_sort(&mut [3, 5, 1, 2, 4]));
}
 */

pub fn cycle_sort(input: &mut [u32; N]) -> SortRecordingLinearCycleSort {
    let mut sort_recording = SortRecordingLinearCycleSort::new();
    sort_recording.push(&input);
    for cycle_start in 0..input.len() - 1 {
        let mut item = input[cycle_start];
        let mut pos = cycle_start;
        for i in (cycle_start + 1)..input.len() {
            if input[i] < item {
                pos += 1;
            }
        }
        if pos == cycle_start {
            continue;
        }
        while item == input[pos] {
            pos += 1;
        }
        let tmp = input[pos];
        input[pos] = item;
        item = tmp;
        sort_recording.push(&input);
        while pos != cycle_start {
            pos = cycle_start;
            for i in (cycle_start + 1)..input.len() {
                if input[i] < item {
                    pos += 1;
                }
            }
            while item == input[pos] {
                pos += 1;
            }
            let tmp = input[pos];
            input[pos] = item;
            item = tmp;
            sort_recording.push(&input);
        }
    }
    return sort_recording;
}

pub fn quick_sort(input: &mut [u32; N]) -> SortRecordingQuadraticQuickSort {
    let mut sort_recording = SortRecordingQuadraticQuickSort::new();
    sort_recording.push(&input);
    quicksort(input, 0, input.len() - 1, &mut sort_recording);
    return sort_recording;
}

fn quicksort(
    input: &mut [u32; N],
    lo: usize,
    hi: usize,
    sort_recording: &mut SortRecordingQuadraticQuickSort,
) -> () {
    if lo < hi {
        let p = partition_lomuto_scheme(input, lo, hi, sort_recording);
        if p > 0 {
            quicksort(input, lo, p - 1, sort_recording);
        }
        quicksort(input, p + 1, hi, sort_recording);
    }
    return;
}

fn partition_lomuto_scheme(
    input: &mut [u32; N],
    lo: usize,
    hi: usize,
    sort_recording: &mut SortRecordingQuadraticQuickSort,
) -> usize {
    let pivot = input[hi];
    let mut pivot_index = lo;
    for i in lo..=hi - 1 {
        if input[i] <= pivot {
            input.swap(pivot_index, i);
            sort_recording.push(&input);
            pivot_index += 1;
        }
    }
    input.swap(pivot_index, hi);
    sort_recording.push(&input);
    pivot_index
}

fn is_sorted(input: &[u32; N]) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] > input[i + 1] {
            return false;
        }
    }
    return true;
}
