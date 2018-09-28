use std::cell::{Ref, RefCell, RefMut};

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
    val: RefCell<T>,
    generation: u64,
}

pub struct GenerationalIndexArray<T> {
    data: Vec<Option<GenerationalArrayEntry<T>>>,
}

impl<T> GenerationalIndexArray<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn set(&mut self, gen_idx: &GenerationalIndex, val: T) -> bool {
        if gen_idx.idx >= self.data.len() {
            // If the index we're setting is larger than the current array, resize it and fill the new slots with `None`s.
            while self.data.len() <= gen_idx.idx {
                self.data.push(None);
            }
            // TODO: Soooon you'll be able to do this.  Nightly-only, right now.
            // self.data.resize_with(gen_idx.idx + 1, None);
        }

        // TODO: Should we be checking generations at all here?
        match self.data[gen_idx.idx] {
            Some(ref mut e) => {
                // Don't allow old generations to overwrite new generations.
                if e.generation > gen_idx.generation {
                    panic!("Can this even happen?");
                }
            },
            _ => (),
        };

        self.data[gen_idx.idx] = Some(GenerationalArrayEntry {
            val: RefCell::new(val),
            generation: gen_idx.generation,
        });
        true
    }

    pub fn borrow(&self, gen_idx: &GenerationalIndex) -> Option<Ref<T>> {
        // TODO: Dedup this and `get_mut`.
        if !self.check_idx(gen_idx) {
            return None;
        }

        match self.data[gen_idx.idx] {
            Some(ref e) => {
                Some(e.val.borrow())
            },
            None => None,
        }
    }

    pub fn borrow_mut(&self, gen_idx: &GenerationalIndex) -> Option<RefMut<T>> {
        if !self.check_idx(gen_idx) {
            return None;
        }

        match self.data[gen_idx.idx] {
            Some(ref e) => {
                Some(e.val.borrow_mut())
            },
            None => None,
        }
    }

    pub fn has_entry(&self, gen_idx: &GenerationalIndex) -> bool {
        return self.check_idx(gen_idx) && self.data[gen_idx.idx].is_some()
    }

    /// Returns true if `gen_idx` points to a valid entry.  Otherwise, false.
    fn check_idx(&self, gen_idx: &GenerationalIndex) -> bool {
        if gen_idx.idx >= self.data.len() {
            return false;
        }
        if let Some(ref e) = self.data[gen_idx.idx] {
            if e.generation != gen_idx.generation  {
                return false;
            }
        }
        true
    }
}
