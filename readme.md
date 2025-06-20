# 一个解线性方程组的玩意

输入未知数个数 再按提示输入增广矩阵即可

如下对应$\begin{bmatrix}
1 & 1 & 4\\
1 & 4 & 1\\
1 & 9 & 8\\
\end{bmatrix}
\begin{bmatrix}
x_1\\
x_2\\
x_3
\end{bmatrix}
=\begin{bmatrix}
5\\
9\\
1
\end{bmatrix}\Rightarrow \begin{bmatrix}
x_1\\
x_2\\
x_3
\end{bmatrix}=\begin{bmatrix}
9.778\\
0.111\\
-1.222
\end{bmatrix}$

![alt text](image.png)

```
$ cargo run --release
请输入一个整数 n:
3
请输入增广矩阵的每一行（用空格分隔，每行 4 个数）:
1 1 4 5
1 4 1 9 
1 9 8 1
结果: Vec<f64> [
  9.777777777777779,
  0.1111111111111111,
  -1.2222222222222223,
]

```
