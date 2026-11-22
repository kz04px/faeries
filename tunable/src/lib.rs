//! Values that a tuner can change at run time.
//!
//! ```ignore
//! use tunable::tunable;
//!
//! #[tunable(min = 0, max = 60, step = 3)]
//! const SURROUNDED_BONUS: i32 = 15;
//!
//! #[tunable(min = -50, max = 100, step = 4)]
//! const PST: [i32; 3] = [10, 20, 30];
//!
//! let bonus = SURROUNDED_BONUS.get();
//! let square = PST[1];
//! ```
//!
//! The crate using the attribute declares a `tune` feature. Without it a tunable is a
//! `const` whose `.get()` and indexing fold to the default, so the code is the same as if the
//! numbers were written inline. With it, each tunable is an atomic, read with `Relaxed`
//! ordering (an ordinary load on most targets), and is added to a registry that [`set`],
//! [`print_options`] and [`print_tune`] work from.
//!
//! A type made of integers can be tagged too, if it implements [`Parts`]: a `Score` of a
//! midgame and an endgame value, say. It is written with a constructor call of integer
//! literals, read like any other tunable, and listed to the tuner as its parts one after
//! another, `<name>_<K*i>` to `<name>_<K*i + K-1>` for element `i` of a `K`-part type:
//!
//! ```ignore
//! #[tunable(min = -50, max = 100, step = 4)]
//! const PST: [Score; 2] = [S(10, 20), S(30, 40)];
//!
//! let square = PST[1];
//! ```
//!
//! The registry is filtered by module path, so one binary holding several games' search code
//! only lists the tunables under the module path it's asked about.
//!
//! `skip = [..]` on an array tag (`skip = [0, 2..4]`) leaves those elements at their defaults:
//! they aren't listed as options and a tuner doesn't see them.
//!
//! A nested array (`[[i32; 9]; 9]`, or of a [`Parts`] type) is read as `GRID[row][col]` and
//! listed to the tuner as one flat array, row by row: element `(r, c)` is `<name>_<r*W + c>`
//! (times `K` for a `K`-part type), and `skip` counts the same way.
//!
//! A tuner finds the tags by building with `TUNE_MANIFEST=<file>` set, which makes the tags
//! append one JSON line each to that file: name, array or not, defaults, range, step, the
//! skipped indices and the row width of a nested array (0 if it isn't one).

use std::marker::PhantomData;
use std::ops::Index;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub use inventory;
pub use tunable_macros::tunable;

/// A tunable with its value fixed at the default
pub struct Fixed<const N: usize> {
    values: [i32; N],
}

impl<const N: usize> Fixed<N> {
    #[must_use]
    pub const fn new(values: [i32; N]) -> Self {
        Self { values }
    }

    #[must_use]
    pub const fn get(&self) -> i32 {
        self.values[0]
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<const N: usize> Index<usize> for Fixed<N> {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        &self.values[i]
    }
}

/// A tunable that can be changed at run time
///
/// A value can only be handed out by reference, as indexing needs, if it never changes in
/// place. So a change leaves the old value alone and points at a new one. The new value is
/// leaked, so the reference is good forever; this costs four bytes for every change, and a
/// change is a `setoption` before a search.
pub struct Live<const N: usize> {
    defaults: [i32; N],
    // Null while the default is in use
    pub cells: [AtomicPtr<i32>; N],
}

impl<const N: usize> Live<N> {
    #[must_use]
    pub const fn new(values: [i32; N]) -> Self {
        Self {
            defaults: values,
            cells: [const { AtomicPtr::new(ptr::null_mut()) }; N],
        }
    }

    #[must_use]
    pub fn get(&self) -> i32 {
        self[0]
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<const N: usize> Index<usize> for Live<N> {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        let value = self.cells[i].load(Ordering::Acquire);
        if value.is_null() {
            &self.defaults[i]
        } else {
            // SAFETY: a non-null pointer here came from `Box::leak` in `Cell::store`. It
            // is never freed and never written through again, so it is valid for as long
            // as the program runs and nothing mutates it.
            unsafe { &*value }
        }
    }
}

/// A type made of `K` integers, so that it can be a tunable
///
/// A tunable of such a type is listed to the tuner as its parts, one after another: element
/// `i` of a tunable named `x` is `x_<K*i>` to `x_<K*i + K-1>`.
pub trait Parts<const K: usize>: Copy + 'static {
    #[must_use]
    fn from_parts(parts: [i32; K]) -> Self;

