// Test 002 : Correcting previous test. Now works.
use nscfg::{ meta_cfg, target_cfg, match_cfg};

target_cfg!{
	linux | windows => {
		fn foo1() -> String {
			String::from("This is ")
		}
	},
}


fn foo2() -> String {
    match_cfg!{
        linux | windows => {
            String::from("hello world ")
        },
        _ => {},
    }
}

#[meta_cfg(linux | windows)]
fn foo3() -> String {
    String::from("from nscfg!")
}


fn main() {
    println!("{}{}{}", foo1(), foo2(), foo3());
}
