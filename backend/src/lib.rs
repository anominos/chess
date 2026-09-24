pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    include!("generated/magics.rs");
}
pub mod board;
pub mod move_gen;
