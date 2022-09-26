# differ

Runs a local server on `:3000` with an API like so:
```
GET http://localhost:3000/?a=dGVzdAoxCjIgMyA1CmhlbGxvCndvcmxk&b=dGVzdAoxCjIgMyA2CmhlbGxvCndvcmxk

test
1
>>> a (line: 3): -2 3 5-
>>> b (line: 3): +2 3 6+
hello
world
```

When the inputs are the same, it just returns `input is the same`
