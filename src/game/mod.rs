mod deck;

use std::collections::HashMap;

use entity::user::{
    ActiveModel as UserActiveModel, Column, Entity as UserEntity, Model as UserModel,
};
use playin_cards::Card;
use rand::{thread_rng, Rng};
use sea_orm::prelude::Uuid;

use self::deck::Deck;

#[derive(PartialEq)]
pub struct Player {
    uuid: Uuid,
    user: UserModel,
    cards: HashMap<u8, Card>,
}

pub struct Room {
    admin_uuid: Option<Uuid>,
    users: HashMap<UserModel, ReadyToPlay>,
    game: Option<Game>,
    room_status: RoomStatus,
}

#[derive(PartialEq, Eq)]
pub struct ReadyToPlay(bool);

impl Default for ReadyToPlay {
    fn default() -> Self {
        Self(false)
    }
}

pub enum RoomStatus {
    GameNotStarted,
    GameIsWaitingToStart,
    GameInProgress,
}

pub struct Game {
    shoe: Deck,
    players: Vec<Player>,
    status: (),
}

impl Default for Room {
    fn default() -> Self {
        Self {
            admin_uuid: None,
            users: HashMap::default(),
            game: None,
            room_status: RoomStatus::GameNotStarted,
        }
    }
}

pub enum ActionResult {
    Failed,
    AlreadyDone,
    Success,
}

impl Room {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn join(mut self, user: UserModel) -> ActionResult {
        if let Some(_) = self.users.get(&user) {
            ActionResult::AlreadyDone
        } else {
            if let None = self.admin_uuid {
                let user_uuid = match (&user).uuid {
                    Some(uuid) => uuid.clone(),
                    None => return ActionResult::Failed,
                };

                self.admin_uuid = Some(user_uuid);
            }

            self.users.insert(user, Default::default());

            ActionResult::Success
        }
    }

    pub fn leave(&mut self, user: &UserModel) -> ActionResult {
        match user.uuid {
            None => ActionResult::Failed,
            Some(user_uuid) => match &self.users.remove(user) {
                Some(_) => match self.admin_uuid {
                    Some(admin_uuid) => {
                        if &user_uuid == &admin_uuid {
                            for (u, s) in &self.users {
                                if let Some(random_user_uuid) = u.uuid {
                                    self.admin_uuid = Some(random_user_uuid.clone());
                                }
                            }
                        }

                        return ActionResult::Success;
                    }
                    None => return ActionResult::Success,
                },
                None => ActionResult::AlreadyDone,
            },
        }
    }

    pub fn change_user_ready_status(
        &mut self,
        user: &UserModel,
        new_status: ReadyToPlay,
    ) -> ActionResult {
        match self.users.get_mut(user) {
            Some(current_status) => {
                if *current_status == new_status {
                    ActionResult::AlreadyDone
                } else {
                    *current_status = new_status;

                    ActionResult::Success
                }
            }
            None => ActionResult::Failed,
        }
    }

    pub fn replace_admin(&mut self, user: &UserModel) -> ActionResult {
        match (user.uuid, self.admin_uuid) {
            (None, None) | (None, Some(_)) => ActionResult::Failed,
            (Some(user_uuid), None) => {
                self.admin_uuid = Some(user_uuid.clone());

                ActionResult::Success
            }
            (Some(user_uuid), Some(admin_uuid)) => {
                if &user_uuid == &admin_uuid {
                    ActionResult::AlreadyDone
                } else {
                    self.admin_uuid = Some(user_uuid.clone());

                    ActionResult::Success
                }
            }
        }
    }
}

// fn init_game() {
//     let mut rng = thread_rng();
//     let shoe = Deck::new_shuffled(false, &mut rng);

//     let r = Room {
//         players: vec![],
//         shoe,
//     };
// }
