use common::Numbered;

pub struct AA;
pub struct A {}

impl Numbered<0> for A { type T = AA; }

