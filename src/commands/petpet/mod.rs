mod generator;

use once_cell::sync::OnceCell;
use generator::Generator;

pub fn get_generator<'a>() -> anyhow::Result<&'a Generator> {
  static GENERATOR: OnceCell<Generator> = OnceCell::new();
  GENERATOR.get_or_try_init(|| Generator::new())
}
