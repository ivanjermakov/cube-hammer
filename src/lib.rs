use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Cube3by3 {
    /// UDLFRB order, clockwise stickers from top left
    faces: [Face; 6],
}

impl Cube3by3 {
    pub fn new() -> Cube3by3 {
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
        // [fs[0][0], fs[0][1], fs[0][2], fs[0][3], fs[0][4], fs[0][5], fs[0][6], fs[0][7]],
        // [fs[1][0], fs[1][1], fs[1][2], fs[1][3], fs[1][4], fs[1][5], fs[1][6], fs[1][7]],
        // [fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5], fs[2][6], fs[2][7]],
        // [fs[3][0], fs[3][1], fs[3][2], fs[3][3], fs[3][4], fs[3][5], fs[3][6], fs[3][7]],
        // [fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5], fs[4][6], fs[4][7]],
        // [fs[5][0], fs[5][1], fs[5][2], fs[5][3], fs[5][4], fs[5][5], fs[5][6], fs[5][7]],

        let fs = self.faces;
        match m {
            Move::R => {
                self.faces = [
                    [fs[0][0], fs[0][1], fs[3][2], fs[3][3], fs[3][4], fs[0][5], fs[0][6], fs[0][7]],
                    [fs[1][0], fs[1][1], fs[5][6], fs[5][7], fs[5][0], fs[1][5], fs[1][6], fs[1][7]],
                    [fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5], fs[2][6], fs[2][7]],
                    [fs[3][0], fs[3][1], fs[1][2], fs[1][3], fs[1][4], fs[3][5], fs[3][6], fs[3][7]],
                    [fs[4][6], fs[4][7], fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5]],
                    [fs[0][4], fs[5][1], fs[5][2], fs[5][3], fs[5][4], fs[5][5], fs[0][3], fs[0][2]],
                ]
            }
            Move::Rp => for _ in 0..3 {
                self.apply_move(&Move::R);
            }
            Move::R2 => for _ in 0..2 {
                self.apply_move(&Move::R);
            },
            Move::L => {
                self.faces = [
                    [fs[5][4], fs[0][1], fs[0][2], fs[0][3], fs[0][4], fs[0][5], fs[5][3], fs[5][2]],
                    [fs[3][0], fs[1][1], fs[1][2], fs[1][3], fs[1][4], fs[1][5], fs[3][6], fs[3][7]],
                    [fs[2][6], fs[2][7], fs[2][0], fs[2][1], fs[2][2], fs[2][3], fs[2][4], fs[2][5]],
                    [fs[0][0], fs[3][1], fs[3][2], fs[3][3], fs[3][4], fs[3][5], fs[0][6], fs[0][7]],
                    [fs[4][0], fs[4][1], fs[4][2], fs[4][3], fs[4][4], fs[4][5], fs[4][6], fs[4][7]],
                    [fs[5][0], fs[5][1], fs[2][6], fs[2][7], fs[2][0], fs[5][5], fs[5][6], fs[5][7]],
                ]
            },
            Move::Lp => for _ in 0..3 {
                self.apply_move(&Move::L);
            },
            Move::L2 => for _ in 0..2 {
                self.apply_move(&Move::L);
            },
            Move::U => todo!(),
            Move::Up => todo!(),
            Move::U2 => todo!(),
            Move::D => todo!(),
            Move::Dp => todo!(),
            Move::D2 => todo!(),
            Move::F => todo!(),
            Move::Fp => todo!(),
            Move::F2 => todo!(),
            Move::B => todo!(),
            Move::Bp => todo!(),
            Move::B2 => todo!(),
        }
    }
    pub fn check_parity(&self) -> bool {
        true
    }
}

impl Default for Cube3by3 {
    fn default() -> Self {
        Self::new()
    }
}

/// ```plain
///
///        ⬜⬜⬜
///        ⬜⬜⬜
///        ⬜⬜⬜
///  🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
///  🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
///  🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
///        🟨🟨🟨
///        🟨🟨🟨
///        🟨🟨🟨
/// ```
impl fmt::Display for Cube3by3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fs = self.faces;
        let mut fmtd = String::new();
        fmtd += &format!("      {}{}{}\n", fs[0][0], fs[0][1], fs[0][2]);
        fmtd += &format!("      {}⬜{}\n", fs[0][7], fs[0][3]);
        fmtd += &format!("      {}{}{}\n", fs[0][6], fs[0][5], fs[0][4]);

        fmtd += &format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}\n",
            fs[2][0], fs[2][1], fs[2][2],
            fs[3][0], fs[3][1], fs[3][2],
            fs[4][0], fs[4][1], fs[4][2],
            fs[5][0], fs[5][1], fs[5][2],
        );
        fmtd += &format!("{}🟧{}{}🟩{}{}🟥{}{}🟦{}\n",
            fs[2][7], fs[2][3],
            fs[3][7], fs[3][3],
            fs[4][7], fs[4][3],
            fs[5][7], fs[5][3],
        );
        fmtd += &format!("{}{}{}{}{}{}{}{}{}{}{}{}\n",
            fs[2][6], fs[2][5], fs[2][4],
            fs[3][6], fs[3][5], fs[3][4],
            fs[4][6], fs[4][5], fs[4][4],
            fs[5][6], fs[5][5], fs[5][4],
        );

        fmtd += &format!("      {}{}{}\n", fs[1][0], fs[1][1], fs[1][2]);
        fmtd += &format!("      {}🟨{}\n", fs[1][7], fs[1][3]);
        fmtd += &format!("      {}{}{}", fs[1][6], fs[1][5], fs[1][4]);
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

#[cfg(test)]
mod tests {
    use super::Move::*;
    use super::*;

    fn align(repr: &str) -> String {
        String::from("      ") + repr
    }

    #[test]
    fn print_solved() {
        let cube = Cube3by3::new();
        assert_eq!(
            cube.to_string(),
            align(
                "\
      ⬜⬜⬜
      ⬜⬜⬜
      ⬜⬜⬜
🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
🟧🟧🟧🟩🟩🟩🟥🟥🟥🟦🟦🟦
      🟨🟨🟨
      🟨🟨🟨
      🟨🟨🟨"
            )
        )
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
🟨🟨🟧🟩🟨🟥🟦🟧🟧⬜🟨🟧
🟥🟧🟩⬜🟩🟨🟥🟥🟧⬜🟦🟩
🟩🟥🟦🟨🟦🟦🟧🟥⬜🟥🟦🟨
      🟥🟧🟨
      🟦🟨⬜
      🟥⬜🟩"
            )
        )
    }
}
