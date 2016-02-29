## k-means

My 2nd homework for data mining class. Implements a simple k-means algorithm.

The language for implementation was left up to the student, 
so I decided to write my program in Rust. Mainly becuase...
  - Everybody else will probably use C/C++, Java or Python
  - I wanted to learn some rust
  - rust is awesome

## Description

The program reads in a data file (see input/input1.txt), and groups the data objects
(lines in the data file) into clusters based on similarity between
other data objects in the data set with respect to the k constraint (desired number of clusters).

The k-means algorithm uses the Euclidean distance approach 
to calculate the similarity/dissimilarity between data objects/centroids.

## build/run instructions

First make sure you have [rust][rustsite] installed, & clone this repo.

Then do a build (optional):

```
cargo build
```

Finally, run the code:

```
cargo run <filename> <clusters>
```

As an example, you can use the provided input files:

```
cargo run input/input1.txt 3
```

#### required params

_filename_:
 - the path to the file containing the data set.

_clusters_:
 - the desired number of clusters to establish.



[rustsite]: https://www.rust-lang.org/