    #[must_use]
    fn to_parts(self) -> [i32; K];
}

/// A tunable of a [`Parts`] type with its value fixed at the default
pub struct FixedParts<T, const K: usize, const N: usize> {
    values: [T; N],
}

impl<T: Parts<K>, const K: usize, const N: usize> FixedParts<T, K, N> {
    #[must_use]
    pub const fn new(values: [T; N]) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn get(&self) -> T {
        self.values[0]
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T, const K: usize, const N: usize> Index<usize> for FixedParts<T, K, N> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.values[i]
    }
}

/// A tunable of a [`Parts`] type that can be changed at run time
///
/// Each element is one leaked `T` behind one pointer, so it can be handed out by reference
/// like a `Live` value. Changing one part leaks a new `T` with that part replaced.
pub struct LiveParts<T, const K: usize, const N: usize> {
    defaults: [T; N],
    // Null while the default is in use
    cells: [AtomicPtr<T>; N],
    _parts: PhantomData<[(); K]>,
}

impl<T: Parts<K>, const K: usize, const N: usize> LiveParts<T, K, N> {
    #[must_use]
    pub const fn new(values: [T; N]) -> Self {
        Self {
            defaults: values,
            cells: [const { AtomicPtr::new(ptr::null_mut()) }; N],
            _parts: PhantomData,
        }
    }

    #[must_use]
    pub fn get(&self) -> T {
        self[0]
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T, const K: usize, const N: usize> Index<usize> for LiveParts<T, K, N> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let value = self.cells[i].load(Ordering::Acquire);
        if value.is_null() {
            &self.defaults[i]
        } else {
            // SAFETY: a non-null pointer here came from `Box::leak` in `Group::set`. It is
            // never freed and never written through again, so it is valid for as long as
            // the program runs and nothing mutates it.
            unsafe { &*value }
        }
    }
}

/// A [`LiveParts`] as the registry sees it: a list of integers
pub trait Group: Sync {
    /// How many integers there are in all
    fn values(&self) -> usize;
    fn get(&self, j: usize) -> i32;
    fn set(&self, j: usize, value: i32);
    /// Back to the default
    fn restore(&self, j: usize);
}

impl<T: Parts<K> + Sync, const K: usize, const N: usize> Group for LiveParts<T, K, N> {
    fn values(&self) -> usize {
        K * N
    }

    fn get(&self, j: usize) -> i32 {
        self[j / K].to_parts()[j % K]
    }

    fn set(&self, j: usize, value: i32) {
        let mut parts = self[j / K].to_parts();
        parts[j % K] = value;
        let new = Box::leak(Box::new(T::from_parts(parts)));
        self.cells[j / K].store(new, Ordering::Release);
    }

