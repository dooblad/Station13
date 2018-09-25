pub struct GenerationalIndex {
    idx: usize,
    generation: u64,
}

struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}

pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> GenerationalIndex {
        if let Some(e_idx) = self.free.pop() {
            // Use item from the free list.
            let mut entry = &mut self.entries[e_idx];
            entry.is_live = true;
            entry.generation += 1;
            let gen_idx = GenerationalIndex { idx: e_idx, generation: entry.generation };
            gen_idx
        } else {
            // No free entries.  Allocate new one.
            let gen_idx = GenerationalIndex { idx: self.entries.len(), generation: 0 };
            self.entries.push(AllocatorEntry { is_live: true, generation: 0 });
            gen_idx
        }
    }

    pub fn deallocate(&mut self, gen_idx: &GenerationalIndex) -> bool {
        if self.is_live(gen_idx) {
            let entry = &mut self.entries[gen_idx.idx];
            entry.is_live = false;
            self.free.push(gen_idx.idx);
            true
        } else {
            false
        }
    }

    pub fn is_live(&self, gen_idx: &GenerationalIndex) -> bool {
        let entry = &self.entries[gen_idx.idx];
        entry.is_live && entry.generation == gen_idx.generation
    }
}

pub struct GenerationalArrayEntry<T> {
    val: T,
    generation: u64,
}

pub struct GenerationalIndexArray<T>(pub Vec<Option<GenerationalArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn set(&mut self, gen_idx: &GenerationalIndex, val: T) -> bool {
        if gen_idx.idx >= self.0.len() {
            // If the index we're setting is larger than the current array, resize it and fill the new slots with `None`s.
            while self.0.len() <= gen_idx.idx {
                self.0.push(None);
            }
            // TODO: Soooon you'll be able to do this.  Nightly-only, right now.
            // self.0.resize_with(gen_idx.idx + 1, None);
        }

        // TODO: Should we be checking generations at all here?
        match self.0[gen_idx.idx] {
            Some(ref mut e) => {
                // Don't allow old generations to overwrite new generations.
                if e.generation > gen_idx.generation {
                    panic!("Can this even happen?");
                }
            },
            _ => (),
        };

        self.0[gen_idx.idx] = Some(GenerationalArrayEntry {
            val,
            generation: gen_idx.generation,
        });
        true
    }

    pub fn borrow(&self, gen_idx: &GenerationalIndex) -> Option<&T> {
        // TODO: Dedup this and `get_mut`.
        if !self.check_idx(gen_idx) {
            return None;
        }

        match self.0[gen_idx.idx] {
            Some(ref e) => {
                Some(&e.val)
            },
            None => None,
        }
    }

    pub fn borrow_mut(&mut self, gen_idx: &GenerationalIndex) -> Option<&mut T> {
        if !self.check_idx(gen_idx) {
            return None;
        }

        match self.0[gen_idx.idx] {
            Some(ref mut e) => {
                Some(&mut e.val)
            },
            None => None,
        }
    }

    /// Returns true if `gen_idx` points to a valid entry.  Otherwise, false.
    fn check_idx(&self, gen_idx: &GenerationalIndex) -> bool {
        if gen_idx.idx >= self.0.len() {
            return false;
        }
        if let Some(ref e) = self.0[gen_idx.idx] {
            if e.generation != gen_idx.generation  {
                return false;
            }
        }
        true
    }
}
