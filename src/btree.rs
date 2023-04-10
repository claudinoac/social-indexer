use std::io::{Read, Write, Seek, SeekFrom};

#[derive(Clone)]
pub struct Node {
    address: u64,
    keys: Vec<i32>,
    children: Vec<Node>,
}

pub const MAX_KEYS: usize = 4;

impl Node {
    pub fn new(address: u64) -> Self {
        Node {
            address,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn write_to_disk(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        // Escreve o endereço do nó no arquivo
        file.write_all(&self.address.to_ne_bytes())?;
        // Escreve o número de chaves no arquivo
        let num_keys = self.keys.len() as u32;
        file.write_all(&num_keys.to_ne_bytes())?;
        // Escreve as chaves no arquivo
        for key in &self.keys {
            file.write_all(&key.to_ne_bytes())?;
        }
        // Escreve os endereços dos filhos no arquivo
        for child in &self.children {
            file.write_all(&child.address.to_ne_bytes())?;
        }
        Ok(())
    }
    pub fn read_from_disk(address: u64, file: &mut std::fs::File) -> std::io::Result<Node> {
        // Move o cursor para o endereço do nó no arquivo
        file.seek(SeekFrom::Start(address))?;
        // Lê o número de chaves do arquivo
        let mut buf = [0; 4];
        let mut buf2 = [0; 8];

        file.read_exact(&mut buf)?;
        let num_keys = u32::from_ne_bytes(buf) as usize;
        // Lê as chaves do arquivo
        let mut keys = Vec::with_capacity(num_keys);
        for _ in 0..num_keys {
            buf = [0; 4];
            file.read_exact(&mut buf)?;
            keys.push(i32::from_ne_bytes(buf));
        }
        // Lê os endereços dos filhos do arquivo
        let mut children = Vec::with_capacity(num_keys + 1);
        for _ in 0..num_keys + 1 {
            buf2 = [0; 8];
            file.read_exact(&mut buf2)?;
            let child_address = u64::from_ne_bytes(buf2);
            let child = Node::read_from_disk(child_address, file)?;
            children.push(child);
        }
        Ok(Node {
            address,
            keys,
            children,
        })
    }
    pub fn insert(&mut self, key: i32, file: &mut std::fs::File) -> Option<Node> {
        let mut i = 0;
        while i < self.keys.len() && key > self.keys[i] {
            i += 1;
        }
        if !self.children.is_empty() {
            // A chave deve ser inserida em um dos filhos
            let mut child = self.children[i].clone();
            let new_child = child.insert(key, file);
            if let Some(new_child) = new_child {
                // O filho ficou cheio e foi dividido
                self.keys.insert(i, new_child.keys[0]);
                self.children.insert(i + 1, new_child.children[0].clone());
                self.children[i] = new_child.children[1].clone();
            }
        } else {
            // O nó é uma folha, a chave deve ser inserida aqui
            self.keys.insert(i, key);
            if self.keys.len() > MAX_KEYS {
                // O nó ficou cheio e deve ser dividido
                let split_idx = self.keys.len() / 2;
                let new_node = Node {
                    address: 0, // endereço será definido ao escrever no disco
                    keys: self.keys.split_off(split_idx),
                    children: vec![],
                };
                let new_key = new_node.keys[0];
                if self.write_to_disk(file).ok().is_none() {
                    return None;
                }
                if new_node.write_to_disk(file).ok().is_none() {
                    return None;
                }
                return Some(Node {
                    address: 0, // endereço será definido ao escrever no disco
                    keys: vec![new_key],
                    children: vec![self.clone(), new_node],
                });
            }
        }
        None
    }
    pub fn search(&self, key: i32) -> bool {
        let mut i = 0;
        while i < self.keys.len() && key > self.keys[i] {
            i += 1;
        }
        if self.keys.get(i) == Some(&key) {
            // A chave foi encontrada neste nó
            return true;
        }
        if self.children.get(i).is_some() {
            // A chave pode estar em um dos filhos
            return self.children[i].search(key);
        }
        false
    } 
}



