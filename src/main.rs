#[derive(PartialEq, Debug, Clone, Default)]
struct A {
    a1: String,
    a2: Option<Box<B>>,
}

#[derive(PartialEq, Debug, Clone, Default)]
struct B {
    b1: String,
    b2: Option<Box<A>>,
}

fn main() {
    let mut a1 = A::default();
    let mut a2 = A::default();
    let mut b1 = B::default();
    let b2 = B::default();
    assert_eq!(a1, a2, "default A values are not equal");
    assert_eq!(b1, b2, "default B values are not equal");
    b1.b1 = String::new();
    assert_eq!(b1, b2, "new empty Strings are not equal");
    a1.a2 = Some(Box::new(b1.clone()));
    a2.a2 = Some(Box::new(b1.clone()));
    assert_eq!(a1, a2, "pointer to same val in a2 is not equal");
    let mut b2= b2.clone();
    b2.b2 = Some(Box::new(a1.clone()));
    a2.a2 = Some(Box::new(b2));
    assert_ne!(a1, a2, "deep equality is not being checked")
}
