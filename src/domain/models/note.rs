use serde::{Deserialize, Serialize};

// Struct untuk entitas Note dalam domain bisnis
#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// impl Note {
//     // Constructor untuk membuat instance baru dari Note
//     pub fn new(id: String, title: String, content: String, is_published: bool,
//                created_at: chrono::DateTime<chrono::Utc>, updated_at: chrono::DateTime<chrono::Utc>) -> Self {
//         Note { id, title, content, is_published, created_at, updated_at }
//     }
// }

// Struct untuk representasi data dari Note untuk operasi IO (Input/Output)
#[derive(Debug, Deserialize, Serialize)]
pub struct NoteDto {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Note> for NoteDto {
    fn from(note: Note) -> Self {
        NoteDto {
            id: note.id,
            title: note.title,
            content: note.content,
            is_published: note.is_published,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}
