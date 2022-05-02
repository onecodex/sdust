# Symmetric DUST implementation

Finds low-complexity regions in DNA sequences.

Comes with an optional binary that can be used similarly to dustmasker, although it currently only supports fasta output,
both soft, equivalent to `--outfmt fasta` of dustmasker, or hard that replace all masked bases by Ns.


- Paper: https://pubmed.ncbi.nlm.nih.gov/16796549/
- Acknowledgments: https://github.com/lh3/minimap2/blob/master/sdust.c was used to verify input
and its logic has been reused
