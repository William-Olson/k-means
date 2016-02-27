## k-means

This repo is for my 2nd homework for data mining class.
The program reads in a data file, and sorts the data objects
(lines in the data file) by clusters based on similarity between
other data objects in the data set.

## build/run instructions

clone this repo:

```
git clone https://willko@bitbucket.org/willko/k-means.git
```

do a build (optional):

```
cargo build
```

run the code:

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
    - calc_means
    - run
  - Util
    - write_to_file

should also:
  - add param & return rustdoc comment sections
  - do some unit tests?
