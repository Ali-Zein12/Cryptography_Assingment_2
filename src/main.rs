use tfhe::shortint::prelude::*;

fn query(key: ServerKey, mut target: Ciphertext, inventory: &[(u8, u8)]) -> Ciphertext {
  todo!()
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

    let item_code = 0u8;

    let item_code_ciphertext = client_key.encrypt(item_code as u64);

    let stock_ciphertext = query(server_key, item_code_ciphertext, &[
      (1, 2),
      (2, 1)
    ]);

    let stock_count = client_key.decrypt(&stock_ciphertext);

    assert_eq!(stock_count, 0);
  }
}
