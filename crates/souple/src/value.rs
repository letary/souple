//! Lengths: what a style holds for every size, edge and gap.
//!
//! The engine knows two bases, px and percent. Everything else — `em`, `vw`, `safe-top` — is a
//! VARIABLE: a number the embedder defines on a node, read by the nearest definition up the tree
//! (`vars.rs`). `2em` is `Length::Var(EM, 2.0)` under whatever id the embedder gave `em`.
//!
//! A `calc()` is a [`Formula`]: a postfix program over px, percents, variables and numbers. It is
//! built once ([`Formula::build`] validates and folds the constants), shared by reference, and
//! evaluated when the percent base is known.

use std::sync::Arc;

/// How many variables one tree can tell apart: ids `0..MAX_VARS`. Every node records the
/// variables its style reads, and every subtree those read under it, as one 64-bit word
/// ([`VarSet`]) — what keeps a variable change to the subtrees that read it, at no cost per
/// variable. An embedder with more units than this maps several onto one id, or resolves the rest
/// to px itself.
pub const MAX_VARS: usize = 64;

/// A variable: a number defined on nodes ([`crate::Tree::set_var`]) or for the whole tree
/// ([`crate::Tree::set_global`]), read by a [`Length::Var`] or a [`Formula`] through the scope
/// chain. What it stands for — `em`, `vw`, a safe-area inset — is the embedder's: the engine knows
/// ids only, and the embedder numbers them, `0..MAX_VARS`, as it likes (a `const` per unit).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VarId(u8);

impl VarId {
    /// The variable numbered `index`.
    ///
    /// # Panics
    ///
    /// If `index >= MAX_VARS` (64) — at compile time in a `const`.
    pub const fn new(index: u8) -> Self {
        assert!((index as usize) < MAX_VARS, "a variable id is below MAX_VARS");
        Self(index)
    }

    /// The number this variable was made with.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A set of variables: one bit per [`VarId`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct VarSet(u64);

impl VarSet {
    /// No variable.
    pub const EMPTY: Self = Self(0);
    /// Every variable.
    pub const ALL: Self = Self(u64::MAX);

    /// The set of `var` alone.
    pub const fn of(var: VarId) -> Self {
        Self(1 << var.0)
    }

    /// The set holds no variable.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// `var` is in the set.
    pub const fn contains(self, var: VarId) -> bool {
        self.0 & (1 << var.0) != 0
    }

    /// Every variable of `other` is in this set.
    pub const fn includes(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Some variable is in both sets.
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// The variables of either set.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// This set less the variables of `other`.
    pub const fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

/// A CSS length: what a style holds for every size, edge, gap and inset.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Length {
    /// The property's automatic value: content size, no minimum, no inset, an auto margin.
    #[default]
    Auto,
    /// Logical px.
    Px(f32),
    /// A percent of the property's base: `Percent(50.0)` is 50%.
    Percent(f32),
    /// A multiple of a variable: `Var(EM, 2.0)` is `2em`.
    Var(VarId, f32),
    /// A formula — `calc()`, `min()`, `max()`, `clamp()` — made by [`Formula::build`].
    Calc(Arc<Formula>),
}

impl Length {
    /// `0px`.
    pub const ZERO: Self = Self::Px(0.0);

    /// The length is `Auto`.
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }

    /// The variables this length reads.
    pub fn vars(&self) -> VarSet {
        match self {
            Self::Var(var, _) => VarSet::of(*var),
            Self::Calc(formula) => formula.vars,
            _ => VarSet::EMPTY,
        }
    }

    /// The same length with every variable replaced by its value: px and percent only.
    pub fn substitute(&self, value_of: &impl Fn(VarId) -> f32) -> Self {
        match self {
            Self::Var(var, factor) => Self::Px(factor * value_of(*var)),
            Self::Calc(formula) if !formula.vars.is_empty() => {
                let ops: Vec<Op> = formula
                    .ops
                    .iter()
                    .map(|op| match *op {
                        Op::Var(var, factor) => Op::Px(factor * value_of(var)),
                        op => op,
                    })
                    .collect();
                Formula::build(&ops).expect("a built formula stays valid when its variables become px")
            }
            other => other.clone(),
        }
    }