    fn restore(&self, j: usize) {
        self.cells[j / K].store(ptr::null_mut(), Ordering::Release);
    }
}

/// The storage behind a tagged const
#[derive(Clone, Copy)]
pub enum Cells {
    /// An `i32` or `[i32; N]`
    Int(&'static [AtomicPtr<i32>]),
    /// A [`Parts`] type, or an array of them
    Group(&'static dyn Group),
    /// An array of arrays: one of the above for each row, every row the same length
    Rows(&'static [Cells]),
}

impl Cells {
    /// How many values the tuner sees
    fn len(self) -> usize {
        match self {
            Self::Int(cells) => cells.len(),
            Self::Group(group) => group.values(),
            Self::Rows(rows) => rows.iter().map(|row| row.len()).sum(),
        }
    }

    /// The `j`th value
    fn cell(self, j: usize) -> Cell {
        match self {
            Self::Int(cells) => Cell::Int(&cells[j]),
            Self::Group(group) => Cell::Part(group, j),
            Self::Rows(rows) => {
                let width = rows[0].len();
                rows[j / width].cell(j % width)
            }
        }
    }
}

/// One value of a tunable
#[derive(Clone, Copy)]
enum Cell {
    Int(&'static AtomicPtr<i32>),
    /// One integer of a [`Group`]
    Part(&'static dyn Group, usize),
}

impl Cell {
    fn get(self, default: i32) -> i32 {
        match self {
            Self::Int(cell) => {
                let value = cell.load(Ordering::Acquire);
                if value.is_null() {
                    default
                } else {
                    // SAFETY: as in `Live::index`
                    unsafe { *value }
                }
            }
            Self::Part(group, j) => group.get(j),
        }
    }

    fn store(self, value: i32) {
        match self {
            Self::Int(cell) => cell.store(Box::leak(Box::new(value)), Ordering::Release),
            Self::Part(group, j) => group.set(j, value),
        }
    }

    /// Back to the default
    fn restore(self) {
        match self {
            Self::Int(cell) => cell.store(ptr::null_mut(), Ordering::Release),
            Self::Part(group, j) => group.restore(j),
        }
    }
}

/// Fails to compile if a default is outside its range
pub const fn check_range(values: &[i32], min: i32, max: i32) {
    assert!(min <= max, "a tunable's min is above its max");
    let mut i = 0;
    while i < values.len() {
        assert!(
            min <= values[i] && values[i] <= max,
            "a tunable's default is outside its range"
        );
        i += 1;
    }
}

/// One tagged const, as listed in the registry
pub struct Entry {
    module: &'static str,
    name: &'static str,
    array: bool,
    default: &'static [i32],
    min: i32,
    max: i32,
    step: i32,
    // Elements that are left at their defaults and not listed
    skip: &'static [usize],
    cells: Cells,
}

impl Entry {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        module: &'static str,
        name: &'static str,
        array: bool,
        default: &'static [i32],
        min: i32,
        max: i32,
        step: i32,
        skip: &'static [usize],
        cells: Cells,
    ) -> Self {
        Self {
            module,
            name,
            array,
            default,
            min,
            max,
            step,
            skip,
            cells,
        }
    }
}

inventory::collect!(Entry);

/// One value of a tunable, as the tuner and protocol see it
pub struct Param {
    pub name: String,
    pub default: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    /// More than one when the same code is compiled into several crates of one binary
    cells: Vec<Cell>,
}

impl Param {
    #[must_use]
    pub fn get(&self) -> i32 {
        self.cells[0].get(self.default)
    }

    /// Set the value without clamping it to its range, for a fitter that probes outside it
    ///
    /// Every call leaks a little memory, as with [`set`].
    pub fn set_unclamped(&self, value: i32) {
        for cell in &self.cells {
            cell.store(value);
        }
    }

    /// Back to the default
    pub fn restore(&self) {
        for cell in &self.cells {
            cell.restore();
        }
    }
}

/// Every value under the module path `prefix`, in a stable order
///
/// Entries with the same module path and name are one value: a module that is declared in
/// both a library and a binary of one package is registered once by each, and setting the
/// value has to reach both copies.
#[must_use]
pub fn params(prefix: &str) -> Vec<Param> {
    let mut entries: Vec<&Entry> = inventory::iter::<Entry>()
        .filter(|e| e.module.starts_with(prefix))
        .collect();
    entries.sort_by_key(|e| (e.module, e.name));

    let mut out: Vec<Param> = Vec::new();
    let mut previous: Option<(&str, &str)> = None;
    let mut first = 0;
    for e in entries {
        let kept = (0..e.cells.len())
            .filter(|i| !e.skip.contains(i))
            .map(|i| (i, e.cells.cell(i)));
        if previous == Some((e.module, e.name)) {
            for (j, (_, cell)) in kept.enumerate() {
                out[first + j].cells.push(cell);
            }
            continue;
        }
        previous = Some((e.module, e.name));
        first = out.len();
        for (i, cell) in kept {
            out.push(Param {
                name: if e.array {
                    format!("{}_{i}", e.name)
                } else {
                    e.name.to_owned()
                },
                default: e.default[i],
                min: e.min,
                max: e.max,
                step: e.step,
                cells: vec![cell],
            });
        }
    }
    out
}

/// Set a value under `prefix` by name, clamping it to its range. Returns false if there is
/// no such value; an unparsable number leaves the value alone.
pub fn set(prefix: &str, name: &str, value: &str) -> bool {
    let Some(param) = params(prefix).into_iter().find(|p| p.name == name) else {
        return false;
    };
    if let Ok(v) = value.parse::<i32>() {
        param.set_unclamped(v.clamp(param.min, param.max));
    }
    true
}

/// Put every value back to its default
pub fn reset(prefix: &str) {
    for p in params(prefix) {
        p.restore();
    }
}

/// List every value as a UGI spin option
pub fn print_options(prefix: &str) {
    for p in params(prefix) {
        println!(
            "option name {} type spin default {} min {} max {}",
            p.name, p.default, p.min, p.max
        );
    }
}

/// List every value for the tuner: name, type, default, min, max, step
pub fn print_tune(prefix: &str) {
    for p in params(prefix) {
        println!(
            "{}, int, {}, {}, {}, {}",
            p.name, p.default, p.min, p.max, p.step
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static LIVE: Live<3> = Live::new([10, 20, 30]);

    inventory::submit! {
        Entry::new("tunable::tests", "live", true, &[10, 20, 30], 0, 100, 5, &[2], Cells::Int(&LIVE.cells))
    }

    #[test]
    fn fixed_indexes_its_values() {
        let fixed = Fixed::new([4, 5, 6]);
        assert_eq!(fixed[0], 4);
        assert_eq!(fixed[2], 6);
        assert_eq!(fixed.get(), 4);
        assert_eq!(fixed.len(), 3);
    }

    #[test]
    fn range_check_accepts_values_in_range() {
        check_range(&[0, 50, 100], 0, 100);
    }

    #[test]
    #[should_panic(expected = "outside its range")]
    fn range_check_rejects_values_out_of_range() {
        check_range(&[0, 101], 0, 100);
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Pair(i32, i32);

    impl Parts<2> for Pair {
        fn from_parts([a, b]: [i32; 2]) -> Self {
            Self(a, b)
        }

        fn to_parts(self) -> [i32; 2] {
            [self.0, self.1]
        }
    }

    static LIVE_PAIRS: LiveParts<Pair, 2, 2> = LiveParts::new([Pair(1, 2), Pair(3, 4)]);

    inventory::submit! {
        Entry::new("tunable::pair_tests", "pairs", true, &[1, 2, 3, 4], 0, 100, 5, &[], Cells::Group(&LIVE_PAIRS))
    }

    #[test]
    fn live_parts_are_listed_and_set_by_part() {
        let names: Vec<String> = params("tunable::pair_tests")
            .into_iter()
            .map(|p| p.name)
            .collect();
        assert_eq!(names, ["pairs_0", "pairs_1", "pairs_2", "pairs_3"]);

        assert_eq!(LIVE_PAIRS[1], Pair(3, 4));
        assert!(set("tunable::pair_tests", "pairs_3", "44"));
        assert_eq!(LIVE_PAIRS[1], Pair(3, 44));
        assert_eq!(LIVE_PAIRS[0], Pair(1, 2));
        reset("tunable::pair_tests");
        assert_eq!(LIVE_PAIRS[1], Pair(3, 4));
    }

    // One test, as the value is shared and the registry is global
    #[test]
    fn live_values_change_through_the_registry() {
        // The skipped element isn't listed, and keeps its name's index
        let names: Vec<String> = params("tunable::tests")
            .into_iter()
            .map(|p| p.name)
            .collect();
        assert_eq!(names, ["live_0", "live_1"]);

        assert_eq!(LIVE[0], 10);
        assert_eq!(LIVE[1], 20);
        assert!(set("tunable::tests", "live_1", "55"));
        assert_eq!(LIVE[1], 55);
        assert_eq!(LIVE[0], 10);
        assert_eq!(LIVE[2], 30);

        // Values are clamped to the range, and junk leaves the value alone
        assert!(set("tunable::tests", "live_1", "1000"));
        assert_eq!(LIVE[1], 100);
        assert!(set("tunable::tests", "live_1", "junk"));
        assert_eq!(LIVE[1], 100);

        // A skipped element can't be set, and neither can an unknown name
        assert!(!set("tunable::tests", "live_2", "1"));
        assert!(!set("tunable::tests", "other", "1"));
        assert_eq!(LIVE[2], 30);

        // Another module path doesn't see these
        assert!(!set("elsewhere", "live_1", "1"));

        reset("tunable::tests");
        assert_eq!(LIVE[1], 20);
        let live_1 = params("tunable::tests")
            .into_iter()
            .find(|p| p.name == "live_1");
        assert_eq!(live_1.unwrap().get(), 20);
    }
}
