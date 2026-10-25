use crate::proc_gen::{
    geometry::{Edge, Point},
    room_generation::{RoomMap, map_rooms},
    triangulation_generation::triangulate,
    mst::minimum_spanning_tree,
};

pub struct DungeonLayout {
    pub map: RoomMap,
    pub points: Vec<Point>,
    pub delauney_edges: Vec<Edge>,
    pub mst_edges: Vec<Edge>,
    pub selected_edges: Vec<Edge>,
}

pub fn generate_dungeon(seed: u64) -> DungeonLayout {
    // Generate the rooms
    let map = map_rooms(seed);

    // Get the center all rooms
    let points: Vec<Point> = map.rooms.iter().map(|room| {
        let (cx, cy) = room.center();
        Point { x: cx, y: cy }
    }).collect();

    // Delaunay triangulation
    let delauney_edges = triangulate(&points);
    // MST from Delaunay edges
    let mst_edges = minimum_spanning_tree(&points, &delauney_edges);

    // Select edges for hallways
    let selected_edges = mst_edges.clone();

    DungeonLayout {
        map,
        points,
        delauney_edges,
        mst_edges,
        selected_edges,
    }
}