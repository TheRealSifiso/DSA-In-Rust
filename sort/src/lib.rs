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

/*

The architecture of this program closely resembles the typical State
Design pattern - an object oriented behavioural design pattern that
allows an object to change behaviour when its internal state changes.

"The pattern extracts state-related behaviors into separate state 
classes and forces the original object to delegate the work to an 
instance of these classes, instead of acting on its own.
"

The biggest advantage of the State Design Pattern as opposed to the use
of numerous conditional statements is that it improves scalability.

In rust, each state is represented by a separate type that implements
a common state trait. "Transitions between states depend on the
particular trait implementation for each state type."

*/

pub trait Sorter {
    
}