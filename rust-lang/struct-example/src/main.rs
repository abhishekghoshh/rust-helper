#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn clear(&mut self) {
        self.height = 0;
        self.width = 0;
    }
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle {
            height: side,
            width: side,
        }
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(rect.area(), 1500);
    }

    #[test]
    fn test_clear() {
        let mut rect = Rectangle {
            width: 30,
            height: 50,
        };
        rect.clear();
        assert_eq!(rect.width, 0);
        assert_eq!(rect.height, 0);
    }

    #[test]
    fn test_square() {
        let square = Rectangle::square(10);
        assert_eq!(square.area(), 100);
        assert_eq!(square.width, 10);
        assert_eq!(square.height, 10);
    }
}

#[derive(Debug)]
struct Dimension(i32, i32);

impl Dimension {
    fn distance(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }
    fn distance_from(&self, dimension: &Dimension) -> f32 {
        let delta_x = self.0 - dimension.1;
        let delta_y = self.1 - dimension.1;
        let sqr_distance: f32 = (delta_x * delta_x + delta_y * delta_y) as f32;
        sqr_distance.sqrt()
    }
}

fn main() {
    let rect = Rectangle {
        width: 120,
        height: 80,
    };
    println!("normal {:?}", rect);
    println!("formatted {:#?}", rect);

    let mut square = Rectangle {
        width: 0,
        height: 0,
    };
    square.height = 100;
    square.width = 100;
    println!("square {:#?}", square);

    println!("the area is {}", rect.area());

    let mut another_square = Rectangle::square(30);
    println!("another square {:?}", another_square);
    another_square.clear();
    println!("another square cleared {:?}", another_square);

    let dimension: Dimension = Dimension(2, 2);
    println!("dimension is {:?}", dimension);
    println!("distance from 0 is {}", dimension.distance());
    let another_point = Dimension(-4, 3);
    println!(
        "distance from between {:?} and {:?} is {}",
        dimension,
        another_point,
        dimension.distance_from(&anotherPoint)
    );
}
