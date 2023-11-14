#![allow(nonstandard_style)]

/// The base point type used for all geometries.
///
/// - See [`PointFuncs`]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tg_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}


/// The base segment type used in [`tg_line`] and [`tg_ring`] for joining two vertices.
///
/// - See [`SegmentFuncs`]`
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tg_segment {
    pub a: tg_point,
    pub b: tg_point,
}

/// A rectangle defined by a minimum and maximum coordinates.
/// Returned by the [`tg_geom_rect()`][GeometryAccessors::tg_geom_rect], [`tg_ring_rect()`][RingFuncs::tg_ring_rect], and other `\*_rect()`
/// functions for getting a geometry's minumum bounding rectangle.
/// Also used internally for geometry indexing.
///
/// - See [`RectFuncs`]
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
/// - See [`tg_geom_typeof()`][`GeometryAccessors::tg_geom_type_string`]`
/// - See [`tg_geom_type_string()`][`GeometryAccessors::tg_geom_type_string`]
/// - See [`GeometryAccessors`]
#[repr(C)]
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
/// [`tg_ring_index_*()`][RingFuncs] and [`tg_ring_nearest_segment()`][RingFuncs::tg_ring_nearest_segment] functions.
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

/// A line is a series of tg_segment that make up a linestring.
///
/// **Creating**
///
/// To create a new line use the tg_line_new() function.
///
/// ```c
/// struct tg_line *line = tg_line_new(points, npoints);
/// ```
///
/// **Upcasting**
///
/// A tg_line can always be safely upcasted to a [`tg_geom`]; allowing
/// it to use any `tg_geom_*()` function.
///
/// ```c
/// struct tg_geom *geom = (struct tg_geom*)line; // Cast to a tg_geom
/// ```
///
/// - See [`LineFuncs`]
#[repr(C)]
pub struct tg_line {
    _unused: [u8; 0],
}

/// A ring is series of [`tg_segment`] which creates a shape that does not
/// self-intersect and is fully closed, where the start and end points are the
/// exact same value.
///
/// **Creating**
///
/// To create a new ring use the [`tg_ring_new()`][RingFuncs::tg_ring_new] function.
///
/// ```c
/// struct tg_ring *ring = tg_ring_new(points, npoints);
/// ```
///
/// **Upcasting**
///
/// A tg_ring can always be safely upcasted to a [`tg_poly`] or [`tg_geom`]; allowing
/// it to use any [`tg_poly_*()`][PolyFuncs] or [`tg_geom_*()`][RingFuncs] function.
///
/// ```c
/// struct tg_poly *poly = (struct tg_poly*)ring; // Cast to a tg_poly
/// struct tg_geom *geom = (struct tg_geom*)ring; // Cast to a tg_geom
/// ```
/// - See [`RingFuncs`]
/// - See [`PolyFuncs`]
#[repr(C)]
pub struct tg_ring {
    _unused: [u8; 0],
}

/// A polygon consists of one exterior ring and zero or more holes.
///
/// **Creating**
///
/// To create a new polygon use the [`tg_poly_new()`][PolyFuncs::tg_poly_new] function.
///
/// ```c
/// struct tg_poly *poly = tg_poly_new(exterior, holes, nholes);
/// ```
///
/// **Upcasting**
///
/// A tg_poly can always be safely upcasted to a [`tg_geom`]; allowing
/// it to use any tg_geom_&ast;() function.
///
/// ```c
/// struct tg_geom *geom = (struct tg_geom*)poly; // Cast to a tg_geom
/// ```
///
/// - See [`PolyFuncs`]
#[repr(C)]
pub struct tg_poly {
    _unused: [u8; 0],
}

