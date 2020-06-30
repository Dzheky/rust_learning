struct Point {
    x: i32,
    y: i32,
}

struct Rect {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rect) -> i32 {
    let Rect {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right:
            Point {
                x: bottom_right_x,
                y: bottom_right_y,
            },
    } = rect;

    let left = bottom_right_x - top_left_x;
    let right = bottom_right_y - top_left_y;

    left * right
}

fn square(point: Point, size: i32) -> Rect {
    Rect {
        top_left: Point {
            x: point.x,
            y: point.y - size,
        },
        bottom_right: Point {
            x: point.x + size,
            y: point.y,
        },
    }
}

fn main() {
    let rect = square(Point { x: 10, y: 20 }, 10);

    println!("{}", rect_area(rect));
}
