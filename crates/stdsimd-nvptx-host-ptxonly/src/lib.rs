pub static REF_PTX: &'static str = include_str!("../assets/reference.ptx");

#[cfg(test)]
mod tests {
include!(concat!(env!("OUT_DIR"), "/testglue.rs"));
}
