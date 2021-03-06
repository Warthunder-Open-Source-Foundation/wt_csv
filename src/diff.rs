use crate::wtcsv::core::wtcsv::WTCSV;

#[derive(Debug)]
pub struct Diff {
	pub id: String,
	pub old: String,
	pub new: String,
}

#[allow(clippy::missing_errors_doc)]
impl Diff {
	pub fn from_wtcsv(left: &WTCSV, right: &WTCSV) -> Result<Vec<Self>, String> {
		left.is_compatible(right)?;

		let mut diffs: Vec<Self> =  Vec::new();

		for (i, record) in left.records.iter().enumerate() {
			for (j, item) in record.items.iter().enumerate() {
				if right.records[i].items[j] != *item {
					diffs.push(Diff {
						id: record.items[0].clone(),
						old: item.clone(),
						new: right.records[i].items[j].clone() ,
					});
					break;
				}
			}
		}

		Ok(diffs)
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use crate::diff::Diff;
	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	fn expect_diff() {
		let init = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let init = WTCSV::new_from_file(&init, "_common_languages").unwrap();

		let mut diff = init.clone();
		diff.edit_record_by_id("country_britain", "tea").unwrap();

		let diff_res = Diff::from_wtcsv(&init, &diff).unwrap();

		assert_eq!(r#"[Diff { id: "country_britain", old: "Great Britain", new: "tea" }]"#, format!("{:?}", diff_res));
	}
	#[test]
	fn expect_diff_invert() {
		let init = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let init = WTCSV::new_from_file(&init, "_common_languages").unwrap();

		let mut diff = init.clone();
		diff.edit_record_by_id("country_britain", "tea").unwrap();

		let diff_res = Diff::from_wtcsv(&diff, &init).unwrap();

		assert_eq!(r#"[Diff { id: "country_britain", old: "tea", new: "Great Britain" }]"#, format!("{:?}", diff_res));
	}
}