/// A geometry is the common generic type that can represent a Point,
/// LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon, or
/// GeometryCollection.
///
/// For geometries that are derived from GeoJSON, they may have addtional
/// attributes such as being a Feature or a FeatureCollection; or include
/// extra json fields.
///
/// **Creating**
///
/// To create a new geometry use one of the [`GeometryConstructors`] or
/// [`GeometryParsing`] functions.
///
/// ```c
/// struct tg_geom *geom = tg_geom_new_point(point);
/// struct tg_geom *geom = tg_geom_new_polygon(poly);
/// struct tg_geom *geom = tg_parse_geojson(geojson);
/// ```
///
/// **Upcasting**
///
/// Other types, specifically [`tg_line`], [`tg_ring`], and [`tg_poly`], can be safely
/// upcasted to a tg_geom; allowing them to use any tg_geom_&ast;()
/// function.
///
/// ```c
/// struct tg_geom *geom1 = (struct tg_geom*)line; // Cast to a LineString
/// struct tg_geom *geom2 = (struct tg_geom*)ring; // Cast to a Polygon
/// struct tg_geom *geom3 = (struct tg_geom*)poly; // Cast to a Polygon
/// ```
///
/// - See [`GeometryConstructors`]
/// - See [`GeometryAccessors`]
/// - See [`GeometryPredicates`]
/// - See [`GeometryParsing`]
/// - See [`GeometryWriting`]
#[repr(C)]
pub struct tg_geom {
    _unused: [u8; 0],
}

/// Geometry constructors
///
/// Functions for creating and freeing geometries.
pub mod GeometryConstructors {
    use crate::tg_geom;
    use crate::{tg_line, tg_point, tg_poly};

    #[link(name = "tg")]
    extern "C" {
        /// Creates a Point geometry
        ///
        /// Returns a newly allocated geometry, or NULL if system is out of
        /// memory.
        ///
        /// # Safety:
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_point(
            point: tg_point) -> *mut tg_geom;

        /// Creates a LineString geometry.
        ///
        /// Caller retains ownership of the input pointer.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_linestring(line: *const tg_line) -> *mut tg_geom;

        /// Creates a Polygon geometry.
        ///
        /// Caller retains ownership of the input pointer.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_polygon(poly: *const tg_poly) -> *mut tg_geom;

        /// Creates a MultiPoint geometry.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Params
        ///
        /// - `points`: An array of points, caller retains ownership.
        /// - `npoints`: The number of points in array
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_multipoint(
            points: *const tg_point,
            npoints: libc::c_int,
        ) -> *mut tg_geom;


        /// Creates a MultiLineString geometry.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Params
        ///
        /// - `lines`: An array of lines, caller retains ownership.
        /// - `nlines`: The number of lines in array
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_multilinestring(
            lines: *const *const tg_line,
            nlines: libc::c_int,
        ) -> *mut tg_geom;

        /// Creates a MultiPolygon geometry.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Params
        ///
        /// - `polys`: An array of polygons, caller retains ownership.
        /// - `npolys`: The number of polygons in array
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_multipolygon(
            polys: *const *const tg_poly,
            npolys: libc::c_int,
        ) -> *mut tg_geom;

        /// Creates a GeometryCollection geometry.
        ///
        /// Returns a newly allocated geometry or NULL if system is out of memory.
        ///
        /// # Params
        ///
        /// - `geoms`: An array of geometries, caller retains ownership.
        /// - `ngeoms`: The number of geometries in array
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_new_geometrycollection(
            geoms: *const *const tg_geom,
            ngeoms: libc::c_int,
        ) -> *mut tg_geom;

        /// Clones a geometry
        ///
        /// Returns a duplicate of the provided geometry.
        ///
        /// This method of cloning uses implicit sharing through an atomic
        /// reference counter.
        ///
        /// # Params
        ///
        /// - `geom`: Input geometry, caller retains ownership.
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with [`tg_geom_free()`][tg_geom_free].
        pub fn tg_geom_clone(geom: *const tg_geom) -> *mut tg_geom;

        /// Copies a geometry
        ///
        /// Returns a duplicate of the provided geometry or NULL if out of memory
        ///
        /// This method performs a deep copy of the entire geometry to new memory.
        ///
        /// # Params
        ///
        /// - `geom`: Input geometry, caller retains ownership.
        ///
        /// # Safety
        ///
        /// The caller is responsible for freeing with tg_geom_free().
        pub fn tg_geom_copy(geom: *const tg_geom) -> *mut tg_geom;

        /// Releases the memory associated with a geometry.
        /// # Params
        ///
        /// - `geom`: Input geometry
        pub fn tg_geom_free(geom: *mut tg_geom);
    }
}

