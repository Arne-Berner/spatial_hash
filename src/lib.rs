#![allow(dead_code)]
#![allow(unused)]
pub mod error;
pub mod vec2;
use crate::error::Error;
use crate::vec2::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Cell {
    col: i32,
    row: i32,
}
impl Cell {
    fn new(pos: &Vec2, spatial_hash: &SpatialHash) -> anyhow::Result<Self, Error> {
        // TODO turn this into a "CheckedBoundsVec"
        if *pos < spatial_hash.start {
            return Err(Error::OutOfBounds);
        }
        let rel_start = (pos - &spatial_hash.start) / (&spatial_hash.end - &spatial_hash.start);
        Ok(Self {
            col: (rel_start.x * spatial_hash.num_cells.cols as f32).floor() as i32,
            row: (rel_start.y * spatial_hash.num_cells.rows as f32).floor() as i32,
        })
    }
}

#[derive(Debug)]
pub struct Dimensions {
    cols: u32,
    rows: u32,
}

#[derive(Debug)]
pub struct Entity {
    pos: Vec2,
    start: Cell,
    end: Cell,
    size: Vec2,
    id: u32,
}
impl Entity {
    fn new(pos: Vec2, start: Cell, end: Cell, size: Vec2, id: u32) -> Self {
        Self {
            pos,
            start,
            end,
            size,
            id,
        }
    }
}

#[derive(Debug)]
pub struct SpatialHash {
    cells: HashMap<Cell, Vec<u32>>, // Cellindex + uuids
    start: Vec2,
    end: Vec2,
    num_cells: Dimensions,
    id: u32,
}

impl SpatialHash {
    pub fn new(cell_size: Vec2, start: Vec2, end: Vec2) -> anyhow::Result<Self, Error> {
        let cells = HashMap::new();
        // if start 0 and end 99 then this corrects it to 100 entries
        let end = end.add(1.0);
        let num_cells_rel = ((&end - &start) / &cell_size).ceil();
        let padded_end = &start + &num_cells_rel * &cell_size;

        if num_cells_rel.x <= 0.0 || num_cells_rel.y <= 0.0 {
            return Err(Error::NumCellsEqualZero);
        }
        let num_cells = Dimensions {
            cols: num_cells_rel.x as u32,
            rows: num_cells_rel.y as u32,
        };
        // dbg! {&start, &end, &num_cells_rel, &cell_size, &num_cells};
        let id = 0u32;
        Ok(Self {
            cells,
            start,
            end: padded_end,
            num_cells,
            id,
        })
    }

    pub fn create(self: &mut Self, pos: Vec2, size: Vec2) -> anyhow::Result<Entity, Error> {
        let (start_pos, end_pos) = Self::get_start_and_end(&pos, &size);
        let start_idx = Cell::new(&start_pos, self)?;
        let end_idx = Cell::new(&end_pos, self)?;
        let entity = Entity::new(pos, start_idx, end_idx, size, self.id);
        self.insert(&entity);
        self.id += 1;
        Ok(entity)
    }

