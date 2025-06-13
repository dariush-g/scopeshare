use scopeshare::syncshare::SyncShare;

fn main() {
    let state = SyncShare::new(vec![1, 2, 3]);
    state.with_mut(|v| v.push(4));
    state.with(|v| println!("{v:?}"));
}
