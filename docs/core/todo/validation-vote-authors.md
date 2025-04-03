When a vote is cast, the assert_set_doc function needs to to check:
- the data.author needs to be = the &context user's key or principal (can't enable this in playground mode)
- the key and target both exist.
- the weight of the vote is within the ranges expected by the tag (usually either -1 or 1. no other numbers or decimals. but some tags might want to expand, just no 0 votes)
