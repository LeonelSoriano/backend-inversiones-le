use rand::Rng;

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NAME_LEN: usize = 64;

pub fn get_random_name() -> String {
    let mut id = String::with_capacity(NAME_LEN);
    let mut rng = rand::thread_rng();
    for _ in 0..NAME_LEN {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }
    id
}
