
struct MyBackend {}

impl Backend for MyBackend {
    fn name(&self) -> String {
        todo!()
    }

    fn find_all_certificates(&self) -> native_pkcs11_traits::Result<Vec<Box<dyn native_pkcs11_traits::Certificate>>> {
        todo!()
    }

    fn find_private_key(&self, query: native_pkcs11_traits::KeySearchOptions) -> native_pkcs11_traits::Result<Option<std::sync::Arc<dyn native_pkcs11_traits::PrivateKey>>> {
        todo!()
    }

    fn find_public_key(&self, query: native_pkcs11_traits::KeySearchOptions) -> native_pkcs11_traits::Result<Option<Box<dyn native_pkcs11_traits::PublicKey>>> {
        todo!()
    }

    fn find_all_private_keys(&self) -> native_pkcs11_traits::Result<Vec<std::sync::Arc<dyn native_pkcs11_traits::PrivateKey>>> {
        todo!()
    }

    fn find_all_public_keys(&self) -> native_pkcs11_traits::Result<Vec<std::sync::Arc<dyn native_pkcs11_traits::PublicKey>>> {
        todo!()
    }

    fn generate_key(
        &self,
        algorithm: native_pkcs11_traits::KeyAlgorithm,
        label: Option<&str>,
    ) -> native_pkcs11_traits::Result<std::sync::Arc<dyn native_pkcs11_traits::PrivateKey>> {
        todo!()
    }
}


use native_pkcs11::{CKR_OK, CK_FUNCTION_LIST_PTR_PTR, CK_RV, FUNC_LIST};
use native_pkcs11_traits::Backend;



#[no_mangle]
pub extern "C" fn C_GetFunctionList(ppFunctionList: CK_FUNCTION_LIST_PTR_PTR) -> CK_RV {
    native_pkcs11_traits::register_backend(Box::new(MyBackend {}));
    unsafe { *ppFunctionList = &mut FUNC_LIST };
    return CKR_OK;
}
