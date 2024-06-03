// Test 042 : CfgBoostError::MixedSyntaxError corrected.
use nscfg::{ target_cfg };


target_cfg! {

	linux => {},
	#[cfg(unix)] => {
		pub fn foo() -> String {
			String::from("Test 042 completed!")
		}
	}

}



fn main() {
    println!("{}", foo());
}