/// Geometry accessors
///
/// Functions for accessing various information about geometries, such as
/// getting the geometry type or extracting underlying components or
/// coordinates.
pub mod GeometryAccessors {
    // done
    use crate::{tg_geom, tg_geom_type, tg_line, tg_point, tg_poly, tg_rect};

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
        pub fn tg_geom_line_at(geom: *const tg_geom, index: libc::c_int) -> *const tg_line;
        pub fn tg_geom_num_polys(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_poly_at(geom: *const tg_geom, index: libc::c_int) -> *const tg_poly;
        pub fn tg_geom_num_geometries(geom: *const tg_geom) -> libc::c_int;
        pub fn tg_geom_geometry_at(geom: *const tg_geom, index: libc::c_int) -> *const tg_geom;
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
            iter: extern "C" fn(
                geom: *const tg_geom,
                index: libc::c_int,
                udata: *mut libc::c_void,
            ) -> bool,
            udata: *mut libc::c_void,
        );
    }
}

/// Geometry predicates
///
/// Functions for testing the spatial relations of two geometries.
pub mod GeometryPredicates {
    // done
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
}

/// Geometry parsing
///
/// Functions for parsing geometries from external data representations.
/// It's recommended to use tg_geom_error() after parsing to check for errors.
pub mod GeometryParsing {
    // done

    use crate::{tg_geom, tg_index};

