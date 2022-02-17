
# benchmarking between python and rust


this code was run for rust
```rs
fn main() {
    let mut _vector: Vec<i8> = vec![];
    let mut _counter: HashMap<i8, i32> = HashMap::new();
    let mut _hashinside: HashMap<i8, HashMap<&str, f32>> = HashMap::new()

    let _size = 100000000i32;
    for i in 1.._size {
        let secret_number =
            rand::thread_rng().gen_range(1..101);

        let modifier = _hashinside.entry(secret_number)
            .or_insert(HashMap::from([("frequency", 0f32), ("probability", 0f32)]));

        *modifier.get_mut(&"frequency").unwrap() += 1f32;

        *modifier.get_mut(&"probability").unwrap() =
            *modifier.get("frequency").unwrap() / _size as f32;

        *_counter.entry(secret_number).or_insert(0) += 1;

        _vector.push(secret_number);
    }

    for (k, v) in _hashinside.iter() {
        println!("{}-{:?}", k, v);
    }
    println!("\ndone");
}
```

this rust package will run in 20 seconds for that amount of iterations
`cargo run --release  22.63s user 0.22s system 102% cpu 22.258 total`
22 seconds with release (best of rust)

and this code for python, same as rust's but converted to python

```python
print("start")
_vector = []
_counter = {}
_hashinside = {}

from random import randint

total = 100_000_000
for i in range(total):
    secret_number = randint(1, 101)
    modifier = _hashinside.get(secret_number, None)
    if modifier:
        modifier = {
            "freq": modifier["freq"] + 1,
            "prob": (modifier["freq"] + 1) / total
        }
    else:
        modifier = {
            "freq": 0,
            "prob": 0.0
        }

    _hashinside[secret_number] = modifier

    _counter[secret_number] = _counter.get(secret_number, 0) + 1

    _vector.append(secret_number)


for k, v in _hashinside.items():
    print(k, v)
print("done")
```

pystson will run in:
`pyston test.py  73.58s user 0.40s system 99% cpu 1:14.20 total`
1 minute and 14 seconds

python will run in:
`python test.py  123.12s user 0.50s system 99% cpu 2:03.99 total`
2 minutes and 3 seconds

