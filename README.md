## k-means

Data Mining Homework 2



Reads in a data file (see [input/input1.txt][inputlink]), and groups
the data objects (lines in the data file) into clusters based on similarity
between other data objects in the data set with respect to the k constraint
(desired number of clusters).

Uses the Euclidean distance approach
to calculate the dissimilarity of data objects & centroids, and chooses the
_initial centroid means_ from randomly chosen data objects out of the data set.

## build/run instructions

First make sure you have [rust][rustsite] installed, & clone this repo.

Then do a build (optional):

```
cargo build
```

Finally, run the code:

```
cargo run <filename> <k>
```

As an example, you can use the provided input files:

```
cargo run input/input1.txt 3
```

#### required params

_filename_:
 - the path to the file containing the data set.

_k_:
 - the desired number of clusters to establish.


## Design

There are 3 important modules in the project.
The **Worker**, **DataObject**, and **Cluster** modules.

The DataObject and Cluster modules are constructor modules
for their respective structs. The Worker module is for maintaining
an array (Vec) of Clusters and an array (Vec) of DataObjects. The
Worker module also is what implements the k-means algorithm.

The **util** module was created to abstract out the
argument parsing and file io handling.

The **main.rs** file is where the program first starts its control
flow and should be pretty readable as far as what it's doing.
There are some print methods throughout the program for debugging
purposes as well.  Feel free to try any you find if needed.

#### Important Locations:

  - dissimilarity method : `src/worker/data_object.rs` as `dist_cmp()`
  - k-means method : `src/worker/mod.rs` as `run()`

**src dir**

```bash
 src/
  ├── main.rs
  ├── util.rs
  └── worker/
      ├── cluster.rs
      ├── data_object.rs
      └── mod.rs
```


[rustsite]: https://www.rust-lang.org/
[inputlink]: https://github.com/William-Olson/k-means/blob/master/input/input1.txt