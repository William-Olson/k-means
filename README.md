## k-means

My 2nd homework for data mining class. Implements a simple k-means algorithm.

The language for implementation was left up to the student, 
so I decided to write my program in Rust. Mainly becuase...
  - Everybody else will probably use C/C++, Java or Python
  - I wanted to learn some rust
  - rust is awesome

The program reads in a data file (see input.txt), and groups the data objects
(lines in the data file) into clusters based on similarity between
other data objects in the data set with respect to the k constraint (cluster count).

The k-means algorithm uses the Euclidean distance approach 
to calculate the similarity/dissimilarity between data objects/centroids.

## build/run instructions

First make sure you have rust installed, & clone this repo.

Then do a build (optional):

```
cargo build
```

Finally, run the code:

```
cargo run <filename> <clusters>
```


#### required params

_filename_:
 - the path to the file containing the data set.

_clusters_:
 - the desired number of clusters to establish.

###### TODO:

need to implement:
  - Worker
    - (k < rows) error check
  - Util
    - write_to_file

should also:
  - add param & return rustdoc comment sections
  - do some unit tests?
