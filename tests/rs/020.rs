// Test 020 : CfgBoostError::WildcardArmMissing corrected.
use nscfg::{ target_cfg, match_cfg };

target_cfg!{
    
    x86_64:ar => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    !x86_64:ar => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    
}

fn foo2() -> String {
    match_cfg!{
        x86_64:ar => {
            String::from("020") 
        },
        _ => String::from("020")
    }
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), "completed!");
}
