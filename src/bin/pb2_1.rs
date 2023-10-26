#[derive(Clone, Debug)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, i32),
    Joined(Vec<Box<Text>>, Box<Text>)
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, num ) => {
                let mut txt = String::new();
                for _ in 0..*num {

                    txt.push_str(&t.value());
                }

                txt
            },
            Text::Joined(tvec, sperator ) => {
                
                let mut txt = String::new();
                for i in 0..tvec.len() {
                    txt.push_str(&tvec[i].value());
                    if i < (tvec.len() -1) {
                        txt.push_str(&sperator.value());
                    }
                    
                }
                txt
            }
        }
    }
}
impl From<&Text> for Box<Text> {

    fn from(t: &Text) -> Box<Text> { Box::new(t.clone()) }
}


impl AsRef<Text> for Text {

    fn as_ref(&self) -> &Text { &self }
}



#[test]
fn test_text_composition() {

    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}

fn main() {

    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    
    println!("Join ptn : {:?}", ptn);
    println!("Result : {}", t6.value());
}