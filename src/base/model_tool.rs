//macro_rules! extract_struct_fields {
//    (struct $name:ident{$($field_name:ident: $field_type:ty,)*}) => {
//    struct $name {
//            $($field_name: $field_type,)*
//        }
//       impl $name {
//            fn get_field_names() -> Vec<&'static str> {
//                vec![$(stringify!($field_name)),*]
//            }
//        }
//    };
//}
//
//extract_struct_fields! {
//    struct S {
//        a: String,
//        b: String,
//    }
//}
//
//// S::get_field_names() == vec!["a", "b"]