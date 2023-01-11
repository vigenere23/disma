#![allow(non_camel_case_types)]

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Serialize, Deserialize, Clone, Debug, Display, Eq, PartialEq, Hash, EnumIter)]
pub enum Permission {
    CREATE_INSTANT_INVITE,
    KICK_MEMBERS,
    BAN_MEMBERS,
    ADMINISTRATOR,
    MANAGE_CHANNELS,
    MANAGE_GUILD,
    ADD_REACTIONS,
    VIEW_AUDIT_LOG,
    PRIORITY_SPEAKER,
    STREAM,
    VIEW_CHANNEL,
    SEND_MESSAGES,
    SEND_TTS_MESSAGES,
    MANAGE_MESSAGES,
    EMBED_LINKS,
    ATTACH_FILES,
    READ_MESSAGE_HISTORY,
    MENTION_EVERYONE,
    USE_EXTERNAL_EMOJIS,
    VIEW_GUILD_INSIGHTS,
    CONNECT,
    SPEAK,
    MUTE_MEMBERS,
    DEAFEN_MEMBERS,
    MOVE_MEMBERS,
    USE_VAD,
    CHANGE_NICKNAME,
    MANAGE_NICKNAMES,
    MANAGE_ROLES,
    MANAGE_WEBHOOKS,
    MANAGE_EMOJIS_AND_STICKERS,
    USE_APPLICATION_COMMANDS,
    REQUEST_TO_SPEAK,
    MANAGE_EVENTS,
    MANAGE_THREADS,
    CREATE_PUBLIC_THREADS,
    CREATE_PRIVATE_THREADS,
    USE_EXTERNAL_STICKERS,
    SEND_MESSAGES_IN_THREADS,
    USE_EMBEDDED_ACTIVITIES,
    MODERATE_MEMBERS,
}

