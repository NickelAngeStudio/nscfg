// Test 046 : CfgBoostError::CfgBoostError::ModifierNotFirst corrected.
use nscfg::{ target_cfg, match_cfg, meta_cfg };


target_cfg! {

	linux => {},
	+ #[cfg(windows)] => {
		pub fn foo() -> String {
			String::from("Test")
		}
	}

}

pub fn foo2() -> String {
	match_cfg!{
		+ foo1:os => String::from("046"),
		foo2:os => String::from("000"),
		foo3:os => String::from("000"),
		_ => String::from("000"),
	}
}

#[meta_cfg(+ !doc & linux)]
pub fn foo3() -> String {
	String::from("completed")
}

#[meta_cfg(- !doc & linux)]
pub fn foo3() -> String {
	String::from("failed")
}

fn main() {
    println!("{} {} {}!", foo(), foo2(), foo3());
}
