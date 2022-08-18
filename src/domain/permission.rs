#![allow(dead_code)]

use core::fmt;
use std::{collections::HashSet, fmt::Display};

#[non_exhaustive]
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Permission {
    pub name: &'static str,
    pub code: u64,
}

impl Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl Permission {
    pub const CREATE_INSTANT_INVITE: Permission = Permission {
        name: "Create instant invite",
        code: 1 << 0,
    };

    pub const KICK_MEMBERS: Permission = Permission {
        name: "Kick members",
        code: 1 << 1,
    };

    pub const BAN_MEMBERS: Permission = Permission {
        name: "Ban members",
        code: 1 << 2,
    };

    pub const ADMINISTRATOR: Permission = Permission {
        name: "Administrator",
        code: 1 << 3,
    };

    pub const MANAGE_CHANNELS: Permission = Permission {
        name: "Manage channels",
        code: 1 << 4,
    };

    pub const MANAGE_GUILD: Permission = Permission {
        name: "Manage guild",
        code: 1 << 5,
    };

    pub const ADD_REACTIONS: Permission = Permission {
        name: "Add reactions",
        code: 1 << 6,
    };

    pub const VIEW_AUDIT_LOG: Permission = Permission {
        name: "View audit logs",
        code: 1 << 7,
    };

    pub const PRIORITY_SPEAKER: Permission = Permission {
        name: "Speak with priority",
        code: 1 << 8,
    };

    pub const STREAM: Permission = Permission {
        name: "Share screen",
        code: 1 << 9,
    };

    pub const VIEW_CHANNEL: Permission = Permission {
        name: "View channel",
        code: 1 << 10,
    };

    pub const SEND_MESSAGES: Permission = Permission {
        name: "Send messages",
        code: 1 << 11,
    };

    pub const SEND_TTS_MESSAGES: Permission = Permission {
        name: "Send TTS messages",
        code: 1 << 12,
    };

    pub const MANAGE_MESSAGES: Permission = Permission {
        name: "Manage messages",
        code: 1 << 13,
    };

    pub const EMBED_LINKS: Permission = Permission {
        name: "Emed links (GIFs, HTML, etc.)",
        code: 1 << 14,
    };

    pub const ATTACH_FILES: Permission = Permission {
        name: "Attach files",
        code: 1 << 15,
    };

    pub const READ_MESSAGE_HISTORY: Permission = Permission {
        name: "Read messages history",
        code: 1 << 16,
    };

    pub const MENTION_EVERYONE: Permission = Permission {
        name: "Mention everyone",
        code: 1 << 17,
    };

    pub const USE_EXTERNAL_EMOJIS: Permission = Permission {
        name: "Use external emojis",
        code: 1 << 18,
    };

    pub const VIEW_GUILD_INSIGHTS: Permission = Permission {
        name: "View guild insights",
        code: 1 << 19,
    };

    pub const CONNECT: Permission = Permission {
        name: "Join voice channels",
        code: 1 << 20,
    };

    pub const SPEAK: Permission = Permission {
        name: "Speak in voice channel",
        code: 1 << 21,
    };

    pub const MUTE_MEMBERS: Permission = Permission {
        name: "Mute members",
        code: 1 << 22,
    };

    pub const DEAFEN_MEMBERS: Permission = Permission {
        name: "Deafen members",
        code: 1 << 23,
    };

    pub const MOVE_MEMBERS: Permission = Permission {
        name: "Move members (voice channels)",
        code: 1 << 24,
    };

    pub const USE_VAD: Permission = Permission {
        name: "Use Voice Activity Detection",
        code: 1 << 25,
    };

    pub const CHANGE_NICKNAME: Permission = Permission {
        name: "Change own nickname (server name)",
        code: 1 << 26,
    };

    pub const MANAGE_NICKNAMES: Permission = Permission {
        name: "Manage nicknames (server names)",
        code: 1 << 27,
    };

    pub const MANAGE_ROLES: Permission = Permission {
        name: "Manage roles",
        code: 1 << 28,
    };

    pub const MANAGE_WEBHOOKS: Permission = Permission {
        name: "Manage webhooks",
        code: 1 << 29,
    };

