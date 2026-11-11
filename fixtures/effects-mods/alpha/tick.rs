fn tick(entity:&mut Entity) { let health=entity.health; entity.__curse_mixins_alpha_history_mut().push(health); }
