# 소유권(Ownership)

소유권(ownership) 메모리 안정성 보장을 위한 러스트가 고안한 강력한 규칙이다.

## 원칙

- 모든값은 소유자가 있다.
- 소유자는 단 하나만 존재한다.
- 소유자가 범위를 벗어나면 Drop된다.

## 기본 동작 확인

```rust
let cart = String::from("my cart");
let other_cart = cart; // other의 소유권이 other_cart로 옮겨진다.

println!("{}", cart); // 이 카트는 비워졌으므로 꺼낼 물건이 없다. error!!
println!("{}", other_cart);
```

cart를 보고 똑같은 물건을 other_car에 별개로 담으면 된다. (서로 독립적이다.)

```rust
let cart = String::from("my cart");
let other_cart = cart.clone(); // 복제한다.

println!("{}", cart); 
println!("{}", other_cart);
```