/// NOTE : code has been tested on rust playground to check if they are working as intended
/// Understanding the difference between Structs, Trait, Impl and Self
/// Reference : https://users.rust-lang.org/t/confusion-struct-impl-self-trait/3941
///
/// Structs : Data structures, a bit like classes in other languages. Structs are essentially the basic structures for Object Oriented Programming in Rust.
///
/// Trait : Obvious enough, a trait of an existing struct. For example, iamgine we have a struct named "Dog".
///
/// "Dog" can have a trait like "roll tongue" or maybe "blue eyes"
///
/// Impl : This can let you implemenet a trait for a struct. So, you can implement the trait "have tails" for the struct "Dogs". Impl is used to add a trait to an existing struct.
///
/// Self : Self is usually the object instance it self. When you create a function/procedure for a dog, you should add "self" to the list of arguments.
///
/// An example of using self is : fn wag_tail(&self) --> as that will allow us to use it in the following manner
///
/// let my_dog = Dog;
/// my_dog.wag_tail()
///
/// wheareas if we defined  something like : struct Dog { color : String, age : u32, eyeColor : String }
///
/// Consider the following example (Reference : https://stackoverflow.com/questions/70936236/what-is-the-purpose-of-b-here)
///
/// An important understanding to note regarding strings and why we use b when it comes to Strng
/// Any string prefixed by a "b" tells the compiler that the string should be treated a byte sequence. This is called a byte string literal.
///
/// A string in Rust is a valid sequence of unicode characters and hence it can be represented as &[u8] (A slice contianing unsigned 8-bit integers)
/// A String is stored as a vector of bytes (Vec<u8>), but guranteed to always be a valid UTF-8 sequence. String is heap alloated, growable and not null terminated
///
/// Official definition of struct : A struct is a custom data type that lets us name and package together multiple related values that make up a meaningful group (reference : https://doc.rust-lang.org/stable/book/structs.html)
///
/// Official definition of impl : Methods are different from functions in that they're defined within the context of a struct, but their first parameter is always self (this is similar to what we do when it comes to python, but at a more high level)
///
/// specifying self as the first parameter is used to represent the instance of the struct the method is being called upon

// consider the following custom type named Rectangle, that we named using struct
// the components that make up an area is width and height
// NOTE : don't make the following syntax error
// struct : like interfaces in typescript
// are seperated by commas(,) not semicolongs (;)
// an important distinguishment to remember moving forward
struct Rectangle {
    // i32 : signed integer 32-bit values
    width: i32,
    height: i32,
}

// now if we want to add a method to the Reactangle, the following would be the implemenetation
// Impl are the way you attach methods to a struct or a trait
impl Rectangle {
    // we can also define a constructor
    // observe that we have not added any parameter named &self in this case
    // since this isn't a method that is referencing the object
    // this is something that is to be used when instantiating the object instead
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }

    // this method will now allow us to calculate the area when the area method is invoked
    // this is something that is to be used AFTER instantiating the object
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

pub fn test() {
    // now if we were to take what we have defined and test it out here
    // we are naming it mutable so that it can referenced
    let mut custom_rect = Rectangle::new(10, 8);

    // this would be similar to writting
    // Rectangle::area(&mut custom_rect);
    // in this case the object being referenced is the custom_rect when the method is being invoked

    // this is one method of calculating the area
    let area = custom_rect.area();

    // consider another method of calculating the area
    // this is a "hackier" way of going about
    // this will break down the object by it's values
    // custom_rect.width = 10, custom_rect.height = 8
    let same_area = Rectangle::area(&mut custom_rect);

    // use string formatting to print out the area in this scenario
    println!("area of the rectangle (method 1) : {}", area);
    println!("area of the rectangle (method 2) : {}", same_area);
}

// TODO : write methods/Class based instances as needed
struct Dog {
    breed: String,
    color: String,
}

/// Trait : are a way to give different stuff to similar interfaces (interfaces : struct) in this scenario, generally speaking, traits are used to add onto an existing struct
///

/// before moving to trait, the context in this case is a different structure
/// structs should be treated as both interface + object simultaneously
/// interfaces are nothing but a TypeScript based terminology for writting custom types
struct Player {
    name: String, // this means name is of mutable String datatype
    health: u32,  // this means health is of unsigned 32 bit datatype
    damage: u32,  // similarly, this means damage dealt to player is also unsigned 32 bit datatype
}

/// next we define any relevant object instance based methods or class instance based properties
/// recall that the primary difference between the two
/// is that object based instance --> takes in &self as the first parameter
/// since it's referring to the object that has been instantiated
///
/// whereas class based instance doesn't take in &self as parameter
/// generally speaking, impl <struct_name> is the format that is followed
///
/// impl are the way you can attach methods to a struct or a trait

