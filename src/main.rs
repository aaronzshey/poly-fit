use nalgebra::DMatrix;

fn main() {
    let q: Vec<i32> = vec![1, 8, 27, 64, 125];

    let mut prev = diff(&q);
    let mut next = diff(&q);
//test
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
    println!("{}", mt[(0,0)]);

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


fn rref(mut mtx: DMatrix<f32>) -> () {

    //declare a new matrix to operate on
    //the old matrix is left unchanged

    let result = &mut mtx;


    let lead = 0;
    let row_count = result.nrows();
    let column_count = result.ncols();

    for r in 0..row_count {
        if column_count <= lead {
            return;
        }

        let mut i = r;

        while result[(i, lead)] == 0.0 {
            i += 1;

            if row_count == i {
                i = r;
                if column_count == lead {
                    return;
                }
            }

        }
        //std::mem::swap(result.row(i), result.row(r))
        //swap rows i and r?

        

    }
}