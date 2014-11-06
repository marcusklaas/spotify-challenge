spotify-challenge
=================
Solution to the spotify hard programming challenge as defined on https://labs.spotify.com/puzzles/ in Rust.

problem definition
------------------
The latest reality show has hit the TV: “Cat vs. Dog”. In this show, a bunch of cats and dogs compete for the very prestigious Best Pet Ever title. In each episode, the cats and dogs get to show themselves off, after which the viewers vote on which pets should stay and which should be forced to leave the show.

Each viewer gets to cast a vote on two things: one pet which should be kept on the show, and one pet which should be thrown out. Also, based on the universal fact that everyone is either a cat lover (i.e. a dog hater) or a dog lover (i.e. a cat hater), it has been decided that each vote must name exactly one cat and exactly one dog.

Ingenious as they are, the producers have decided to use an advancement procedure which guarantees that as many viewers as possible will continue watching the show: the pets that get to stay will be chosen so as to maximize the number of viewers who get both their opinions satisfied. Write a program to calculate this maximum number of viewers.

approach
--------
We model the problem using an undirected graph. Every voter is represented by a vertex. We add a vertex between two voters if their votes are incompatible. Since a cat lover's vote can only conflict with a dog lover's vote, this defines a bipartite graph. Note that the the maximum number of satisfied voters is equal to the maximum size of an independent set in this graph.

The complement of any independent set is a vertex cover and vice versa. It follows that the size of a maximum independent set is equal to the total number of vertices minus the size of a minimum vertex cover. [König's theorem](http://en.wikipedia.org/wiki/K%C3%B6nig%27s_theorem_%28graph_theory%29) states that in any bipartite graph, the size of a minimum vertex cover is equal to the number of edges in a maximum matching.

This gives us all we need to compute the solution. First, we transform our list of voters into a bipartite graph as described above. We then create a maximum matching and take its size. By our previous logic, the answer is equal to the number of voters minus the size of this matching.

Finding a maximum matching in a bipartite graph is straightforward. It can be done by starting with an empty matching and repeatedly computing augmenting paths for this matching. With each augmenting path, the matching grows by exactly one edge. By [Berge's lemma](http://en.wikipedia.org/wiki/Berge%27s_lemma), we are done when we can no longer find such a path.

running the program
-------------------
Easy as can be. Make sure you have installed [rustc and its build manager Cargo](http://www.rust-lang.org/install.html). Checkout this repository and execute `cargo build` to compile to `target/main`. The program takes its input from stdio, so its possible to read from the command line and file. There is some test data located in the `tests` directory. Try `target/main < tests/testcase1.txt` for example.

*Disclaimer*: the Rust language is not stable yet and prone to backward incompatible changes. The current code was writter for `rustc 0.13.0-nightly (adb44f53d 2014-10-12 00:07:15 +0000)`. It is likely that it will not compile in future version of Rust.
