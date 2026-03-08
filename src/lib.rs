use wasm_bindgen::prelude::*;
use futures::channel::mpsc;
use futures::SinkExt; // for send()
use futures::StreamExt;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub struct SieveStream {
    receiver: mpsc::Receiver<JsValue>,
}

#[wasm_bindgen]
impl SieveStream {
    pub async fn next(&mut self) -> JsValue {
        self.receiver.next().await.unwrap_or(JsValue::NULL)
    }
}

#[wasm_bindgen]
pub fn start_sieve_stream(n: usize) -> SieveStream {
    // 1. Create a bounded channel (Backpressure!)
    // If JS doesn't "await stream.next()", Rust will stop at 32 events.
    let (mut tx, rx) = mpsc::channel(32);
    
    spawn_local(async move {
        let limit = n * n;
        let mut is_prime = vec![true; limit + 1];
        
        for p in 2..=limit {
            if is_prime[p] {
                // Send Prime
                let _ = tx.send(serialize_event(p, "prime")).await;
                yield_now().await; // Give JS time to render

                let mut i = p * p;
                while i <= limit {
                    if is_prime[i] {
                        is_prime[i] = false;
                        // Send Crossed
                        let _ = tx.send(serialize_event(i, "crossed")).await;
                        
                        // Yield to keep UI buttery smooth
                        yield_now().await;
                    }
                    i += p;
                }
            }
        }
    });

    SieveStream { receiver: rx }
}

// Conceptual "Yield": pauses Rust and lets the Browser's Event Loop run
async fn yield_now() {
    let promise = js_sys::Promise::resolve(&JsValue::NULL);
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn serialize_event(number: usize, status: &str) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"number".into(), &number.into()).unwrap();
    js_sys::Reflect::set(&obj, &"status".into(), &status.into()).unwrap();
    obj.into()
}
