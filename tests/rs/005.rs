// Test 005 : CfgBoostError::InvalidCharacter.
use nscfg::{ meta_cfg };


#[meta_cfg(linux ^ windows ^ macos)]
fn foo() -> String {
    String::from("Test 005 completed!")
}


fn main() {
    println!("{}", foo());
}
