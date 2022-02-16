import dictdiffer

def assert_dictdiffer(
	result,
	expected_result,
	check_unexpected_added  =True,
	check_unexpected_changed=True,
	check_unexpected_removed=True,
	absolute_tolerance=0.001
	):

	comparison = list(
		dictdiffer.diff(
			expected_result,
			result,
			absolute_tolerance=absolute_tolerance
		)
	)

	if not (check_unexpected_added or check_unexpected_changed or check_unexpected_removed):
		raise Exception("No checks were specified. Please specify at least one check.")

	output = []
	for difftype, key, value in comparison:
		if difftype==dictdiffer.ADD:
			if check_unexpected_added:
				output.append(f"ADDED VALUE   : {key}")
				output.append(f"                Expected Nothing")
				output.append(f"                Found    {value}")
		elif difftype==dictdiffer.CHANGE:
			if check_unexpected_changed:
				output.append(f"CHANGED VALUE : {key}")
				output.append(f"                Expected {value[0]}")
				output.append(f"                Found    {value[1]}")
		elif difftype==dictdiffer.REMOVE:
			if check_unexpected_removed:
				output.append(f"REMOVED VALUE : {key}")
				output.append(f"                Expected {value}")
				output.append(f"                Found    Nothing")
		else:
			raise Exception("Testing failed. Did something about the `dictdiffer` library change?")

	if output:
		raise Exception("Unexpected result - dictdiff:\n" + "\n".join(output))
