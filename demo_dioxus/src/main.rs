
#![allow(non_snake_case)]
use dioxus::prelude::*;


fn main() {

    dioxus_desktop::launch(App);
    fn App(cx: Scope) -> Element {

 let mut count = use_state(cx, || 0);
 let mut i =0;


        cx.render(rsx! {
            div {
              h1{  "Hello, world!"
              }
                    button{
                        onclick: move |_| {count.set(0)},
                        "puxx"
                    
                    }

                                button{
               onclick: move |event| {
                    count+=1;
                    i+=1;
                   println!("Clicked! Event: {event:?}")},
                "hello"
                }

                
                p{
                   " {count}"
                }
               for num in (0..*{count.get()}){
                div{"hello"}
               } 
            
            }
        })
}
}
