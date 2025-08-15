use common::Numbered;

// This is identical to the `a` crate.
mod c {
    use common::Numbered;

    pub struct AA;
    pub struct A {}

    impl Numbered<0> for A { type T = AA; }
}

pub struct B {}

impl AsRef<String> for B { fn as_ref(&self) -> &String { unimplemented!() }}
impl AsRef<<a::A as Numbered<0>>::T> for B { fn as_ref( & self ) -> &<a::A as Numbered<0>>::T { unimplemented!() }}
impl AsRef<<c::A as Numbered<0>>::T> for B { fn as_ref( & self ) -> &<c::A as Numbered<0>>::T { unimplemented!() }}
