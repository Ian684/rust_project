#[derive(Clone,Debug)]
enum Exercise29State{Idle,Running,Finished}

struct Exercise29Record{id:usize,label:String,score:i32,state:Exercise29State}

impl Exercise29Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_29_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+29)).sum()
}

fn main(){
    println!("Exercise 29: file_lines");
    let records=[
        Exercise29Record{id:1,label:"Ada".into(),score:98,state:Exercise29State::Finished},
        Exercise29Record{id:2,label:"Linus".into(),score:86,state:Exercise29State::Running},
        Exercise29Record{id:3,label:"Grace".into(),score:73,state:Exercise29State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_29_operation(&[12,7,19,3,25]));
}
