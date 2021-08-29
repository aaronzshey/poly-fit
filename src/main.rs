use nalgebra::DMatrix;

fn main() {
    let q: Vec<i32> = vec![1, 8, 27, 64, 125];

    let mut prev = diff(&q);
    let mut next = diff(&q);
    let mut j = 0;

    let degree = loop {
        if vec_homogeneous(&next) == true || j == q.len() - 1 {
            j += 1;
            break j;
        } else {
            std::mem::swap(&mut next, &mut prev);
            next = diff(&prev);
        }

        j += 1;
    };

    println!("function degree is {}", degree);

    //augment with this
    println!("{:?}", &q[0..degree]);

    //build matrix to solve
    let vals = (1..=degree as i32)
        .flat_map(|x: i32| (1..=degree).map(move |y| y.pow(x as u32)))
        .map(|x| x as f32)
        .collect::<Vec<_>>();
    let mt = DMatrix::from_vec(degree as usize, degree as usize, vals);

    println!("{}", mt);
    println!("{}", mt[(0, 0)]);

  
    println!("{:?}", rref(&mt));

    //rref(mt);
}

fn vec_homogeneous(v: &Vec<i32>) -> bool {
    return v.iter().all(|&x| x == v[0]);
    //.all predicates truth, .any uses false
}

fn diff(v: &Vec<i32>) -> Vec<i32> {
    return v[1..]
        .iter()
        .enumerate()
        .map(|x| x.1 - v[x.0])
        .collect::<Vec<_>>();
}

fn rref(matrix: &DMatrix<f32>) -> &DMatrix<f32> {
    let mut aab = matrix;
    let mut b = &mut aab;
    let mut matrix_out: &mut DMatrix<f32> = b;
    let mut pivot = 0;
    let row_count = matrix_out.nrows();
    let column_count = matrix_out.ncols();
 
    for r in 0..row_count {
        if column_count <= pivot {
            break;
        }
        let mut i = r;

        while matrix_out[(i, pivot)] == 0.0 {
            i = i+1;
            if i == row_count {
                i = r;
                pivot = pivot + 1;
                if column_count == pivot {
                    pivot = pivot - 1;
                    break;
                }
            }
        }
        for j in 0..row_count {
            let temp = matrix_out[(r,j)];
            matrix_out[(r,j)] = matrix_out[(i,j)];
            matrix_out[(i,j)] = temp;
        }
        let divisor = matrix_out[(r,pivot)];
        if divisor != 0.0 {
            for j in 0..column_count {
                matrix_out[(r,j)] = matrix_out[(r,j)] / divisor;
            }
        }
        for j in 0..row_count {
            if j != r {
                let hold = matrix_out[(j,pivot)];
                for k in 0..column_count {
                    matrix_out[(j,k)] = matrix_out[(j,k)] - ( hold * matrix_out[(r,k)]);
                }
            }
        }
        pivot = pivot + 1;
    }
    matrix_out
}