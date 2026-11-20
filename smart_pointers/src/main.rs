#[derive(Clone,Debug)]
enum Exercise21State{Idle,Running,Finished}

struct Exercise21Record{id:usize,label:String,score:i32,state:Exercise21State}

impl Exercise21Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_21_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+21)).sum()
}

fn main(){
    println!("Exercise 21: smart_pointers");
    let records=[
        Exercise21Record{id:1,label:"Ada".into(),score:98,state:Exercise21State::Finished},
        Exercise21Record{id:2,label:"Linus".into(),score:86,state:Exercise21State::Running},
        Exercise21Record{id:3,label:"Grace".into(),score:73,state:Exercise21State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_21_operation(&[12,7,19,3,25]));
}