    /// The length in px. `base` is what a percent is of; `None` = not known (an indefinite size).
    /// `value_of` is a variable's value where the length is read — [`crate::Tree::var`] of the node,
    /// or the embedder's own numbers — as [`Formula::eval`] takes it. `None` back: the length is
    /// `Auto`, or holds a percent of an unknown base — which CSS treats as `auto` too.
    ///
    /// ```
    /// # use souple::{Length, VarId};
    /// const EM: VarId = VarId::new(0);
    /// let em = |_: VarId| 16.0;
    /// assert_eq!(Length::Var(EM, 2.0).resolve(None, &em), Some(32.0));
    /// assert_eq!(Length::Percent(50.0).resolve(Some(300.0), &em), Some(150.0));
    /// assert_eq!(Length::Percent(50.0).resolve(None, &em), None);
    /// ```
    pub fn resolve(&self, base: Option<f32>, value_of: &impl Fn(VarId) -> f32) -> Option<f32> {
        match self {
            Self::Auto => None,
            Self::Px(px) => Some(*px),
            Self::Percent(percent) => base.map(|base| base * percent / 100.0),
            Self::Var(var, factor) => Some(factor * value_of(*var)),
            Self::Calc(formula) => formula.eval(base, value_of),
        }
    }

    /// [`Length::resolve`] with `Auto` and an unknown percent as 0: for paddings, borders, gaps.
    pub fn resolve_or_zero(&self, base: Option<f32>, value_of: &impl Fn(VarId) -> f32) -> f32 {
        self.resolve(base, value_of).unwrap_or(0.0)
    }

    /// A COMPUTED length in px (the algorithm's reads: every variable substituted already).
    pub(crate) fn px(&self, base: Option<f32>) -> Option<f32> {
        self.resolve(base, &|_| {
            debug_assert!(false, "a computed length holds no variable");
            0.0
        })
    }

    /// [`Length::px`] with `Auto` and an unknown percent as 0.
    pub(crate) fn px_or_zero(&self, base: Option<f32>) -> f32 {
        self.px(base).unwrap_or(0.0)
    }

    /// A formula length — `calc()`, `min()`, `max()`, `clamp()` — from its postfix program: the
    /// program validated and every constant part folded, so the answer is the simplest length that
    /// says the same (`calc(10px + 5px)` is `Length::Px(15.0)`, a [`Length::Calc`] only when one is
    /// needed). The way to make one ([`Formula::build`] is the same call).
    ///
    /// ```
    /// # use souple::{Length, Op};
    /// // calc(100% - 20px)
    /// let length = Length::calc(&[Op::Percent(100.0), Op::Px(20.0), Op::Sub]).unwrap();
    /// assert_eq!(length.resolve(Some(300.0), &|_| 0.0), Some(280.0));
    /// assert_eq!(Length::calc(&[Op::Px(10.0), Op::Px(5.0), Op::Add]), Ok(Length::Px(15.0)));
    /// ```
    pub fn calc(ops: &[Op]) -> Result<Length, FormulaError> {
        Formula::build(ops)
    }
}

/// One step of a [`Formula`], in postfix order: an operand pushes a value, an operator replaces its
/// arguments with the result.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Op {
    /// A length in px.
    Px(f32),
    /// A percent of the property's base.
    Percent(f32),
    /// A multiple of a variable.
    Var(VarId, f32),
    /// A bare number: a factor or a divisor.
    Num(f32),
    /// Length + length (or number + number).
    Add,
    /// Length − length (or number − number).
    Sub,
    /// Length × number, number × length, number × number.
    Mul,
    /// Length ÷ number, number ÷ number; a constant divisor of 0 is refused.
    Div,
    /// The smallest of the last `n` values.
    Min(u8),
    /// The largest of the last `n` values.
    Max(u8),
    /// `clamp(min, value, max)` of the last three values, in that order.
    Clamp,
}

/// Why [`Formula::build`] refused a program.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum FormulaError {
    /// An operator found fewer values than it takes, or the program left more than one.
    Unbalanced,
    /// A length where a number must be or the reverse: `1px * 1px`, `1px + 2`, `2 / 1px`, a formula
    /// that is a bare number.
    Type,
    /// A division by a number that is (or folds to) 0.
    DivisionByZero,
    /// More nested than the evaluator's stack ([`Formula::MAX_DEPTH`]).
    TooDeep,
}

/// A validated, constant-folded postfix program whose value is a length.
#[derive(Clone, Debug, PartialEq)]
pub struct Formula {
    ops: Box<[Op]>,
    vars: VarSet,
    has_percent: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Number,
    Length,
}

/// A value on the builder's stack: where its ops start in the output, and its value when constant.
struct Entry {
    start: usize,
    kind: Kind,
    constant: Option<f32>,
}

