use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;
use walkdir::WalkDir;

fn main() {
    struct GenSerde;

    impl CustomizeCallback for GenSerde {
        fn message(&self, _message: &MessageDescriptor) -> Customize {
            Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
        }

        fn field(&self, field: &FieldDescriptor) -> Customize {
            if field.proto().type_() == Type::TYPE_ENUM {
                // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
                Customize::default().before(
                    "#[serde(serialize_with = \"crate::I_LOVE_PROTOBUF::serialize_enum_or_unknown\", deserialize_with = \"crate::I_LOVE_PROTOBUF::deserialize_enum_or_unknown\")]")
            } else {
                Customize::default()
            }
        }

        fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
            Customize::default().before("#[serde(skip)]")
        }
    }


    let mut inputs = Vec::new();
    for entry in WalkDir::new("proto")
        .into_iter().filter_map(|e| e.ok()) {

        if entry.file_type().is_file() {
            if let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) {
                if filename.ends_with(".proto") {
                    inputs.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    Codegen::new()
        .cargo_out_dir("protos")
        .include("proto")
        .inputs(&inputs)
        .customize_callback(GenSerde)
        .run_from_script();
}
