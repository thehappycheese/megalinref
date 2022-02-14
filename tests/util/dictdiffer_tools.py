import dictdiffer

def assert_dictdiffer(result_dict, expected_result, expected_result_is_subset=False, absolute_tolerance=0.001):

	comparison = list(
		dictdiffer.diff(
			expected_result,
			result_dict,
			absolute_tolerance=absolute_tolerance
		)
	)

	output = []
	for difftype, key, value in comparison:
		if difftype==dictdiffer.ADD:
			if not expected_result_is_subset:
				output.append(f"ADDED VALUE   : {key}")
				output.append(f"                Expected Nothing")
				output.append(f"                Found    {value}")
		elif difftype==dictdiffer.CHANGE:
			output.append(f"CHANGED VALUE : {key}")
			output.append(f"                Expected {value[0]}")
			output.append(f"                Found    {value[1]}")
		elif difftype==dictdiffer.REMOVE:
			output.append(f"REMOVED VALUE : {key}")
			output.append(f"                Expected {value}")
			output.append(f"                Found    Nothing")
		else:
			raise Exception("Testing failed. Did something about the `dictdiffer` library change?")

	if output:
		raise Exception("Unexpected result - dictdiff:\n" + "\n".join(output))
