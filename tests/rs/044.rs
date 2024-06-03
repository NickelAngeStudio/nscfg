// Test 044 : CfgBoostError::ContentSeparatorMissing corrected.
use nscfg::{ target_cfg };


target_cfg! {

	linux => {},
	#[cfg(unix)] => {
		pub fn foo() -> String {
			String::from("Test 044 completed!")
		}
	}

}



fn main() {
    println!("{}", foo());
}
