mod run_tt;
pub use run_tt::*;


pub trait MsgId{
    fn get_id(&self) -> i32;
}

macro_rules! msg_id {
    ($name:ident) => {
        impl MsgId for $name{
            paste::paste! {
                fn get_id(&self) -> i32 {
                    [<$name:lower>]::MsgId::Id as i32
                }
            }
        }

        impl $name{
            pub const fn get_msg_id()->i32{
                paste::paste! {
                    [<$name:lower>]::MsgId::Id as i32
                }
            }
        }
    };
}

macro_rules! const_id {
    ($name:ident) => {
       paste::paste! {
            pub const [<$name:upper _ID>]:i32 =$name::get_msg_id();
       }
    };
}



msg_id!(Foo);
const_id!(Foo);

