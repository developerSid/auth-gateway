use auth_gateway_sdk::add;

#[test]
fn add_returns_sum_of_inputs() {
   assert_eq!(add(2, 2), 4);
}
