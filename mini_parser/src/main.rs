#[derive(Clone,Debug)]
enum Exercise12State{Idle,Running,Finished}

struct Exercise12Record{id:usize,label:String,score:i32,state:Exercise12State}

impl Exercise12Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_12_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+12)).sum()
}

fn main(){
    println!("Exercise 12: mini_parser");
    let records=[
        Exercise12Record{id:1,label:"Ada".into(),score:98,state:Exercise12State::Finished},
        Exercise12Record{id:2,label:"Linus".into(),score:86,state:Exercise12State::Running},
        Exercise12Record{id:3,label:"Grace".into(),score:73,state:Exercise12State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_12_operation(&[12,7,19,3,25]));
}
