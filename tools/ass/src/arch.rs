#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register8 {
  Ax = 0,  Ay = 1,
  Bx = 2,  By = 3,
  Cx = 4,  Cy = 5,
  Dx = 6,  Dy = 7,
  Ex = 8,  Ey = 9,
  Fx = 10, Fy = 11,
  Gx = 12, Gy = 13,
  Hx = 14, Hy = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register16 {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
  F = 5,
  G = 6,
  H = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
  Register8(Register8),
  Register16(Register16)
}