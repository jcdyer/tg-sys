#![allow(nonstandard_style)]

/// The base point type used for all geometries.
/// @see PointFuncs
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tg_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}

/// The base segment type used in tg_line and tg_ring for joining two vertices.
/// @see SegmentFuncs
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tg_segment {
    pub a: tg_point,
    pub b: tg_point,
}

/// A rectangle defined by a minimum and maximum coordinates.
/// Returned by the tg_geom_rect(), tg_ring_rect(), and other \*_rect()
/// functions for getting a geometry's minumum bounding rectangle.
/// Also used internally for geometry indexing.
/// @see RectFuncs
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tg_rect {
    pub min: tg_point,
    pub max: tg_point,
}

/// Geometry types.
///
/// All tg_geom are one of the following underlying types.
///
/// @see tg_geom_typeof()
/// @see tg_geom_type_string()
/// @see GeometryAccessors
#[repr(usize)]
pub enum tg_geom_type {
    ///< Point
    TG_POINT = 1,
    ///< LineString
    TG_LINESTRING = 2,
    ///< Polygon
    TG_POLYGON = 3,
    ///< MultiPoint, collection of points
    TG_MULTIPOINT = 4,
    ///< MultiLineString, collection of linestrings
    TG_MULTILINESTRING = 5,
    ///< MultiPolygon, collection of polygons
    TG_MULTIPOLYGON = 6,
    ///< GeometryCollection, collection of geometries
    TG_GEOMETRYCOLLECTION = 7,
}

/// Geometry indexing options.
///
/// Used for polygons, rings, and lines to make the point-in-polygon and
/// geometry intersection operations fast.
///
/// An index can also be used for efficiently traversing, searching, and
/// performing nearest-neighbor (kNN) queries on the segment using
/// tg_ring_index_*() and tg_ring_nearest() functions.
#[repr(C)]
pub enum tg_index {
    ///< default is TG_NATURAL or tg_env_set_default_index().
    TG_DEFAULT,
    ///< no indexing available, or disabled.
    TG_NONE,
    ///< indexing with natural ring order, for rings/lines
    TG_NATURAL,
    ///< indexing using segment striping, rings only
    TG_YSTRIPES,
}

///< Find the description in the tg.c file.
pub enum tg_line {}
///< Find the description in the tg.c file.
pub enum tg_ring {}
///< Find the description in the tg.c file.
pub enum tg_poly {}
///< Find the description in the tg.c file.
pub enum tg_geom {}

/// Functions for creating and freeing geometries.
pub mod GeometryConstructors {
    use crate::tg_geom;
    use crate::{tg_line, tg_point, tg_poly};

    #[link(name = "tg")]
    extern "C" {
        pub fn tg_geom_new_point(point: tg_point) -> *mut tg_geom;
        pub fn tg_geom_new_linestring(line: *const tg_line) -> *mut tg_geom;
        pub fn tg_geom_new_polygon(poly: *const tg_poly) -> *mut tg_geom;
        pub fn tg_geom_new_multipoint(
            points: *const tg_point,
            npoints: libc::c_int,
        ) -> *mut tg_geom;
        // ...
        pub fn tg_geom_clone(geom: *const tg_geom) -> *mut tg_geom;
        pub fn tg_geom_copy(geom: *const tg_geom) -> *mut tg_geom;
        pub fn tg_geom_free(geom: *mut tg_geom);

    }

    /*
    struct tg_geom *tg_geom_new_point(struct tg_point point);
    struct tg_geom *tg_geom_new_linestring(const struct tg_line *line);
    struct tg_geom *tg_geom_new_polygon(const struct tg_poly *poly);
    struct tg_geom *tg_geom_new_multipoint(const struct tg_point *points, int npoints);
    struct tg_geom *tg_geom_new_multilinestring(const struct tg_line *const lines[], int nlines);
    struct tg_geom *tg_geom_new_multipolygon(const struct tg_poly *const polys[], int npolys);
    struct tg_geom *tg_geom_new_geometrycollection(const struct tg_geom *const geoms[], int ngeoms);
    struct tg_geom *tg_geom_clone(const struct tg_geom *geom);
    struct tg_geom *tg_geom_copy(const struct tg_geom *geom);
    void tg_geom_free(struct tg_geom *geom);
    */
}

