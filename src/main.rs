#[derive(Default)]
struct Cli {}

struct Tran<'a> {
    client: &'a mut Cli,
}

impl Cli {
    async fn tran(&mut self) -> Tran<'_> {
        Tran { client: self }
    }
}

impl<'c> Tran<'c> {
    async fn commit(self) {}
}

impl Drop for Tran<'_> {
    fn drop(&mut self) {}
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
