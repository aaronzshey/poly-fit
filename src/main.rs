use nalgebra::DMatrix;

fn main() {
    let q: Vec<f32> = vec![1.0, 8.0, 27.0, 64.0, 125.0];

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
    let mut vals = (1..=degree as i32)
        .flat_map(|x: i32| (1..=degree).map(move |y| y.pow(x as u32)))
        .map(|x| x as f32)
        .collect::<Vec<_>>();
    let mut augmenter: Vec<f32> = q[0..degree as usize].to_vec();
    vals.append(&mut augmenter);
    let mut mt = DMatrix::from_vec(degree as usize, degree + 1 as usize, vals);

    println!("{}", mt);
  
    println!("{}", rref(&mut mt));

}

fn vec_homogeneous(v: &Vec<f32>) -> bool {
    return v.iter().all(|&x| x == v[0]);
    //.all predicates truth, .any uses false
}

fn diff(v: &Vec<f32>) -> Vec<f32> {
    return v[1..]
        .iter()
        .enumerate()
        .map(|x| x.1 - v[x.0])
        .collect::<Vec<_>>();
}

fn rref(matrix: &mut DMatrix<f32>) -> &DMatrix<f32> {
    let matrix_out = matrix;
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
            println!("before: {}", matrix_out);
            let temp = matrix_out[(r,j)];
            matrix_out[(r,j)] = matrix_out[(i,j)];
            matrix_out[(i,j)] = temp;
            println!("after: {}", matrix_out);
        }
        let divisor = matrix_out[(r,pivot)];
        
        if divisor != 0.0 {
            for j in 0..column_count {
                //this part takes the divisor and divides the whole row by it 
                //thhe pivot travels diagonally down the matrix 
                println!("dividing {} by {}",matrix_out[(r,j)], divisor);
                matrix_out[(r,j)] = matrix_out[(r,j)] / divisor;
                println!("to get {}", matrix_out);
                
            }
        }
        for j in 0..row_count {
            if j != r {
                let hold = matrix_out[(j,pivot)];
                println!("{}", matrix_out);
                for k in 0..column_count {
                    println!("subtracting {} from {} and {} multiplied", matrix_out[(j,k)], hold, matrix_out[(r,k)]);
                    matrix_out[(j,k)] = matrix_out[(j,k)] - ( hold * matrix_out[(r,k)]);
                    println!("to get {}", matrix_out);
                }
            }
        }
        pivot = pivot + 1;
    }
    matrix_out
}

/*
use nalgebra::Matrix3;
use nalgebra::Matrix3x1;


fn main() {
    let mt = Matrix3::from_vec( [7.0, 14.0, -7.0, -2.0, -7.0, 11.0, 1.0, -3.0, 18.0].to_vec());
    let sols = Matrix3x1::from_vec( [12.0,17.0,5.0].to_vec() );
    println!("{}", mt);
    println!("{}", mt.lu().solve(&sols).unwrap());
}

*/
