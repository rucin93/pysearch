use crate::{expr::Expr, operator::*};

pub type Num = f64;

pub struct Input {
    pub name: &'static str,
    pub vec: &'static [Num],
    pub min_uses: u8,
    pub max_uses: u8,
    pub offset: Num,
}

pub const INPUTS: &[Input] = &[
Input {
  name: "a",
  vec: &[
    0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0
  ],
  min_uses: 1,
  max_uses: 1,
  offset: 0.0
},
Input {
  name: "c",
  vec: &[
    0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0
  ],
  min_uses: 1,
  max_uses: 3,
  offset: 0.0
}];

pub const GOAL: &[Num] = &[
  48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0
];
pub struct Matcher {}

impl Matcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn match_one(&mut self, index: usize, output: Num) -> bool {
      output as i64 == GOAL[index] as i64
    }

    // Will be called after match_one returns true for all outputs
    pub fn match_final(self, _el: Option<&Expr>, _er: &Expr, _op: OpIndex) -> bool {
        true
    }

    pub fn match_all(expr: &Expr) -> bool {
        let mut matcher = Self::new();
        expr.output
            .iter()
            .enumerate()
            .all(|(i, &o)| matcher.match_one(i, o))
            && matcher.match_final(
                expr.left.map(|e| unsafe { e.as_ref() }),
                unsafe { expr.right.unwrap().as_ref() },
                expr.op_idx,
            )
    }
}

pub const MAX_LENGTH: usize = 13;
pub const MAX_CACHE_LENGTH: usize = 8;
pub const MIN_MULTITHREAD_LENGTH: usize = MAX_CACHE_LENGTH + 1;
pub const LITERALS: &[Num] = &[1.0,2.0,3.0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,32.0,26.0, 97.0, 96.0, 65.0];
/// If not 0, include all numbers in 1..=MAX_LITERAL in addition to LITERALS.
pub const MAX_LITERAL: Num = 0.0;

#[rustfmt::skip]
pub const BINARY_OPERATORS: &[BinaryOp] = &[
  OP_OR_SYMBOL, // a || b
  OP_AND_SYMBOL, // a && b
  // OP_LT, // a < b
  // OP_LE, // a <= b
  // OP_GT, // a > b
  // OP_GE, // a >= b
  // OP_EQ, // a == b
  // OP_NE, // a != b
  OP_BIT_OR, // a | b
  OP_BIT_XOR, // a ^ b
  OP_BIT_AND, // a & b
  // OP_BIT_SHL_WRAP, // a << b
  // OP_BIT_SHR_WRAP, // a >> b
  OP_ADD, // a + b
  OP_SUB, // a - b
  OP_MUL, // a * b
  OP_MOD_TRUNC, // a % b, remainder 
  OP_DIV_TRUNC, // a / b | 0, truncated (e.g. -21 / 4 = -5.25 => -5, -23 / 4 = -5.75 => -5)
  // OP_EXP, // a ** b

  // Not applicable to JavaScript
  // OP_OR, // aorb
  // OP_SPACE_OR, // a orb
  // OP_OR_SPACE, // aor b
  // OP_SPACE_OR_SPACE, // a or b
  // OP_OR_LOGICAL, // +!!(a || b), ensures output is 0 or 1
  // OP_AND, // aandb
  // OP_SPACE_AND, // a andb
  // OP_AND_SPACE, // aand b
  // OP_SPACE_AND_SPACE, // a and b
  // OP_AND_LOGICAL, // +!!(a && b), ensures output is 0 or 1
  // OP_BIT_SHL, // a << b but with checks to ensure b is between [0, bits]
  // OP_BIT_SHR, // a >> b but with checks to ensure b is between [0, bits)
  // OP_MOD_FLOOR, // ((a % b) + b) % b, modulus
  // OP_DIV_FLOOR, // a / b, rounded to negative infinity (e.g. -21 / 4 = -5.25 => -6, -23 / 4 = -5.75 => -6)
  // OP_GCD, // greatest common divisor(a, b)
];

#[rustfmt::skip]
pub const UNARY_OPERATORS: &[UnaryOp] = &[
    OP_BIT_NEG,
    OP_NEG,
    // OP_NOT,
];

/// If set, use ternary operator `a ? b : c` 
pub const USE_TERNARY: bool = false;

/// Match leaf expressions 1 output at a time to avoid unnecessary precalculations
pub const MATCH_1BY1: bool = true;

/// If set, e.g. to `Some(-159236)`, this arbitrary number is chosen to represent errors.
/// That is, pysearch will pretend 1/0 = -159236, and -159236 * 2 = -159236, and so on.
pub const ERROR_VALUE: Option<Num> = None;
// pub const ERROR_VALUE: Option<Num> = Some(0);