impl Formula {
    /// How deep a formula's evaluation stack may grow (after folding): deeper is
    /// [`FormulaError::TooDeep`].
    pub const MAX_DEPTH: usize = 16;

    /// Validates `ops` and folds every constant part. The answer is the simplest length that says
    /// the same: `calc(10px + 5px)` is `Length::Px(15.0)`, a formula only when one is needed. The
    /// same as [`Length::calc`], the documented way to make a formula length.
    ///
    /// Any number is accepted in an operand (NaN and infinities included: the layout makes
    /// something finite of them); only the program's shape and types are checked.
    pub fn build(ops: &[Op]) -> Result<Length, FormulaError> {
        let mut out: Vec<Op> = Vec::with_capacity(ops.len());
        let mut stack: Vec<Entry> = Vec::new();

        for &op in ops {
            let arity = arity(op);
            if arity == 0 {
                let (kind, constant) = match op {
                    Op::Px(px) => (Kind::Length, Some(px)),
                    Op::Num(number) => (Kind::Number, Some(number)),
                    _ => (Kind::Length, None),
                };
                stack.push(Entry { start: out.len(), kind, constant });
                out.push(op);
                continue;
            }
            if matches!(op, Op::Min(0) | Op::Max(0)) || stack.len() < arity {
                return Err(FormulaError::Unbalanced);
            }
            let args = stack.split_off(stack.len() - arity);
            let kind = result_kind(op, &args)?;
            let constant = args
                .iter()
                .map(|arg| arg.constant)
                .collect::<Option<Vec<f32>>>()
                .map(|values| apply(op, &values));
            let start = args[0].start;
            match constant {
                Some(value) => {
                    out.truncate(start);
                    out.push(if kind == Kind::Length { Op::Px(value) } else { Op::Num(value) });
                }
                None => match scaled(op, &args, &out) {
                    // `2 * 1em` is `2em`, `50% / 2` is `25%`: one operand, no operator.
                    Some(operand) => {
                        out.truncate(start);
                        out.push(operand);
                    }
                    None => out.push(op),
                },
            }
            stack.push(Entry { start, kind, constant });
        }

        if stack.len() != 1 {
            return Err(FormulaError::Unbalanced);
        }
        if stack[0].kind != Kind::Length {
            return Err(FormulaError::Type);
        }
        // The depth of what is left after folding: that is the program the evaluator runs.
        let (mut depth, mut deepest) = (0usize, 0usize);
        for op in &out {
            depth = depth + 1 - arity(*op);
            deepest = deepest.max(depth);
        }
        if deepest > Self::MAX_DEPTH {
            return Err(FormulaError::TooDeep);
        }
        Ok(match out[..] {
            [Op::Px(px)] => Length::Px(px),
            [Op::Percent(percent)] => Length::Percent(percent),
            [Op::Var(var, factor)] => Length::Var(var, factor),
            _ => {
                let mut vars = VarSet::EMPTY;
                let mut has_percent = false;
                for op in &out {
                    match op {
                        Op::Var(var, _) => vars = vars.union(VarSet::of(*var)),
                        Op::Percent(_) => has_percent = true,
                        _ => {}
                    }
                }
                Length::Calc(Arc::new(Formula { ops: out.into(), vars, has_percent }))
            }
        })
    }

    /// The program, folded, in postfix order.
    pub fn ops(&self) -> &[Op] {
        &self.ops
    }

    /// The variables the formula reads.
    pub fn vars(&self) -> VarSet {
        self.vars
    }

    /// The formula holds a percent: its value needs the percent base.
    pub fn has_percent(&self) -> bool {
        self.has_percent
    }

    /// The value in px. `None`: the formula holds a percent and `base` is unknown.
    pub fn eval(&self, base: Option<f32>, value_of: &impl Fn(VarId) -> f32) -> Option<f32> {
        if self.has_percent && base.is_none() {
            return None;
        }
        let base = base.unwrap_or(0.0);
        let mut stack = [0.0f32; Self::MAX_DEPTH];
        let mut len = 0;
        for &op in &self.ops {
            let arity = arity(op);
            let value = match op {
                Op::Px(px) => px,
                Op::Num(number) => number,
                Op::Percent(percent) => base * percent / 100.0,
                Op::Var(var, factor) => factor * value_of(var),
                op => apply(op, &stack[len - arity..len]),
            };
            len -= arity;
            stack[len] = value;
            len += 1;
        }
        Some(stack[0])
    }
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unbalanced => "an operator is missing a value, or a value is left over",
            Self::Type => "a length where a number must be, or a number where a length must be",
            Self::DivisionByZero => "a division by zero",
            Self::TooDeep => "the formula is nested too deep",
        })
    }
}

