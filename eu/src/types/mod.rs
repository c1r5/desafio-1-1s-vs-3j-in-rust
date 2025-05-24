use std::{collections::HashMap, sync::RwLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub name: String,
    pub leader: bool,
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub date: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usuario {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub score: u32,
    pub active: bool,
    pub country: String,
    pub team: Team,
    pub logs: Vec<Log>,
}

pub type UserDB = RwLock<Vec<Usuario>>;

pub trait UserClient {
    fn new() -> Self;
}

impl UserClient for UserDB {
    fn new() -> Self {
        RwLock::new(Vec::new())
    } 
}