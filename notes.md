# Chapter 4

#### Why Ownership?

There are 2 other ways:
- Garbage Collection:
    - it is error free,
    - it is easier to write
    - buut, no control over memory,
    - slower and unpredictable
    - larger program sise as garbage collector is bundled in the binary
- Manual Memory Management:
    - control over memory
    - faster runtime
    - smaller program size
    - buuuut, very error prone

Garbage collection, is basically best of both worlds:
- It is error free
- It is faster
- It is smaller


#### Stack VS Heap
**stack**: Fixed size, cannot grow, but is faster.
**heap**: Dynamic in size, can grow, lifetime is controlled by us.

#### Ownership:

##### Rules:
1. Each value has an owner.
2. There can be only 1 owner at a time.
3. When the owner goes out of scope, the value is dropped.


#### References:

##### Rules:
1. You can have only 1 mutable reference at any time
2. You may have any number of immutable references
3. References must always be valid
