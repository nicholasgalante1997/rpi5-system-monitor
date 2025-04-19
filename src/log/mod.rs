use debugrs::RsDebugger;

pub fn logger() -> RsDebugger {
    RsDebugger::new("nick@raspberrypi:system-info:middleware".to_string())
}