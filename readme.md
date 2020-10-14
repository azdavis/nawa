# nawa

A simple implementation of a generic [rope][1] data structure.

"Simple" means there is no effort made to balance the internal binary trees, so
performance will probably suffer accordingly for pathological workloads.

"Generic" means it contain any element type, not just characters as is
conventional for string ropes.

[1]: https://en.wikipedia.org/wiki/Rope_(data_structure)
