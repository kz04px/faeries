//! The `#[tunable(min = .., max = .., step = .. [, skip = [..]] [, nondecreasing | nonincreasing] [, zero_mean])]` attribute. See the
//! `tunable` crate.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{fs::OpenOptions, io::Write};
use syn::{
    Expr, ExprArray, ExprLit, ExprRange, ExprUnary, ItemConst, Lit, Meta, RangeLimits, Token, Type,
    UnOp, parse::Parser, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

struct Args {
    min: i64,
    max: i64,
    step: i64,
    min_expr: Expr,
    max_expr: Expr,
    step_expr: Expr,
    // Elements of an array that are left at their defaults
    skip: Vec<usize>,
    // The tuner keeps the elements of an array in non-decreasing order
    nondecreasing: bool,
    // The tuner keeps the elements of an array in non-increasing order
    nonincreasing: bool,
    // A fitter keeps the tuned elements adding up to zero
    zero_mean: bool,
}

/// The value of an integer literal, possibly negated
fn literal(expr: &Expr) -> syn::Result<i64> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i.base10_parse(),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => Ok(-literal(expr)?),
        Expr::Paren(p) => literal(&p.expr),
        Expr::Group(g) => literal(&g.expr),
        other => Err(syn::Error::new(
            other.span(),
            "a tunable's default, min, max and step must be integer literals",
        )),
    }
}

/// The indices in `skip = [..]`: integer literals and ranges of them, like `[0, 2..4]`
fn skip_list(expr: &Expr) -> syn::Result<Vec<usize>> {
    let Expr::Array(ExprArray { elems, .. }) = expr else {
        return Err(syn::Error::new(
            expr.span(),
            "`skip` takes a list of indices, such as `skip = [0, 2..4]`",
        ));
    };
    let index = |e: &Expr| -> syn::Result<usize> {
        usize::try_from(literal(e)?)
            .map_err(|_| syn::Error::new(e.span(), "a skipped index can't be negative"))
    };
    let mut out = Vec::new();
    for elem in elems {
        match elem {
            Expr::Range(ExprRange {
                start: Some(start),
                end: Some(end),
                limits,
                ..
            }) => {
                let (start, end) = (index(start)?, index(end)?);
                let end = if matches!(limits, RangeLimits::Closed(_)) {
                    end + 1
                } else {
                    end
                };
                out.extend(start..end);
            }
            other => out.push(index(other)?),
        }
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

fn parse_args(args: TokenStream2) -> syn::Result<Args> {
    let metas = Punctuated::<Meta, Token![,]>::parse_terminated.parse2(args.clone())?;
    let (mut min, mut max, mut step) = (None, None, None);
    let mut skip = Vec::new();
    let mut nondecreasing = false;
    let mut nonincreasing = false;
    let mut zero_mean = false;
    for meta in metas {
        if matches!(&meta, Meta::Path(p) if p.is_ident("zero_mean")) {
            zero_mean = true;
            continue;
        }
        if matches!(&meta, Meta::Path(p) if p.is_ident("nondecreasing")) {
            nondecreasing = true;
            continue;
        }
        if matches!(&meta, Meta::Path(p) if p.is_ident("nonincreasing")) {
            nonincreasing = true;
            continue;
        }
        let Meta::NameValue(nv) = meta else {
            return Err(syn::Error::new(meta.span(), "expected `name = value`"));
        };
        if nv.path.is_ident("skip") {
            skip = skip_list(&nv.value)?;
            continue;
        }
        let slot = if nv.path.is_ident("min") {
            &mut min
        } else if nv.path.is_ident("max") {
            &mut max
        } else if nv.path.is_ident("step") {
            &mut step
        } else {
            return Err(syn::Error::new(
                nv.path.span(),
                "expected min, max, step, skip, nondecreasing, nonincreasing or zero_mean",
            ));
        };
        literal(&nv.value)?;
        *slot = Some(nv.value);
    }
    let missing = |name| syn::Error::new(args.span(), format!("missing `{name} = ..`"));
    let min_expr = min.ok_or_else(|| missing("min"))?;
    let max_expr = max.ok_or_else(|| missing("max"))?;
    let step_expr = step.ok_or_else(|| missing("step"))?;
    Ok(Args {
        min: literal(&min_expr)?,
        max: literal(&max_expr)?,
        step: literal(&step_expr)?,
        min_expr,
        max_expr,
        step_expr,
        skip,
        nondecreasing,
        nonincreasing,
        zero_mean,
    })
}

fn is_i32(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.is_ident("i32"))
}

/// The arguments of a constructor call such as `S(10, 20)`
fn parts_args(expr: &Expr) -> syn::Result<Vec<&Expr>> {
    match expr {
        Expr::Call(call) if matches!(&*call.func, Expr::Path(_)) && !call.args.is_empty() => {
            Ok(call.args.iter().collect())
        }
        other => Err(syn::Error::new(
            other.span(),
            "expected a constructor call of integer literals, such as `S(10, 20)`",
        )),
    }
}

/// Add this tunable to the tuner's manifest, if one was asked for
fn write_manifest(
    name: &str,
    array: bool,
    cols: usize,
    width: usize,
    defaults: &[i64],
    args: &Args,
) {
    let Ok(path) = std::env::var("TUNE_MANIFEST") else {
        return;
    };
    let list: Vec<String> = defaults.iter().map(i64::to_string).collect();
    let skip: Vec<String> = args.skip.iter().map(usize::to_string).collect();
    let line = format!(
        "{{\"name\":\"{name}\",\"array\":{array},\"default\":[{}],\"min\":{},\"max\":{},\"step\":{},\"skip\":[{}],\"nondecreasing\":{},\"nonincreasing\":{},\"zero_mean\":{},\"cols\":{},\"width\":{}}}\n",
        list.join(","),
        args.min,
        args.max,
        args.step,
        skip.join(","),
        args.nondecreasing,
        args.nonincreasing,
        args.zero_mean,
        cols,
        width
    );
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(line.as_bytes());
    }
}

