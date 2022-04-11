# yavom
Yet Another Variation of Myers for generic containers (for example std::Vec)

This is an implementation of a quasi-Myers algorithm to determine the differences between two generic containers. Its usage is quite simple, for example:

```rust
let mut a: Vec<String> = vec!["A", "W", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
            .collect();
let b: Vec<String> = vec!["S", "T", "R", "A", "N", "G", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
            .collect();

// Create the diff (vector of moves)
let moves = crate::yavom::myers(&a, &b);
```
The return value *moves* is a vector of *Move* objects. You can apply moves to the array as follows:
```rust
moves.iter().for_each(|m| { crate::yavom::apply_move(m, &mut a); });
// now a's contents are the same as b's
```
You can serialize / deserialize *Move* objects as you deem necessary. To do so please consider the following definitions (found in *diff.h*):
```rust
pub enum OP {
    INSERT,
    DELETE,
    _DELETE,
}

pub struct Point(pub i64, pub i64);

pub struct Move<K>(pub OP, pub Point, pub Point, pub Option<Vec<K>>);

```
The *Vec<K>* field stores the values to be inserted. For example:
```rust
 let ops = myers_unfilled(&a, &b);
 let mut patch = vec![];
 for o in ops {
      let Move(op, s, t, _) = o;
      match op {
          yavomrs::yavom::OP::INSERT => {
              let from = s.1 as usize;
              let to = (s.1 + count) as usize;
              let insert_position = s.1;
              let values = &new[from..to];
              // TODO Serialize INSERT of values at insert_position
          }
          yavomrs::yavom::OP::DELETE => {
              let count = t.0 - s.0;
              let delete_position = s.1;
              // TODO Serialize DELETE of count values starting from delete_position
          }
          yavomrs::yavom::OP::_DELETE => {
              let Point(count, start) = s;
              // TODO Serialize DELETE of count values starting from start
              patch.push(json!([PATCH_DELETE, count, start]));
          }
      }
  }
  
```
If you are interested in knowning how many moves will be necessary but do not want to generate complete moves (with complete insert data), you can
use the *myers_unfilled* function:
```rust
let mut moves = crate::yavom::myers_unfilled(&a, &b);
eprintln!("{} moves", moves.len());
``` 
Subsequently you can fill the insertion data:
```rust
crate::yavom::myers_fill(&b, &mut moves);
```

## Credits & License
This code is Copyright (C) 2022 Amos Brocco (contact@amosbrocco.ch)

BSD 3-Clause License
