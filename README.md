# ngRSm /ˈɛngræzm̩/ [![Build Status](https://travis-ci.org/kmein/ngRSm.svg?branch=master)](https://travis-ci.org/kmein/ngRSm)
*N-gram analysis in Rust*

## Example (Homer's *Iliad*)
The following command finds out the 20 most frequent four-grams in the entire corpus of Homer's *Iliad*.

```bash
$ cargo run -- --size=4 --file=iliad.txt --count=20
    35 ὣς ἔφαθ᾽, οἳ δ᾽
    35 τὸν δ᾽ ἠμείβετ᾽ ἔπειτα
    25 ἔφαθ᾽, οἳ δ᾽ ἄρα
    24 τὸν δ᾽ ἀπαμειβόμενος προσέφη
    23 προσέφη πόδας ὠκὺς ἀχιλλεύς·
    22 τὸν δ᾽ αὖτε προσέειπε
    18 φωνήσας ἔπεα πτερόεντα προσηύδα·
    18 ὣς ἔφατ᾽, οὐδ᾽ ἀπίθησε
    17 οἳ δ᾽ ὅτε δὴ
    16 ἔπος τ᾽ ἔφατ᾽ ἔκ
    16 τ᾽ ἔφατ᾽ ἔκ τ᾽
    15 μιν φωνήσας ἔπεα πτερόεντα
    15 καί μιν φωνήσας ἔπεα
    15 οἳ δ᾽ ἄρα πάντες
    13 ἄρ᾽ ὑπόδρα ἰδὼν προσέφη
    13 δ᾽ ἄρ᾽ ὑπόδρα ἰδὼν
    13 τὸν δ᾽ ἄρ᾽ ὑπόδρα
    12 τὸν δ᾽ αὖτε προσέειπεν
    12 τὴν δ᾽ ἠμείβετ᾽ ἔπειτα
    12 ἀπαμειβόμενος προσέφη πόδας ὠκὺς
```
