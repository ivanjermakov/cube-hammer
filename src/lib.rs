use core::fmt;
use std::collections::HashMap;
use Move::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Side {
    R,
    L,
    U,
    D,
    F,
    B,
}

impl Side {
    pub fn color(&self) -> String {
        String::from(match self {
            Side::R => "🟥",
            Side::L => "🟧",
            Side::U => "⬜",
            Side::D => "🟨",
            Side::F => "🟩",
            Side::B => "🟦",
        })
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color())
    }
}

pub type Face = [Side; 8];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Cube3by3 {
    /// UDLFRB order, clockwise stickers from top left
    faces: [Face; 6],
}

const SOLVED: Cube3by3 = Cube3by3::new();

impl Cube3by3 {
    pub const fn new() -> Cube3by3 {
        Cube3by3 {
            faces: [
                [Side::U; 8],
                [Side::D; 8],
                [Side::L; 8],
                [Side::F; 8],
                [Side::R; 8],
                [Side::B; 8],
            ],
        }
    }

    pub fn apply(&mut self, scramble: &[Move]) {
        for m in scramble {
            self.apply_move(m);
        }
    }

    #[rustfmt::skip]
    pub fn apply_move(&mut self, m: &Move) {
        let fs = self.faces;
        match m {
            R => {
                self.faces = [
                    [fs[0][0], fs[0][1], fs[3][2], fs[3][3], fs[3][4], fs[0][5], fs[0][6], fs[0][7]],
                    [fs[1][0], fs[1][1], fs[5][6], fs[5][7], fs[5][0], fs[1][5], fs[1][6], fs[1][7]],
                    [fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5], fs[2][6], fs[2][7]],
                    [fs[3][0], fs[3][1], fs[1][2], fs[1][3], fs[1][4], fs[3][5], fs[3][6], fs[3][7]],
                    [fs[4][6], fs[4][7], fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5]],
                    [fs[0][4], fs[5][1], fs[5][2], fs[5][3], fs[5][4], fs[5][5], fs[0][2], fs[0][3]],
                ]
            }
            Rp => for _ in 0..3 {
                self.apply_move(&R);
            }
            R2 => for _ in 0..2 {
                self.apply_move(&R);
            },
            L => {
                self.faces = [
                    [fs[5][4], fs[0][1], fs[0][2], fs[0][3], fs[0][4], fs[0][5], fs[5][2], fs[5][3]],
                    [fs[3][0], fs[1][1], fs[1][2], fs[1][3], fs[1][4], fs[1][5], fs[3][6], fs[3][7]],
                    [fs[2][6], fs[2][7], fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5]],
                    [fs[0][0], fs[3][1], fs[3][2], fs[3][3], fs[3][4], fs[3][5], fs[0][6], fs[0][7]],
                    [fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5], fs[4][6], fs[4][7]],
                    [fs[5][0], fs[5][1], fs[1][6], fs[1][7], fs[1][0], fs[5][5], fs[5][6], fs[5][7]],
                ]
            },
            Lp => for _ in 0..3 {
                self.apply_move(&L);
            },
            L2 => for _ in 0..2 {
                self.apply_move(&L);
            },
            U => {
                self.faces = [
                    [fs[0][6], fs[0][7], fs[0][0], fs[0][1], fs[0][2], fs[0][3], fs[0][4], fs[0][5]],
                    [fs[1][0], fs[1][1], fs[1][2], fs[1][3], fs[1][4], fs[1][5], fs[1][6], fs[1][7]],
                    [fs[3][0], fs[3][1], fs[3][2], fs[2][3], fs[2][4], fs[2][5], fs[2][6], fs[2][7]],
                    [fs[4][0], fs[4][1], fs[4][2], fs[3][3], fs[3][4], fs[3][5], fs[3][6], fs[3][7]],
                    [fs[5][0], fs[5][1], fs[5][2], fs[4][3], fs[4][4], fs[4][5], fs[4][6], fs[4][7]],
                    [fs[2][0], fs[2][1], fs[2][2], fs[5][3], fs[5][4], fs[5][5], fs[5][6], fs[5][7]],
                ]
            },
            Up => for _ in 0..3 {
                self.apply_move(&U);
            },
            U2 => for _ in 0..2 {
                self.apply_move(&U);
            },
            D => {
                self.faces = [
                    [fs[0][0], fs[0][1], fs[0][2], fs[0][3], fs[0][4], fs[0][5], fs[0][6], fs[0][7]],
                    [fs[1][6], fs[1][7], fs[1][0], fs[1][1], fs[1][2], fs[1][3], fs[1][4], fs[1][5]],
                    [fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[5][4], fs[5][5], fs[5][6], fs[2][7]],
                    [fs[3][0], fs[3][1], fs[3][2], fs[3][3], fs[2][4], fs[2][5], fs[2][6], fs[3][7]],
                    [fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[3][4], fs[3][5], fs[3][6], fs[4][7]],
                    [fs[5][0], fs[5][1], fs[5][2], fs[5][3], fs[4][4], fs[4][5], fs[4][6], fs[5][7]],
                ]
            },
            Dp => for _ in 0..3 {
                self.apply_move(&D);
            },
            D2 => for _ in 0..2 {
                self.apply_move(&D);
            },
            F => {
                self.faces = [
                    [fs[0][0], fs[0][1], fs[0][2], fs[0][3], fs[2][2], fs[2][3], fs[2][4], fs[0][7]],
                    [fs[4][6], fs[4][7], fs[4][0], fs[1][3], fs[1][4], fs[1][5], fs[1][6], fs[1][7]],
                    [fs[2][0], fs[2][1], fs[1][0], fs[1][1], fs[1][2], fs[2][5], fs[2][6], fs[2][7]],
                    [fs[3][6], fs[3][7], fs[3][0], fs[3][1], fs[3][2], fs[3][3], fs[3][4], fs[3][5]],
                    [fs[0][6], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5], fs[0][4], fs[0][5]],
                    [fs[5][0], fs[5][1], fs[5][2], fs[5][3], fs[5][4], fs[5][5], fs[5][6], fs[5][7]],
                ]
            },
            Fp => for _ in 0..3 {
                self.apply_move(&F);
            },
            F2 => for _ in 0..2 {
                self.apply_move(&F);
            },
            B => {
                self.faces = [
                    [fs[4][2], fs[4][3], fs[4][4], fs[0][3], fs[0][4], fs[0][5], fs[0][6], fs[0][7]],
                    [fs[1][0], fs[1][1], fs[1][2], fs[1][3], fs[2][6], fs[2][7], fs[2][0], fs[1][7]],
                    [fs[0][2], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5], fs[0][0], fs[0][1]],
                    [fs[3][0], fs[3][1], fs[3][2], fs[3][3], fs[3][4], fs[3][5], fs[3][6], fs[3][7]],
                    [fs[4][0], fs[4][1], fs[1][4], fs[1][5], fs[1][6], fs[4][5], fs[4][6], fs[4][7]],
                    [fs[5][6], fs[5][7], fs[5][0], fs[5][1], fs[5][2], fs[5][3], fs[5][4], fs[5][5]],
                ]
            },
            Bp => for _ in 0..3 {
                self.apply_move(&B);
            },
            B2 => for _ in 0..2 {
                self.apply_move(&B);
            },
        }
    }

    pub fn check_parity(&self) -> bool {
        let dict: HashMap<&Side, i32> =
            self.faces
                .iter()
                .flatten()
                .fold(HashMap::new(), |mut map, color| {
                    *map.entry(color).or_insert(0) += 1;
                    map
                });
        dict.len() == 6 && dict.values().all(|v| *v == 8)
    }

    pub fn is_solved(&self) -> bool {
        self.eq(&SOLVED)
    }

    pub fn solve(&self, move_count: u8) -> Option<Vec<Move>> {
        // Cube3by3::solve_mitm(
        //     *self,
        //     Vec::with_capacity(move_count as usize / 2),
        //     SOLVED,
        //     Vec::with_capacity(move_count as usize / 2),
        //     move_count,
        // )
        Cube3by3::solve_naive(*self, &[], move_count)
    }

    fn solve_naive(state: Self, moves: &[Move], move_count: u8) -> Option<Vec<Move>> {
        if move_count == 0 {
            return None;
        }
        let mut start_move_candidates = Vec::with_capacity(18);
        match moves.last() {
            None => start_move_candidates.extend_from_slice(&Move::values()),
            Some(last) => start_move_candidates.extend_from_slice(&last.next_move_set()),
        };
        for next_move in start_move_candidates {
            let mut start_state = state;
            start_state.apply_move(&next_move);
            let mut moves = moves.to_vec();
            moves.push(next_move);
            if start_state.eq(&SOLVED) {
                return Some(moves);
            }
            let res = Cube3by3::solve_naive(start_state, &moves, move_count - 1);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    fn solve_mitm(
        start_state: Self,
        start_moves: Vec<Move>,
        end_state: Self,
        end_moves: Vec<Move>,
        move_count: u8,
    ) -> Option<Vec<Move>> {
        if start_moves.len() + end_moves.len() > move_count as usize {
            return None;
        }
        println!("{start_moves:?} {end_moves:?}");
        if start_state.eq(&end_state) {
            todo!("solved");
        }
        let mut start_move_candidates = Vec::with_capacity(18);
        match start_moves.last() {
            None => start_move_candidates.extend_from_slice(&Move::values()),
            Some(last) => start_move_candidates.extend_from_slice(&last.next_move_set()),
        };
        let mut end_move_candidates = Vec::with_capacity(18);
        match end_moves.last() {
            None => end_move_candidates.extend_from_slice(&Move::values()),
            Some(last) => end_move_candidates.extend_from_slice(&last.next_move_set()),
        };
        for start_move in start_move_candidates.iter() {
            for end_move in end_move_candidates.iter() {
                let mut start_state = start_state;
                let mut end_state = end_state;
                start_state.apply_move(start_move);
                if start_state.eq(&end_state) {
                    todo!("solved");
                }
                end_state.apply_move(end_move);
                if start_state.eq(&end_state) {
                    todo!("solved");
                }
                let mut start_moves = start_moves.clone();
                start_moves.push(*start_move);
                let mut end_moves = end_moves.clone();
                end_moves.push(*end_move);
                let res = Cube3by3::solve_mitm(
                    start_state,
                    start_moves,
                    end_state,
                    end_moves,
                    move_count,
                );
                if res.is_some() {
                    return res;
                }
            }
        }
        None
    }
}

impl Default for Cube3by3 {
    fn default() -> Self {
        Self::new()
    }
}

/// ```plain
///
///         ⬜⬜⬜
///         ⬜⬜⬜
///         ⬜⬜⬜
///
/// 🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦
/// 🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦
/// 🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦
///
///         🟨🟨🟨
///         🟨🟨🟨
///         🟨🟨🟨
/// ```
impl fmt::Display for Cube3by3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fs = self.faces;
        let mut fmtd = String::new();
        fmtd += &format!("        {}{}{}\n", fs[0][0], fs[0][1], fs[0][2]);
        fmtd += &format!("        {}⬜{}\n", fs[0][7], fs[0][3]);
        fmtd += &format!("        {}{}{}\n\n", fs[0][6], fs[0][5], fs[0][4]);

        fmtd += &format!(
            "{}{}{}  {}{}{}  {}{}{}  {}{}{}\n",
            fs[2][0], fs[2][1], fs[2][2],
            fs[3][0], fs[3][1], fs[3][2],
            fs[4][0], fs[4][1], fs[4][2],
            fs[5][0], fs[5][1], fs[5][2],
        );
        fmtd += &format!("{}🟧{}  {}🟩{}  {}🟥{}  {}🟦{}\n",
            fs[2][7], fs[2][3],
            fs[3][7], fs[3][3],
            fs[4][7], fs[4][3],
            fs[5][7], fs[5][3],
        );
        fmtd += &format!("{}{}{}  {}{}{}  {}{}{}  {}{}{}\n\n",
            fs[2][6], fs[2][5], fs[2][4],
            fs[3][6], fs[3][5], fs[3][4],
            fs[4][6], fs[4][5], fs[4][4],
            fs[5][6], fs[5][5], fs[5][4],
        );

        fmtd += &format!("        {}{}{}\n", fs[1][0], fs[1][1], fs[1][2]);
        fmtd += &format!("        {}🟨{}\n", fs[1][7], fs[1][3]);
        fmtd += &format!("        {}{}{}", fs[1][6], fs[1][5], fs[1][4]);
        write!(f, "{fmtd}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Move {
    R,
    Rp,
    R2,
    L,
    Lp,
    L2,
    U,
    Up,
    U2,
    D,
    Dp,
    D2,
    F,
    Fp,
    F2,
    B,
    Bp,
    B2,
}

impl Move {
    pub fn values() -> [Move; 18] {
        [
            R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2,
        ]
    }

    #[rustfmt::skip]
    pub fn next_move_set(&self) -> [Move; 15] {
        match self {
             R => [L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            Rp => [L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            R2 => [L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
             L => [R, Rp, R2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            Lp => [R, Rp, R2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            L2 => [R, Rp, R2, U, Up, U2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
             U => [R, Rp, R2, L, Lp, L2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            Up => [R, Rp, R2, L, Lp, L2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
            U2 => [R, Rp, R2, L, Lp, L2, D, Dp, D2, F, Fp, F2, B, Bp, B2],
             D => [R, Rp, R2, L, Lp, L2, U, Up, U2, F, Fp, F2, B, Bp, B2],
            Dp => [R, Rp, R2, L, Lp, L2, U, Up, U2, F, Fp, F2, B, Bp, B2],
            D2 => [R, Rp, R2, L, Lp, L2, U, Up, U2, F, Fp, F2, B, Bp, B2],
             F => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, B, Bp, B2],
            Fp => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, B, Bp, B2],
            F2 => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, B, Bp, B2],
             B => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2],
            Bp => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2],
            B2 => [R, Rp, R2, L, Lp, L2, U, Up, U2, D, Dp, D2, F, Fp, F2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn align(repr: &str) -> String {
        String::from("        ") + repr
    }

    #[test]
    fn print_solved() {
        let cube = Cube3by3::new();
        println!("{}", cube);
        assert_eq!(
            cube.to_string(),
            align(
                "\
        ⬜⬜⬜
        ⬜⬜⬜
        ⬜⬜⬜

🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦
🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦
🟧🟧🟧  🟩🟩🟩  🟥🟥🟥  🟦🟦🟦

        🟨🟨🟨
        🟨🟨🟨
        🟨🟨🟨"
            )
        );
        assert!(cube.check_parity());
    }

    #[test]
    fn scramble() {
        let mut cube = Cube3by3::new();
        cube.apply(&[
            D2, R2, U2, L2, Dp, R2, Dp, B2, D2, F2, Dp, Fp, Rp, Fp, Dp, R, U2, Fp, Lp, Rp,
        ]);
        println!("{}", cube);
        assert_eq!(
            cube.to_string(),
            align(
                "\
        🟩🟩🟦
        🟦⬜🟩
        ⬜🟧⬜

🟨🟨🟧  🟩🟨🟥  🟦🟧🟧  ⬜🟨🟧
🟥🟧🟩  ⬜🟩🟨  🟥🟥🟧  ⬜🟦🟩
🟩🟥🟦  🟨🟦🟦  🟧🟥⬜  🟥🟦🟨

        🟥🟧🟨
        🟦🟨⬜
        🟥⬜🟩"
            )
        );
        assert!(cube.check_parity());
    }

    #[test]
    fn solve() {
        let mut cube = Cube3by3::new();
        cube.apply(&[
            D2, R2, U2, L2, Dp, //, R2, Dp, B2, D2, F2, Dp, Fp, Rp, Fp, Dp, R, U2, Fp, Lp, Rp,
        ]);
        println!("{}", cube);
        let solution = cube.solve(6);
        println!("{:?}", solution);
        assert!(solution.is_some());
        cube.apply(&solution.unwrap());
        assert!(cube.is_solved());
    }
}
