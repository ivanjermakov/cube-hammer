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

    pub fn apply_move(&mut self, m: &Move) {
        match m {
            Move::R => todo!(),
            Move::Rp => todo!(),
            Move::L => todo!(),
            Move::Lp => todo!(),
            Move::U => todo!(),
            Move::Up => todo!(),
            Move::D => todo!(),
            Move::Dp => todo!(),
            Move::F => todo!(),
            Move::Fp => todo!(),
            Move::B => todo!(),
            Move::Bp => todo!(),
        }
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
            fs[2][0], fs[2][1], fs[2][2],
            fs[3][0], fs[3][1], fs[3][2],
            fs[4][0], fs[4][1], fs[4][2],
            fs[5][0], fs[5][1], fs[5][2],
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
    L,
    Lp,
    U,
    Up,
    D,
    Dp,
    F,
    Fp,
    B,
    Bp,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn align(repr: &str) -> String {
        String::from("      ") + repr
    }

    #[test]
    fn print_solved() {
        let cube = Cube3by3::new();
        let fmtd = format!("{}", cube);
        assert_eq!(
            fmtd,
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
}
