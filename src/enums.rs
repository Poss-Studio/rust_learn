#[derive(Debug)]
enum TrafficLight
{
    Red,
    Green,
    Yellow
}
impl TrafficLight
{
    fn isred(&self)->self::TrafficLight{
	TrafficLight::Red
    }
    fn isgreen(&self) -> self::TrafficLight{
	TrafficLight::Green
    }
    fn isyellow(&self) -> self::TrafficLight{
	TrafficLight::Yellow
    }
}
#[derive(Debug)]
enum ips{
    V4(&'static str),
    V6(&'static str)
}
impl ips{
    fn person_use() -> self::ips{
        ips::V4("127.0.0.1")
    }
    fn comapny_use() -> self::ips{
        ips::V6("::1")
    }
}
pub fn light(){
    let mut ax = TrafficLight::Red;
    let bx = &mut ax;
    println!("The Traffic of the ax is {:?}",*bx);
    println!("The Traffic of the ax is {:#?}",crate::enums::TrafficLight::isgreen(&ax));
    println!("The address of local wifi is {:?}",ips::person_use());
}

