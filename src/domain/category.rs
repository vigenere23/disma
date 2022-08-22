// pub struct AwaitingCategory {
//     name: String,
//     channels: Vec<AwaitingChannel>,
//     //permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
// }

#[derive(Debug)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    //permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}
