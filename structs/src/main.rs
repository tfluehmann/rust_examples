#[derive(Debug)]
struct Volume {
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn circumference(&self) -> f32 {
        self.radius * 2.0 * 3.14
    }

    fn can_hold(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }

    fn associated_function(size: f32) -> Circle {
        Circle { radius: size }
    }
}

fn main() {
    let height = 30;
    let width = 30;
    let depth = 20;

    println!("Volume is: {}", volume(height, width, depth));

    let dimensions = (30,30,20);
    println!("Volume with tuples is: {}", volume_dim(dimensions));

    let volume = Volume {
        width: 30,
        height: 30,
        depth: 20,
    };
    println!("Debug output of volume: {:?}", volume);
    println!("This debug output makes it easier to read: {:#?}", volume);
    println!("Volume with structs is: {}", volume_struct(&volume));
    let volume2 = Volume {
        width: 300,
        ..volume
    };
    println!("another volume with struct is: {}", volume_struct(&volume2));

    let the_golden_circle = Circle {
        radius: 15.0,
    };
    let circle2 = Circle {
        radius: 14.9,
    };
    
    println!("The circles circumference is: {}", the_golden_circle.circumference());
    println!("The circles definition is: {:#?}", the_golden_circle);
    println!("The golden circle can hold the other one {}, the other way around...: {}", the_golden_circle.can_hold(&circle2), circle2.can_hold(&the_golden_circle));

    println!("Creating a new Circle with an associated function, not method, does not belong to an instance of struct: {:?}", Circle::associated_function(40.0));
}

fn volume(height: u32, width: u32, depth: u32) -> u32 {
    height * width * depth
}

fn volume_dim(dimensions: (u32, u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 * dimensions.2
}

fn volume_struct(volume: &Volume) -> u32 {
    volume.width * volume.height * volume.depth
}
