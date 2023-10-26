
trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText { chars: String }

#[derive(Clone)]
struct RepeatedText { chars: String }

#[derive(Clone)]
struct  JoinedText {
    chars : Vec<Box<dyn Text>>,
    seperator: PlainText
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText{ chars: text.to_string() }
    }
}

impl From<&str> for RepeatedText {
    fn from(text: &str) -> RepeatedText {
        RepeatedText{ chars: text.to_string() }
    }
}

impl Text for PlainText {
    fn value(&self) -> String { self.chars.clone() }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
}


impl Text for RepeatedText {

    fn value(&self) -> String { self.chars.clone() }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
}


impl Text for JoinedText {

    fn value(&self) -> String { 

        let mut txt = String::new();
        let tvec = &self.chars;
        let sperator = &self.seperator;

        for i in 0..tvec.len() {
            txt.push_str(&tvec[i].value());
            if i < (tvec.len() -1) {
                txt.push_str(&sperator.value());
            }
        }
        txt
    }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) { self }
}

impl AsRef<dyn Text> for RepeatedText {
    fn as_ref(&self) -> &(dyn Text + 'static) { self }
}


impl RepeatedText {
    
    fn with_parts(t : &dyn Text, num : i32)-> RepeatedText{

        let mut txt = String::new();
        for _ in 0..num {

            txt.push_str(&t.value());
        }

        
        RepeatedText { chars: txt }
    }
}

impl JoinedText {
    
    fn with_parts(t : &Vec<Box<dyn Text>>, seperator : &PlainText)-> JoinedText{
        
        JoinedText { chars: t.to_vec(), seperator: seperator.clone()  }
    }
}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}


fn main() {

    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    
    println!("Result of JoinedText : {}", t6.value());
}