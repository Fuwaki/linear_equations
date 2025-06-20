mod fraction;
use std::io;
use facet_pretty::FacetPretty;

fn gussian_elimination(matrix:&Vec<Vec<fraction::fraction>> )->Vec<fraction::fraction> {
    let add_k_row=|k:fraction::fraction,origin:usize,dest:usize,mat:&mut Vec<Vec<fraction::fraction>>|{
        // println!("{k:?}");
        for i in 0..mat.first().unwrap().len(){
            mat[dest][i]=mat[dest][i]+mat[origin][i]*k;
        }
    };
    let mul_k_row=|k:fraction::fraction,row:usize,mat:&mut Vec<Vec<fraction::fraction>>|{
        for i in 0..mat.first().unwrap().len(){
            mat[row][i]=mat[row][i]*k;
        }
    };
    let mut m = matrix.iter().map(|row| row.to_vec()).collect::<Vec<Vec<fraction::fraction>>>();
    // println!("{:?}",m);

    for i in 0..matrix.len()-1{
        for j in i+1..matrix.len(){
            add_k_row(-m[j][i]/m[i][i],i,j,&mut m);
        }
    }
    // println!("{:?}",m);
    for i in (1..matrix.len()).rev(){
        for j in (0..i).rev(){
            add_k_row(-m[j][i]/m[i][i],i,j,&mut m);
        }
    }
    // println!("{:?}",m);
    for i in 0..matrix.len(){
        mul_k_row(fraction::fraction::new(1, 1)/m[i][i],i,&mut m);
    }
    m.into_iter().map(|x|*(x.last().unwrap())).collect()
}
fn main(){
    let mut input = String::new();
    println!("请输入一个整数 n:");
    io::stdin().read_line(&mut input).expect("读取失败");
    let n: usize = input.trim().parse().expect("请输入有效的整数");
    let mut matrix = vec![vec![0.0; n+1]; n];     // 创建一个 n 行 n+1 列的矩阵
    println!("请输入增广矩阵的每一行（用空格分隔，每行 {} 个数）:", n + 1);
    for row in matrix.iter_mut() {
        input.clear();
        io::stdin().read_line(&mut input).expect("读取失败");
        let parsed_row: Vec<f64> = input
            .split_whitespace()
            .map(|x| x.parse().expect("请输入有效的数"))
            .collect();
        if parsed_row.len() != n + 1 {
            panic!("每行必须输入 {} 个数", n + 1);
        }
        *row = parsed_row;
    }


    let matrix_fraction: Vec<Vec<fraction::fraction>> = matrix
        .iter()
        .map(|row| row.iter().map(|&x| fraction::fraction::from(x)).collect())
        .collect();

    let result = gussian_elimination(&matrix_fraction).iter().map(|x|x.to_num::<f64>()).collect::<Vec<f64>>();
    println!("结果: {}", result.pretty());
    
}
