#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Whealt,
}

pub fn cereal() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Barley);
    println!("{:?}", grains);
}
