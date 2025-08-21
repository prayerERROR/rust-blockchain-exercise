use super::hash::get_hash;

pub struct MerkleTree {
    root: Option<String>,
    leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let leaves: Vec<String> = data.iter().map(|x| get_hash(x)).collect();
        let root = match leaves.is_empty() {
            true => None,
            false => Some(Self::calculate_root(&leaves)),
        };

        MerkleTree {
            root: root,
            leaves: leaves,
        }
    }

    pub fn calculate_root(leaves: &[String]) -> String {
        let mut curr_level: Vec<String> = leaves.to_vec();

        while curr_level.len() > 1 {
            let mut upper_level: Vec<String> = Vec::new();

            // If the size is odd number, copy the last element
            if curr_level.len() % 2 == 1 {
                curr_level.push(curr_level.last().cloned().unwrap());
            }
            
            // Calculate upper level hash
            for i in 0..curr_level.len() / 2 {
                let left = &curr_level[2*i];
                let right = &curr_level[2*i+1];
                let combined = format!("{left}{right}");
                upper_level.push(get_hash(combined.as_bytes()));
            }

            // Move the upper level to current level
            curr_level = upper_level;
        }

        curr_level.pop().unwrap()
    }

    pub fn get_root(&self) -> Option<String> {
        self.root.clone()
    }
}