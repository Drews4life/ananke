#[derive(Clone)]
pub struct Configuration {
    pub target_host: String,
    pub force_update_all: bool,
    pub pull: bool,
}
//
// impl Clone for Configuration {
//     fn clone(&self) -> Self {
//         Configuration {
//             target_host: self.target_host.clone(),
//             force_update_all: self.force_update_all,
//             pull: self.pull
//         }
//     }
// }