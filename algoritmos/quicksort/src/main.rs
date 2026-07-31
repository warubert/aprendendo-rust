use std::convert::TryInto;

fn main() {
    let arr = [10, 5, 2, 3, 1, 4, 6, 9, 8, 7];
    let sorted = quick_sort(&arr);
    println!("Sorted array: {:?}", sorted);
}

fn quick_sort<const N: usize>(arr: &[i32; N]) -> [i32; N] {
    fn sort_slice(arr: &[i32]) -> Vec<i32> {
        if arr.len() <= 1 {
            return arr.to_vec();
        }

        let pivot = arr[0];
        let menores: Vec<i32> = arr[1..]
            .iter()
            .copied()
            .filter(|x| *x < pivot)
            .collect();

        let maiores: Vec<i32> = arr[1..]
            .iter()
            .copied()
            .filter(|x| *x >= pivot)
            .collect();

        let mut v = sort_slice(&menores);
        v.push(pivot);
        v.extend(sort_slice(&maiores));
        v
    }

    sort_slice(arr)
        .try_into()
        .expect("tamanho diferente do esperado")
}