impl std::error::Error for FormulaError {}

/// How many values an op takes off the stack.
fn arity(op: Op) -> usize {
    match op {
        Op::Px(_) | Op::Percent(_) | Op::Var(..) | Op::Num(_) => 0,
        Op::Add | Op::Sub | Op::Mul | Op::Div => 2,
        Op::Min(n) | Op::Max(n) => n as usize,
        Op::Clamp => 3,
    }
}

/// A percent or a variable times (or over) a constant number, as the one operand it is.
fn scaled(op: Op, args: &[Entry], out: &[Op]) -> Option<Op> {
    let single = |arg: &Entry, end: usize| (end - arg.start == 1).then(|| out[arg.start]);
    let (operand, factor) = match op {
        Op::Mul => match (args[0].constant, args[1].constant) {
            (None, Some(factor)) => (single(&args[0], args[1].start)?, factor),
            (Some(factor), None) => (single(&args[1], out.len())?, factor),
            _ => return None,
        },
        Op::Div => (single(&args[0], args[1].start)?, 1.0 / args[1].constant?),
        _ => return None,
    };
    match operand {
        Op::Percent(percent) => Some(Op::Percent(percent * factor)),
        Op::Var(var, by) => Some(Op::Var(var, by * factor)),
        _ => None,
    }
}

/// The type rules of `calc()`: what `op` over `args` is, or why it is not.
fn result_kind(op: Op, args: &[Entry]) -> Result<Kind, FormulaError> {
    let all_same = args.iter().all(|arg| arg.kind == args[0].kind);
    match op {
        Op::Mul => match (args[0].kind, args[1].kind) {
            (Kind::Length, Kind::Length) => Err(FormulaError::Type),
            (Kind::Number, Kind::Number) => Ok(Kind::Number),
            _ => Ok(Kind::Length),
        },
        Op::Div => {
            if args[1].kind != Kind::Number {
                return Err(FormulaError::Type);
            }
            // A number is always constant: every operator over numbers folds.
            if args[1].constant == Some(0.0) {
                return Err(FormulaError::DivisionByZero);
            }
            Ok(args[0].kind)
        }
        _ if all_same => Ok(args[0].kind),
        _ => Err(FormulaError::Type),
    }
}