    pub const MANAGE_EMOJIS_AND_STICKERS: Permission = Permission {
        name: "Manage emojis and stickers",
        code: 1 << 30,
    };

    pub const USE_APPLICATION_COMMANDS: Permission = Permission {
        name: "Use application (bot) commands",
        code: 1 << 31,
    };

    pub const REQUEST_TO_SPEAK: Permission = Permission {
        name: "Request to speak (stage channels)",
        code: 1 << 32,
    };

    pub const MANAGE_EVENTS: Permission = Permission {
        name: "Manage events",
        code: 1 << 33,
    };

    pub const MANAGE_THREADS: Permission = Permission {
        name: "Manage threads",
        code: 1 << 34,
    };

    pub const CREATE_PUBLIC_THREADS: Permission = Permission {
        name: "Create public thread",
        code: 1 << 35,
    };

    pub const CREATE_PRIVATE_THREADS: Permission = Permission {
        name: "Create private thread",
        code: 1 << 36,
    };

    pub const USE_EXTERNAL_STICKERS: Permission = Permission {
        name: "Use external stickers",
        code: 1 << 37,
    };

    pub const SEND_MESSAGES_IN_THREADS: Permission = Permission {
        name: "Send messages in threads",
        code: 1 << 38,
    };

    pub const USE_EMBEDDED_ACTIVITIES: Permission = Permission {
        name: "Use embedded activities",
        code: 1 << 39,
    };

    pub const MODERATE_MEMBERS: Permission = Permission {
        name: "Moderate members",
        code: 1 << 40,
    };
}

pub struct PermissionsList {
    permissions: HashSet<Permission>,
}

impl PermissionsList {
    const ALL_PERMISSIONS: [Permission; 2] =
        [Permission::ADMINISTRATOR, Permission::MODERATE_MEMBERS];

    pub fn new(permissions: HashSet<Permission>) -> Self {
        Self { permissions }
    }

    pub fn empty() -> Self {
        Self {
            permissions: HashSet::new(),
        }
    }

    pub fn code(&self) -> String {
        let mut code: u64 = 0;

        for permission in self.permissions.iter() {
            code |= permission.code
        }

        format!("{}", code)
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        HashSet::from_iter(self.permissions.iter().copied())
    }
}

impl PartialEq for PermissionsList {
    fn eq(&self, other: &Self) -> bool {
        self.permissions.eq(&other.permissions)
    }
}

impl From<String> for PermissionsList {
    fn from(code: String) -> Self {
        let num_code: u64 = code.parse().unwrap();
        Self::from(num_code)
    }
}

impl From<u64> for PermissionsList {
    fn from(code: u64) -> Self {
        let mut permissions = HashSet::new();

        for permission in Self::ALL_PERMISSIONS {
            if (code & permission.code) == permission.code {
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

impl<const N: usize> From<[Permission; N]> for PermissionsList {
    fn from(permissions: [Permission; N]) -> Self {
        Self {
            permissions: HashSet::from(permissions),
        }
    }
}

#[cfg(test)]
mod tests {
    mod persission {
        use std::collections::HashSet;

        use crate::domain::permission::Permission;

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
    }

    mod permission_list {
        use std::collections::HashSet;

        use crate::domain::permission::{Permission, PermissionsList};

        #[test]
        fn when_empty_then_code_is_0() {
            let permission_list = PermissionsList::empty();
            assert_eq!(permission_list.code(), "0");
        }

        #[test]
        fn does_not_care_about_duplicates() {
            let permissions = [Permission::ADMINISTRATOR, Permission::ADMINISTRATOR];
            let permission_list = PermissionsList::from(permissions);

            let expected_code = format!("{}", Permission::ADMINISTRATOR.code);
            assert_eq!(permission_list.code(), expected_code);
        }

        #[test]
        fn can_create_code_from_permissions() {
            let permissions = [
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
            let permission_list = PermissionsList::from("278592".to_string());

            let permissions = permission_list.permissions();

            let expected_permissions = HashSet::from([
                Permission::ADD_REACTIONS,
                Permission::EMBED_LINKS,
                Permission::USE_EXTERNAL_EMOJIS,
            ]);
            let diff: Vec<_> = permissions.difference(&expected_permissions).collect();
            assert_eq!(diff.len(), 0);
        }
    }
}
