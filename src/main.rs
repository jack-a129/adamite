use std::fs;
use std::vec::Vec;

const PATH :&str = "sample.txt";

struct Machine{
    token :Vec<String>,
}

fn atoi(x :&String) -> i32{
    match x.trim().parse(){
        Ok(n) => n,
        Err(_) => 0,
    }
}

impl Machine{
    fn if_vec(&self,token :&Vec<String>,i :usize) -> (Vec<String>,usize){
        let mut c :usize = 0;
        let mut e :usize = 0;
        let mut b :bool = false;
        let mut vector :Vec<String> = Vec::new();
        for v in token{
            if v == "begin"{
                break;
            }
            c+=1;
        }
        for q in token{
            if q == "fin"{
                if b {
                    b = false;
                }else{
                    break;
                }
            }else if q == "begin"{
                b = true;
            }
            e+=1;
        }
        let ifv = &token[i+(c+1)..e];
        println!("{:?}",ifv);
        for r in ifv{
            vector.push(r.to_string());
        }
        (vector,e-1)
    }

    fn core(&self,token :&Vec<String>,i :usize) -> i32{
        if token[i] == "=="{
            return if self.core(token,i+1) == self.core(token,i+2){0}else{1};
        }
        else if token[i] == "+"{
            return if token[i+1] == "+" || token[i+1] == "*" {self.core(token,i+1) + self.core(token,i+4)} else {self.core(token,i+1) + self.core(token,i+2)};
        }
        else if token[i] == "*"{
            return if token[i+1] == "+" || token[i+1] == "*" {self.core(token,i+1) * self.core(token,i+4)} else {self.core(token,i+1) * self.core(token,i+2)};
        }
        else if token[i] == "cout"{
            println!("{}",self.core(token,i+1));
            return 0;
        }
        else if token[i] == "if"{
            let (v,p) = self.if_vec(&token,i);
            if self.core(token,i+1) == 0{
                self.core(&v,0);
            }
            self.core(token,i+p);
            return 0;
        }
        else{
            return atoi(&token[i]);
        }
    }

    fn evel(&mut self){
        self.core(&self.token,0);
    }
}

fn open(x :&str) -> String{
    fs::read_to_string(x).unwrap()
}

fn split(s :&String) -> Vec<String>{
    let mut x:Vec<String> = Vec::new();
    let y = s.split(' ').collect::<Vec<_>>();
    for v in y{
        x.push(v.to_string());
    }
    println!("{:?}",x);
    x
}

fn main() {
    let mut s = open(PATH);
    s = s.replace("{"," begin ");
    s = s.replace("}"," fin");
    let mut x = Machine{
        token: split(&s),
    };
    x.evel();
}
