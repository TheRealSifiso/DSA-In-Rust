/*
A few traits you must be familiar with:
-> PartialEq
-> Eq
-> PartialOrd
-> ord

To express what it means for values of your own custom types to be equal,
you must implement the 'PartialEq' trait. PartialEq is a trait that defines
partial equality or partial equivalence relations. This means that this
trait allows for types that do not have a full equivalence relation. For
example, in floating point numbers "NaN != NaN", therefore floating point
types implement 'PartialEq' but not 'Eq' (a marker trait used to declare
that PartialEq is also reflexive).

The following are links to very useful introductions to the concept of NaN:
-> https://www.lucidchart.com/techblog/2022/03/04/if-its-not-a-number-what-is-it-demystifying-nan-for-the-working-programmer/
-> https://stackoverflow.com/questions/59335027/what-is-nan-not-a-number-in-the-words-of-a-beginner

To break it all down:

A partial equivalence relation is one that is:

1) Symmetric: if a == b, then b == a for all 'a' and 'b' of the respective
    type; and

2) Transitive: if a == b and b == c, then a == c for all 'a', 'b' and 'c'
    of the type.

This trait can be derived with #[derive]. When derived on structs, two
instances equal of all fields are equal, and not equal if any fields are
not equal. But when derived on enums, two instances are equal if they are
the same variant and all fields are equal:

#[derive(Eq)]

You can, if appropriate, decide that two instances of a struct are equal
is only one or more of their fields are 'equal', whatever that means in
context:

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {}
}

For more on 'PartialEq', have a look at this article:
https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

The 'Eq' trait is used to define "full" equivalence relations. This means
that, in addition to being Symmetric and Transitive, the relation is
Reflexive: a == a for all 'a' of the respective type. This equality
cannot be checked by the compiler, and therefore 'Eq' implied 'PartialEq',
and had no extra methods.

Remember: 'f32' and 'f64' types implement 'PartialEq', but not 'Eq'
because NaN != NaN, according to the IEEE-754 standard - that descibes
floating-point formats to represent real numbers in hardware.

In most cases, you'll implement both the 'PartialEq' and 'Eq' trait.

This trait can also be derived: #[derive(Eq)]

Have a look the following article:

https://llogiq.github.io/2015/07/30/traits.html


'PartialOrd' is a trait for types that form a partial order:

1) a == b
2) a < b
3) a > b
4) a <= b if and only if a < b || a == b
5) a >= b if and only if a > b || a == b
6) a != b if and only if !(a == b)

PartialOrd extends the equality of PartialEq by the Ordering relation.
Partial in this case means there may be instances of your type that
cannot be meaningfully compared e.g. A String "42" vs a u8 42.

The partial_cmp(...) method returns Option<Ordering>. Therefore, it can
return None for incomparable instances. 

*/

pub trait Sorter {
    fn sort<T: Ord>(slice: &mut [T]);
}

pub fn sort<T: Ord, S: Sorter>(slice: &mut [T]) {
    S::sort(slice)
}

mod bubblesort;

pub use bubblesort::BubbleSort;

#[cfg(test)]
mod tests {
    use super::*;
 
    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T: Ord>(slice: &mut [T]) {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![9, 8, 3 , 5, 0, 7];

        sort::<_, BubbleSort>(&mut things);

        assert_eq!(things, &[0, 3, 5, 7, 8, 9]);
    }
}