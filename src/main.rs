mod protobuf;

use crate::protobuf::{Foo, PhoneNumber, PhoneType};
use prost::Message;
use prost_msg_id::MsgId;

fn main() -> anyhow::Result<()> {
    println!("{:?}", protobuf::msg_ids());
    let a = Foo {
        name: "123123".to_string(),
        id: 10,
        phones: vec![PhoneNumber {
            number: "33321111".to_string(),
            r#type: PhoneType::Home.into(),
        }],
    };

    let data = get_buff(&a);
    let type_id = Foo::get_msg_id();
    match type_id {
        protobuf::FOO_ID => {
            let b = Foo::decode(&data[..])?;
            println!("{:?}", b.get_msg_id());
        }
        _ => {}
    }

    let foo= Foo::try_from(data)?;
    println!("{}",foo.name);
    Ok(())
}

fn get_buff<T: Message + MsgId>(a: &T) -> Vec<u8> {
    println!("{}", a.get_msg_id());
    a.encode_to_vec()
}