    pub fn remove(self: &mut Self, start: &Cell, end: &Cell, id: u32) -> anyhow::Result<()> {
        for col in start.col..=end.col {
            for row in start.row..=end.row {
                let cell = Cell { col, row };
                if let Some(vec) = self.cells.get_mut(&cell) {
                    vec.retain(|&other_id| other_id != id);
                    if vec.is_empty() {
                        self.cells.remove(&cell);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update(self: &mut Self, mut entity: Entity) -> anyhow::Result<Entity, Error> {
        self.remove(&entity.start, &entity.end, entity.id);
        let (start_pos, end_pos) = Self::get_start_and_end(&entity.pos, &entity.size);
        entity.start = Cell::new(&start_pos, &self)?;
        entity.end = Cell::new(&end_pos, &self)?;
        self.insert(&entity);
        Ok(entity)
    }

    /// This doubles the size of the entity to search around it
    pub fn find_nearest(self: &Self, entity: &Entity) -> anyhow::Result<HashSet<u32>, Error> {
        let (start_pos, end_pos) = Self::get_start_and_end(&entity.pos, &(&entity.size * 2.0));
        let start_idx = Cell::new(&start_pos, &self)?;
        let end_idx = Cell::new(&end_pos, &self)?;
        let mut clients = HashSet::new();

        for col in start_idx.col..=end_idx.col {
            for row in start_idx.row..=end_idx.row {
                let cell = Cell { col, row };
                if let Some(vec) = self.cells.get(&cell) {
                    clients.extend(vec.iter().cloned());
                }
            }
        }
        Ok(clients)
    }

    fn get_start_and_end(pos: &Vec2, size: &Vec2) -> (Vec2, Vec2) {
        let is_zero_x = size.x.abs() <= f32::EPSILON;
        let is_zero_y = size.y.abs() <= f32::EPSILON;
        // TODO replace 0.0001 with something sensible.
        let start_half = pos - (size / 2.0) + 0.0001;
        let start_pos = Vec2::new(
            if is_zero_x { pos.x } else { start_half.x },
            if is_zero_y { pos.y } else { start_half.y },
        );

        let end_half = pos + (size / 2.0) - 0.0001;
        let end_pos = Vec2::new(
            if is_zero_x { pos.x } else { end_half.x },
            if is_zero_y { pos.y } else { end_half.y },
        );
        dbg! {&start_pos, &end_pos};
        (start_pos, end_pos)
    }

    fn insert(self: &mut Self, entity: &Entity) {
        for col in entity.start.col..=entity.end.col {
            for row in entity.start.row..=entity.end.row {
                let cell = Cell { col, row };
                self.cells.entry(cell).or_default().push(entity.id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    #[test]
    fn new_grid() {
        let cell_size = Vec2::new(10.0, 10.0);
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(99.0, 99.0);
        let res = SpatialHash::new(cell_size, start, end);
        assert!(res.is_ok());
        let grid = res.unwrap();
        assert_eq!(grid.num_cells.cols, 10);
        assert_eq!(grid.num_cells.rows, 10);
    }

    #[test]
    fn grid_where_cell_size_and_bounds_are_not_dividable() {
        let cell_size = Vec2::new(3.0, 3.0);
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(99.0, 99.0);
        let res = SpatialHash::new(cell_size, start, end);
        assert!(res.is_ok());
        let grid = res.unwrap();
        // 100 -> 102 / 3 = 34
        assert_eq!(grid.num_cells.cols, 34);
        assert_eq!(grid.num_cells.rows, 34);
    }

    #[test]
    fn new_grid_where_num_cells_zero() {
        let cell_size = Vec2::new(3.0, 3.0);
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(0.0, -1.0);
        let res = SpatialHash::new(cell_size, start, end);
        assert!(res.is_err());
    }

    #[test]
    fn create_entity() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let expected_cell = Cell::new(&pos, &grid)?;
        let size = Vec2::new(1.0, 1.0);

        // Act
        let res = grid.create(pos, size);

        // Assert
        assert!(res.is_ok());
        let entity = res.unwrap();
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 1, "expected 1 cell being occopied");
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_some());
        let id = res.unwrap()[0];
        assert_eq!(entity.id, id);
        Ok(())
    }

    #[test]
    fn remove_entity() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let expected_cell = Cell::new(&pos, &grid)?;
        let size = Vec2::new(1.0, 1.0);
        grid.create(pos, size);

        // Act
        grid.remove(&expected_cell, &expected_cell, 0);

        // Assert
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 0, "expected 0 cell being occopied");
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    fn update_entity() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let expected_none_cell = Cell::new(&pos, &grid)?;
        let size = Vec2::new(1.0, 1.0);
        let mut entity = grid.create(pos, size).unwrap();
        let new_pos = Vec2::new(41.5, 41.5);
        let expected_cell = Cell::new(&new_pos, &grid)?;
        entity.pos = new_pos;

        // Act
        let entity = grid.update(entity)?;

        // Assert
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 1, "expected 1 cell being occopied");
        let res = grid.cells.get(&expected_none_cell);
        assert!(res.is_none());
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_some());
        let id = res.unwrap()[0];
        assert_eq!(entity.id, id);
        Ok(())
    }

    // New tests for find_nearest
    #[test]
    fn find_nearest_single_entity() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let size = Vec2::new(1.0, 1.0);

        // Act
        let entity = grid.create(pos, size)?;
        let found = grid.find_nearest(&entity)?;

        // Assert: should find itself only
        assert_eq!(found.len(), 1);
        assert!(found.contains(&entity.id));
        Ok(())
    }

    #[test]
    fn find_nearest_includes_neighbor_when_large() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        // create a larger entity that spans multiple cells
        let pos_a = Vec2::new(42.5, 42.5);
        let size_a = Vec2::new(3.0, 3.0); // spans approx cols 41..=43
        let entity_a = grid.create(pos_a, size_a)?;

        // create a nearby entity inside that span
        let pos_b = Vec2::new(43.5, 42.5);
        let size_b = Vec2::new(1.0, 1.0);
        let entity_b = grid.create(pos_b, size_b)?;

        // Act
        let found = grid.find_nearest(&entity_a)?;

        // Assert: should find both ids (self and neighbor)
        assert!(found.contains(&entity_a.id), "should contain self id");
        assert!(found.contains(&entity_b.id), "should contain neighbor id");
        assert_eq!(found.len(), 2);
        Ok(())
    }

