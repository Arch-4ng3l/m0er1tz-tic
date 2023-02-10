use stylist::{Style};
use yew::prelude::*;
use yew::{ html, Html};


const STYLE_FILE: &str = include_str!("min.css");
const STYLE_RES: &str = include_str!("restart.css");
pub fn hello() {
    println!("Hello world");
}

pub enum Msg {
    Turn(usize), 
    Start,
}
pub struct Field {
    pub arr: [u8; 9],
    pub p: u8, 
    pub draw: [char; 9],
    pub count: u8,
    pub start: bool,
}
impl Component for Field {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Field::new()
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Turn(i) => {
                let temp = self.p; 
                self.turn(i);

                if self.check() {
                    self.p = temp;
                }
                
                true
            }

            Msg::Start => {
                self.start = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
       
        let stylesheet = Style::new(STYLE_FILE).unwrap();
        let stylesheet2 = Style::new(STYLE_RES).unwrap();
        let mut field = Vec::new();

        let link = ctx.link();
        if self.check() && self.start{
            html! {
                <div class={stylesheet2}>
                <p>{format!("player {} won", self.p)} <br/>

                <span class="fancy" >
                    {"refresh to play again"}
                </span>
                
                <form>
                <input class="button" type="button" onClick="history.go(0)" value="Refresh"/>
                </form>

                </p>
                </div>
            }
        }
        else if self.count >= 9 && self.start{
            html! {
                <div class={stylesheet2}>
                    <p>{"nobody won"} <br/>

                    <span class="fancy" >
                        {"refresh to play again"}
                    </span>
                        
                    
                    <form>
                    <input class="button" type="button" onClick="history.go(0)" value="Refresh"/>
                    </form>
                    </p>
                </div>
            }
        }
        else if self.start{

            for i in (0..9).step_by(3) {
                let l = i.clone();
                field.push(html!{
                    <div class="container">
                        
                        <button class="button" onclick={link.callback(move|_| Msg::Turn(l.clone()))}>{self.draw[i]}</button>
                        <button class="button" onclick={link.callback(move|_| Msg::Turn((l+1).clone()))}>{self.draw[i+1]}</button> 
                        <button class="button" onclick={link.callback(move|_| Msg::Turn((l+2).clone()))}>{self.draw[i+2]}</button> 
                        
                    </div> 
                } 
                );
            
            }
            
            html! {
                <div class={stylesheet}>
                    <table align="center">
                        {field}
                    </table>
                </div>
            }


        }

        else {
            
            html! {
                <div class={stylesheet}>
                    <div  class="container">
                    
                    <button align="center" class="button2" onclick={link.callback(move|_| Msg::Start)}>{"Start"}</button>
                    
                    </div>
                </div>

            }
        }
    }
}
impl Field {
    pub fn new() -> Field {
        Field {
            arr: [0; 9],  
            p: 1, 
            draw: [' '; 9],
            count: 0,
            start: false,
        }
    }
    pub fn turn(&mut self, turn:usize) {
        
        if self.arr[turn] == 0 {
            self.arr[turn] = self.p;
            if self.p == 1 {
                self.draw[turn] = 'X';
                self.p += 1;
            }
            else {
                self.draw[turn] = 'O';
                self.p -= 1;
            }

            self.count += 1;
        }
    }
    pub fn check(&self) -> bool {

        for i in (0..9).step_by(3) {
            if self.arr[i] != 0 && self.arr[i] == self.arr[i+1] &&  self.arr[i] == self.arr[i+2] {
                return true; 
            }
        }
        for i in 0..3 {
            if self.arr[i] != 0 && self.arr[i] ==self.arr[i+3] && self.arr[i] == self.arr[i+6] {
                return true; 
            }
        }

        if self.arr[0] != 0 && self.arr[0] == self.arr[4] && self.arr[0] == self.arr[8] {
            return true;
        }

        if self.arr[2] == self.arr[4] && self.arr[2] == self.arr[6] && self.arr[2] != 0 {
            return true;
        }
        
        
        return false;
    }
}