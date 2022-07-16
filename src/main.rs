enum TrafficLight{
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn show_time(&self) {
        match self {
            TrafficLight::Red => println!("50秒"),
            TrafficLight::Green => println!("60秒"),
            TrafficLight::Yellow => println!("5秒")
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    red.show_time();
    let yel = TrafficLight::Yellow;
    yel.show_time();
    let gre = TrafficLight::Green;
    gre.show_time();
}
// trait尝试后实现失败
// trait ShowTime {
//     fn show_time(&self) -> String;
// }
// enum TrafficLight{
//     Red,
//     Green,
//     Yellow
// }
// impl ShowTime for TrafficLight {
//     fn show_time(&self) -> String{
//         if self == TrafficLight::Red {
//             "50秒".to_string()
//         }else if self == TrafficLight::Green {
//             "60秒".to_string()
//         }else{
//             "5秒".to_string()  
//         }

//         // match self {
//         //     TrafficLight::Red => "50秒",
//         //     TrafficLight::Green => "60秒",
//         //     TrafficLight::Yellow => "5秒"
//         // }
//     }
// }
// fn main() {
//     let red = TrafficLight::Red;
//     red.show_time();
//     let yel = TrafficLight::Yellow;
//     yel.show_time();
//     let gre = TrafficLight::Green;
//     gre.show_time();
// }
