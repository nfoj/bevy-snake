# Step-by-step!

- Start!

```
  // name: folder
  cargo new bevy-snake
  cd bevy-snake
    
  // test
  cargo run

  // install
  cargo add bevy

  // edit
  src/main.rs

  // access: https://bevyengine.org/learn/quick-start/getting-started/
     
```

- Dependencies

```
  // edit
  Cargo.toml

  // details
  [package]
  name = "bevy-snake"
  version = "0.1.0"
  edition = "2021"
  authors = ["Name <link//email>"]


  [dependencies]
  bevy = "0.14.2"
  rand = "0.8.5"

  // acess: https://crates.io/crates/rand

```

bevy = "0.14.2" : bevy game
rand = "0.8.5" : random number generators



- Use


- Const

```
  
  // interger : i8 - i64 | u8 - u64
  const AREA_WIDTH : u32 = 20; 
  const AREA_HEIGHT : u32 = 20;

  
  // Color::srgba(red: f32, green: f32, blue: f32, alpha: f32); 
  // Color (0-1) : 0. - 1. = (1., 0.2, 0.7)
  const SNAKE_HEAD_COLOR : Color = Color::srgb(0.9, 0.6, 0.5);
  
  
```


- #[derive(...)]

When you put #[derive(Component, Copy, Clone, PartialEq, Eq)] on a struct or enum, you're asking the compiler to automatically implement the following behaviors for that type:

Component: The type can be used as a component in some ECS system.
Copy: The type can be copied simply and without additional cost.
Clone: The type can be explicitly duplicated using the .clone() method.
PartialEq: The type can be compared for partial equality.
Eq: The type can be compared for total equality.
