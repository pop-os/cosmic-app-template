// This example script asks and reacts to the the users selection of license

print("Please select *a valid* identifier from <https://github.com/spdx/license-list-data/blob/v3.20/licenses.md>");
let license = variable::prompt("Which license do you want to use?",
				"MPL-2.0",
                ["MPL-2.0",
					"MIT",
					"AGPL-3.0",
					"Apache-2.0",
					"Unlicense",
					"Custom",
					"None"
				]
);

if license == "Custom" {
        print("You selected to use a *custom* license");
        print("Please *manually* add a correct LICENSE file");
        print("Also update source files *manually* to reference your license choice");
}

variable::set("license", license);

if license != "None" {
	file::rename(`generate/licenses/${license}`, "LICENSE");
}

// Some licences require a copyright year
// Cargo generate's `authors` is used for owner
let _year = system::date().year;
let year = `${_year}`;
variable::set("year", year);

