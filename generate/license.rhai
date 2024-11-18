// This example script asks the users selection of license

let license = variable::prompt("Which license do you want to use?",
				"MPL-2.0",
                ["MPL-2.0",
				 "MIT",
				 "AGPL-3.0",
				 "Apache-2.0",
				 "Unlicense",
				 "(Other)",
				 "None"
				]
);

if license == "(Other)" {
	license = variable::prompt("Please define *a valid* identifier from <https://github.com/spdx/license-list-data/blob/v3.20/licenses.md>")
}

print(`📝 Remeber to add a correct LICENSE file for ${license} - found at <https://github.com/spdx/license-list-data/blob/v3.20/licenses.md> `);
variable::set("license", license);
