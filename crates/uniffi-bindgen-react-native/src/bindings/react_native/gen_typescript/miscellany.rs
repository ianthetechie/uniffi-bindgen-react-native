use super::oracle::CodeType;
use paste::paste;
use uniffi_bindgen::ComponentInterface;

macro_rules! impl_code_type_for_miscellany {
    ($T:ty, $class_name:literal, $canonical_name:literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self, _ci: &ComponentInterface) -> String {
                    $class_name.into()
                }

                fn canonical_name(&self) -> String {
                   $canonical_name.into()
               }
            }
        }
    };
}

impl_code_type_for_miscellany!(TimestampCodeType, "java.time.Instant", "Timestamp");

impl_code_type_for_miscellany!(DurationCodeType, "java.time.Duration", "Duration");
