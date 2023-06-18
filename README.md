# fake-computer-dices

Commandline virtual dices with an easy to type syntax and clean colored output. 

Supports several dice types:
- d4, d6, d8, d10, d12, d20: dice with that amount of faces
- Coin: 0 or 1

```
$ roll d20
   ╷Type  ╷Rolled value
  1│D20   │7
   ╵      ╵
::: Final result: 7
```

You can roll as many dices as you want in one command.
Both the value of each invidivual dice, and the total sum are shown.
```
$ roll d4 d4 d4 d10
   ╷Type  ╷Rolled value
  1│D4    │4
  2│D4    │3
  3│D4    │1
  4│D10   │2
   ╵      ╵
::: Final result: 10
```

Multiple dices of the same type can be abbreviated.

This command is equivalent to the previous example:
```
$ roll d4 x3 d10
```

You can also add a flat value to whatever the total sum is:
```
$ roll d20 +6
   ╷Type  ╷Rolled value
  1│D20   │5
   ╵      ╵
::: Final result: rolled 5, add 6 = 11
```

Multiple values are summed together:
```
$ roll d20 -4 +3
   ╷Type  ╷Rolled value
  1│D20   │8
   ╵      ╵
::: Final result: rolled 8, subtract 1 = 7
```