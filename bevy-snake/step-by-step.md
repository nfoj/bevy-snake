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

- Test

```
  // install watch
  cargo install cargo-watch

  // run
  cargo watch -x run

  // path > modification
  cargo watch -w src -x run

  // ignore files
  cargo watch -w src -i target -x run

  // https://crates.io/crates/cargo-watch
  
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

  // https://bevyengine.org/ 
  // https://crates.io/crates/rand

```

bevy = "0.14.2" : bevy game
rand = "0.8.5" : random number generators


- Const 

```
  
  // interger : i8 - i64 | u8 - u64
  // The lower the number, the bigger the snake and the food
  const AREA_WIDTH : u32 = 120; // test 
  const AREA_HEIGHT : u32 = 120; // test

  // Color::srgba(red: f32, green: f32, blue: f32, alpha: f32);
  // Color::srgb(red: f32, green: f32, blue: f32);
  // Color (0-1) : 0 - 1 = (0.2, 0.7, 1.)
  const SNAKE_HEAD_COLOR : Color = Color::srgb(0.9, 0.6, 0.5);
  const SNAKE_SEGMENT_COLOR : Color = Color::srgb(0.3, 0.5, 0.);
  
```

alpha = transparency


- #[derive(...)]

When you put #[derive(Component, Copy, Clone, PartialEq, Eq)] on a struct or enum, you're asking the compiler to automatically implement the following behaviors for that type:

Component: The type can be used as a component in some ECS system.
Copy: The type can be copied simply and without additional cost.
Clone: The type can be explicitly duplicated using the .clone() method.
PartialEq: The type can be compared for partial equality.
Eq: The type can be compared for total equality.

- Example:

```
  // Debug > Derive
  #[derive(Debug)]
  struct Foo {
    a:i32,
  } 


  // Debug not Derive
  use std::fmt;

  struct Foo {
    a:i32,
  }

  impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Foo: {}", self.a)
    }
  }
  
```

- Position 

```
  #[derive(Component, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
  }

  // Compoment: tag types as entity components in the ECS system, representing the position of this entity in space.
  // Copy: allows position values to be copied rather than moved;
  // Clone: allows you to create an explicit copy of a value;
  // PartialEq: allows you to compare position values using the == (equality) operator; // Compare x and y
  // Eq: ensures that equality is total(that is, if a == b, that b == a).
  
```

- Size

```
  #[derive(Component)]
  struct Size {
      width: f32,
      height: f32,
  }
  
```

- Direction

```
  #[derive(Clone, Copy, Debug, PartialEq)]
  enum Direction {
      Left,
      Up,
      Right,
      Down,
  }

  // Debug: allows the direction type to be printed with the {:?} format, making debugging easier;
  // PartialEq: check if two directions are equal, as if they point to the same direction (e. g., check if both are Left or Up).

  impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
  
```

- SnakeHead

```
  #[derive(Component)]
  struct SnakeHead {
    direction: Direction,
  }

  // stores the direction of the snake's head in the game.
  
```

- SnakeSegment

```
  #[derive(Component)]
  struct SnakeSegment;

  // store the direction of the snake's body in the game.
  
```

- SnakeSegments

```
  #[derive(Resource, Default)]
  struct SnakeSegments(Vec<Entity>);

  // Resource: used to store data that is shared globally
  // Default: Allows initializing a SnakeSegments with an empty vector (Vec<Entity>), useful when you want to start with an empty list of snake segments.
  
```

- Food

```
  #[derive(Component)]
  struct Food;

  // represent pieces of food in the game.
  
```

- GrowthEvent

```
  #[derive(Event)]
  struct GrowthEvent;

  // used to signal that the snake has grown.
  
```

- LastTailPosition

```
  #[derive(Resource, Default)]
  struct LastTailPosition(Option<Position>);

  // Resource: stores the snake's tail's previous position. This is useful for knowing where the tail was before it grew;
  // Default: initialized with a default value, which in this case is None.
  
```

- GameOverEvent

```
  #[derive(Event)]
  struct GameOverEvent;

  // used to indicate that the game is over.
  
```
