mod protobuf;

use crate::protobuf::PhoneNumber;
use prost::Message;
use prost_msg_id::MsgId;

fn main() -> anyhow::Result<()> {
    protobuf::msg_ids();
    let a = protobuf::Foo {
        name: "123123".to_string(),
        id: 10,
        phones: vec![PhoneNumber {
            number: "33321".to_string(),
            r#type: 11,
        }],
    };

    let data = a.encode_to_vec();
    let x= a.get_msg_id();
    // let type_id = 123;
    // match type_id {
    //     protobuf::FOO_ID => {
    //         let b = protobuf::Foo::decode(&data[..])?;
    //         println!("{:?}", protobuf::Foo::get_msg_id());
    //         println!("{:?}", b.get_type_id());
    //     }
    //     _ => {}
    // }

    Ok(())
}
