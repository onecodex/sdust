# Symmetric DUST implementation

Finds low-complexity regions in DNA sequences.

Comes with an optional binary that can be used similarly to dustmasker, although it currently only supports fasta output,
both soft, equivalent to `--outfmt fasta` of dustmasker, or hard that replace all masked bases by Ns.

The output of the CLI matches the dustmasker output except some one-off base difference out of a several thousand masked bases. It does match the output of [minimap2](https://github.com/lh3/minimap2) however.


- Paper: https://pubmed.ncbi.nlm.nih.gov/16796549/
- Acknowledgments: https://github.com/lh3/minimap2/blob/master/sdust.c was used to verify input
and its logic has been reused