/// Geometry accessors
///
/// Functions for accessing various information about geometries, such as
/// getting the geometry type or extracting underlying components or
/// coordinates.
pub mod GeometryAccessors {
    use crate::{tg_geom_type, tg_geom, tg_rect, tg_point, tg_line, tg_poly};

    extern "C" {
        pub fn tg_geom_typeof(geom: *const tg_geom) -> tg_geom_type;
        pub fn tg_geom_type_string(r#type: tg_geom_type) -> *const libc::c_char;
        pub fn tg_geom_rect(geom: *const tg_geom) -> tg_rect;
        pub fn tg_geom_is_feature(geom: *const tg_geom) -> bool;
        pub fn tg_geom_is_featurecollection(geom: *const tg_geom) -> bool;
        pub fn tg_geom_point(geom: *const tg_geom) -> tg_point;
        pub fn tg_geom_line(geom: *const tg_geom) -> *const tg_line;
        pub fn tg_geom_poly(geom: *const tg_geom) -> *const tg_poly;
        pub fn tg_geom_num_points(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_point_at(geom: *const tg_geom, index: libc::c_int) -> tg_point;
        pub fn tg_geom_num_lines(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_line_at(geom: *const tg_geom, index: libc::c_int)-> *const tg_line ;
        pub fn tg_geom_num_polys(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_poly_at(geom: *const tg_geom, index: libc::c_int)-> *const tg_poly ;
        pub fn tg_geom_num_geometries(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_geometry_at(geom: *const tg_geom, index: libc::c_int)-> *const tg_geom ;
        pub fn tg_geom_extra_json(geom: *const tg_geom) -> *const libc::c_char;
        pub fn tg_geom_is_empty(geom: *const tg_geom) -> bool;
        pub fn tg_geom_dims(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_has_z(geom: *const tg_geom) -> bool;
        pub fn tg_geom_has_m(geom: *const tg_geom) -> bool;
        pub fn tg_geom_z(geom: *const tg_geom) -> libc::c_double;
        pub fn tg_geom_m(geom: *const tg_geom) -> libc::c_double;
        pub fn tg_geom_extra_coords(geom: *const tg_geom) -> *const libc::c_double;
        pub fn tg_geom_num_extra_coords(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_memsize(geom: *const tg_geom) -> libc::size_t;
        pub fn tg_geom_search(
            geom: *const tg_geom,
            rect: tg_rect,
            iter: extern "C" fn(geom: *const tg_geom, index: libc::c_int, udata: *mut libc::c_void) -> bool,
            udata: *mut libc::c_void,
        );
    // pub fn tg_geom_search
    }

    /*
    void tg_geom_search(const struct tg_geom *geom, struct tg_rect rect,
        bool (*iter)(const struct tg_geom *geom, int index, void *udata),
        void *udata);
    */
}
/// GeometryPredicates
///
/// Functions for testing the spatial relations of two geometries.
pub mod GeometryPredicates {

    use crate::tg_geom;
    use crate::tg_rect;

    #[link(name = "tg")]
    extern "C" {
        pub fn tg_geom_equals(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_intersects(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_disjoint(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_contains(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_within(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_covers(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_coveredby(a: *const tg_geom, b: *const tg_geom) -> bool;
        pub fn tg_geom_intersects_rect(a: *const tg_geom, b: tg_rect) -> bool;
        pub fn tg_geom_intersects_xy(
            a: *const tg_geom,
            x: libc::c_double,
            y: libc::c_double,
        ) -> bool;
    }
    /*
    bool tg_geom_equals(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_intersects(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_disjoint(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_contains(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_within(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_covers(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_coveredby(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_touches(const struct tg_geom *a, const struct tg_geom *b);
    bool tg_geom_intersects_rect(const struct tg_geom *a, struct tg_rect b);
    bool tg_geom_intersects_xy(const struct tg_geom *a, double x, double y);
    */
}

/// Geometry Parsing
///
/// Functions for parsing geometries from external data representations.
/// It's recommended to use tg_geom_error() after parsing to check for errors.
pub mod GeometryParsing {
    use crate::{tg_geom, tg_index};

    #[link(name = "tg")]
    extern "C" {
        pub fn tg_parse_geojson(geojson: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_geojsonn(geojson: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_geojson_ix(geojson: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_geojsonn_ix(geojson: *const libc::c_char, len: libc::size_t, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_wkt(wkt: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_wktn(wkt: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_wkt_ix(wkt: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_wktn_ix(wkt: *const libc::c_char, len: libc::size_t, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_wkb(wkb: *const u8, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_wkb_ix(wkb: *const u8, len: libc::size_t, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_hex(hex: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_hexn(hex: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_hex_ix(hex: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_hexn_ix(hex: *const libc::c_char, len: libc::size_t, ix: tg_index) -> *mut tg_geom;
        pub fn tg_geom_error(geom: *const tg_geom) -> *const libc::c_char;
    }
}

/// Point Functions
///
/// Functions for working directly with the tg_point type.
pub mod PointFuncs {
    use crate::{tg_point, tg_rect};
    #[link(name = "tg")]
    extern "C" {
        pub fn tg_point_rect(point: tg_point) -> tg_rect;
        pub fn tg_point_intersects_rect(point: tg_point, rect: tg_rect) -> bool;
    }
}

/// Segment Functions
///
/// Functions for working directly with the tg_segment type.
pub mod SegmentFuncs {
    use crate::{tg_rect, tg_segment};
    #[link(name = "tg")]
    extern "C" {
        pub fn tg_segment_rect(s: tg_segment) -> tg_rect;
        pub fn tg_segment_intersects_segment(a: tg_segment, b: tg_segment);
    }
}

////////////////////////////
// Code from tg.h follows //
////////////////////////////

/*

// https://github.com/tidwall/tg
//
// Copyright 2023 Joshua J Baker. All rights reserved.
// Use of this source code is governed by a license
// that can be found in the LICENSE file.

#ifndef TG_H
#define TG_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>




/// @defgroup GeometryConstructors Geometry constructors
/// @}

/// @defgroup GeometryAccessors Geometry accessors
/// @defgroup GeometryAccessors Geometry accessors
/// Functions for accessing various information about geometries, such as
/// getting the geometry type or extracting underlying components or
/// coordinates.
/// @{
enum tg_geom_type tg_geom_typeof(const struct tg_geom *geom);
const char *tg_geom_type_string(enum tg_geom_type type);
struct tg_rect tg_geom_rect(const struct tg_geom *geom);
bool tg_geom_is_feature(const struct tg_geom *geom);
bool tg_geom_is_featurecollection(const struct tg_geom *geom);
struct tg_point tg_geom_point(const struct tg_geom *geom);
const struct tg_line *tg_geom_line(const struct tg_geom *geom);
const struct tg_poly *tg_geom_poly(const struct tg_geom *geom);
int tg_geom_num_points(const struct tg_geom *geom);
struct tg_point tg_geom_point_at(const struct tg_geom *geom, int index);
int tg_geom_num_lines(const struct tg_geom *geom);
const struct tg_line *tg_geom_line_at(const struct tg_geom *geom, int index);
int tg_geom_num_polys(const struct tg_geom *geom);
const struct tg_poly *tg_geom_poly_at(const struct tg_geom *geom, int index);
int tg_geom_num_geometries(const struct tg_geom *geom);
const struct tg_geom *tg_geom_geometry_at(const struct tg_geom *geom, int index);
const char *tg_geom_extra_json(const struct tg_geom *geom);
bool tg_geom_is_empty(const struct tg_geom *geom);
int tg_geom_dims(const struct tg_geom *geom);
bool tg_geom_has_z(const struct tg_geom *geom);
bool tg_geom_has_m(const struct tg_geom *geom);
double tg_geom_z(const struct tg_geom *geom);
double tg_geom_m(const struct tg_geom *geom);
const double *tg_geom_extra_coords(const struct tg_geom *geom);
int tg_geom_num_extra_coords(const struct tg_geom *geom);
size_t tg_geom_memsize(const struct tg_geom *geom);
void tg_geom_search(const struct tg_geom *geom, struct tg_rect rect,
    bool (*iter)(const struct tg_geom *geom, int index, void *udata),
    void *udata);
/// @}

/// @defgroup GeometryPredicates Geometry predicates
/// @}

/// @defgroup GeometryParsing Geometry parsing


/// @defgroup GeometryWriting Geometry writing
/// Functions for writing geometries as external data representations.
/// @{
size_t tg_geom_geojson(const struct tg_geom *geom, char *dst, size_t n);
size_t tg_geom_wkt(const struct tg_geom *geom, char *dst, size_t n);
size_t tg_geom_wkb(const struct tg_geom *geom, uint8_t *dst, size_t n);
size_t tg_geom_hex(const struct tg_geom *geom, char *dst, size_t n);
/// @}

/// @defgroup GeometryConstructorsEx Geometry with alternative dimensions
/// Functions for working with geometries that have more than two dimensions or
/// are empty. The extra dimensional coordinates contained within these
/// geometries are only carried along and serve no other purpose than to be
/// available for when it's desired to export to an output representation such
/// as GeoJSON, WKT, or WKB.
/// @{
struct tg_geom *tg_geom_new_point_z(struct tg_point point, double z);
struct tg_geom *tg_geom_new_point_m(struct tg_point point, double m);
struct tg_geom *tg_geom_new_point_zm(struct tg_point point, double z, double m);
struct tg_geom *tg_geom_new_point_empty(void);
struct tg_geom *tg_geom_new_linestring_z(const struct tg_line *line, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_linestring_m(const struct tg_line *line, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_linestring_zm(const struct tg_line *line, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_linestring_empty(void);
struct tg_geom *tg_geom_new_polygon_z(const struct tg_poly *poly, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_polygon_m(const struct tg_poly *poly, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_polygon_zm(const struct tg_poly *poly, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_polygon_empty(void);
struct tg_geom *tg_geom_new_multipoint_z(const struct tg_point *points, int npoints, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipoint_m(const struct tg_point *points, int npoints, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipoint_zm(const struct tg_point *points, int npoints, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipoint_empty(void);
struct tg_geom *tg_geom_new_multilinestring_z(const struct tg_line *const lines[], int nlines, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multilinestring_m(const struct tg_line *const lines[], int nlines, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multilinestring_zm(const struct tg_line *const lines[], int nlines, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multilinestring_empty(void);
struct tg_geom *tg_geom_new_multipolygon_z(const struct tg_poly *const polys[], int npolys, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipolygon_m(const struct tg_poly *const polys[], int npolys, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipolygon_zm(const struct tg_poly *const polys[], int npolys, const double *extra_coords, int ncoords);
struct tg_geom *tg_geom_new_multipolygon_empty(void);
struct tg_geom *tg_geom_new_geometrycollection_empty(void);
/// @}


/// @defgroup PointFuncs Segment functions

/// @defgroup SegmentFuncs Segment functions

/// @defgroup RectFuncs Rectangle functions
/// Functions for working directly with the tg_rect type.
/// @{
struct tg_rect tg_rect_expand(struct tg_rect rect, struct tg_rect other);
struct tg_rect tg_rect_expand_point(struct tg_rect rect, struct tg_point point);
struct tg_point tg_rect_center(struct tg_rect rect);
bool tg_rect_intersects_rect(struct tg_rect a, struct tg_rect b);
bool tg_rect_intersects_point(struct tg_rect a, struct tg_point b);
/// @}

/// @defgroup RingFuncs Ring functions
/// Functions for working directly with the tg_ring type.
///
/// There are no direct spatial predicates for tg_ring.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the ring to a tg_geom, like such:
///
/// ```
/// tg_geom_interects((struct tg_geom*)ring, geom);
/// ```
/// @{
struct tg_ring *tg_ring_new(const struct tg_point *points, int npoints);
struct tg_ring *tg_ring_new_ix(const struct tg_point *points, int npoints, enum tg_index ix);
void tg_ring_free(struct tg_ring *ring);
struct tg_ring *tg_ring_clone(const struct tg_ring *ring);
struct tg_ring *tg_ring_copy(const struct tg_ring *ring);
size_t tg_ring_memsize(const struct tg_ring *ring);
struct tg_rect tg_ring_rect(const struct tg_ring *ring);
int tg_ring_num_points(const struct tg_ring *ring);
struct tg_point tg_ring_point_at(const struct tg_ring *ring, int index);
const struct tg_point *tg_ring_points(const struct tg_ring *ring);
int tg_ring_num_segments(const struct tg_ring *ring);
struct tg_segment tg_ring_segment_at(const struct tg_ring *ring, int index);
bool tg_ring_convex(const struct tg_ring *ring);
bool tg_ring_clockwise(const struct tg_ring *ring);
int tg_ring_index_spread(const struct tg_ring *ring);
int tg_ring_index_num_levels(const struct tg_ring *ring);
int tg_ring_index_level_num_rects(const struct tg_ring *ring, int levelidx);
struct tg_rect tg_ring_index_level_rect(const struct tg_ring *ring, int levelidx, int rectidx);
bool tg_ring_nearest_segment(const struct tg_ring *ring,
    double (*rect_dist)(struct tg_rect rect, int *more, void *udata),
    double (*seg_dist)(struct tg_segment seg, int *more, void *udata),
    bool (*iter)(struct tg_segment seg, double dist, int index, void *udata),
    void *udata);
void tg_ring_line_search(const struct tg_ring *a, const struct tg_line *b,
    bool (*iter)(struct tg_segment aseg, int aidx, struct tg_segment bseg,
        int bidx, void *udata),
    void *udata);
void tg_ring_ring_search(const struct tg_ring *a, const struct tg_ring *b,
    bool (*iter)(struct tg_segment aseg, int aidx, struct tg_segment bseg,
        int bidx, void *udata),
    void *udata);
double tg_ring_area(const struct tg_ring *ring);
double tg_ring_perimeter(const struct tg_ring *ring);
/// @}

/// @defgroup LineFuncs Line functions
/// Functions for working directly with the tg_line type.
///
/// There are no direct spatial predicates for tg_line.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the line to a tg_geom, like such:
///
/// ```
/// tg_geom_interects((struct tg_geom*)line, geom);
/// ```
/// @{
struct tg_line *tg_line_new(const struct tg_point *points, int npoints);
struct tg_line *tg_line_new_ix(const struct tg_point *points, int npoints, enum tg_index ix);
void tg_line_free(struct tg_line *line);
struct tg_line *tg_line_clone(const struct tg_line *line);
struct tg_line *tg_line_copy(const struct tg_line *line);
size_t tg_line_memsize(const struct tg_line *line);
struct tg_rect tg_line_rect(const struct tg_line *line);
int tg_line_num_points(const struct tg_line *line);
const struct tg_point *tg_line_points(const struct tg_line *line);
struct tg_point tg_line_point_at(const struct tg_line *line, int index);
int tg_line_num_segments(const struct tg_line *line);
struct tg_segment tg_line_segment_at(const struct tg_line *line, int index);
bool tg_line_clockwise(const struct tg_line *line);
int tg_line_index_spread(const struct tg_line *line);
int tg_line_index_num_levels(const struct tg_line *line);
int tg_line_index_level_num_rects(const struct tg_line *line, int levelidx);
struct tg_rect tg_line_index_level_rect(const struct tg_line *line, int levelidx, int rectidx);
bool tg_line_nearest_segment(const struct tg_line *line,
    double (*rect_dist)(struct tg_rect rect, int *more, void *udata),
    double (*seg_dist)(struct tg_segment seg, int *more, void *udata),
    bool (*iter)(struct tg_segment seg, double dist, int index, void *udata),
    void *udata);
void tg_line_line_search(const struct tg_line *a, const struct tg_line *b,
    bool (*iter)(struct tg_segment aseg, int aidx, struct tg_segment bseg,
        int bidx, void *udata),
    void *udata);
double tg_line_length(const struct tg_line *line);
/// @}

/// @defgroup PolyFuncs Polygon functions
/// Functions for working directly with the tg_poly type.
///
/// There are no direct spatial predicates for tg_poly.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the poly to a tg_geom, like such:
///
/// ```
/// tg_geom_interects((struct tg_geom*)poly, geom);
/// ```
/// @{
struct tg_poly *tg_poly_new(const struct tg_ring *exterior, const struct tg_ring *const holes[], int nholes);
void tg_poly_free(struct tg_poly *poly);
struct tg_poly *tg_poly_clone(const struct tg_poly *poly);
struct tg_poly *tg_poly_copy(const struct tg_poly *poly);
size_t tg_poly_memsize(const struct tg_poly *poly);
const struct tg_ring *tg_poly_exterior(const struct tg_poly *poly);
int tg_poly_num_holes(const struct tg_poly *poly);
const struct tg_ring *tg_poly_hole_at(const struct tg_poly *poly, int index);
struct tg_rect tg_poly_rect(const struct tg_poly *poly);
bool tg_poly_clockwise(const struct tg_poly *poly);
/// @}

/// @defgroup GlobalFuncs Global environment
/// Functions for optionally setting the behavior of the TG environment.
/// These, if desired, should be called only once at program start up and prior
/// to calling any other tg_*() functions.
/// @{
void tg_env_set_allocator(void *(*malloc)(size_t), void *(*realloc)(void*, size_t), void (*free)(void*));
void tg_env_set_index(enum tg_index ix);
void tg_env_set_index_spread(int spread);
/// @}


#endif // TG_H

*/
