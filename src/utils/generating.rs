use rand::Rng;

pub fn generate_collection_name() -> String {
    let mut rng = rand::thread_rng();
    let name_length = 8; // Longueur du nom de la collection
    let collection_name: String = (0..name_length)
        .map(|_| {
            let idx = rng.gen_range(0..26); // Choisir un index aléatoire entre 0 et 25
            (b'a' + idx) as char // Convertir l'index en caractère (a-z)
        })
        .collect();
    
    collection_name
}