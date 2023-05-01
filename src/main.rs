use rosrust_msg::geometry_msgs::*;
use rosrust_msg::std_msgs::*;
use rosrust_msg::visualization_msgs::*;

fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("talker");

    assert!(rosrust::param("test").is_some());

    // Create publisher
    let chatter_pub = rosrust::publish("chatter", 2).unwrap();
    chatter_pub.wait_for_subscribers(None).unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    let mut count = 0;

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        let msg = Marker {
            header: Header {
                stamp: rosrust::now(),
                frame_id: "my_frame".to_string(),
                ..Default::default()
            },
            ns: "sslocate".to_string(),
            id: 0,
            type_: Marker::CUBE as i32,
            pose: Pose {
                position: Point {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                orientation: Default::default(),
            },
            color: ColorRGBA {
                r: 1.,
                a: 1.,
                ..Default::default()
            },
            scale: Vector3 {
                x: 1.,
                y: 1.,
                z: 1.,
            },
            action: Marker::ADD as i32,
            lifetime: rosrust::Duration::from_seconds(100),
            ..Default::default()
        };
        // // Create string message
        // let msg = rosrust_msg::std_msgs::String {
        //     data: format!("hello world from rosrust {}", count),
        // };

        // Log event
        rosrust::ros_info!("Publishing: {:?}", msg);

        // Send string message to topic via publisher
        chatter_pub.send(msg).unwrap();

        if log_names {
            rosrust::ros_info!("Subscriber names: {:?}", chatter_pub.subscriber_names());
        }

        // Sleep to maintain 10Hz rate
        rate.sleep();

        count += 1;
    }
}
