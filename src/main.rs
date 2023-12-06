use tfhe::shortint::prelude::*;

fn query(key: ServerKey, mut target: Ciphertext, inventory: &[(u8, u8)]) -> Ciphertext
{
  // Process the first object in the array
    let (first_item_code, first_amount) = inventory.first().expect("Inventory is not empty");
    let mut result = key.smart_scalar_equal(&mut target, *first_item_code);
    let mut acc = key.smart_scalar_mul(&mut result, *first_amount);

    // Loop starting from the second object
    for (item_code, amount) in inventory.iter().skip(1) {
        result = key.smart_scalar_equal(&mut target, *item_code);
        let mut mul = key.smart_scalar_mul(&mut result, *amount);
        acc = key.smart_add(&mut mul, &mut acc);
    }

    acc
  
}

fn main() {
  // nothing to do here
}




#[cfg(test)]
mod tests {
  use tfhe::shortint::prelude::*;
  use tfhe::shortint::parameters::PARAM_MESSAGE_4_CARRY_0_KS_PBS;

  use crate::query;
  
  #[test]
  fn test_it() {
    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_4_CARRY_0_KS_PBS);

    let item_code = 5u8;

    let item_code_ciphertext = client_key.encrypt(item_code as u64);

    let stock_ciphertext = query(server_key, item_code_ciphertext, &[
      (1, 2),
      (2, 1),
      (2, 3)
    ]);

    let stock_count = client_key.decrypt(&stock_ciphertext);

    assert_eq!(stock_count, 0);
  }
}
