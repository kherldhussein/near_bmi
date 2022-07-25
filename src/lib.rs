use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

pub type AccountId = String;

/*  Body Mass Index (BMI) is a value derieved from person's weight and height.
    The result of BMI measurement can give an idea about weather a person has correct weight and height.
*/

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AppUser {
  id: u32,
  uid: AccountId,
  u_name: Option<String>,
}

impl AppUser {
  pub fn new_user(id: u32, u_name: String) -> Self {
    AppUser {
      id,
      uid: env::signer_account_id().to_string(),
      u_name: Some(u_name),
    }
  }
}

#[derive(Clone, Deserialize, Serialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Data {
  uid: String,
  bmi: f32,
}

impl Data {
  pub fn new(uid: String, bmi: f32) -> Self {
    Self { uid, bmi }
  }
}

// Get user consent to set bio security measures the data
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DataPermission(Option<bool>);

impl DataPermission {
  pub fn new<T: Into<Option<bool>>>(data: T) -> Self {
    let data: Option<bool> = data.into();
    match data {
      Some(data) => Self(Some(data)),
      None => Self(None),
    }
  }
}

// Bio security measures defaults to true
impl Default for DataPermission {
  fn default() -> Self {
    Self(Some(true))
  }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Contract {
  uid: AccountId,
  app_user: HashMap<String, AppUser>,
  data: HashMap<String, Data>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(uid: AccountId) -> Self {
    let app_user: HashMap<String, AppUser> = HashMap::new();
    let data: HashMap<String, Data> = HashMap::new();
    Contract {
      uid,
      data,
      app_user,
    }
  }
  /*
      BMI calculation is based on a simple formula using a person's weight and height.
      The Formular for BMI= kg/m2 where kg is person's weight in kilograms and m2 is their height in meters squared.
      in simple fomart it would be BMI = (weight in kilograms)/(Heights in meters * Heights in meters)
  */

  pub fn compute(&mut self, weight: u32, height: f32, permit: &DataPermission) -> i32 {
    // let id = self.app_user.len() as u32;

    let u_name = env::signer_account_id().to_string();

    let height = height / 100.0;

    // For example if a person's weight is 92  and height is 136 then BMI=  92/(1.36^2) = 50
    let bmi = weight as f32 / height.powi(2);

    // For better readability we return 32-bit signed integer type when dealing with conversion.
    let n_bmi = ((bmi * 100f32).trunc() / 100.0) as i32;

    /*  BMI calculatar indicate wheather person falls under healthy weight, underweight or overweight.
        If a person's BMI is out of healthy range, their health risk may significantly increases.
        BMI range for adults BMI: weight status Below 18.5: Underweight 18.5 - 24.9, Normal or healthy weight 25.0 - 29.9, Overweight 30.0 & above: Obese
    */

    match bmi {
      bmi if bmi < 18.5 => log!("{} You are Underweight  ", u_name),
      bmi if (18.5..=24.9).contains(&bmi) => log!("{} You are Underweight  ", u_name),
      bmi if (25.0..=29.9).contains(&bmi) => log!("{} You are Underweight  ", u_name),
      _other => log!("{} You are Obese  ", u_name),
    }

    log!("BMI: {}", n_bmi);

    match permit.0 {
      Some(_data) => {
        if _data {
          match self.data.get(&u_name) {
            Some(_) => {
              env::log_str("We've got your data😍😍");
            }
            None => {
              env::log_str("Permission Accepted");

              self
                .data
                .insert(u_name, Data::new(env::signer_account_id().to_string(), bmi));

              env::log_str("BIOSECURITY MEASURES ARE IN EFFECT");
            }
          }
        } else {
          env::log_str("Kindly accept Permission to secure your Data");
        }
      }
      None => (),
    }

    n_bmi
  }

  pub fn set_user(&mut self, u_name: String) {
    let uid = self.app_user.len() as u32;
    let _app_user = env::signer_account_id().to_string();
    let current_user = self.app_user.get(&_app_user);
    match current_user {
      Some(_) => env::log_str("The provided uid is already in use by an existing user"),
      None => {
        self
          .app_user
          .insert(_app_user, AppUser::new_user(uid, u_name));
        env::log_str("Data set successfully");
      }
    }
  }

  // Get user data after saved
  pub fn get_data(&mut self, uid: String) -> Option<String> {
    let d = self.data.get(&uid);
    match d {
      Some(_data) => {
        let msg = format!("BMI Data: {} {}", _data.bmi, _data.uid);
        Some(msg)
      }
      None => {
        env::log_str("No Data Found");
        None
      }
    }
  }

  pub fn delete_data(&mut self, uid: String, permit: &DataPermission) {
    match permit.0 {
      Some(_data) => {
        if _data {
          self.data.remove(&uid);
          env::log_str("Your Data Is Delete");
        } else {
          env::log_str("Kindly accept Permission to delete your Data");
        }
      }
      None => (),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::testing_env;
  use near_sdk::AccountId;

  fn to_valid_account(account: &str) -> AccountId {
    AccountId::try_from(account.to_string()).expect("Invalid account")
  }

  fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.signer_account_id(predecessor);
    builder
  }

  #[test]
  fn set_user_test() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut _data = Contract::new(kherld.to_string());
    _data.set_user("Eternity Pro ".to_owned());
    let data = _data.app_user.len();
    assert_eq!(data, 1, "Should be one user");
  }

  #[test]
  fn compute_data_test() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut _data = Contract::new(kherld.to_string());
    let permit = DataPermission::default();
    let compute = _data.compute(45, 125.0, &permit);
    println!("The following information is 💖 to your health");
    assert_eq!(
      28, compute,
      "Should be match the expected result from computation",
    );
  }

  #[test]
  fn get_data_test() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut _data = Contract::new(kherld.to_string());
    let test_get = _data.get_data(kherld.to_string());
    assert!(test_get.is_none());
  }

  #[test]
  fn delete_data_test() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut _data = Contract::new(kherld.to_string());
    let permit = DataPermission::default();
    let delete_test = _data.delete_data(kherld.to_string(), &permit);
    assert_eq!((), delete_test);
  }
}