    #[link(name = "tg")]
    extern "C" {
        pub fn tg_parse_geojson(geojson: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_geojsonn(geojson: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_geojson_ix(geojson: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_geojsonn_ix(
            geojson: *const libc::c_char,
            len: libc::size_t,
            ix: tg_index,
        ) -> *mut tg_geom;
        pub fn tg_parse_wkt(wkt: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_wktn(wkt: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_wkt_ix(wkt: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_wktn_ix(
            wkt: *const libc::c_char,
            len: libc::size_t,
            ix: tg_index,
        ) -> *mut tg_geom;
        pub fn tg_parse_wkb(wkb: *const u8, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_wkb_ix(wkb: *const u8, len: libc::size_t, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_hex(hex: *const libc::c_char) -> *mut tg_geom;
        pub fn tg_parse_hexn(hex: *const libc::c_char, len: libc::size_t) -> *mut tg_geom;
        pub fn tg_parse_hex_ix(hex: *const libc::c_char, ix: tg_index) -> *mut tg_geom;
        pub fn tg_parse_hexn_ix(
            hex: *const libc::c_char,
            len: libc::size_t,
            ix: tg_index,
        ) -> *mut tg_geom;
        pub fn tg_geom_error(geom: *const tg_geom) -> *const libc::c_char;
    }
}

/// Geometry writing
///
/// Functions for writing geometries as external data representations.
pub mod GeometryWriting {
    // done

    use crate::tg_geom;

    extern "C" {

        pub fn tg_geom_geojson(
            geom: *const tg_geom,
            dst: *mut libc::c_char,
            n: libc::size_t,
        ) -> libc::size_t;
        pub fn tg_geom_wkt(
            geom: *const tg_geom,
            dst: *mut libc::c_char,
            n: libc::size_t,
        ) -> libc::size_t;
        pub fn tg_geom_wkb(geom: *const tg_geom, dst: *mut u8, n: libc::size_t) -> libc::size_t;
        pub fn tg_geom_hex(
            geom: *const tg_geom,
            dst: *mut libc::c_char,
            n: libc::size_t,
        ) -> libc::size_t;
    }
}

/// Geometry with alternative dimensions
///
/// Functions for working with geometries that have more than two dimensions or
/// are empty. The extra dimensional coordinates contained within these
/// geometries are only carried along and serve no other purpose than to be
/// available for when it's desired to export to an output representation such
/// as GeoJSON, WKT, or WKB.
pub mod GeometryConstructorsEx {
    // done

    use crate::{tg_geom, tg_line, tg_point, tg_poly};

    extern "C" {

        pub fn tg_geom_new_point_z(point: tg_point, z: libc::c_double) -> *mut tg_geom;
        pub fn tg_geom_new_point_m(point: tg_point, m: libc::c_double) -> *mut tg_geom;
        pub fn tg_geom_new_point_zm(
            point: tg_point,
            z: libc::c_double,
            m: libc::c_double,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_point_empty() -> *mut tg_geom;
        pub fn tg_geom_new_linestring_z(
            line: *const tg_line,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_linestring_m(
            line: *const tg_line,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_linestring_zm(
            line: *const tg_line,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_linestring_empty() -> *mut tg_geom;
        pub fn tg_geom_new_polygon_z(
            poly: *const tg_poly,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_polygon_m(
            poly: *const tg_poly,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_polygon_zm(
            poly: *const tg_poly,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_polygon_empty() -> *mut tg_geom;
        pub fn tg_geom_new_multipoint_z(
            points: *const tg_point,
            npoints: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipoint_m(
            points: *const tg_point,
            npoints: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipoint_zm(
            points: *const tg_point,
            npoints: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipoint_empty() -> *mut tg_geom;
        pub fn tg_geom_new_multilinestring_z(
            lines: *const *const tg_line,
            nlines: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multilinestring_m(
            lines: *const *const tg_line,
            nlines: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multilinestring_zm(
            lines: *const *const tg_line,
            nlines: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multilinestring_empty() -> *mut tg_geom;
        pub fn tg_geom_new_multipolygon_z(
            polys: *const *const tg_poly,
            npolys: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipolygon_m(
            polys: *const *const tg_poly,
            npolys: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipolygon_zm(
            polys: *const *const tg_poly,
            npolys: libc::c_int,
            extra_coords: *const libc::c_double,
            ncoords: libc::c_int,
        ) -> *mut tg_geom;
        pub fn tg_geom_new_multipolygon_empty() -> *mut tg_geom;
        pub fn tg_geom_new_geometrycollection_empty() -> *mut tg_geom;
    }
}

/// Point functions
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

/// Segment functions
///
/// Functions for working directly with the tg_segment type.
pub mod SegmentFuncs {
    use crate::{tg_rect, tg_segment};
    #[link(name = "tg")]
    extern "C" {
        pub fn tg_segment_rect(s: tg_segment) -> tg_rect;
        pub fn tg_segment_intersects_segment(a: tg_segment, b: tg_segment) -> bool;
    }
}

/// Rectangle functions
///
/// Functions for working directly with the tg_rect type.
pub mod RectFuncs {
    // done

    use crate::{tg_point, tg_rect};

    extern "C" {
        pub fn tg_rect_expand(rect: tg_rect, other: tg_rect) -> tg_rect;
        pub fn tg_rect_expand_point(rect: tg_rect, point: tg_point) -> tg_rect;
        pub fn tg_rect_center(rect: tg_rect) -> tg_point;
        pub fn tg_rect_intersects_rect(a: tg_rect, b: tg_rect) -> bool;
        pub fn tg_rect_intersects_point(a: tg_rect, b: tg_point) -> bool;
    }
}

/// Ring functions
///
/// Functions for working directly with the tg_ring type.
///
/// There are no direct spatial predicates for tg_ring.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the ring to a tg_geom, like such:
///
/// ```c
/// tg_geom_intersects((struct tg_geom*)ring, geom);
/// ```
pub mod RingFuncs {
    // done
    use crate::{tg_index, tg_line, tg_point, tg_rect, tg_ring, tg_segment};
    extern "C" {
        pub fn tg_ring_new(points: *const tg_point, npoints: libc::c_int) -> *mut tg_ring;
        pub fn tg_ring_new_ix(
            points: *const tg_point,
            npoints: libc::c_int,
            ix: tg_index,
        ) -> *mut tg_ring;
        pub fn tg_ring_free(ring: *mut tg_ring);
        pub fn tg_ring_clone(ring: *const tg_ring) -> *mut tg_ring;
        pub fn tg_ring_copy(ring: *const tg_ring) -> *mut tg_ring;
        pub fn tg_ring_memsize(ring: *const tg_ring) -> libc::size_t;
        pub fn tg_ring_rect(ring: *const tg_ring) -> tg_rect;
        pub fn tg_ring_num_points(ring: *const tg_ring) -> libc::c_int;
        pub fn tg_ring_point_at(ring: *const tg_ring, index: libc::c_int) -> tg_point;
        pub fn tg_ring_points(ring: *const tg_ring) -> *const tg_point;
        pub fn tg_ring_num_segments(ring: *const tg_ring) -> libc::c_int;
        pub fn tg_ring_segment_at(ring: *const tg_ring, index: libc::c_int) -> tg_segment;
        pub fn tg_ring_convex(ring: *const tg_ring) -> bool;
        pub fn tg_ring_clockwise(ring: *const tg_ring) -> bool;
        pub fn tg_ring_index_spread(ring: *const tg_ring) -> libc::c_int;
        pub fn tg_ring_index_num_levels(ring: *const tg_ring) -> libc::c_int;
        pub fn tg_ring_index_level_num_rects(
            ring: *const tg_ring,
            levelidx: libc::c_int,
        ) -> libc::c_int;
        pub fn tg_ring_index_level_rect(
            ring: *const tg_ring,
            levelidx: libc::c_int,
            rectidx: libc::c_int,
        ) -> tg_rect;
        pub fn tg_ring_nearest_segment(
            ring: *const tg_ring,
            seg_dist: extern "C" fn(
                seg: tg_segment,
                more: libc::c_int,
                udata: *mut libc::c_void,
            ) -> libc::c_double,
            rect_dist: extern "C" fn(
                rect: tg_rect,
                more: libc::c_int,
                udata: *mut libc::c_void,
            ) -> libc::c_double,
            iter: extern "C" fn(
                seg: tg_segment,
                dist: libc::c_double,
                index: libc::c_int,
                udata: *mut libc::c_void,
            ),
            udata: *mut libc::c_void,
        ) -> bool;

        pub fn tg_ring_line_search(
            a: *const tg_ring,
            b: *const tg_line,
            iter: extern "C" fn(
                aseg: tg_segment,
                aidx: libc::c_int,
                bseg: tg_segment,
                bidx: libc::c_int,
                udata: *mut libc::c_void,
            ),
            udata: *mut libc::c_void,
        );
        pub fn tg_ring_ring_search(
            a: *const tg_ring,
            b: *const tg_ring,
            iter: extern "C" fn(
                aseg: tg_segment,
                aidx: libc::c_int,
                bseg: tg_segment,
                bidx: libc::c_int,
                udata: *mut libc::c_void,
            ),
            udata: *mut libc::c_void,
        );
        pub fn tg_ring_area(libc: *const tg_ring) -> libc::c_double;
        pub fn tg_ring_perimeter(libc: *const tg_ring) -> libc::c_double;
    }
}

/// Line functions
///
/// Functions for working directly with the tg_line type.
///
/// There are no direct spatial predicates for tg_line.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the line to a tg_geom, like such:
///
/// ```c
/// tg_geom_intersects((struct tg_geom*)line, geom);
/// ```
pub mod LineFuncs {
    // done

    use crate::{tg_index, tg_line, tg_point, tg_rect, tg_segment};

    extern "C" {

        pub fn tg_line_new(points: *const tg_point, npoints: libc::c_int) -> *mut tg_line;
        pub fn tg_line_new_ix(
            points: *const tg_point,
            npoints: libc::c_int,
            ix: tg_index,
        ) -> *mut tg_line;
        pub fn tg_line_free(line: *mut tg_line);
        pub fn tg_line_clone(line: *const tg_line) -> *mut tg_line;
        pub fn tg_line_copy(line: *const tg_line) -> *mut tg_line;
        pub fn tg_line_memsize(line: *const tg_line) -> libc::size_t;
        pub fn tg_line_rect(line: *const tg_line) -> tg_rect;
        pub fn tg_line_num_points(line: *const tg_line) -> libc::c_int;
        pub fn tg_line_points(line: *const tg_line) -> *const tg_point;
        pub fn tg_line_point_at(line: *const tg_line, index: libc::c_int) -> tg_point;
        pub fn tg_line_num_segments(line: *const tg_line) -> libc::c_int;
        pub fn tg_line_segment_at(line: *const tg_line, index: libc::c_int) -> tg_segment;
        pub fn tg_line_clockwise(line: *const tg_line) -> bool;
        pub fn tg_line_index_spread(line: *const tg_line) -> libc::c_int;
        pub fn tg_line_index_num_levels(line: *const tg_line) -> libc::c_int;
        pub fn tg_line_index_level_num_rects(
            line: *const tg_line,
            levelidx: libc::c_int,
        ) -> libc::c_int;
        pub fn tg_line_index_level_rect(
            line: *const tg_line,
            levelidx: libc::c_int,
            rectidx: libc::c_int,
        ) -> tg_rect;
        pub fn tg_line_nearest_segment(
            line: *const tg_line,
            rect_dist: extern "C" fn(
                rect: tg_rect,
                more: *const libc::c_int,
                udata: *mut libc::c_void,
            ) -> libc::c_double,
            seg_dist: extern "C" fn(
                seg: tg_segment,
                more: *const libc::c_int,
                udata: *mut libc::c_void,
            ) -> libc::c_double,
            iter: extern "C" fn(
                seg: tg_segment,
                dist: libc::c_double,
                index: libc::c_int,
                udata: *mut libc::c_void,
            ) -> bool,
            udata: *mut libc::c_void,
        ) -> bool;
        pub fn tg_line_line_search(
            a: *const tg_line,
            b: *const tg_line,
            iter: extern "C" fn(
                aseg: tg_segment,
                aidx: libc::c_int,
                bseg: tg_segment,
                bidx: libc::c_int,
                udata: *mut libc::c_void,
            ) -> bool,
            udata: *mut libc::c_void,
        );
        pub fn tg_line_length(line: *const tg_line) -> libc::c_double;
    }
}

/// Polygon functions
///
/// Functions for working directly with the tg_poly type.
///
/// There are no direct spatial predicates for tg_poly.
/// If you want to perform operations like "intersects" or "covers" then you
/// must upcast the poly to a tg_geom, like such:
///
/// ```c
/// tg_geom_intersects((struct tg_geom*)poly, geom);
/// ```
pub mod PolyFuncs {
    // done

    use crate::{tg_poly, tg_rect, tg_ring};

    extern "C" {
        pub fn tg_poly_new(
            exterior: *const tg_ring,
            holes: *const *const tg_ring,
            nholes: libc::c_int,
        ) -> *mut tg_poly;
        pub fn tg_poly_free(poly: *mut tg_poly);
        pub fn tg_poly_clone(poly: *const tg_poly) -> *mut tg_poly;
        pub fn tg_poly_copy(poly: *const tg_poly) -> *mut tg_poly;
        pub fn tg_poly_memsize(poly: *const tg_poly) -> libc::size_t;
        pub fn tg_poly_exterior(poly: *const tg_poly) -> *const tg_ring;
        pub fn tg_poly_num_holes(poly: *const tg_poly) -> libc::c_int;
        pub fn tg_poly_hole_at(poly: *const tg_poly, index: libc::c_int) -> *const tg_ring;
        pub fn tg_poly_rect(poly: *const tg_poly) -> tg_rect;
        pub fn tg_poly_clockwise(poly: *const tg_poly) -> bool;
    }
}

/// @defgroup GlobalFuncs Global environment
/// Functions for optionally setting the behavior of the TG environment.
/// These, if desired, should be called only once at program start up and prior
/// to calling any other tg_*() functions.
pub mod GlobalFuncs {
    // done

    use crate::tg_index;
    extern "C" {

        pub fn tg_env_set_allocator(
            malloc: extern "C" fn(size: libc::size_t) -> *mut libc::c_void,
            realloc: extern "C" fn(
                alloc: *mut libc::c_void,
                size: libc::size_t,
            ) -> *mut libc::c_void,
            free: extern "C" fn(),
        );
        pub fn tg_env_set_index(ix: tg_index);
        pub fn tg_env_set_index_spread(spread: libc::c_int);
    }
}

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

/// [SNIP]

#endif // TG_H

*/
