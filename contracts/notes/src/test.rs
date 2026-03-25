#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_crud() {
    let env = Env::default();
    let contract_id = env.register(NotesContract, ());
    let client = NotesContractClient::new(&env, &contract_id);

    // Create note 1
    let title1 = String::from_str(&env, "First Note");
    let content1 = String::from_str(&env, "This is the first note.");
    let id1 = client.create_note(&title1, &content1);

    // Create note 2
    let title2 = String::from_str(&env, "Second Note");
    let content2 = String::from_str(&env, "This is the second note.");
    let id2 = client.create_note(&title2, &content2);

    let notes = client.get_notes();
    assert_eq!(notes.len(), 2);
    
    assert_eq!(notes.get(0).unwrap().title, title1);
    assert_eq!(notes.get(1).unwrap().title, title2);

    // Test update
    let updated_title = String::from_str(&env, "Updated First Note");
    let success_update = client.update_note(&id1, &updated_title, &content1);
    assert!(success_update);

    let notes_after_update = client.get_notes();
    assert_eq!(notes_after_update.get(0).unwrap().title, updated_title);

    // Test delete
    let success_delete = client.delete_note(&id1);
    assert!(success_delete);
    
    let notes_after_delete = client.get_notes();
    assert_eq!(notes_after_delete.len(), 1);
    assert_eq!(notes_after_delete.get(0).unwrap().id, id2);
}
