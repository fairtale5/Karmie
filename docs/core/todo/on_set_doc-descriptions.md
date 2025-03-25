implement an on_set_doc functions that, whenever a new document is created, overwrites the descriptions with the correct fields

i wonder if, instead of setting those descriptions on input, i i should create a on_set_doc function that, whenever a new document is created, builds and applies the descriptions according to the DocumentDescription helper? it seems safer, but it would be a second write operation which could be worse performance wise? what do you think?