fn apply(op: Op, args: &[f32]) -> f32 {
    match op {
        Op::Add => args[0] + args[1],
        Op::Sub => args[0] - args[1],
        Op::Mul => args[0] * args[1],
        Op::Div => args[0] / args[1],
        Op::Min(_) => args.iter().copied().fold(f32::INFINITY, f32::min),
        Op::Max(_) => args.iter().copied().fold(f32::NEG_INFINITY, f32::max),
        // CSS: clamp(min, value, max) = max(min, min(value, max)) — the minimum wins a conflict.
        Op::Clamp => args[0].max(args[1].min(args[2])),
        Op::Px(_) | Op::Percent(_) | Op::Var(..) | Op::Num(_) => unreachable!("an operand is not applied"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EM: VarId = VarId::new(0);
    const VW: VarId = VarId::new(1);

    fn calc(ops: &[Op]) -> Length {
        Formula::build(ops).unwrap()
    }

    #[test]
    fn constants_fold_to_a_plain_length() {
        assert_eq!(calc(&[Op::Px(10.0), Op::Px(5.0), Op::Add]), Length::Px(15.0));
        assert_eq!(calc(&[Op::Px(10.0), Op::Num(2.0), Op::Num(3.0), Op::Add, Op::Mul]), Length::Px(50.0));
        assert_eq!(calc(&[Op::Percent(40.0)]), Length::Percent(40.0));
        assert_eq!(calc(&[Op::Var(EM, 2.0)]), Length::Var(EM, 2.0));
    }

    #[test]
    fn a_constant_part_folds_inside_a_formula() {
        // calc(100% - (4px + 6px)) keeps one subtraction.
        let Length::Calc(formula) = calc(&[Op::Percent(100.0), Op::Px(4.0), Op::Px(6.0), Op::Add, Op::Sub]) else {
            panic!("a percent keeps the formula")
        };
        assert_eq!(formula.ops(), &[Op::Percent(100.0), Op::Px(10.0), Op::Sub]);
        assert!(formula.has_percent());
    }

    #[test]
    fn a_scaled_operand_needs_no_formula() {
        assert_eq!(calc(&[Op::Num(-1.0), Op::Var(EM, 1.0), Op::Mul]), Length::Var(EM, -1.0));
        assert_eq!(calc(&[Op::Percent(50.0), Op::Num(2.0), Op::Div]), Length::Percent(25.0));
        // Seventeen constants are deeper than the evaluator's stack, but fold to one value.
        let mut ops = vec![Op::Px(1.0); 17];
        ops.push(Op::Min(17));
        assert_eq!(calc(&ops), Length::Px(1.0));
    }

    #[test]
    fn percent_and_px_mix() {
        let length = calc(&[Op::Percent(100.0), Op::Px(20.0), Op::Sub]);
        assert_eq!(length.px(Some(300.0)), Some(280.0));
        assert_eq!(length.px(None), None, "a percent of an unknown base is auto");
    }

    #[test]
    fn min_max_clamp() {
        let min = calc(&[Op::Percent(50.0), Op::Px(100.0), Op::Px(400.0), Op::Min(3)]);
        assert_eq!(min.px(Some(1000.0)), Some(100.0));
        assert_eq!(min.px(Some(100.0)), Some(50.0));
        let clamp = calc(&[Op::Px(10.0), Op::Percent(50.0), Op::Px(30.0), Op::Clamp]);
        assert_eq!(clamp.px(Some(10.0)), Some(10.0));
        assert_eq!(clamp.px(Some(40.0)), Some(20.0));
        assert_eq!(clamp.px(Some(400.0)), Some(30.0));
    }

    #[test]
    fn substitution_leaves_px_and_percent() {
        let values = |var: VarId| if var == EM { 16.0 } else { 4.0 };
        assert_eq!(Length::Var(EM, 2.0).substitute(&values), Length::Px(32.0));
        // calc(2em + 10vw) has no percent: one number once the variables are known.
        let no_percent = calc(&[Op::Var(EM, 2.0), Op::Var(VW, 10.0), Op::Add]);
        assert_eq!(no_percent.vars(), VarSet::of(EM).union(VarSet::of(VW)));
        assert_eq!(no_percent.substitute(&values), Length::Px(72.0));
        // calc(100% - 2em) stays a formula, without the variable.
        let mixed = calc(&[Op::Percent(100.0), Op::Var(EM, 2.0), Op::Sub]).substitute(&values);
        assert!(mixed.vars().is_empty());
        assert_eq!(mixed.px(Some(100.0)), Some(68.0));
    }

    #[test]
    fn a_variable_length_resolves_with_its_values() {
        let values = |var: VarId| if var == EM { 16.0 } else { 4.0 };
        assert_eq!(Length::Var(EM, 2.0).resolve(None, &values), Some(32.0));
        // calc(100% - 2em), the variable read where it is resolved: the same as substituted first.
        let mixed = Length::calc(&[Op::Percent(100.0), Op::Var(EM, 2.0), Op::Sub]).unwrap();
        assert_eq!(mixed.resolve(Some(100.0), &values), Some(68.0));
        assert_eq!(mixed.resolve(Some(100.0), &values), mixed.substitute(&values).px(Some(100.0)));
        assert_eq!(Length::Auto.resolve(Some(100.0), &values), None);
        assert_eq!(Length::Percent(10.0).resolve_or_zero(None, &values), 0.0);
    }

    #[test]
    fn invalid_programs_are_refused() {
        assert_eq!(Formula::build(&[Op::Px(1.0), Op::Add]), Err(FormulaError::Unbalanced));
        assert_eq!(Formula::build(&[Op::Px(1.0), Op::Px(2.0)]), Err(FormulaError::Unbalanced));
        assert_eq!(Formula::build(&[Op::Px(1.0), Op::Px(2.0), Op::Mul]), Err(FormulaError::Type));
        assert_eq!(Formula::build(&[Op::Px(1.0), Op::Num(2.0), Op::Add]), Err(FormulaError::Type));
        assert_eq!(Formula::build(&[Op::Num(2.0), Op::Px(1.0), Op::Div]), Err(FormulaError::Type));
        assert_eq!(Formula::build(&[Op::Num(2.0), Op::Num(3.0), Op::Mul]), Err(FormulaError::Type));
        assert_eq!(Formula::build(&[Op::Px(1.0), Op::Num(0.0), Op::Div]), Err(FormulaError::DivisionByZero));
    }
}