    #[test]
    fn find_nearest_does_not_include_far_entities() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos_a = Vec2::new(42.5, 42.5);
        let size_a = Vec2::new(1.0, 1.0);
        let entity_a = grid.create(pos_a, size_a)?;

        // create a far away entity
        let pos_far = Vec2::new(50.5, 50.5);
        let size_far = Vec2::new(1.0, 1.0);
        let entity_far = grid.create(pos_far, size_far)?;

        // Act
        let found = grid.find_nearest(&entity_a)?;

        // Assert: should only find the near entity (self)
        assert!(found.contains(&entity_a.id));
        assert!(!found.contains(&entity_far.id));
        assert_eq!(found.len(), 1);
        Ok(())
    }

    // TODO change Entity to invalid position
    // TODO update two Entities that are on the same spot at the beginning
    #[test]
    fn create_entity_no_size() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.0, 42.0);
        let expected_cell = Cell::new(&pos, &grid)?;
        dbg! {&expected_cell};
        let size = Vec2::new(0.0, 0.0);

        // Act
        let res = grid.create(pos, size);

        // Assert
        assert!(res.is_ok());
        let entity = res.unwrap();
        dbg! {&entity};
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 1, "expected 1 cell being occopied");
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_some());
        let id = res.unwrap()[0];
        assert_eq!(entity.id, id);
        Ok(())
    }

    #[test]
    fn create_two_entities() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let pos2 = Vec2::new(30.5, 30.5);
        let expected_cell = Cell::new(&pos, &grid)?;
        let expected_cell2 = Cell::new(&pos2, &grid)?;
        let size = Vec2::new(1.0, 1.0);

        // Act
        let res = grid.create(pos, size.clone());
        let res2 = grid.create(pos2, size);

        // Assert
        assert!(res.is_ok());
        assert!(res2.is_ok());
        let entity = res.unwrap();
        let entity2 = res2.unwrap();
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 2, "expected 2 cell being occopied");
        let res = grid.cells.get(&expected_cell);
        let res2 = grid.cells.get(&expected_cell2);
        assert!(res.is_some());
        assert!(res2.is_some());
        let id = res.unwrap()[0];
        let id2 = res2.unwrap()[0];
        assert_eq!(entity.id, id);
        assert_eq!(entity2.id, id2);
        Ok(())
    }

    #[test]
    fn create_two_overlapping_entities() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let pos2 = Vec2::new(42.5, 42.5);
        let expected_cell = Cell::new(&pos, &grid)?;
        let size = Vec2::new(1.0, 1.0);

        // Act
        let res = grid.create(pos, size.clone());
        let res2 = grid.create(pos2, size);

        // Assert
        assert!(res.is_ok());
        assert!(res2.is_ok());
        let entity = res.unwrap();
        let entity2 = res2.unwrap();
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        assert_eq!(occupied_cells, 2, "expected 2 cell being occopied");
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_some());
        let id = res.unwrap()[0];
        let id2 = res.unwrap()[1];
        assert_eq!(entity.id, id);
        assert_eq!(entity2.id, id2);
        Ok(())
    }

    #[test]
    fn create_bigger_entity() -> anyhow::Result<(), Error> {
        // Arrange
        let mut grid = create_grid();
        let pos = Vec2::new(42.5, 42.5);
        let expected_cell = Cell::new(&pos, &grid)?;
        let size = Vec2::new(3.0, 3.0);

        // Act
        let res = grid.create(pos, size);

        // Assert
        assert!(res.is_ok());
        let entity = res.unwrap();
        let occupied_cells: usize = grid.cells.values().map(|v| v.len()).sum();
        // checks if every
        assert!(grid
            .cells
            .values()
            .all(|v| v.iter().all(|&id| id == entity.id)));
        assert_eq!(occupied_cells, 9, "expected 3x3 cells being occopied");
        let res = grid.cells.get(&expected_cell);
        assert!(res.is_some());
        let id = res.unwrap()[0];
        assert_eq!(entity.id, id);
        // check if every identity is the same
        Ok(())
    }

    fn create_grid() -> SpatialHash {
        let cell_size = Vec2::new(1.0, 1.0);
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(99.0, 99.0);
        SpatialHash::new(cell_size, start, end).unwrap()
    }
}
