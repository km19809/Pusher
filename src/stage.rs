//! Stage for pusher
//!
//! It contains Stage structure of pusher.

use super::vector2::Vector2;
use std::convert::TryInto;
use std::fmt;
use std::ops;
/// A character that represents Wall
const WALL_CHR: char = '#';
/// A character that represents Emptiness
const EMPTY_CHR: char = '.';
/// A character that represents Ball
const BALL_CHR: char = 'O';
/// A character that represents Goal
const GOAL_CHR: char = '+';
/// A character that represents Ball on the Goal,
/// Only appears in Display trait
const BALL_ON_GOAL_CHR: char = '$';
/// A character that represents Player
const PLAYER_CHR: char = '@';

/// Describes type of Object: Empty, Ball, Player
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Empty,
    Ball,
    Player,
}

impl fmt::Display for ObjectType {
    #[cfg(not(feature = "color"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let charactor = match self {
            ObjectType::Empty => EMPTY_CHR,
            ObjectType::Ball => BALL_CHR,
            ObjectType::Player => PLAYER_CHR,
        };
        write!(f, "{}", charactor)
    }
    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let styled_charactor = match self {
            ObjectType::Empty => ("\x1b[1;30m", EMPTY_CHR),
            ObjectType::Ball => ("\x1b[0;37m", BALL_CHR),
            ObjectType::Player => ("\x1b[0;32m", PLAYER_CHR),
        };
        write!(f, "{}{}", styled_charactor.0, styled_charactor.1)
    }
}
/// Describes type of Tile: Wall, Plain, Goal\
/// Plain and Goal can hold an Object.
#[derive(Debug, Clone)]
pub enum Tile {
    Wall,
    Plain(ObjectType),
    Goal(ObjectType),
}

impl fmt::Display for Tile {
    #[cfg(not(feature = "color"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "{}", WALL_CHR),
            Tile::Plain(obj) => write!(f, "{}", obj),
            Tile::Goal(ObjectType::Empty) => write!(f, "{}", GOAL_CHR),
            Tile::Goal(ObjectType::Ball) => write!(f, "{}", BALL_ON_GOAL_CHR),
            Tile::Goal(obj) => write!(f, "{}", obj),
        }
    }
    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "\x1b[1;31m{}", WALL_CHR),
            Tile::Plain(obj) => write!(f, "{}", obj),
            Tile::Goal(ObjectType::Empty) => write!(f, "\x1b[0;33m{}", GOAL_CHR),
            Tile::Goal(ObjectType::Ball) => write!(f, "\x1b[0;33m{}", BALL_ON_GOAL_CHR),
            Tile::Goal(obj) => write!(f, "{}", obj),
        }
    }
}
/// Describes direction.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/// Describes game stage's state.
#[derive(Debug)]
pub struct Stage {
    ///Stage's width
    width: usize,
    ///Stage's height
    height: usize,
    ///Stores Tile data
    data: Vec<Tile>,
    ///Stores Player position
    player_position: Vector2,
    ///Total count of goals
    total_goals: u32,
    ///Count of balls on goals
    matched_goals: u32,
}

