use serenity::prelude::TypeMapKey;
use std::{collections::HashMap, ops::Deref};

use crate::Event;

pub struct Entry {
    inner: Event,
    flagged: bool,
}

impl Entry {
    pub const fn new(inner: Event) -> Self {
        Self {
            inner,
            flagged: false,
        }
    }

    pub fn update(&mut self, inner: Event) {
        self.inner = inner;
        self.flagged = false
    }

    pub const fn is_flagged(&self) -> bool {
        self.flagged
    }

    pub fn flag(&mut self) {
        self.flagged = true
    }
}

impl Deref for Entry {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Events;

impl TypeMapKey for Events {
    type Value = HashMap<String, Entry>;
}