impl Permission {
    fn code(&self) -> u64 {
        match self {
            Self::CREATE_INSTANT_INVITE => 1 << 0,
            Self::KICK_MEMBERS => 1 << 1,
            Self::BAN_MEMBERS => 1 << 2,
            Self::ADMINISTRATOR => 1 << 3,
            Self::MANAGE_CHANNELS => 1 << 4,
            Self::MANAGE_GUILD => 1 << 5,
            Self::ADD_REACTIONS => 1 << 6,
            Self::VIEW_AUDIT_LOG => 1 << 7,
            Self::PRIORITY_SPEAKER => 1 << 8,
            Self::STREAM => 1 << 9,
            Self::VIEW_CHANNEL => 1 << 10,
            Self::SEND_MESSAGES => 1 << 11,
            Self::SEND_TTS_MESSAGES => 1 << 12,
            Self::MANAGE_MESSAGES => 1 << 13,
            Self::EMBED_LINKS => 1 << 14,
            Self::ATTACH_FILES => 1 << 15,
            Self::READ_MESSAGE_HISTORY => 1 << 16,
            Self::MENTION_EVERYONE => 1 << 17,
            Self::USE_EXTERNAL_EMOJIS => 1 << 18,
            Self::VIEW_GUILD_INSIGHTS => 1 << 19,
            Self::CONNECT => 1 << 20,
            Self::SPEAK => 1 << 21,
            Self::MUTE_MEMBERS => 1 << 22,
            Self::DEAFEN_MEMBERS => 1 << 23,
            Self::MOVE_MEMBERS => 1 << 24,
            Self::USE_VAD => 1 << 25,
            Self::CHANGE_NICKNAME => 1 << 26,
            Self::MANAGE_NICKNAMES => 1 << 27,
            Self::MANAGE_ROLES => 1 << 28,
            Self::MANAGE_WEBHOOKS => 1 << 29,
            Self::MANAGE_EMOJIS_AND_STICKERS => 1 << 30,
            Self::USE_APPLICATION_COMMANDS => 1 << 31,
            Self::REQUEST_TO_SPEAK => 1 << 32,
            Self::MANAGE_EVENTS => 1 << 33,
            Self::MANAGE_THREADS => 1 << 34,
            Self::CREATE_PUBLIC_THREADS => 1 << 35,
            Self::CREATE_PRIVATE_THREADS => 1 << 36,
            Self::USE_EXTERNAL_STICKERS => 1 << 37,
            Self::SEND_MESSAGES_IN_THREADS => 1 << 38,
            Self::USE_EMBEDDED_ACTIVITIES => 1 << 39,
            Self::MODERATE_MEMBERS => 1 << 40,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionsList {
    permissions: HashSet<Permission>,
}

impl PermissionsList {
    fn new(permissions: HashSet<Permission>) -> Self {
        Self { permissions }
    }

    pub fn code(&self) -> String {
        let mut code: u64 = 0;

        for permission in self.permissions.iter() {
            code |= permission.code()
        }

        format!("{}", code)
    }

    pub fn to_list(&self) -> Vec<Permission> {
        self.permissions.iter().cloned().collect()
    }
}

impl From<&str> for PermissionsList {
    fn from(code: &str) -> Self {
        let num_code: u64 = code.parse().unwrap();
        Self::from(num_code)
    }
}

impl From<u64> for PermissionsList {
    fn from(code: u64) -> Self {
        let mut permissions = HashSet::new();

        for permission in Permission::iter() {
            let permission_code = permission.code();
            if (code & permission_code) == permission_code {
                permissions.insert(permission);
            }
        }

        Self::new(permissions)
    }
}

impl From<Vec<Permission>> for PermissionsList {
    fn from(permissions: Vec<Permission>) -> Self {
        Self {
            permissions: HashSet::from_iter(permissions.into_iter()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod persission {
        use std::collections::HashSet;

        use crate::permission::Permission;

        #[test]
        fn same_permissions_are_equal() {
            let permission1 = Permission::ADMINISTRATOR;
            let permission2 = Permission::ADMINISTRATOR;

            assert!(permission1 == permission2);
            assert!(permission1.eq(&permission2));

            let set = HashSet::from([permission1, permission2]);
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn different_permissions_are_not_equal() {
            let permission1 = Permission::ADMINISTRATOR;
            let permission2 = Permission::SEND_MESSAGES;

            assert!(permission1 != permission2);
            assert!(!permission1.eq(&permission2));

            let set = HashSet::from([permission1, permission2]);
            assert_eq!(set.len(), 2);
        }

        #[test]
        fn can_format_to_str() {
            let permission = Permission::SEND_MESSAGES;

            assert_eq!(permission.to_string(), "SEND_MESSAGES");
        }
    }

    mod permissions_list {
        use crate::permission::{Permission, PermissionsList};

        #[test]
        fn when_empty_then_code_is_0() {
            let permission_list = PermissionsList::from(vec![] as Vec<Permission>);
            assert_eq!(permission_list.code(), "0");
        }

        #[test]
        fn does_not_care_about_duplicates() {
            let permissions = vec![Permission::ADMINISTRATOR, Permission::ADMINISTRATOR];
            let permission_list = PermissionsList::from(permissions);

            let expected_code = Permission::ADMINISTRATOR.code().to_string();
            assert_eq!(permission_list.code(), expected_code);
        }

        #[test]
        fn can_create_code_from_permissions() {
            let permissions = vec![
                Permission::ADD_REACTIONS,
                Permission::EMBED_LINKS,
                Permission::USE_EXTERNAL_EMOJIS,
            ];
            let permission_list = PermissionsList::from(permissions);
            let code = permission_list.code();

            assert_eq!(code, "278592");
        }

        #[test]
        fn can_create_permissions_from_code() {
            let permission_list = PermissionsList::from("278592");
            let code = permission_list.code();

            assert_eq!(code, "278592");
        }

        #[test]
        fn lists_are_equal_if_same_permissions() {
            let list1 =
                PermissionsList::from(vec![Permission::ADD_REACTIONS, Permission::ADMINISTRATOR]);
            let list2 =
                PermissionsList::from(vec![Permission::ADD_REACTIONS, Permission::ADMINISTRATOR]);

            assert_eq!(list1, list2);
        }

        #[test]
        fn lists_are_not_equal_if_different_permissions() {
            let list1 =
                PermissionsList::from(vec![Permission::ADD_REACTIONS, Permission::ADMINISTRATOR]);
            let list2 =
                PermissionsList::from(vec![Permission::ADD_REACTIONS, Permission::SEND_MESSAGES]);

            assert_ne!(list1, list2);
        }
    }
}
