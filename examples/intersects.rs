use std::ffi::CStr;

use tg_sys::{
    GeometryConstructors::tg_geom_free,
    GeometryParsing::{tg_geom_error, tg_parse_wkt},
    GeometryPredicates::tg_geom_intersects,
};

fn main() {
    if std::env::args().len() != 3 {
        eprintln!(
            "Usage: {} <geom-a> <geom-b>",
            std::env::args().next().as_deref().unwrap_or("__")
        );
        std::process::exit(1);
    }

    // Parse the input geometries and check for errors.
    let mut a = std::env::args().nth(1).unwrap();
    a.push('\0');
    let a = CStr::from_bytes_with_nul(a.as_bytes()).unwrap();
    let a = unsafe { tg_parse_wkt(a.as_ptr()) };
    let err = unsafe { tg_geom_error(a) };
    if !err.is_null() {
        let err = unsafe { CStr::from_ptr(err) };
        eprintln!(
            "In {}: {:?}",
            std::env::args().nth(1).unwrap(),
            err.to_str().expect("INVALID ERROR STRING")
        );
        std::process::exit(1);
    }

    let mut b = std::env::args().nth(2).unwrap();
    b.push('\0');
    let b = CStr::from_bytes_with_nul(b.as_bytes()).unwrap();
    let b = unsafe { tg_parse_wkt(b.as_ptr()) };
    let err = unsafe { tg_geom_error(b) };
    if !err.is_null() {
        let err = unsafe { CStr::from_ptr(err) };
        eprintln!(
            "In {}: {:?}",
            std::env::args().nth(2).unwrap(),
            err.to_str().expect("INVALID ERROR STRING")
        );
        std::process::exit(1);
    }

    // Execute the "intersects" predicate to test if both geometries intersect.
    if unsafe { tg_geom_intersects(a, b) } {
        println!("yes");
    } else {
        println!("no");
    }

    // Free geometries when done.
    unsafe {
        tg_geom_free(a);
        tg_geom_free(b);
    }
}

/*
int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <geom-a> <geom-b>\n", argv[0]);
        return 1;
    }

    // Parse the input geometries and check for errors.
    struct tg_geom *a = tg_parse_wkt(argv[1]);
    if (tg_geom_error(a)) {
        fprintf(stderr, "%s\n", tg_geom_error(a));
        return 1;
    }
    struct tg_geom *b = tg_parse_wkt(argv[2]);
    if (tg_geom_error(b)) {
        fprintf(stderr, "%s\n", tg_geom_error(b));
        return 1;
    }

    // Execute the "intersects" predicate to test if both geometries intersect.
    if (tg_geom_intersects(a, b)) {
        printf("yes\n");
    } else {
        printf("no\n");
    }

    // Free geometries when done.
    tg_geom_free(a);
    tg_geom_free(b);
    return 0;
}
*/
