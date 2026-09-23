pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    include!("generated/magics.rs");
}

mod board;
mod move_gen;
