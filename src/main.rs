struct Holder<'a> {
    parent: &'a mut (),
}

impl Drop for Holder<'_> {
    fn drop(&mut self) {}
}

fn main() {
    let mut parent = ();
    let mut tran = Holder {
        parent: &mut parent,
    };

    loop {
        drop(tran);
        tran = Holder {
            parent: &mut parent,
        };
    }
}
