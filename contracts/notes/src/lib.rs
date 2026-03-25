#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan notes
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    // data note
}

// Storage key untuk data notes
// const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    // Fungsi untuk mendapatkan semua notes
    pub fn get_notes(env: Env) -> Vec<Note> {
        // 1. ambil data notes dari storage
    }

    // Fungsi untuk membuat note baru
    pub fn create_note(env: Env, title: String, content: String) -> String {
        // 1. ambil data notes dari storage
        
        // 2. Buat object note baru

        // 3. tambahkan note baru ke notes lama
        
        // 4. simpan notes ke storage
        
        return String::from_str(&env, "Notes berhasil ditambahkan");
    }
    // Fungsi untuk menghapus notes berdasarkan id
    pub fn delete_note(env: Env, id: u64) -> String {
        // 1. ambil data notes dari storage 
        
        // 2. buat variable index

        // 3. cari index note yang akan dihapus menggunakan perulangan

        // 4. hapus note berdasarkan index

        return String::from_str(&env, "Notes tidak ditemukan")
    }
}

mod test;











// pub fn get_notes(env: Env) -> Vec<Note> {
//     return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env))
// }

// pub fn create_note(env: Env, title: String, content: String) -> String {
//     // ambil notes lama (kalau ada)
//     let mut notes: Vec<Note> = env
//         .storage()
//         .instance()
//         .get(&NOTE_DATA)
//         .unwrap_or(Vec::new(&env));
    
//     // Buat object note baru
//     let note = Note {
//         id: env.prng().gen::<u64>(),
//         title,
//         content,
//         created_at: env.ledger().timestamp(),
//     };

//     // tambahkan note baru ke notes lama
//     notes.push_back(note.clone());
    
//     // simpan notes ke storage
//     env.storage().instance().set(&NOTE_DATA, &notes);
//     env.storage().instance().extend_ttl(1000, 1000);
    
//     return String::from_str(&env, "Notes berhasil ditambahkan");
// }

// pub fn delete_note(env: Env, id: u64) -> String {
//     // ambil notes lama
//     let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
//     let mut index: Option<u32> = None;

//     for i in 0..notes.len() {
//         if notes.get(i).unwrap().id == id {
//             index = Some(i);
//             break;
//         }
//     }

//     if let Some(i) = index {
//         notes.remove(i);
//         env.storage().instance().set(&NOTE_DATA, &notes);
//         env.storage().instance().extend_ttl(1000, 1000);

//         return String::from_str(&env, "Notes berhasil dihapus");
//     }

//     String::from_str(&env, "Notes tidak ditemukan")
// }