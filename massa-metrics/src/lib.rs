use lazy_static::lazy_static;
use prometheus::{register_int_counter_vec, register_int_gauge, IntCounterVec, IntGauge};

mod server;

// TODO load only if feature metrics is enabled
lazy_static! {
    static ref BLOCKS_COUNTER: IntGauge =
    register_int_gauge!("blocks_counter", "blocks len").unwrap();

        static ref OPERATIONS_COUNTER: IntGauge =
        register_int_gauge!("operations_counter", "operations counter").unwrap();

        static ref IN_CONNECTIONS: IntGauge =
        register_int_gauge!("in_connections", "in connections").unwrap();

        static ref OUT_CONNECTIONS: IntGauge =
        register_int_gauge!("out_connections", "out connections").unwrap();

        static ref ENDORSEMENTS_COUNTER: IntGauge =
        register_int_gauge!("endorsements_counter", "endorsements counter").unwrap();

        static ref CURRENT_SLOT: IntCounterVec = register_int_counter_vec!("current_slot", "help current slot", &["period","thread"]).unwrap();


        static ref FINAL_CURSOR_PERIOD: IntGauge = register_int_gauge!("final_cursor_period", "execution final cursor period").unwrap();
        static ref FINAL_CURSOR_THREAD: IntGauge = register_int_gauge!("final_cursor_thread", "execution final cursor thread").unwrap();


        static ref ACTIVE_CURSOR_PERIOD: IntGauge = register_int_gauge!("active_cursor_period", "execution active cursor period").unwrap();
        static ref ACTIVE_CURSOR_THREAD: IntGauge = register_int_gauge!("active_cursor_thread", "execution active cursor thread").unwrap();
    // static ref A_INT_GAUGE: IntGauge = register_int_gauge!("A_int_gauge", "foobar").unwrap();
}

pub fn start_metrics_server(addr: std::net::SocketAddr) {
    server::bind_metrics(addr);
}

pub fn inc_blocks_counter() {
    BLOCKS_COUNTER.inc();
}

pub fn dec_blocks_counter() {
    BLOCKS_COUNTER.dec();
}

pub fn inc_operations_counter() {
    OPERATIONS_COUNTER.inc();
}

pub fn dec_operations_counter() {
    OPERATIONS_COUNTER.dec();
}

pub fn inc_endorsements_counter() {
    ENDORSEMENTS_COUNTER.inc();
}

pub fn dec_endorsements_counter() {
    ENDORSEMENTS_COUNTER.dec();
}

pub fn set_connections(in_connections: usize, out_connections: usize) {
    IN_CONNECTIONS.set(in_connections as i64);
    OUT_CONNECTIONS.set(out_connections as i64);
}

pub fn set_active_cursor(period: u64, thread: u8) {
    ACTIVE_CURSOR_THREAD.set(thread as i64);
    ACTIVE_CURSOR_PERIOD.set(period as i64);
}

pub fn set_final_cursor(period: u64, thread: u8) {
    FINAL_CURSOR_THREAD.set(thread as i64);
    FINAL_CURSOR_PERIOD.set(period as i64);
}

// mod test {
//     use massa_channel::MassaChannel;

//     use crate::start_metrics_server;

//     #[tokio::test]
//     async fn test_channel_metrics() {
//         let addr = ([192, 168, 1, 183], 9898).into();

//         start_metrics_server(addr);
//         std::thread::sleep(std::time::Duration::from_millis(500));
//         let (sender, receiver) = MassaChannel::new("operations".to_string(), None);

//         let (sender2, receiver2) = MassaChannel::new("second_channel".to_string(), None);

//         sender2.send("hello_world".to_string()).unwrap();
//         let data = receiver2.recv().unwrap();
//         assert_eq!(data, "hello_world".to_string());

//         for i in 0..100 {
//             sender.send(i).unwrap();
//         }

//         for _i in 0..20 {
//             receiver.recv().unwrap();
//         }

//         assert_eq!(receiver.len(), 80);
//         std::thread::sleep(std::time::Duration::from_secs(5));
//         drop(sender2);
//         drop(receiver2);
//         std::thread::sleep(std::time::Duration::from_secs(100));
//     }

//     #[tokio::test]
//     async fn test_channel() {
//         let addr = ([192, 168, 1, 183], 9898).into();

//         start_metrics_server(addr);
//         std::thread::sleep(std::time::Duration::from_millis(500));

//         let (sender, receiver) = MassaChannel::new("test2".to_string(), None);

//         let cloned = receiver.clone();

//         sender.send("msg".to_string()).unwrap();

//         std::thread::spawn(move || {
//             dbg!("spawned");

//             loop {
//                 dbg!("loop");
//                 dbg!(receiver.recv().unwrap());
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//             }
//         });
//         std::thread::sleep(std::time::Duration::from_secs(2));
//         std::thread::spawn(move || {
//             std::thread::sleep(std::time::Duration::from_secs(5));

//             drop(sender);
//         });

//         std::thread::sleep(std::time::Duration::from_secs(20));
//     }
// }
