use cube_hammer::Cube3by3;
use cube_hammer::Move::*;

fn main() {
    let mut cube = Cube3by3::new();
    cube.apply(&[
        D2, R2, U2, L2, Dp, R2, Dp, //B2, D2, F2, Dp, Fp, Rp, Fp, Dp, R, U2, Fp, Lp, Rp,
    ]);
    println!("{}", cube);
    let solution = cube.solve(6);
    println!("{:?}", solution);
    assert!(solution.is_some_and(|s| !s.is_empty()));
    assert!(cube.is_solved());
}
