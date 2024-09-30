use std::collections::VecDeque;

use crate::engine::document::Document;


#[derive(Debug, Clone)]
pub struct Collection {
    name: String,
    documents: VecDeque<Document>, // Contient les documents de la collection
}

impl Collection {
    pub fn new(name: String) -> Self {  
        Collection {
            name,
            documents: VecDeque::new(),
        }
    }

    // Méthodes pour gérer les documents
    pub fn add_document(&mut self, document: Document) {
        self.documents.push_back(document);
    }

    pub fn get_documents(&self) -> &VecDeque<Document> {
        &self.documents
    }
}