# Machi

```
aama
illa

machi
  sollu "Vanakam!"
sari

machi
  ipo 2 > 1: okaySollen
  ilana:  
sari

machi okaySollen
  sollu "Okay"
sari

machi
  n = 1
  pannu:
    ipo n > 3:
      podhum
    sollu "Hi"
    n += 1
    
```

Grammar

expression -> literal | binary
literal -> NUMBER | STRING
binary -> expression operator expression
operator -> "=" | "==" | ">=" | ">" | "<" | "<=" | "+" | "-" | "/"