impl Stage {
    ///Constructor of stage\
    /// It reads text file as stage\
    /// `WALL_CHR` is a wall, `EMPTY_CHR` is an empty plain, `GOAL_CHR` is goal, `PLAYER_CHR` is player on a plain
    /// # Example
    /// ```
    /// use pusher::stage::ObjectType;
    /// use pusher::stage::Tile;
    /// use pusher::stage::Stage;
    /// use pusher::vector2::Vector2;
    /// let stage_data="#.@.O+#";
    /// let stage=Stage::new(stage_data).unwrap();
    /// assert_eq!(
    ///     format!("{:?}",stage),
    ///     format!("Stage {{ \
    /// width: 7, \
    ///  height: 1, \
    ///  data: [Wall, \
    ///  Plain(Empty), \
    ///  Plain(Player), \
    ///  Plain(Empty), \
    ///  Plain(Ball), \
    ///  Goal(Empty), \
    ///  Wall], \
    ///  player_position: Vector2 {{ x: 2, y: 0 }}, \
    ///  total_goals: 1, \
    ///  matched_goals: 0 \
    ///  }}")
    /// );
    /// assert_eq!(
    ///     format!("{}",stage),
    ///     "#.@.O+#\nMatched goal(s): 0/1\n"
    /// );
    /// ```
    /// # Errors
    /// It returns `Err(&'static str)` when:
    /// * Invalid charactors are detected.
    /// * Stage widths are inconsistent
    /// * Stage width or height is 0
    /// * Balls are not an many as goals.
    /// * The number of Player is less or more than 1
    pub fn new(string: &str) -> Result<Stage, &'static str> {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut balls = 0;
        let mut total_goals = 0;
        let mut player_position = Vector2::new(0, 0);
        let mut player_count = 0;
        for (y, line) in string.lines().enumerate() {
            let line_width = line.len();
            if y == 0 {
                width = line_width;
            } else if width != line_width {
                return Err("Invalid Stage: Widths are inconsistent.");
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    WALL_CHR => data.push(Tile::Wall),
                    EMPTY_CHR => data.push(Tile::Plain(ObjectType::Empty)),
                    BALL_CHR => {
                        balls += 1;
                        data.push(Tile::Plain(ObjectType::Ball))
                    }
                    GOAL_CHR => {
                        total_goals += 1;
                        data.push(Tile::Goal(ObjectType::Empty))
                    }
                    PLAYER_CHR => {
                        player_position.set(x as i32, y as i32);
                        player_count += 1;
                        data.push(Tile::Plain(ObjectType::Player))
                    }
                    '\n' | '\r' => (),
                    _ => return Err("Invalid Stage: invalid charactor detected."),
                }
            }
            height = y + 1;
        }
        if width <= 0 || height <= 0 {
            return Err("Invalid Stage: Width or height is smaller than 1.");
        }
        if balls != total_goals {
            return Err("Invalid Stage: Balls are not as many as goals.");
        }
        if player_count != 1 {
            return Err("Invalid Stage: Player has to be only one.");
        }
        Ok(Stage {
            width,
            height,
            data,
            player_position,
            total_goals,
            matched_goals: 0,
        })
    }
    /// Checks win state.\
    /// Return true when `self.matched_goals == self.total_goals`
    pub fn is_won(&self) -> bool {
        self.matched_goals == self.total_goals
    }
    /// Moves player and pushes a ball (if it exists).\
    /// it uses `self.move_object` to move.
    /// # Errors
    /// It returns `Err(&'static str)` when movement failed due to:
    /// * car`self.move_object` has failed.
    /// * Cannot push the blocking ball.
    /// * Blocked by a wall.
    /// * Blocked by boundary.
    /// # Panics
    /// Panics if `self.player_position` is out of index.
    pub fn move_player(&mut self, dir: Direction) -> Result<(), &'static str> {
        let delta_pos = match dir {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        };
        let pos = self.vector2_as_index(self.player_position).unwrap();
        let next_pos = self
            .vector2_as_index(self.player_position + delta_pos)
            .map_err(|_err| "[Player] Blocked by boundary.")?;
        match &self.data[next_pos] {
            Tile::Goal(ObjectType::Ball) | Tile::Plain(ObjectType::Ball) => {
                let beyond_next_pos = self
                    .vector2_as_index(self.player_position + delta_pos * 2)
                    .map_err(|_err| "[Player] Blocked by the Ball and boundary.")?;
                self.move_object(next_pos, beyond_next_pos)
                    .map_err(|_err| "[Player] Blocked by the Ball.")?;
                self.move_object(pos, next_pos)?;
                self.player_position += delta_pos;
                if let Tile::Goal(ObjectType::Ball) = self.data[beyond_next_pos] {
                    self.matched_goals += 1;
                }
                Ok(())
            }
            Tile::Goal(_) | Tile::Plain(_) => {
                self.move_object(pos, next_pos)?;
                self.player_position += delta_pos;
                Ok(())
            }
            Tile::Wall => Err("[Player] Blocked by the Wall."),
        }
    }
    /// Moves Object from a Tile to an Empty Tile.
    /// # Errors
    /// It returns `Err(&'static str)` when movement failed due to:
    /// * Trying to move to same place.
    /// * Trying to move a wall.
    /// * Blocked by a wall.
    /// * Destination tile is not empty.
    fn move_object(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        let (low, high) = match from.cmp(&to) {
            // no swapping necessary
            std::cmp::Ordering::Equal => return Err("[Object] Trying to move to same place."),
            // get the smallest and largest of the two indices
            std::cmp::Ordering::Less => (from, to),
            std::cmp::Ordering::Greater => (to, from),
        };
        let (head, tail) = self.data.split_at_mut(high);
        let (from_tile, to_tile) = match from.cmp(&to) {
            // no swapping necessary
            std::cmp::Ordering::Equal => panic!("How can it be?"),
            // get the smallest and largest of the two indices
            std::cmp::Ordering::Less => (&mut head[low], &mut tail[0]),
            std::cmp::Ordering::Greater => (&mut tail[0], &mut head[low]),
        };
        match to_tile {
            Tile::Goal(ref mut t_obj) | Tile::Plain(ref mut t_obj) => {
                if let ObjectType::Empty = t_obj {
                    match from_tile {
                        Tile::Goal(ref mut f_obj) | Tile::Plain(ref mut f_obj) => {
                            *t_obj = *f_obj;
                            *f_obj = ObjectType::Empty;
                            Ok(())
                        }
                        Tile::Wall => Err("[Object] Cannot move the Wall"),
                    }
                } else {
                    Err("[Object] Destination is not empty.")
                }
            }
            Tile::Wall => Err("[Object] Blocked by the Wall."),
        }
    }
    /// Converts vector2 to 1d index.
    /// # Errors
    /// It returns `Err(String)` when:
    /// * x is bigger or equal than width of stage, or y is bigger or equal than height of stage
    /// * either x or y is incompatible with usize.  
    fn vector2_as_index(&self, v: Vector2) -> Result<usize, String> {
        let x: usize = v
            .get_x()
            .try_into()
            .map_err(|_err| String::from("x cannot be converted to usize."))?;
        let y: usize = v
            .get_y()
            .try_into()
            .map_err(|_err| String::from("y cannot be converted to usize."))?;
        if x < self.width && y < self.height {
            return Ok(x + y * self.width);
        } else {
            Err(format!(
                "Index out of range! got Vector 2 {} while w:h={}:{}.",
                v, self.width, self.height
            ))
        }
    }
}

impl fmt::Display for Stage {
    #[cfg(not(feature = "color"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in (&self.data).chunks(self.width) {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\r\n")?;
        }
        writeln!(
            f,
            "Matched goal(s): {}/{}",
            self.matched_goals, self.total_goals
        )
    }
    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in (&self.data).chunks(self.width) {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\r\n")?;
        }
        writeln!(
            f,
            "\x1b[0mMatched goal(s): {}/{}",
            self.matched_goals, self.total_goals
        )
    }
}

impl ops::Index<Vector2> for Stage {
    type Output = Tile;

    fn index(&self, point: Vector2) -> &Self::Output {
        match self.vector2_as_index(point) {
            Ok(index) => &self.data[index],
            Err(msg) => panic!(msg),
        }
    }
}

impl ops::IndexMut<Vector2> for Stage {
    fn index_mut(&mut self, point: Vector2) -> &mut Tile {
        match self.vector2_as_index(point) {
            Ok(index) => &mut self.data[index],
            Err(msg) => panic!(msg),
        }
    }
}
