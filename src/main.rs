use std::sync::Arc;

#[derive(Default)]
struct Cli {}

struct Tran<'a> {
    client: &'a mut Cli,
    _no_moving: Arc<()>,
}

impl Cli {
    async fn tran(&mut self) -> Tran<'_> {
        Tran {
            client: self,
            _no_moving: Default::default(),
        }
    }
}

impl<'c> Tran<'c> {
    async fn commit(self) {}
}

async fn async_main() {
    let mut client = Cli::default();
    let mut tran = client.tran().await;

    loop {
        tran.commit().await;
        tran = client.tran().await;
    }
}

fn main() {}
