pub trait UUIDProvider {
    fn generate(&self, uuid: &mut str);
}

pub struct DefaultUUIDProviderImpl {}

// impl UUIDProvider for DefaultUUIDProviderImpl {
//     fn generate(&self, uuid: &mut str) {
//         uuid = ;
//     }
// }