fn expand(args: TokenStream2, item: ItemConst) -> syn::Result<TokenStream2> {
    let mut args = parse_args(args)?;

    let ident = &item.ident;
    let (elem_ty, array, grid): (&Type, bool, bool) = match &*item.ty {
        Type::Array(a) => match &*a.elem {
            Type::Array(row) => (&*row.elem, true, true),
            other => (other, true, false),
        },
        other => (other, false, false),
    };
    // Anything but an `i32` is a type made of integers, built by a constructor call
    let parts = !is_i32(elem_ty);

    // The elements of every row; one row for anything but a nested array
    let rows: Vec<Vec<&Expr>> = match &*item.expr {
        Expr::Array(ExprArray { elems, .. }) if grid => elems
            .iter()
            .map(|row| match row {
                Expr::Array(ExprArray { elems, .. }) => Ok(elems.iter().collect()),
                other => Err(syn::Error::new(
                    other.span(),
                    "expected an array of literals",
                )),
            })
            .collect::<syn::Result<_>>()?,
        Expr::Array(ExprArray { elems, .. }) if array => vec![elems.iter().collect()],
        other if !array => vec![vec![other]],
        other => {
            return Err(syn::Error::new(
                other.span(),
                "expected an array of literals",
            ));
        }
    };
    let cols = rows[0].len();
    if let Some(row) = rows.iter().find(|row| row.len() != cols) {
        return Err(syn::Error::new(
            row.first().map_or(ident.span(), |e| e.span()),
            format!("every row needs {cols} elements"),
        ));
    }
    let elems: Vec<&Expr> = rows.iter().flatten().copied().collect();
    let mut width = 1;
    let exprs: Vec<&Expr> = if parts {
        let mut flat = Vec::new();
        let mut first = None;
        for e in &elems {
            let call = parts_args(e)?;
            width = *first.get_or_insert(call.len());
            if call.len() != width {
                return Err(syn::Error::new(
                    e.span(),
                    format!("every element needs {width} integers"),
                ));
            }
            flat.extend(call);
        }
        flat
    } else {
        elems.clone()
    };
    let count = cols;
    if parts {
        // `skip` names elements, so it leaves every integer of one alone
        args.skip = args
            .skip
            .iter()
            .flat_map(|&i| i * width..(i + 1) * width)
            .collect();
    }
    // A skipped element is never tuned, so it may be any constant expression
    let defaults = exprs
        .iter()
        .enumerate()
        .map(|(i, e)| {
            if args.skip.contains(&i) {
                Ok(literal(e).unwrap_or(0))
            } else {
                literal(e)
            }
        })
        .collect::<syn::Result<Vec<i64>>>()?;
    let n = defaults.len();
    if n == 0 {
        return Err(syn::Error::new(
            item.expr.span(),
            "a tunable can't be empty",
        ));
    }

    if args.nondecreasing && args.nonincreasing {
        return Err(syn::Error::new(
            ident.span(),
            "`nondecreasing` and `nonincreasing` can't be used together",
        ));
    }
    let ordering = if args.nondecreasing {
        "nondecreasing"
    } else {
        "nonincreasing"
    };
    if (args.nondecreasing || args.nonincreasing) && !array {
        return Err(syn::Error::new(
            ident.span(),
            format!("`{ordering}` only applies to an array"),
        ));
    }
    if (args.nondecreasing || args.nonincreasing) && grid {
        return Err(syn::Error::new(
            ident.span(),
            format!("`{ordering}` doesn't apply to a nested array"),
        ));
    }
    if args.zero_mean && (!array || grid) {
        return Err(syn::Error::new(
            ident.span(),
            "`zero_mean` only applies to an array that isn't nested",
        ));
    }
    if !args.skip.is_empty() && !array {
        return Err(syn::Error::new(
            ident.span(),
            "`skip` only applies to an array tunable",
        ));
    }
    if let Some(&bad) = args.skip.iter().find(|&&i| i >= n) {
        return Err(syn::Error::new(
            ident.span(),
            format!(
                "`skip` has index {}, but the array has {} elements",
                bad / width,
                n / width
            ),
        ));
    }

    // A type made of integers is listed part by part even when it stands alone
    let array = array || parts;
    let name = ident.to_string().to_lowercase();
    write_manifest(
        &name,
        array,
        if grid { cols * width } else { 0 },
        width,
        &defaults,
        &args,
    );

    let attrs = &item.attrs;
    let vis = &item.vis;
    let (min, max, step) = (&args.min_expr, &args.max_expr, &args.step_expr);
    let skip = &args.skip;
    let zero_mean = args.zero_mean;
    let checked: Vec<&Expr> = exprs
        .iter()
        .enumerate()
        .filter(|(i, _)| !skip.contains(i))
        .map(|(_, e)| *e)
        .collect();
    // What each row is built from: the elements themselves, or the integers of a part type
    let row_init: Vec<TokenStream2> = (0..rows.len())
        .map(|r| {
            let (from, to) = (r * cols, (r + 1) * cols);
            if parts {
                let row = &elems[from..to];
                quote! { [#(#row),*] }
            } else {
                let row = &exprs[from..to];
                quote! { [#(#row),*] }
            }
        })
        .collect();
    let (live_row, fixed_row, live_new, fixed_new) = if parts {
        (
            quote! { ::tunable::LiveParts<#elem_ty, #width, #count> },
            quote! { ::tunable::FixedParts<#elem_ty, #width, #count> },
            quote! { ::tunable::LiveParts::new },
            quote! { ::tunable::FixedParts::new },
        )
    } else {
        (
            quote! { ::tunable::Live<#count> },
            quote! { ::tunable::Fixed<#count> },
            quote! { ::tunable::Live::new },
            quote! { ::tunable::Fixed::new },
        )
    };
    // The storage of row `r`, as the registry sees it
    let row_cells = |target: TokenStream2| {
        if parts {
            quote! { ::tunable::Cells::Group(&#target) }
        } else {
            quote! { ::tunable::Cells::Int(&#target.cells) }
        }
    };
    let height = rows.len();
    // The rows of a nested array are listed from a static of their own
    let rows_ident = quote::format_ident!("__{}_ROWS", ident);
    let (live, fixed, live_init, fixed_init, cells, rows_static) = if grid {
        let row_cells = (0..height).map(|r| row_cells(quote! { #ident[#r] }));
        (
            quote! { [#live_row; #height] },
            quote! { [#fixed_row; #height] },
            quote! { [#(#live_new(#row_init)),*] },
            quote! { [#(#fixed_new(#row_init)),*] },
            quote! { ::tunable::Cells::Rows(&#rows_ident) },
            quote! {
                #[cfg(feature = "tune")]
                static #rows_ident: [::tunable::Cells; #height] = [#(#row_cells),*];
            },
        )
    } else {
        let row = &row_init[0];
        (
            live_row,
            fixed_row,
            quote! { #live_new(#row) },
            quote! { #fixed_new(#row) },
            row_cells(quote! { #ident }),
            quote! {},
        )
    };

    // The manifest variable is read here so that cargo rebuilds when it changes
    Ok(quote! {
        const _: Option<&str> = option_env!("TUNE_MANIFEST");
        const _: () = ::tunable::check_range(&[#(#checked),*], #min, #max);

        #[cfg(feature = "tune")]
        #(#attrs)*
        #vis static #ident: #live = #live_init;

        #rows_static

        #[cfg(feature = "tune")]
        ::tunable::inventory::submit! {
            ::tunable::Entry::new(
                module_path!(),
                #name,
                #array,
                &[#(#exprs),*],
                #min,
                #max,
                #step,
                &[#(#skip),*],
                if #zero_mean { Some(#width) } else { None },
                #cells,
            )
        }

        #[cfg(not(feature = "tune"))]
        #(#attrs)*
        #vis const #ident: #fixed = #fixed_init;
    })
}

/// Mark an `i32`, `[i32; N]`, `[[i32; W]; H]`, or a type of integers (or an array or nested
/// array of them) as tunable
///
/// Without the using crate's `tune` feature the const stays a const. With it, the value can
/// be changed at run time through the `tunable` crate's registry.
#[proc_macro_attribute]
pub fn tunable(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemConst);
    match expand(args.into(), item) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
