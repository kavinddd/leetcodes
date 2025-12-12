fn main() {
    let q1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("{}", solve(q1));
}

fn solve(array: Vec<i32>) -> i32 {
    if array.len() <= 0 {
        return 0;
    }
    if array.len() < 2 {
        return *array.first().unwrap();
    }

    let mut fst_pnt = 0;
    let mut lst_pnt = array.len() - 1;
    let mut max_area = 0;

    while fst_pnt < lst_pnt {
        let n1 = array.get(fst_pnt).unwrap();
        let n2 = array.get(lst_pnt).unwrap();

        let h = if n1 > n2 { n2 } else { n1 };
        let w: i32 = (lst_pnt - fst_pnt).try_into().unwrap();

        let new_area = h * w;

        if new_area > max_area {
            max_area = new_area
        }

        if n1 < n2 {
            fst_pnt += 1;
        } else {
            lst_pnt -= 1;
        }
    }

    max_area
}
