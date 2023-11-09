
use tg_sys::tg_point;

fn main() {
    let pt = tg_point { x: 4.2, y: 2.3 };
    println!("{pt:?}");
    let rect = unsafe { tg_sys::PointFuncs::tg_point_rect(pt) };
    println!("{rect:?}");
}
/*
#include <stdio.h>
#include <tg.h>

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
