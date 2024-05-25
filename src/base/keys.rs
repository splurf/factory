use serenity::prelude::TypeMapKey;
use std::collections::HashMap;

use crate::Item;

pub struct Items;

impl TypeMapKey for Items {
    type Value = HashMap<String, Item>;
}