impl Player {
    // define the constructor
    // this is where rust shines
    // since we can define the constructor whatever name we want
    // observe that constructor is a class instance property
    // since we are not passing in &self into it
    // we are also specifying that this function will return the struct we have defined earlier
    // general convention is to use "new" for writting constructors in rust
    pub fn new(name: String) -> Player {
        // update the struct we have previously defined
        // first time we initialize an struct
        // we generally specify the properties and the types of each of such properties
        // as a followup, we generally specifying initial values
        // this is generally done when an constructor is defined
        Player {
            name: name,
            health: 1,
            damage: 1,
        }
    }

    // now we can define a method
    // in this case, this should be something that is part of the player
    // suppose in the case that when the player consumes a consumable interacting
    // with the environemnt (within an hypothetical game world)
    // we are targeting the unsighned 32 bit health property
    // and increasing it by 1 in this scenario
    // purpose of this function is to simply increment health by unit of 1
    pub fn incr_health(&mut self) {
        self.health += 1;
    }
}

/// this is where the trait based implementation comes in
/// suppose we are interacting with an object
/// let's suppose this object is a treasure chest
/// the treasure chest needs to be attacked to be broken down as well
struct treasure {
    health: u32,
    loot: String,
}

/// suppose some kind of monster were to attack
/// it is expected that the area should break down
/// and that both players and objects surrounding player should be taking damage
/// so we define a trait named Damagable --> we are "attaching" this trait to the treasure as well
/// traits are generally implementations that are "added" onto existing implementaitons
/// traits can also viewed as "pseudo-inheritnace", another customization capabillity offered by rust
trait Damagable {
    // a getter method to retrieve what the current HP of the treasure is
    // suppose the treasure took damange
    fn get_health(&self) -> u32;

    // when defining setters
    // ensure that the self parameter is mutable
    // otherwise, we cannot implement any form of modification to the object
    // after it has been instantiated
    // the "parameter" (since we ignore self as a parameter --> it is just to indicate that it is a method that can be invoked after the object has been instantiated)
    // in this case is damange, if the treasure is attacked, it will take damage --> logically speaking
    // setters performs modifications, but doesn't return anything
    // setter method should be triggered to modify the HP bar of the treasure object
    fn set_health(&mut self, hp: u32);

    // a function to do something upon death
    fn on_death(&self);

    // define the logic for damage
    // as in how damage should be applied when this method gets called upon
    // the &mut self parameter is used to indicate the object itself is being referenced
    // as well as that some form of modifications needs to take place
    fn damage(&mut self, damage: u32) {
        // we can also directly access methods within the object
        // similar to python, where after defining a method within a class
        // we can call on the method using the self.<method_name>
        // rust works similarly
        let hp = self.get_health();

        // if current hp is less than or equal to damage being taken
        // hypothetically, the player should die
        // hp cannot be negative
        // so we have to set it to 0 instead
        // if this conditional executes, this indicates that the player died
        // following this, we call on the function that will handle what to do upon death
        if hp <= damage {
            self.set_health(0);
            self.on_death();
        } else {
            // otherwise, while hp > damange
            // that means the player should recieve damage
            // but hasn't died yet
            // in that case, we call on the setter with minor modification
            // NOTE : although I keep stating Player
            // this can apply for treasures
            // where upon HP turning zero, the content of the loot should be revealed
            self.set_health(hp - damage);
        }
    }
}

/// now that we have defined the trait Damagable, we can apply it to an existing struct
/// of course to apply something to an existing struct
/// we use impl, since impl is used to define the logic for existing struct, unlike traits
/// impl can also be used for existing trait as well
/// this is the part where we are attaching trait
/// impl Damagable for treasure essentially "combines" the methods/properties of the two
/// we did not define any constructor in this case
impl Damagable for treasure {
    // we can easily call on the methods we have initialized and define their logic
    // this logic will apply for treasure only
    // in rust, return does not need to be explcitly stated
    // unless you need to end early during the execution
    // so it's good practice to use return unless some value is returned as the last line
    // of the function
    // when it comes to defining the methods in this scenario
    // the parameters, return type, and function names must match
    // otherwise, an error will be thrown
    fn get_health(&self) -> u32 {
        return self.health;
    }

    fn set_health(&mut self, hp: u32) {
        self.health = hp;
    }

    // specify the content of the loot
    // we can access the loot content using self.loot
    fn on_death(&self) {
        println!("New treasure content : {}", self.loot)
    }
}

fn treasure_test() {
    // of course we need to test it out
    // because we haven't defined a constructor using the new method
    // we will need to initialize it here
    // note the following syntax for defining a constructor when it comes to rust
    // the method remains the same
    let mut treasure_chest = treasure {
        health: 1,
        // we need to ensure that we bind the string
        loot: "rare armor".to_owned(),
    };

    // next we define the player
    // so that the player can attack it
    // the player is defined using the constructor method we have defined within impl
    // we also need to make sure the string we pass in is converted to owned
    // String type needs to be of type owned
    let mut player = Player::new("hero".to_owned());

    // player attacks the trasure_chest
    treasure_chest.damage(player.damage);
}

pub fn main() {
    // now if we call on the test() function within main
    // test functions that has been defined outside
    test();
    treasure_test();
}
