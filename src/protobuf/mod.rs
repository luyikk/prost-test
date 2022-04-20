mod run_tt;
pub use run_tt::*;

// macro_rules! msg_id {
//     ($name:ident) => {
//         impl MsgId for $name {
//             fn get_type_id(&self) -> i32 {
//                 Self::get_msg_id()
//             }
//         }
//     };
// }
//
// macro_rules! const_id {
//     ($name:ident) => {
//         paste::paste! {
//              pub const [<$name:upper _ID>]:i32 =$name::get_msg_id();
//         }
//     };
// }
//
// msg_id!(Foo);
// const_id!(Foo);
