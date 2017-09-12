macro_rules! assert_mock_sequence {
		($trait_to_mock:path, [$($expected_call:ident($expected_param:expr)),*], $test_closure:expr) => {
			use mockers::{Scenario, Sequence};

			let scenario = Scenario::new();
			let mut seq  = Sequence::new();
			let mut mock = scenario.create_mock_for::<$trait_to_mock>();

			$(
				seq.expect(mock.$expected_call($expected_param).and_return(()));
			)*
			scenario.expect(seq);

			$test_closure(mock);
		}
	}

mod tests {
	#[derive(Mock)]
	trait TestTrait {
		fn func_1(&mut self, number: i32);
		fn func_2(&mut self, string: String);
	}

	fn calls_trait_functions<T: TestTrait>(test_trait: &mut T) {
		test_trait.func_1(42);
		test_trait.func_2("pony".to_string());
	}

	fn does_not_call_trait_functions<T: TestTrait>(test_trait: &mut T) {}

	#[test]
	fn assert_macro_succeeds_when_functions_called() {
		assert_mock_sequence!(
				TestTrait,
				[
					func_1_call(42),
					func_2_call("pony".to_string())
				],
				|mut mock| calls_trait_functions(&mut mock)
			);
	}

	#[test]
	#[should_panic]
	fn assert_macro_fails_when_functions_are_not_called() {
		assert_mock_sequence!(
				TestTrait,
				[
					func_1_call(42),
					func_2_call("pony".to_string())
				],
				|mut mock| does_not_call_trait_functions(&mut mock)
			);
	}
}
