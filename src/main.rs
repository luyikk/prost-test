
mod protobuf;

use prost::Message;
use crate::protobuf::{MsgId, PhoneNumber};



fn main()->anyhow::Result<()> {

    let a= protobuf::Foo {
        name: "123123".to_string(),
        id: 10,
        phones: vec![PhoneNumber{
            number: "33321".to_string(),
            r#type: 11
        }]
    };

    let data = a.encode_to_vec();
    println!("{:?}",data);

    let type_id=123;
    match type_id{
        protobuf::FOO_ID =>{
            let b= protobuf::Foo::decode(&data[..])?;
            println!("{:?}", protobuf::Foo::get_type_id());
            println!("{:?}", b.get_type_id());
        },
        _=>{

        }
    }

    Ok(())
}
