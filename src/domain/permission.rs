#![allow(dead_code)]

use core::fmt;
use std::collections::HashSet;

#[non_exhaustive]
#[derive(Eq, PartialEq, Hash)]
pub struct Permission {
    pub name: &'static str,
    pub code: char,
}

impl Permission {
    pub const ADMINISTRATOR: Permission = Permission {
        name: "Administrator",
        code: '\x30',
    };

    pub const READ_MESSAGES: Permission = Permission {
        name: "Read messages",
        code: '\x04',
    };
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{} ({})", &self.name, self.code))
    }
}

pub struct PermissionsList {
    permissions: HashSet<Permission>,
}

impl PermissionsList {
    pub fn new(permissions: HashSet<Permission>) -> Self {
        Self { permissions }
    }

    pub fn code(&self) -> String {
        todo!("Calculate total code")
    }
}

impl From<String> for PermissionsList {
    fn from(_code: String) -> Self {
        todo!("Extract permissions based on global code")
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
    use std::collections::HashSet;

    use super::Permission;

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
        let permission2 = Permission::READ_MESSAGES;

        assert!(permission1 != permission2);
        assert!(!permission1.eq(&permission2));

        let set = HashSet::from([permission1, permission2]);
        assert_eq!(set.len(), 2);
    }
}
